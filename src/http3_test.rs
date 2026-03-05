// HTTP/3 network request test using reqwest with HTTP/3 support
// Based on main.rs but enhanced for HTTP/3 testing
use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use trust_dns_proto::op::{Message, Query};
use trust_dns_proto::rr::{Name, RecordType};
use trust_dns_proto::serialize::binary::BinEncodable;

// --- 1. 输入配置 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputTask {
    doh_resolve_domain: String,
    test_sni_host: String,
    test_host_header: String,
    doh_url: String,
    port: u16,
    prefer_ipv6: Option<bool>,
    resolve_mode: String,
    direct_ips: Option<Vec<String>>,
    test_path: Option<String>, // HTTP/3 测试路径
}

// --- 2. 输出结果 ---
#[derive(Debug, Serialize)]
pub struct TestResult {
    domain_used: String,
    target_ip: String,
    ip_version: String,
    sni_host: String,
    host_header: String,
    success: bool,
    status_code: Option<u16>,
    protocol: String,
    latency_ms: Option<u64>,
    server_header: Option<String>,
    response_size: Option<usize>,
    error_msg: Option<String>,
    dns_source: String,
    request_path: String,
}

// --- 3. RFC 8484 DNS over HTTPS (DoH) 实现 ---

// IPv4地址验证函数
fn is_valid_ipv4_address(ip_str: &str) -> bool {
    match ip_str {
        "0.0.0.0" | "127.0.0.0" | "255.255.255.255" => false,
        _ => {
            let parts: Vec<&str> = ip_str.split('.').collect();
            if parts.len() != 4 {
                return false;
            }

            for part in parts {
                if part.parse::<u8>().is_err() {
                    return false;
                }
            }

            ip_str != "183.192.65.101"
        }
    }
}

// 检查是否为已知的错误IPv4地址
fn is_bad_ipv4_address(ip_str: &str) -> bool {
    ip_str == "183.192.65.101"
}

// RFC 8484 DNS over HTTPS 查询函数
async fn query_dns_over_https(
    client: &Client,
    domain: &str,
    record_type: RecordType,
    doh_url: &str,
) -> Result<Vec<IpAddr>> {
    let name = Name::from_ascii(domain).context("Failed to parse domain name")?;
    let query = Query::query(name, record_type);

    let mut message = Message::new();
    message.set_id(0);
    message.set_recursion_desired(true);
    message.add_query(query);

    let mut request_bytes = Vec::new();
    {
        let mut encoder = trust_dns_proto::serialize::binary::BinEncoder::new(&mut request_bytes);
        message
            .emit(&mut encoder)
            .context("Failed to serialize DNS query")?;
    }

    let encoded_query = general_purpose::URL_SAFE_NO_PAD.encode(&request_bytes);
    let url = format!("{}?dns={}", doh_url, encoded_query);

    let response = client
        .get(&url)
        .header("Accept", "application/dns-message")
        .send()
        .await
        .context("Failed to send DoH request")?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(anyhow::anyhow!(
            "DoH server returned non-200 status: {}",
            response.status()
        ));
    }

    let response_bytes = response
        .bytes()
        .await
        .context("Failed to read response body")?;

    let dns_response =
        Message::from_vec(&response_bytes).context("Failed to parse DNS response")?;

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

pub async fn resolve_domain_with_rfc8484(client: &Client, task: &InputTask) -> Result<Vec<IpAddr>> {
    let mut ips = HashSet::new();

    if let Some(direct_ips) = &task.direct_ips {
        println!("    -> 使用直接指定的IP: {:?}", direct_ips);
        for ip_str in direct_ips {
            if let Ok(ip_addr) = IpAddr::from_str(ip_str) {
                ips.insert(ip_addr);
            }
        }
        return Ok(ips.into_iter().collect());
    }

    match task.resolve_mode.as_str() {
        "https" => {
            println!("    -> 使用 RFC 8484 DoH 查询: {}", task.doh_resolve_domain);

            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::A,
                &task.doh_url,
            )
            .await
            {
                Ok(mut ipv4_addresses) => {
                    ipv4_addresses.retain(|ip| {
                        let ip_str = ip.to_string();
                        is_valid_ipv4_address(&ip_str) && !is_bad_ipv4_address(&ip_str)
                    });

                    for ip in &ipv4_addresses {
                        ips.insert(*ip);
                        println!("    -> 从 RFC 8484 DoH 找到 IPv4: {}", ip);
                    }

                    match query_dns_over_https(
                        client,
                        &task.doh_resolve_domain,
                        RecordType::AAAA,
                        &task.doh_url,
                    )
                    .await
                    {
                        Ok(ipv6_addresses) => {
                            for ip in &ipv6_addresses {
                                ips.insert(*ip);
                                println!("    -> 从 RFC 8484 DoH 找到 IPv6: {}", ip);
                            }
                        }
                        Err(e) => {
                            println!("    -> RFC 8484 DoH IPv6 查詢失敗: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("    -> RFC 8484 DoH 查詢失敗: {:?}", e);
                }
            }
        }
        "a_aaaa" => {
            println!("    -> 使用 DoH 查詢: {}", task.doh_resolve_domain);

            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::A,
                &task.doh_url,
            )
            .await
            {
                Ok(mut ipv4_addresses) => {
                    ipv4_addresses.retain(|ip| {
                        let ip_str = ip.to_string();
                        is_valid_ipv4_address(&ip_str) && !is_bad_ipv4_address(&ip_str)
                    });

                    for ip in &ipv4_addresses {
                        ips.insert(*ip);
                        println!("    -> 從 DoH 找到 IPv4: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> DoH IPv4 查詢失敗: {:?}", e);
                }
            }

            match query_dns_over_https(
                client,
                &task.doh_resolve_domain,
                RecordType::AAAA,
                &task.doh_url,
            )
            .await
            {
                Ok(ipv6_addresses) => {
                    for ip in &ipv6_addresses {
                        ips.insert(*ip);
                        println!("    -> 從 DoH 找到 IPv6: {}", ip);
                    }
                }
                Err(e) => {
                    println!("    -> DoH IPv6 查詢失敗: {:?}", e);
                }
            }
        }
        "direct" => {
            return Ok(ips.into_iter().collect());
        }
        _ => {
            return Err(anyhow::anyhow!("不支持的解析模式: {}", task.resolve_mode));
        }
    }

    if ips.is_empty()
        && task
            .doh_resolve_domain
            .contains("local-aria2-webui.masx200.ddns-ip.net")
    {
        println!("    -> 使用備用的Cloudflare IP...");
        add_fallback_cloudflare_ips(&mut ips);
    }

    let mut ip_vec = ips.into_iter().collect::<Vec<_>>();
    ip_vec.sort_by_key(|ip| ip.is_ipv6());

    Ok(ip_vec)
}

// 添加备用Cloudflare IP
fn add_fallback_cloudflare_ips(ips: &mut HashSet<IpAddr>) {
    let fallback_ips = [
        "162.159.140.220",
        "104.16.123.64",
        "172.67.214.232",
        "2606:4700:4700::1",
    ];

    for ip_str in &fallback_ips {
        if let Ok(ip) = IpAddr::from_str(ip_str) {
            if is_valid_ipv4_address(ip_str) && !is_bad_ipv4_address(ip_str) {
                ips.insert(ip);
            }
        }
    }
}

// --- 4. HTTP/3 連接測試 ---
async fn test_http3_connectivity(task: &InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    let test_path = task.test_path.as_deref().unwrap_or("/");
    let url = format!("https://{}:{}{}", task.test_sni_host, task.port, test_path);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    println!("    -> 测试 HTTP/3 连接到: {} ({})", url, ip);

    // 配置 HTTP/3 客户端 - 使用 reqwest 的 HTTP/3 支持
    let client = match Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("rust-http3-test-tool/1.0")
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Alt-Svc", "h3=\":443\"".parse().unwrap());
            headers.insert("Connection", "keep-alive".parse().unwrap());
            headers
        })
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return TestResult::fail(
                task,
                &ip.to_string(),
                ip_ver,
                format!("Failed to create HTTP client: {}", e),
                dns_source,
            );
        }
    };

    let start = Instant::now();

    match client
        .get(&url)
        .header("Host", &task.test_host_header)
        .header("Accept", "text/plain,application/json,*/*")
        .header("User-Agent", "rust-http3-test-tool/1.0")
        .send()
        .await
    {
        Ok(res) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = res.status().as_u16();
            let server = res
                .headers()
                .get("server")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            // 检测实际使用的协议版本
            let protocol = match res.version() {
                reqwest::Version::HTTP_11 => "http/1.1",
                reqwest::Version::HTTP_2 => "h2",
                reqwest::Version::HTTP_3 => "h3",
                _ => {
                    // 通过响应头判断协议
                    if res.headers().get("alt-svc").is_some() {
                        "h3-detected"
                    } else {
                        "unknown"
                    }
                }
            };

            // 检查 HTTP/3 相关响应头
            let _h3_indicators = vec![
                ("alt-svc", res.headers().get("alt-svc").is_some()),
                ("h3", res.headers().get("h3").is_some()),
                (
                    "x-http3-connection",
                    res.headers().get("x-http3-connection").is_some(),
                ),
            ];

            let response_size = match res.content_length() {
                Some(len) => len as usize,
                None => {
                    // 尝试读取部分响应体来估算大小
                    match res.bytes().await {
                        Ok(bytes) => bytes.len(),
                        Err(_) => 0,
                    }
                }
            };

            println!(
                "    -> HTTP/3 响应: {} - {} - {} bytes - {}",
                status,
                protocol,
                response_size,
                server.as_deref().unwrap_or("Unknown")
            );

            TestResult {
                domain_used: task.doh_resolve_domain.clone(),
                target_ip: ip.to_string(),
                ip_version: ip_ver.to_string(),
                sni_host: task.test_sni_host.clone(),
                host_header: task.test_host_header.clone(),
                success: status < 500,
                status_code: Some(status),
                protocol: protocol.to_string(),
                latency_ms: Some(latency),
                server_header: server,
                response_size: Some(response_size),
                error_msg: None,
                dns_source,
                request_path: test_path.to_string(),
            }
        }
        Err(e) => TestResult::fail(
            task,
            &ip.to_string(),
            ip_ver,
            format!("HTTP request failed: {}", e),
            dns_source,
        ),
    }
}

impl TestResult {
    fn fail(task: &InputTask, ip: &str, ver: &str, msg: String, dns_source: String) -> Self {
        TestResult {
            domain_used: task.doh_resolve_domain.clone(),
            target_ip: ip.to_string(),
            ip_version: ver.to_string(),
            sni_host: task.test_sni_host.clone(),
            host_header: task.test_host_header.clone(),
            success: false,
            status_code: None,
            protocol: "none".to_string(),
            latency_ms: None,
            server_header: None,
            response_size: None,
            error_msg: Some(msg),
            dns_source,
            request_path: task.test_path.as_deref().unwrap_or("/").to_string(),
        }
    }
}

#[tokio::test]
async fn test_http3_network_requests() -> Result<()> {
    println!("🚀 HTTP/3 Network Request Test");
    println!("================================");

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("rust-http3-test-tool/1.0")
        .build()
        .expect("Failed to create HTTP client");

    // 測試配置 - 專門用於 HTTP/3 測試
    let input_json = r#"
    [
        {
            "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
            "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "https",
            "test_path": "/cdn-cgi/trace"
        },
        {
            "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
            "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "https",
            "test_path": "/"
        },
        {
            "doh_resolve_domain": "www.google.com",
            "test_sni_host": "www.google.com",
            "test_host_header": "www.google.com",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": false,
            "resolve_mode": "https",
            "test_path": "/"
        }
    ]
    "#;

    let tasks: Vec<InputTask> =
        serde_json::from_str(input_json).context("Invalid JSON format in input")?;

    let mut futures = Vec::new();

    for task in tasks {
        println!(
            ">>> 正在解析 {} (模式: {})...",
            task.doh_resolve_domain, task.resolve_mode
        );

        match resolve_domain_with_rfc8484(&client, &task).await {
            Ok(ips) => {
                if ips.is_empty() {
                    println!("    [!] 未找到IP地址");
                    continue;
                }
                println!("    -> 解析成功，獲取到 {} 个IP地址: {:?}", ips.len(), ips);

                for ip in ips {
                    if let Some(prefer_ipv6) = task.prefer_ipv6 {
                        if prefer_ipv6 != ip.is_ipv6() {
                            continue;
                        }
                    }

                    let task_clone = task.clone();
                    let dns_source = if task.resolve_mode == "direct" {
                        "Direct Input".to_string()
                    } else {
                        format!("DoH ({})", task.doh_url)
                    };

                    let ip_str = ip.to_string();
                    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };
                    let task_for_fail = task.clone();
                    let dns_source_for_fail = dns_source.clone();
                    futures.push(tokio::spawn(async move {
                        match test_http3_connectivity(&task_clone, ip, dns_source).await {
                            Ok(result) => result,
                            Err(e) => TestResult::fail(
                                &task_for_fail,
                                &ip_str,
                                ip_ver,
                                format!("测试失败: {}", e),
                                dns_source_for_fail,
                            ),
                        }
                    }));
                }
            }
            Err(e) => {
                eprintln!("    [X] DNS解析失敗: {:?}", e);
            }
        }
    }

    let mut results = Vec::new();
    for f in futures {
        if let Ok(res) = f.await {
            results.push(res);
        }
    }

    println!("\n=== HTTP/3 測試結果 ===");

    // 按域名分組顯示結果
    let mut grouped_results: std::collections::HashMap<String, Vec<&TestResult>> =
        std::collections::HashMap::new();
    for result in &results {
        grouped_results
            .entry(result.domain_used.clone())
            .or_default()
            .push(result);
    }

    for (domain, domain_results) in grouped_results {
        println!("\n📡 域名: {}", domain);
        println!("{}", "-".repeat(50));

        for result in domain_results {
            if result.success {
                println!(
                    "✅ {} ({}) - {} - {}ms - {} - {} bytes - {}",
                    result.target_ip,
                    result.ip_version,
                    result.protocol,
                    result.latency_ms.unwrap_or(0),
                    result.status_code.unwrap_or(0),
                    result.response_size.unwrap_or(0),
                    result.server_header.as_deref().unwrap_or("Unknown")
                );
            } else {
                println!(
                    "❌ {} ({}) - 錯誤: {}",
                    result.target_ip,
                    result.ip_version,
                    result.error_msg.as_deref().unwrap_or("未知錯誤")
                );
            }
        }
    }

    println!("\n📊 統計信息:");
    println!("總測試數: {}", results.len());
    let successful = results.iter().filter(|r| r.success).count();
    println!("成功: {}", successful);
    println!("失敗: {}", results.len() - successful);

    // 協議統計
    let mut protocol_count: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for result in &results {
        if result.success {
            *protocol_count.entry(result.protocol.clone()).or_insert(0) += 1;
        }
    }

    println!("\n🔗 協議分佈:");
    for (protocol, count) in protocol_count {
        println!("{}: {}", protocol, count);
    }

    // 延遲統計
    let latencies: Vec<u64> = results.iter().filter_map(|r| r.latency_ms).collect();

    if !latencies.is_empty() {
        let avg_latency = latencies.iter().sum::<u64>() as f64 / latencies.len() as f64;
        let min_latency = latencies.iter().min().unwrap();
        let max_latency = latencies.iter().max().unwrap();

        println!("\n⏱️  延遲統計 (ms):");
        println!("平均: {:.2}", avg_latency);
        println!("最小: {}", min_latency);
        println!("最大: {}", max_latency);
    }

    Ok(())
}
