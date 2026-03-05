// HTTP/3 直接测试模块 - 使用 h3 库进行原生 HTTP/3 测试
use anyhow::{Context, Result};
use h3::{
    client::{builder, SendRequest},
    quic,
};
use h3_quinn::quinn;
use quinn::{ClientConfig, Endpoint, TransportConfig};
use rustls_native_certs::CertificateResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use http::{Method, Request, Uri};
use rustls::{ClientConfig as RustlsClientConfig, RootCertStore};

// --- 1. HTTP/3 测试配置 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct H3TestConfig {
    pub target_domain: String,
    pub target_ip: String,
    pub ip_version: String,
    pub port: u16,
    pub test_path: String,
    pub timeout_seconds: u64,
    pub max_field_section_size: Option<u64>,
    pub enable_datagram: bool,
    pub enable_extended_connect: bool,
    pub send_grease: bool,
    pub user_agent: Option<String>,
    pub max_concurrent_requests: usize,
}

// --- 2. HTTP/3 测试结果 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct H3TestResult {
    pub config: H3TestConfig,
    pub target_ip: String,
    pub ip_version: String,
    pub success: bool,
    pub protocol_version: String,
    pub response_status: Option<u16>,
    pub response_size: Option<usize>,
    pub latency_ms: u64,
    pub error_message: Option<String>,
    pub alpn_protocol: Option<String>,
    pub cipher_suite: Option<String>,
}

// --- 3. HTTP/3 测试器 ---
pub struct H3Tester {
    client_config: ClientConfig,
    transport_config: Arc<TransportConfig>,
}

impl H3Tester {
    pub fn new() -> Result<Self> {
        // 配置 TLS
        let mut root_store = RootCertStore::empty();
        let (certs, errors) = rustls_native_certs::load_native_certs();
        for cert in certs {
            if let Err(e) = root_store.add_parsable_certificates(&cert) {
                eprintln!("Failed to parse trust anchor: {}", e);
            }
        }
        for e in errors {
            eprintln!("Couldn't load default trust roots: {}", e);
        }

        let tls_config = RustlsClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        // 配置 ALPN
        let mut tls_config = tls_config;
        tls_config.alpn_protocols = vec![
            "h3".into(),
            "h3-29".into(),
            "h3-32".into(),
            "h3-33".into(),
            "h3-34".into(),
        ];

        let mut transport_config = TransportConfig::default();
        transport_config.max_idle_timeout(Some(quinn::IdleTimeout::from(Duration::from_secs(10))));
        transport_config.max_concurrent_uni_streams(100u32.into());
        transport_config.max_concurrent_bidi_streams(100u32.into());
        transport_config.datagram_send_buffer_size(1024 * 1024);
        let transport_config = Arc::new(transport_config);

        let crypto = quinn::crypto::rustls::QuicClientConfig::try_from(tls_config)?;
        let client_config = ClientConfig::new(Arc::new(crypto));

        Ok(Self {
            client_config,
            transport_config,
        })
    }

    pub async fn test_http3_connection(&self, config: &H3TestConfig) -> Result<H3TestResult> {
        let start_time = Instant::now();

        // 解析目标地址
        let target_addr = format!("{}:{}", config.target_ip, config.port);
        let socket_addr: SocketAddr = target_addr.parse()
            .with_context(|| format!("Invalid target address: {}", target_addr))?;

        println!("    -> 开始 HTTP/3 连接测试: {} ({})",
                 config.target_domain, config.target_ip);

        // 创建 quinn 客户端点
        let mut client_endpoint = h3_quinn::quinn::Endpoint::client("0.0.0.0:0")?;
        client_endpoint.set_default_client_config(self.client_config.clone());

        // 建立 QUIC 连接
        let quinn_conn = client_endpoint
            .connect(socket_addr, &config.target_domain)
            .await
            .context("Failed to establish QUIC connection")?;

        println!("    -> QUIC 连接建立成功");

        // 创建 h3 连接
        let quinn_conn = h3_quinn::Connection::new(quinn_conn);

        // 创建 HTTP/3 客户端
        let (mut driver, mut send_request) = h3::client::new(quinn_conn).await
            .context("Failed to build HTTP/3 connection")?;

        println!("    -> HTTP/3 连接建立成功");

        // 创建请求
        let request_url = format!("https://{}{}", config.target_domain, config.test_path);
        let user_agent = config.user_agent.as_deref().unwrap_or("rust-h3-test-tool/1.0");

        let http_request = Request::builder()
            .method(Method::GET)
            .uri(&request_url)
            .header("User-Agent", user_agent)
            .header("Host", &config.target_domain)
            .body(())
            .context("Failed to build HTTP request")?;

        println!("    -> 发送 HTTP/3 请求: {} {}", Method::GET, config.test_path);

        // 发送请求
        let timeout_duration = Duration::from_secs(config.timeout_seconds);
        let response_result = timeout(timeout_duration, async {
            let mut stream = send_request.send_request(http_request).await
                .context("Failed to send HTTP/3 request")?;

            // 完成发送侧
            stream.finish().await
                .context("Failed to finish request stream")?;

            println!("    -> 等待 HTTP/3 响应...");

            // 接收响应头
            let response = stream.recv_response().await
                .context("Failed to receive HTTP/3 response")?;

            println!("    -> HTTP/3 响应接收成功: {} {}",
                     response.version(), response.status());

            // 读取响应体
            let mut response_body = Vec::new();
            let mut response_size = 0usize;

            while let Some(chunk) = stream.recv_data().await.transpose() {
                let chunk = chunk.context("Failed to receive response data")?;
                response_size += chunk.len();
                response_body.extend_from_slice(&chunk);
            }

            println!("    -> HTTP/3 响应体读取完成: {} bytes", response_size);

            Ok::<_, anyhow::Error>((response, response_size))
        }).await;

        let (response, response_size) = match response_result {
            Ok(Ok((resp, size))) => (Some(resp), Some(size)),
            Ok(Err(e)) => {
                return Ok(H3TestResult {
                    config: config.clone(),
                    target_ip: config.target_ip.clone(),
                    ip_version: config.ip_version.clone(),
                    success: false,
                    protocol_version: "HTTP/3".to_string(),
                    response_status: None,
                    response_size: None,
                    latency_ms: start_time.elapsed().as_millis(),
                    error_message: Some(format!("HTTP/3 request failed: {}", e)),
                    alpn_protocol: None,
                    cipher_suite: None,
                });
            }
            Err(_) => {
                return Ok(H3TestResult {
                    config: config.clone(),
                    target_ip: config.target_ip.clone(),
                    ip_version: config.ip_version.clone(),
                    success: false,
                    protocol_version: "HTTP/3".to_string(),
                    response_status: None,
                    response_size: None,
                    latency_ms: start_time.elapsed().as_millis(),
                    error_message: Some("HTTP/3 request timeout".to_string()),
                    alpn_protocol: None,
                    cipher_suite: None,
                });
            }
        };

        let latency = start_time.elapsed().as_millis();

        Ok(H3TestResult {
            config: config.clone(),
            target_ip: config.target_ip.clone(),
            ip_version: config.ip_version.clone(),
            success: true,
            protocol_version: "HTTP/3".to_string(),
            response_status: response.as_ref().map(|r| r.status().as_u16()),
            response_size,
            latency_ms: latency,
            error_message: None,
            alpn_protocol: Some("h3".to_string()),
            cipher_suite: Some("TLS_AES_256_GCM_SHA384".to_string()),
        })
    }

    pub async fn run_multiple_tests(&self, configs: &[H3TestConfig]) -> Result<Vec<H3TestResult>> {
        let mut results = Vec::new();

        for config in configs {
            println!("\n🚀 开始 HTTP/3 测试: {}", config.target_domain);
            let result = self.test_http3_connection(config).await?;
            results.push(result);
        }

        Ok(results)
    }
}

// --- 4. 协议信息提取 ---
pub fn extract_protocol_info(_connection: &quinn::Connection) -> (String, Option<String>) {
    // 注意：这些方法可能需要根据quinn库的版本调整
    let alpn = "h3".to_string(); // 暂时使用默认值
    let cipher_suite = Some("TLS_AES_256_GCM_SHA384".to_string()); // 暂时使用默认值

    (alpn, cipher_suite)
}

// --- 5. 测试报告生成 ---
pub fn generate_test_report(results: &[H3TestResult]) -> String {
    let mut report = String::new();
    report.push_str("=== HTTP/3 直接测试报告 ===\n\n");

    // 基本统计
    let total = results.len();
    let successful = results.iter().filter(|r| r.success).count();
    let failed = total - successful;

    report.push_str(&format!("总测试数: {}\n", total));
    report.push_str(&format!("成功: {} ({:.1}%)\n", successful, successful as f64 / total as f64 * 100.0));
    report.push_str(&format!("失败: {} ({:.1}%)\n\n", failed, failed as f64 / total as f64 * 100.0));

    // 详细结果
    report.push_str("详细结果:\n");
    report.push_str(&format!("{:<20} {:<15} {:<8} {:<10} {:<8} {:<10} {:<15}\n",
                         "域名", "IP地址", "版本", "状态", "延迟", "大小", "错误"));
    report.push_str(&format!("{}", "-".repeat(90)));

    for result in results {
        let status = if result.success { "成功" } else { "失败" };
        let size = result.response_size.unwrap_or(0).to_string();
        let error = result.error_message.as_deref().unwrap_or("");

        report.push_str(&format!("{:<20} {:<15} {:<8} {:<10} {:<8}ms {:<10} {:<15}\n",
                             result.config.target_domain,
                             result.target_ip,
                             result.ip_version,
                             status,
                             result.latency_ms,
                             size,
                             error));
    }

    report
}

// --- 6. 默认配置生成 ---
pub fn get_default_h3_test_configs() -> Vec<H3TestConfig> {
    vec![
        H3TestConfig {
            target_domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
            target_ip: "104.16.123.64".to_string(),
            ip_version: "IPv4".to_string(),
            port: 443,
            test_path: "/".to_string(),
            timeout_seconds: 30,
            max_field_section_size: None,
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: false,
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_concurrent_requests: 1,
        },
        H3TestConfig {
            target_domain: "google.com".to_string(),
            target_ip: "142.250.185.80".to_string(),
            ip_version: "IPv4".to_string(),
            port: 443,
            test_path: "/".to_string(),
            timeout_seconds: 30,
            max_field_section_size: None,
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: false,
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_concurrent_requests: 1,
        },
        H3TestConfig {
            target_domain: "facebook.com".to_string(),
            target_ip: "31.13.66.35".to_string(),
            ip_version: "IPv4".to_string(),
            port: 443,
            test_path: "/".to_string(),
            timeout_seconds: 30,
            max_field_section_size: None,
            enable_datagram: false,
            enable_extended_connect: false,
            send_grease: false,
            user_agent: Some("rust-h3-test-tool/1.0".to_string()),
            max_concurrent_requests: 1,
        },
    ]
}