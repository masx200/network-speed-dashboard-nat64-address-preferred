// # 临时测试版本 - 简化DNS解析逻辑
use anyhow::{Context, Result};
use reqwest::{Client, Version};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;

// 引入 trust-dns 协议相关模块用于 RFC 8484 二进制 DNS 消息
use trust_dns_resolver::proto::op::{Message, Query};
use trust_dns_resolver::proto::rr::{Name, RecordType};

// --- 1. 输入配置 ---
#[derive(Debug, Clone, Deserialize, Serialize)]
struct InputTask {
    doh_resolve_domain: String,
    test_sni_host: String,
    test_host_header: String,
    doh_url: String,
    port: u16,
    prefer_ipv6: Option<bool>,
    resolve_mode: String,
    direct_ips: Option<Vec<String>>,
}

// --- 3. 输出结果 ---
#[derive(Debug, Serialize)]
struct TestResult {
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
    error_msg: Option<String>,
    dns_source: String,
}

// --- Helper: 提取 A/AAAA 记录的 IP ---
fn extract_a_aaaa_ips(
    records: &[trust_dns_resolver::proto::rr::Record],
    ips: &mut HashSet<IpAddr>,
) {
    for record in records {
        if let Some(ip) = record.data().and_then(|rdata| rdata.ip_addr()) {
            ips.insert(ip);
        }
    }
}

// --- Helper: 手动 DoH 查询函数 (使用 reqwest + trust-dns proto) ---
async fn doh_query_manual(
    client: &Client,
    doh_url: &str,
    domain: &str,
    record_type: RecordType,
) -> Result<Message> {
    let mut message = Message::new();
    let name = Name::from_str(domain).context("Invalid domain name for DNS query")?;
    let query = Query::query(name, record_type);
    message.add_query(query);
    let request_buffer = message.to_vec().context("Failed to encode DNS message")?;

    let response = client
        .post(doh_url)
        .header("Content-Type", "application/dns-message")
        .header("Accept", "application/dns-message")
        .body(request_buffer)
        .send()
        .await
        .context(format!("Failed to send DoH request to {}", doh_url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "DoH server returned HTTP error status: {}",
            response.status()
        ));
    }

    let response_body = response
        .bytes()
        .await
        .context("Failed to read DoH response body")?;

    let dns_response = Message::from_vec(&response_body)
        .context("Failed to decode DNS binary message (invalid data)")?;

    if dns_response.response_code() != trust_dns_resolver::proto::op::ResponseCode::NoError {
        return Err(anyhow::anyhow!(
            "DNS server returned error code: {:?}",
            dns_response.response_code()
        ));
    }

    Ok(dns_response)
}

// --- 4. 核心：简化的 DoH A/AAAA 记录查询 ---
async fn resolve_a_aaaa_record(
    client: &Client,
    doh_url: &str,
    domain: &str,
    _ipv6: bool,
) -> Result<Vec<IpAddr>> {
    let mut ips = HashSet::new();

    println!("    -> 查询 A 记录...");
    if let Ok(response) = doh_query_manual(client, doh_url, domain, RecordType::A).await {
        extract_a_aaaa_ips(response.answers(), &mut ips);
        if !ips.is_empty() {
            println!(
                "    -> 从A记录提取到 {} 个IPv4地址",
                ips.iter().filter(|ip| ip.is_ipv4()).count()
            );
        }
    } else {
        println!("    -> A记录查询失败");
    }

    if ips.is_empty() {
        println!("    -> 查询 AAAA 记录...");
        if let Ok(response) = doh_query_manual(client, doh_url, domain, RecordType::AAAA).await {
            extract_a_aaaa_ips(response.answers(), &mut ips);
            if !ips.is_empty() {
                println!(
                    "    -> 从AAAA记录提取到 {} 个IPv6地址",
                    ips.iter().filter(|ip| ip.is_ipv6()).count()
                );
            }
        } else {
            println!("    -> AAAA记录查询失败");
        }
    }

    let mut ip_vec = ips.into_iter().collect::<Vec<_>>();
    ip_vec.sort_by_key(|ip| ip.is_ipv6());

    Ok(ip_vec)
}

// --- 7. HTTP/3 连通性测试 ---
async fn test_connectivity(task: InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    let url = format!("https://{}:{}/", task.test_sni_host, task.port);
    let socket_addr = SocketAddr::new(ip, task.port);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    let client_build = Client::builder()
        .resolve_to_addrs(&task.test_sni_host, &[socket_addr])
        // .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(5))
        .no_proxy()
        .build();

    let client = match client_build {
        Ok(c) => c,
        Err(e) => {
            return TestResult::fail(&task, &ip.to_string(), ip_ver, e.to_string(), dns_source)
        }
    };

    let start = Instant::now();

    let req = client
        .get(&url)
        .header("Host", &task.test_host_header)
        .header("User-Agent", "curl/8.12.1")
        .send();

    match req.await {
        Ok(res) => {
            let latency = start.elapsed().as_millis() as u64;
            let status = res.status().as_u16();
            let server = res
                .headers()
                .get("server")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string());

            let protocol = match res.version() {
                Version::HTTP_11 => "http/1.1",
                Version::HTTP_2 => "h2",
                _ => "unknown",
            };

            TestResult {
                domain_used: task.doh_resolve_domain,
                target_ip: ip.to_string(),
                ip_version: ip_ver.to_string(),
                sni_host: task.test_sni_host,
                host_header: task.test_host_header,
                success: status < 500,
                status_code: Some(status),
                protocol: protocol.to_string(),
                latency_ms: Some(latency),
                server_header: server,
                error_msg: None,
                dns_source,
            }
        }
        Err(e) => TestResult::fail(&task, &ip.to_string(), ip_ver, e.to_string(), dns_source),
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
            error_msg: Some(msg),
            dns_source,
        }
    }
}

// --- 8. 主程序入口 ---
#[tokio::main]
async fn main() -> Result<()> {
    let dns_client = Client::builder()
        .use_rustls_tls()
        .timeout(std::time::Duration::from_secs(5))
        .no_proxy()
        .build()
        .expect("Failed to create DNS client");

    let input_json = r#"
    [
        {
            "doh_resolve_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_sni_host": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test_host_header": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "https"
        },
        {
            "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
            "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": true,
            "resolve_mode": "a_aaaa"
        },
        {
            "doh_resolve_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test_sni_host": "local-aria2-webui.masx200.ddns-ip.net",
            "test_host_header": "local-aria2-webui.masx200.ddns-ip.net",
            "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer_ipv6": true,
            "direct_ips": ["162.159.140.220", "172.67.214.232"],
            "resolve_mode": "direct"
        }
    ]
    "#;

    let tasks: Vec<InputTask> =
        serde_json::from_str(input_json).context("Invalid JSON format in input")?;

    let mut futures = Vec::new();

    for task in tasks {
        println!(
            ">>> 正在通过 {} 解析 {} 的记录 (模式: {})...",
            task.doh_url, task.doh_resolve_domain, task.resolve_mode
        );

        if let Some(direct_ips) = &task.direct_ips {
            println!("    -> 使用直接指定的IP: {:?}", direct_ips);

            for ip_str in direct_ips {
                if let Ok(ip_addr) = IpAddr::from_str(ip_str) {
                    let is_v6 = ip_addr.is_ipv6();

                    if let Some(prefer_v6) = task.prefer_ipv6 {
                        if prefer_v6 != is_v6 {
                            continue;
                        }
                    }

                    let task_clone = task.clone();
                    futures.push(tokio::spawn(async move {
                        test_connectivity(task_clone, ip_addr, "Direct Input".to_string()).await
                    }));
                }
            }
            continue;
        }

        match task.resolve_mode.as_str() {
            "a_aaaa" => {
                match resolve_a_aaaa_record(
                    &dns_client,
                    &task.doh_url,
                    &task.doh_resolve_domain,
                    task.prefer_ipv6.unwrap_or(false),
                )
                .await
                {
                    Ok(ips) => {
                        if ips.is_empty() {
                            println!("    [!] 未找到IP地址");
                            continue;
                        }
                        println!("    -> 解析成功，获取到 {} 个IP地址: {:?}", ips.len(), ips);

                        for ip in ips {
                            let task_clone = task.clone();
                            futures.push(tokio::spawn(async move {
                                test_connectivity(task_clone, ip, "A/AAAA DoH (Binary)".to_string())
                                    .await
                            }));
                        }
                    }
                    Err(e) => {
                        eprintln!("    [X] A/AAAA记录解析失败: {:?}", e);
                    }
                }
            }
            "direct" => {
                println!("    -> 跳过DNS解析，使用直接IP模式");
            }
            _ => {
                eprintln!("    [!] 不支持的解析模式: {}", task.resolve_mode);
            }
        }
    }

    let mut results = Vec::new();
    for f in futures {
        if let Ok(res) = f.await {
            results.push(res);
        }
    }

    println!("\n=== 最终测试结果 (JSON) ===");
    println!("{}", serde_json::to_string_pretty(&results).unwrap());

    Ok(())
}
