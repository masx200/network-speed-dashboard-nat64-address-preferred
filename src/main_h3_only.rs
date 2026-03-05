// 纯 HTTP/3 测试工具 - 基于 h3 库
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use bytes::Buf;
use clap::{Arg, Command};
use h3_quinn::quinn;
use reqwest::Client;
use rustls_native_certs::load_native_certs;
use std::collections::HashSet;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{error, info};
use trust_dns_proto::op::{Message, Query};
use trust_dns_proto::rr::{Name, RecordType};
use trust_dns_proto::serialize::binary::BinEncodable;

// 错误转换辅助函数
fn h3_error_to_anyhow(e: impl std::error::Error + Send + Sync + 'static) -> anyhow::Error {
    anyhow!("{:?}", e)
}

static ALPN: &[u8] = b"h3";

#[derive(Debug, Clone)]
pub struct H3TestConfig {
    pub domain: String,
    pub port: u16,
    pub path: String,
    pub doh_server: String,
    pub timeout_seconds: u64,
    pub prefer_ipv6: bool,
}

impl Default for H3TestConfig {
    fn default() -> Self {
        Self {
            domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
            port: 443,
            path: "/".to_string(),
            doh_server: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/".to_string(),
            timeout_seconds: 10,
            prefer_ipv6: false,
        }
    }
}

// RFC 8484 DNS over HTTPS 查询函数
async fn query_dns_over_https(
    client: &Client,
    domain: &str,
    record_type: RecordType,
    doh_server: &str,
) -> Result<Vec<IpAddr>> {
    // 创建 DNS 查询
    let name = Name::from_ascii(domain).context(format!("无效的域名: {}", domain))?;
    let query = Query::query(name, record_type);

    // 创建 DNS 消息
    let mut message = Message::new();
    message.set_id(0); // RFC 8484 建议使用 ID 为 0 以提高缓存效率
    message.set_recursion_desired(true);
    message.add_query(query);

    // 序列化 DNS 查询
    let mut request_bytes = Vec::new();
    {
        let mut encoder = trust_dns_proto::serialize::binary::BinEncoder::new(&mut request_bytes);
        message.emit(&mut encoder).context("序列化 DNS 查询失败")?;
    }

    // 使用 base64url 编码（不包含填充）
    let encoded_query = general_purpose::URL_SAFE_NO_PAD.encode(&request_bytes);

    // 构建 DoH 请求 URL
    let url = format!("{}?dns={}", doh_server, encoded_query);

    info!("📡 正在通过 DoH 查询: {} ({})", domain, record_type);

    // 发送 HTTPS GET 请求
    let response = client
        .get(&url)
        .header("Accept", "application/dns-message")
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .context("发送 DoH 请求失败")?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(anyhow!("DoH 服务器返回错误状态: {}", response.status()));
    }

    // 获取响应体
    let response_bytes = response.bytes().await.context("读取响应体失败")?;

    // 解析 DNS 响应
    let dns_response = Message::from_vec(&response_bytes).context("解析 DNS 响应失败")?;

    // 提取 IP 地址
    let mut ip_addresses = Vec::new();
    let answers = dns_response.answers();

    if !answers.is_empty() {
        for record in answers {
            if record.record_type() == record_type {
                if let Some(rdata) = record.data() {
                    match record.record_type() {
                        RecordType::A => {
                            if let trust_dns_proto::rr::RData::A(ipv4) = rdata {
                                ip_addresses.push(IpAddr::V4(*ipv4));
                            }
                        }
                        RecordType::AAAA => {
                            if let trust_dns_proto::rr::RData::AAAA(ipv6) = rdata {
                                ip_addresses.push(IpAddr::V6(*ipv6));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(ip_addresses)
}

pub struct H3Tester {
    config: H3TestConfig,
}

impl H3Tester {
    pub fn new(config: H3TestConfig) -> Self {
        Self { config }
    }

    pub async fn test_connection(&self) -> Result<()> {
        info!(
            "🚀 开始 HTTP/3 测试: {}:{}",
            self.config.domain, self.config.port
        );
        info!("🔧 使用 DoH 服务器: {}", self.config.doh_server);

        // 1. 创建 HTTP 客户端用于 DoH 查询
        let client = Client::builder()
            .user_agent("rust-http3-test-tool/1.0")
            .timeout(std::time::Duration::from_secs(self.config.timeout_seconds))
            .build()
            .context("创建 HTTP 客户端失败")?;

        // 2. 使用 RFC 8484 DoH 查询域名
        let mut all_ips = HashSet::new();

        // 查询 A 记录 (IPv4)
        match query_dns_over_https(
            &client,
            &self.config.domain,
            RecordType::A,
            &self.config.doh_server,
        )
        .await
        {
            Ok(ipv4_addresses) => {
                info!("✅ 找到 {} 个 IPv4 地址", ipv4_addresses.len());
                for ip in &ipv4_addresses {
                    info!("  📍 IPv4: {}", ip);
                    all_ips.insert(*ip);
                }
            }
            Err(e) => {
                error!("❌ IPv4 查询失败: {:?}", e);
            }
        }

        // 查询 AAAA 记录 (IPv6)
        match query_dns_over_https(
            &client,
            &self.config.domain,
            RecordType::AAAA,
            &self.config.doh_server,
        )
        .await
        {
            Ok(ipv6_addresses) => {
                info!("✅ 找到 {} 个 IPv6 地址", ipv6_addresses.len());
                for ip in &ipv6_addresses {
                    info!("  📍 IPv6: {}", ip);
                    all_ips.insert(*ip);
                }
            }
            Err(e) => {
                error!("❌ IPv6 查询失败: {:?}", e);
            }
        }

        if all_ips.is_empty() {
            return Err(anyhow!("未找到任何 IP 地址"));
        }

        // 3. 过滤 IP 地址（如果设置了 prefer_ipv6）
        let mut ips: Vec<IpAddr> = all_ips.into_iter().collect();
        ips.sort_by_key(|ip| ip.is_ipv6());

        if self.config.prefer_ipv6 {
            ips.reverse();
        }

        let ip_count = ips.len();
        info!("✅ DNS 解析完成，共找到 {} 个 IP 地址", ip_count);

        // 4. 为每个 IP 地址测试 HTTP/3 连接
        let mut success_count = 0;
        for (index, ip) in ips.iter().enumerate() {
            info!(
                "\n🔄 正在测试第 {}/{} 个 IP: {}:{}",
                index + 1,
                ip_count,
                ip,
                self.config.port
            );

            if let Err(e) = self.test_single_connection(*ip).await {
                error!("❌ IP {} 测试失败: {:?}", ip, e);
            } else {
                success_count += 1;
                info!("✅ IP {} 测试成功", ip);
            }
        }

        info!(
            "\n📊 测试总结: {}/{} 个 IP 测试成功",
            success_count, ip_count
        );

        Ok(())
    }

    pub async fn test_single_connection(&self, ip: IpAddr) -> Result<()> {
        // 1. 加载证书
        let mut roots = rustls::RootCertStore::empty();
        match load_native_certs() {
            Ok(certs) => {
                for cert in certs {
                    if let Err(e) = roots.add(cert) {
                        error!("解析信任锚失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("加载系统证书失败: {}", e);
            }
        }

        // 3. 配置 TLS
        let mut tls_config = rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();

        tls_config.enable_early_data = true;
        tls_config.alpn_protocols = vec![ALPN.into()];

        // 4. 创建 QUIC 端点
        let mut client_endpoint = quinn::Endpoint::client("[::]:0".parse().unwrap())
            .context("创建 QUIC 客户端端点失败")?;

        let client_config = quinn::ClientConfig::new(Arc::new(
            quinn::crypto::rustls::QuicClientConfig::try_from(tls_config)
                .context("创建 QUIC TLS 配置失败")?,
        ));
        client_endpoint.set_default_client_config(client_config);

        // 5. 建立连接
        let socket_addr = std::net::SocketAddr::new(ip, self.config.port);
        let start = std::time::Instant::now();
        let conn = client_endpoint
            .connect(socket_addr, &self.config.domain)
            .context(format!("连接建立失败: {}", socket_addr))?
            .await
            .context(format!("连接超时或被拒绝: {}", socket_addr))?;

        let connect_time = start.elapsed();
        info!("✅ QUIC 连接建立成功，耗时: {:?}", connect_time);

        // 6. 创建 H3 客户端
        let quinn_conn = h3_quinn::Connection::new(conn);

        let (driver, mut send_request) = h3::client::new(quinn_conn)
            .await
            .context("创建 H3 客户端失败")?;

        // 7. 发送请求
        let uri = format!("https://{}{}", self.config.domain, self.config.path);
        info!("📡 发送 HTTP/3 请求: {}", uri);

        let req = http::Request::builder()
            .uri(uri)
            .header("Host", &self.config.domain)
            .header("User-Agent", "rust-http3-test-tool/1.0")
            .body(())
            .map_err(|e| anyhow!("构建请求失败: {}", e))?;

        let mut stream = send_request
            .send_request(req)
            .await
            .map_err(h3_error_to_anyhow)?;

        stream.finish().await.map_err(h3_error_to_anyhow)?;

        let resp = stream.recv_response().await.map_err(h3_error_to_anyhow)?;

        let status = resp.status();
        let version = resp.version();

        info!("📨 收到响应: {} {:?}", status, version);
        info!("📋 响应头: {:#?}", resp.headers());

        // 读取响应体
        let mut total_bytes = 0;
        while let Some(chunk) = stream.recv_data().await.map_err(h3_error_to_anyhow)? {
            total_bytes += chunk.remaining();
        }

        info!(
            "✅ HTTP/3 测试成功！状态码: {}, 响应大小: {} 字节",
            status, total_bytes
        );

        // 清理资源
        drop(client_endpoint);

        Ok(())
    }
}

// --- 主程序入口 ---
#[tokio::main]
pub async fn run() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::INFO)
        .init();

    let matches = Command::new("rust-http3-test-tool")
        .version("1.0.0")
        .about("HTTP/3 客户端测试工具 - 基于 h3 库，支持 RFC 8484 DoH")
        .arg(
            Arg::new("domain")
                .short('d')
                .long("domain")
                .value_name("DOMAIN")
                .help("测试域名")
                .default_value("local-aria2-webui.masx200.ddns-ip.net"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("端口号")
                .default_value("443"),
        )
        .arg(
            Arg::new("path")
                .short('t')
                .long("path")
                .value_name("PATH")
                .help("请求路径")
                .default_value("/"),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("超时时间（秒）")
                .default_value("10"),
        )
        .arg(
            Arg::new("doh-server")
                .long("doh-server")
                .value_name("URL")
                .help("DNS over HTTPS 服务器 URL")
                .default_value("https://deno-dns-over-https-server.g18uibxgnb.de5.net/"),
        )
        .arg(
            Arg::new("prefer-ipv6")
                .long("prefer-ipv6")
                .help("优先使用 IPv6 地址")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let domain = matches.get_one::<String>("domain").unwrap().clone();
    let port = matches
        .get_one::<String>("port")
        .unwrap()
        .parse::<u16>()
        .unwrap_or(443);
    let path = matches.get_one::<String>("path").unwrap().clone();
    let timeout = matches
        .get_one::<String>("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(10);
    let doh_server = matches.get_one::<String>("doh-server").unwrap().clone();
    let prefer_ipv6 = matches.get_flag("prefer-ipv6");

    let config = H3TestConfig {
        domain,
        port,
        path,
        doh_server,
        timeout_seconds: timeout,
        prefer_ipv6,
    };

    let tester = H3Tester::new(config);

    if let Err(e) = tester.test_connection().await {
        error!("❌ 测试失败: {:?}", e);
        std::process::exit(1);
    }

    println!("\n✅ HTTP/3 测试完成！");
    Ok(())
}
