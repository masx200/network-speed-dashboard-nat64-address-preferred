// HTTP/3 集成测试模块 - 整合 DNS 解析和 HTTP/3 连接测试
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Instant;

// 重用现有的 DNS 解析模块
use crate::http3_test::resolve_domain_with_rfc8484;

// HTTP/3 输入任务 - 重新定义为公共结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTask {
    pub doh_resolve_domain: String,
    pub test_sni_host: String,
    pub test_host_header: String,
    pub doh_url: String,
    pub port: u16,
    pub prefer_ipv6: Option<bool>,
    pub resolve_mode: String,
    pub direct_ips: Option<Vec<String>>,
}

// HTTP/3 测试结果 - 重新定义为公共结构体
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestResult {
    pub domain_used: String,
    pub target_ip: String,
    pub ip_version: String,
    pub sni_host: String,
    pub host_header: String,
    pub success: bool,
    pub status_code: Option<u16>,
    pub protocol: String,
    pub latency_ms: Option<u64>,
    pub server_header: Option<String>,
    pub error_msg: Option<String>,
    pub dns_source: String,
}

// HTTP/3 测试配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct H3IntegrationTest {
    pub input_task: InputTask,
    pub use_native_h3: bool,
    pub enable_fallback: bool,
    pub timeout_seconds: u64,
    pub max_field_section_size: Option<u64>,
}

impl Default for H3IntegrationTest {
    fn default() -> Self {
        Self {
            input_task: InputTask {
                doh_resolve_domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_sni_host: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_host_header: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/cdn-cgi/trace".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        }
    }
}

// --- 1. HTTP/3 集成测试结果 ---
#[derive(Debug, Clone, Serialize)]
pub struct H3IntegrationResult {
    pub input_task: InputTask,
    pub target_ip: String,
    pub ip_version: String,
    pub test_method: String,
    pub success: bool,
    pub status_code: Option<u16>,
    pub protocol_detected: String,
    pub latency_ms: Option<u64>,
    pub response_size: Option<usize>,
    pub server_header: Option<String>,
    pub alpn_protocol: Option<String>,
    pub error_message: Option<String>,
    pub dns_source: String,
    pub test_path: String,
    pub timestamp: String,
}

impl H3IntegrationResult {
    pub fn success(
        task: &InputTask,
        ip: &str,
        version: &str,
        method: &str,
        dns_source: String,
    ) -> Self {
        Self {
            input_task: task.clone(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            test_method: method.to_string(),
            success: true,
            status_code: Some(200),
            protocol_detected: "HTTP/3".to_string(),
            latency_ms: Some(0),
            response_size: Some(0),
            server_header: None,
            alpn_protocol: Some("h3".to_string()),
            error_message: None,
            dns_source,
            test_path: task.test_path.as_deref().unwrap_or("/").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn failure(
        task: &InputTask,
        ip: &str,
        version: &str,
        method: &str,
        dns_source: String,
        error: String,
    ) -> Self {
        Self {
            input_task: task.clone(),
            target_ip: ip.to_string(),
            ip_version: version.to_string(),
            test_method: method.to_string(),
            success: false,
            status_code: None,
            protocol_detected: "HTTP/3".to_string(),
            latency_ms: None,
            response_size: None,
            server_header: None,
            alpn_protocol: None,
            error_message: Some(error),
            dns_source,
            test_path: task.test_path.as_deref().unwrap_or("/").to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

// --- 2. 协议检测和回退逻辑 ---
pub async fn test_http3_with_fallback(
    client: &reqwest::Client,
    task: &InputTask,
    ip: IpAddr,
    dns_source: String,
) -> Result<H3IntegrationResult> {
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
    let test_path = task.test_path.as_deref().unwrap_or("/");
    let url = format!("https://{}:{}{}", task.test_sni_host, task.port, test_path);

    println!("    -> 开始 HTTP/3 协议检测: {} ({})", url, ip);

    // 首先尝试 HTTP/3
    match test_http3_negotiation(client, task, &ip, &url).await {
        Ok(result) => {
            println!(
                "    -> HTTP/3 成功: {} - {}ms",
                result.protocol_detected,
                result.latency_ms.unwrap_or(0)
            );
            Ok(result)
        }
        Err(e) => {
            println!("    -> HTTP/3 失败: {}, 尝试 HTTP/2 回退", e);

            // 尝试 HTTP/2 回退
            match test_http2_fallback(client, task, &ip, &url).await {
                Ok(result) => {
                    println!(
                        "    -> HTTP/2 回退成功: {} - {}ms",
                        result.protocol_detected,
                        result.latency_ms.unwrap_or(0)
                    );
                    Ok(result)
                }
                Err(e2) => {
                    println!("    -> HTTP/2 回退失败: {}", e2);
                    Ok(H3IntegrationResult::failure(
                        task,
                        &ip.to_string(),
                        ip_ver,
                        "reqwest",
                        dns_source,
                        format!("All protocols failed: HTTP/3({}), HTTP/2({})", e, e2),
                    ))
                }
            }
        }
    }
}

// --- 3. HTTP/3 协议协商 ---
async fn test_http3_negotiation(
    client: &reqwest::Client,
    task: &InputTask,
    ip: &IpAddr,
    url: &str,
) -> Result<H3IntegrationResult> {
    let ip_str = ip.to_string();
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
    let dns_source = format!("DoH ({})", task.doh_url);

    let start_time = Instant::now();

    let response = client
        .get(url)
        .header("Host", &task.test_host_header)
        .header("Accept", "text/plain,application/json,*/*")
        .header("User-Agent", "rust-http3-test-tool/1.0")
        .header("Alt-Svc", "h3=\":443\"")
        .header("Connection", "keep-alive")
        .send()
        .await
        .context("HTTP/3 negotiation request failed")?;

    let latency = start_time.elapsed().as_millis() as u64;
    let status = response.status();
    let server = response
        .headers()
        .get("server")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // 检测协议版本
    let protocol = match response.version() {
        reqwest::Version::HTTP_11 => "http/1.1",
        reqwest::Version::HTTP_2 => "h2",
        reqwest::Version::HTTP_3 => "h3",
        _ => {
            // 通过响应头判断协议
            if response.headers().get("alt-svc").is_some() {
                "h3-detected"
            } else if response.headers().get(":status").is_some() {
                "h2-detected"
            } else {
                "unknown"
            }
        }
    };

    // 检查 HTTP/3 相关响应头
    let h3_indicators = vec![
        ("alt-svc", response.headers().get("alt-svc").is_some()),
        ("h3", response.headers().get("h3").is_some()),
        (
            "x-http3-connection",
            response.headers().get("x-http3-connection").is_some(),
        ),
    ];

    println!("    -> HTTP/3 协商结果: {} - 延迟: {}ms", protocol, latency);
    for (header, present) in h3_indicators {
        println!("    -> {} 头: {}", header, present);
    }

    let response_size = match response.content_length() {
        Some(len) => len as usize,
        None => {
            // 尝试读取部分响应体来估算大小
            match response.bytes().await {
                Ok(bytes) => bytes.len(),
                Err(_) => 0,
            }
        }
    };

    let alpn_protocol = response
        .headers()
        .get("alt-svc")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| {
            if s.contains("h3") {
                Some("h3".to_string())
            } else if s.contains("h2") {
                Some("h2".to_string())
            } else {
                None
            }
        });

    let mut result = H3IntegrationResult::success(task, &ip_str, ip_ver, "reqwest", dns_source);
    result.status_code = Some(status.as_u16());
    result.protocol_detected = protocol.to_string();
    result.latency_ms = Some(latency);
    result.response_size = Some(response_size);
    result.server_header = server;
    result.alpn_protocol = alpn_protocol;

    if status.as_u16() >= 400 {
        result.success = false;
        result.error_message = Some(format!("HTTP error: {}", status));
    }

    Ok(result)
}

// --- 4. HTTP/2 回退 ---
async fn test_http2_fallback(
    client: &reqwest::Client,
    task: &InputTask,
    ip: &IpAddr,
    url: &str,
) -> Result<H3IntegrationResult> {
    let ip_str = ip.to_string();
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
    let dns_source = format!("DoH ({})", task.doh_url);

    let start_time = Instant::now();

    let response = client
        .get(url)
        .header("Host", &task.test_host_header)
        .header("Accept", "text/plain,application/json,*/*")
        .header("User-Agent", "rust-http3-test-tool/1.0")
        .header("Connection", "keep-alive")
        .version(reqwest::Version::HTTP_2)
        .send()
        .await
        .context("HTTP/2 fallback request failed")?;

    let latency = start_time.elapsed().as_millis() as u64;
    let status = response.status();
    let server = response
        .headers()
        .get("server")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let response_size = match response.content_length() {
        Some(len) => len as usize,
        None => match response.bytes().await {
            Ok(bytes) => bytes.len(),
            Err(_) => 0,
        },
    };

    let mut result =
        H3IntegrationResult::success(task, &ip_str, ip_ver, "reqwest-fallback", dns_source);
    result.status_code = Some(status.as_u16());
    result.protocol_detected = "h2".to_string();
    result.latency_ms = Some(latency);
    result.response_size = Some(response_size);
    result.server_header = server;
    result.alpn_protocol = Some("h2".to_string());

    if status.as_u16() >= 400 {
        result.success = false;
        result.error_message = Some(format!("HTTP/2 error: {}", status));
    }

    Ok(result)
}

// --- 5. 主要的 HTTP/3 集成测试 ---
#[tokio::main]
pub async fn run_http3_integration_tests() -> Result<()> {
    println!("🚀 HTTP/3 集成测试开始");
    println!("================================");

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .user_agent("rust-http3-test-tool/1.0")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Alt-Svc", "h3=\":443\"".parse().unwrap());
            headers.insert("Connection", "keep-alive".parse().unwrap());
            headers
        })
        .build()
        .expect("Failed to create HTTP client");

    // 默认测试配置
    let test_configs = vec![
        H3IntegrationTest {
            input_task: InputTask {
                doh_resolve_domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_sni_host: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_host_header: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/cdn-cgi/trace".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        },
        H3IntegrationTest {
            input_task: InputTask {
                doh_resolve_domain: "google.com".to_string(),
                test_sni_host: "google.com".to_string(),
                test_host_header: "google.com".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        },
    ];

    let mut all_results = Vec::new();
    let mut futures = Vec::new();

    for test_config in test_configs {
        println!(
            "\n>> 正在测试 {} (模式: {})...",
            test_config.input_task.doh_resolve_domain, test_config.input_task.resolve_mode
        );

        match resolve_domain_with_rfc8484(&client, &test_config.input_task).await {
            Ok(ips) => {
                if ips.is_empty() {
                    println!("    [!] 未找到IP地址");
                    continue;
                }
                println!("    -> 解析成功，获取到 {} 个IP地址: {:?}", ips.len(), ips);

                for ip in ips {
                    if let Some(prefer_ipv6) = test_config.input_task.prefer_ipv6 {
                        if prefer_ipv6 != ip.is_ipv6() {
                            continue;
                        }
                    }

                    let task_clone = test_config.input_task.clone();
                    let client_clone = client.clone();
                    let dns_source = if test_config.input_task.resolve_mode == "direct" {
                        "Direct Input".to_string()
                    } else {
                        format!("DoH ({})", test_config.input_task.doh_url)
                    };

                    futures.push(tokio::spawn(async move {
                        match test_http3_with_fallback(&client_clone, &task_clone, ip, dns_source)
                            .await
                        {
                            Ok(result) => result,
                            Err(e) => {
                                let ip_str = ip.to_string();
                                let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
                                H3IntegrationResult::failure(
                                    &task_clone,
                                    &ip_str,
                                    ip_ver,
                                    "reqwest",
                                    dns_source,
                                    format!("Test failed: {}", e),
                                )
                            }
                        }
                    }));
                }
            }
            Err(e) => {
                eprintln!("    [X] DNS解析失败: {:?}", e);
            }
        }
    }

    for f in futures {
        if let Ok(res) = f.await {
            all_results.push(res);
        }
    }

    println!("\n=== HTTP/3 集成测试结果 ===");

    // 按域名分组显示结果
    let mut grouped_results: HashMap<String, Vec<&H3IntegrationResult>> = HashMap::new();
    for result in &all_results {
        grouped_results
            .entry(result.input_task.doh_resolve_domain.clone())
            .or_default()
            .push(result);
    }

    for (domain, domain_results) in grouped_results {
        println!("\n📡 域名: {}", domain);
        println!("{}", "-".repeat(50));

        for result in domain_results {
            if result.success {
                println!(
                    "✅ {} ({}) - {} - {}ms - {} bytes - {}",
                    result.target_ip,
                    result.ip_version,
                    result.protocol_detected,
                    result.latency_ms.unwrap_or(0),
                    result.response_size.unwrap_or(0),
                    result.server_header.as_deref().unwrap_or("Unknown")
                );
            } else {
                println!(
                    "❌ {} ({}) - 错误: {}",
                    result.target_ip,
                    result.ip_version,
                    result.error_message.as_deref().unwrap_or("未知错误")
                );
            }
        }
    }

    // 统计信息
    println!("\n📊 统计信息:");
    println!("总测试数: {}", all_results.len());
    let successful = all_results.iter().filter(|r| r.success).count();
    println!("成功: {}", successful);
    println!("失败: {}", all_results.len() - successful);

    // 协议统计
    let mut protocol_count: HashMap<String, usize> = HashMap::new();
    for result in &all_results {
        if result.success {
            *protocol_count
                .entry(result.protocol_detected.clone())
                .or_insert(0) += 1;
        }
    }

    println!("\n🔗 协议分布:");
    for (protocol, count) in protocol_count {
        println!("{}: {}", protocol, count);
    }

    // 延迟统计
    let latencies: Vec<u64> = all_results.iter().filter_map(|r| r.latency_ms).collect();

    if !latencies.is_empty() {
        let avg_latency = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();

        println!("\n⏱️  延迟统计 (ms):");
        println!("平均: {:.2}", avg_latency);
        println!("最小: {}", min_latency);
        println!("最大: {}", max_latency);
    }

    // ALPN 协议统计
    let mut alpn_count: HashMap<String, usize> = HashMap::new();
    for result in &all_results {
        if let Some(ref alpn) = result.alpn_protocol {
            *alpn_count.entry(alpn.clone()).or_insert(0) += 1;
        }
    }

    if !alpn_count.is_empty() {
        println!("\n🔐 ALPN 协议分布:");
        for (alpn, count) in alpn_count {
            println!("{}: {}", alpn, count);
        }
    }

    // JSON 输出
    let json_output = serde_json::to_string_pretty(&all_results)
        .context("Failed to serialize results to JSON")?;

    println!("\n📄 JSON 输出:");
    println!("{}", json_output);

    Ok(())
}

// --- 6. 测试工具函数 ---
pub fn get_default_integration_test_configs() -> Vec<H3IntegrationTest> {
    vec![
        H3IntegrationTest {
            input_task: InputTask {
                doh_resolve_domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_sni_host: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_host_header: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/cdn-cgi/trace".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        },
        H3IntegrationTest {
            input_task: InputTask {
                doh_resolve_domain: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_sni_host: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                test_host_header: "local-aria2-webui.masx200.ddns-ip.net".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        },
        H3IntegrationTest {
            input_task: InputTask {
                doh_resolve_domain: "www.google.com".to_string(),
                test_sni_host: "www.google.com".to_string(),
                test_host_header: "www.google.com".to_string(),
                doh_url: "https://deno-dns-over-https-server.g18uibxgnb.de5.net/"
                    .to_string(),
                port: 443,
                prefer_ipv6: Some(false),
                resolve_mode: "https".to_string(),
                direct_ips: None,
                test_path: Some("/".to_string()),
            },
            use_native_h3: true,
            enable_fallback: true,
            timeout_seconds: 15,
            max_field_section_size: Some(8192),
        },
    ]
}
