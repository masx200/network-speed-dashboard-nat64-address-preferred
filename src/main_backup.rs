use anyhow::{Context, Result};
use reqwest::{Client, Version};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Instant;

// 引入 trust-dns 协议相关模块用于 RFC 8484 二进制 DNS 消息
use trust_dns_resolver::proto::op::{Message, Query};
use trust_dns_resolver::proto::rr::{Name, RData, RecordType};

// --- 1. 输入配置 ---
// CLAUDE.md: "程序接受JSON格式的配置"
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

// --- 2. DoH JSON API 响应格式 (已移除，使用 trust-dns 替换) ---
/*
// 参照 resolve_https_record 和 resolve_a_aaaa_record 中的用法
#[derive(Debug, Deserialize)]
struct DoHResponse {
    #[serde(default)]
    Status: Option<u32>, // Google DNS status field
    #[serde(default)]
    status: u32, // AdGuard/other DNS status field (default)
    #[serde(default)]
    Answer: Option<Vec<Answer>>, // Google DNS answer field
    #[serde(default)]
    answer: Option<Vec<Answer>>, // AdGuard/other DNS answer field
}

// 辅助结构，用于解析 Answer 数组中的单个记录
#[derive(Debug, Deserialize)]
struct Answer {
    #[serde(rename = "type")] // 'type' is a reserved keyword in Rust
    record_type: u16,
    data: String,
}
*/

// --- 3. 输出结果 ---
#[derive(Debug, Serialize)]
struct TestResult {
    domain_used: String, // DoH 解析的域名
    target_ip: String,
    ip_version: String,
    sni_host: String,    // 实际使用的 SNI
    host_header: String, // 实际使用的 Host header
    success: bool,
    status_code: Option<u16>,
    protocol: String, // 实际协商的协议 (h3, h2, http/1.1)
    latency_ms: Option<u64>,
    server_header: Option<String>,
    error_msg: Option<String>,
    dns_source: String, // "Direct Input", "Binary DoH", 或 "JSON DoH"
}

// --- Helper: 提取 A/AAAA 记录的 IP ---
// 提取 A/AAAA 记录 IP 的公共逻辑，可用于 answers, authorities, additionals 三个部分。
fn extract_a_aaaa_ips(
    records: &[trust_dns_resolver::proto::rr::Record],
    ips: &mut HashSet<IpAddr>,
) {
    for record in records {
        // rdata.ip_addr() 是 trust-dns 中用于 A/AAAA 记录的便捷方法
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
    // 1. 构建 DNS 查询消息 (Question)
    let mut message = Message::new();
    let name = Name::from_str(domain).context("Invalid domain name for DNS query")?;

    let query = Query::query(name, record_type);
    message.add_query(query);

    // 2. 消息编码为二进制 - 使用 trust-dns-resolver 的高级 API
    let request_buffer = message.to_vec().context("Failed to encode DNS message")?;

    // 3. 使用 reqwest 发送 POST 请求
    let response = client
        .post(doh_url)
        .header("Content-Type", "application/dns-message")
        .header("Accept", "application/dns-message")
        .body(request_buffer)
        .send()
        .await
        .context(format!("Failed to send DoH request to {}", doh_url))?;

    if !response.status().is_success() {
        // 如果 HTTP 状态码不是 2xx，返回错误
        return Err(anyhow::anyhow!(
            "DoH server returned HTTP error status: {}",
            response.status()
        ));
    }

    // 4. 读取二进制响应体
    let response_body = response
        .bytes()
        .await
        .context("Failed to read DoH response body")?;

    // 5. 解码 DNS 响应消息
    // 修复：使用 Message::from_vec 方法直接从字节数组解析
    let dns_response = Message::from_vec(&response_body)
        .context("Failed to decode DNS binary message (invalid data)")?;

    Ok(dns_response)
}

// --- 4. 核心：DoH HTTPS 记录查询 (RFC 8484 Binary) ---
async fn resolve_https_record(client: &Client, doh_url: &str, domain: &str) -> Result<Vec<IpAddr>> {
    let mut ips = HashSet::new();

    // 1. 查询 HTTPS (SVCB) 记录
    match doh_query_manual(client, doh_url, domain, RecordType::HTTPS).await {
        Ok(response) => {
            for record in response.answers() {
                if let Some(rdata) = record.data() {
                    // 检查是否是 SVCB 记录并提取 IP hints
                    if let RData::SVCB(svc_rec) = rdata {
                        // 从 ipv4hint 中提取 IP
                        if let Some(ipv4_param) = svc_rec.svc_params().get(&SvcParamKey::Ipv4Hint) {
                            if let Some(ip_list) = ipv4_param.ip_addrs() {
                                for ip in ip_list {
                                    ips.insert(*ip);
                                }
                            }
                        }

                        // 从 ipv6hint 中提取 IP
                        if let Some(ipv6_param) = svc_rec.svc_params().get(&SvcParamKey::Ipv6Hint) {
                            if let Some(ip_list) = ipv6_param.ip_addrs() {
                                for ip in ip_list {
                                    ips.insert(*ip);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("    [X] HTTPS记录解析失败: {:?}", e),
    }

    // 2. 如果未在 SVCB 记录中找到 IP，执行 A/AAAA 记录的兜底查询
    if ips.is_empty() {
        println!("    -> HTTPS记录中未找到 IP，尝试 A/AAAA 记录查询作为兜底...");

        // 查询 A 记录
        if let Ok(response) = doh_query_manual(client, doh_url, domain, RecordType::A).await {
            // 检查 Answers, Authorities, Additionals 所有部分
            extract_a_aaaa_ips(response.answers(), &mut ips);
            extract_a_aaaa_ips(response.authoritative(), &mut ips);
            extract_a_aaaa_ips(response.additionals(), &mut ips);
        }

        // 查询 AAAA 记录
        if let Ok(response) = doh_query_manual(client, doh_url, domain, RecordType::AAAA).await {
            extract_a_aaaa_ips(response.answers(), &mut ips);
            extract_a_aaaa_ips(response.authoritative(), &mut ips);
            extract_a_aaaa_ips(response.additionals(), &mut ips);
        }
    }

    // 3. 排序并返回
    let mut ip_vec = ips.into_iter().collect::<Vec<_>>();
    // 排序逻辑 (这里默认 IPv4 优先)
    ip_vec.sort_by_key(|ip| ip.is_ipv6());

    Ok(ip_vec)
}

// --- 5. 兼容性：DoH A/AAAA 记录查询 (RFC 8484 Binary) ---
// 在手动查询模式下，与 resolve_https_record 逻辑相似，直接查询 A/AAAA
async fn resolve_a_aaaa_record(
    client: &Client,
    doh_url: &str,
    domain: &str,
    _ipv6: bool,
) -> Result<Vec<IpAddr>> {
    resolve_https_record(client, doh_url, domain).await
}

// --- 7. HTTP/3 连通性测试 ---
async fn test_connectivity(task: InputTask, ip: IpAddr, dns_source: String) -> TestResult {
    // 关键点 1: URL 决定 SNI。reqwest 将使用 test_sni_host 作为 SNI。
    let url = format!("https://{}:{}/", task.test_sni_host, task.port);

    let socket_addr = SocketAddr::new(ip, task.port);
    let ip_ver = if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    // 创建独立的 Client 实例以绑定特定的 IP
    let client_build = Client::builder()
        // 关键点 2: 强制 IP 连接 (将 test_sni_host 解析到特定的 IP)
        .resolve_to_addrs(&task.test_sni_host, &[socket_addr])
        // .danger_accept_invalid_certs(true)
        .http3_prior_knowledge() // 核心：强制使用 HTTP/3
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

    // 发送请求
    let req = client
        .get(&url)
        // 关键点 3: 覆盖 Host Header
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
                Version::HTTP_3 => "h3",
                Version::HTTP_2 => "h2",
                Version::HTTP_11 => "http/1.1",
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
    // 专门用于 DNS 查询的标准 HTTP 客户端
    let dns_client = Client::builder()
        .use_rustls_tls()
        .timeout(std::time::Duration::from_secs(5))
        .no_proxy()
        .build()
        .expect("Failed to create DNS client");

    // 示例输入 JSON：演示了如何使用不同的域名进行解析和测试
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
            ">>> 正在通过 {} 解析 {} 的 HTTPS 记录 (模式: {})...",
            task.doh_url, task.doh_resolve_domain, task.resolve_mode
        );

        // 检查是否有直接指定的IP
        if let Some(direct_ips) = &task.direct_ips {
            println!("    -> 使用直接指定的IP: {:?}", direct_ips);

            for ip_str in direct_ips {
                if let Ok(ip_addr) = IpAddr::from_str(ip_str) {
                    let is_v6 = ip_addr.is_ipv6();

                    // 应用IP版本过滤
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

        // 根据解析模式选择解析方法
        match task.resolve_mode.as_str() {
            "https" => {
                // 使用 HTTPS 记录查询 (现在是手动 RFC 8484 二进制 DoH)
                match resolve_https_record(&dns_client, &task.doh_url, &task.doh_resolve_domain)
                    .await
                {
                    Ok(ips) => {
                        if ips.is_empty() {
                            println!("    [!] 未找到 IP");
                            continue;
                        }
                        println!(
                            "    -> 解析成功，获取到 {} 个 IP 地址: {:?}",
                            ips.len(),
                            ips
                        );

                        for ip in ips {
                            let is_v6 = ip.is_ipv6();

                            // 应用IP版本过滤
                            if let Some(prefer_v6) = task.prefer_ipv6 {
                                if prefer_v6 != is_v6 {
                                    continue;
                                }
                            }

                            let task_clone = task.clone();
                            futures.push(tokio::spawn(async move {
                                test_connectivity(task_clone, ip, "HTTPS DoH (Binary)".to_string())
                                    .await
                            }));
                        }
                    }
                    Err(e) => {
                        eprintln!("    [X] HTTPS记录解析失败: {:?}", e);
                    }
                }
            }
            "a_aaaa" => {
                // 使用 A/AAAA 记录查询 (现在是手动 RFC 8484 二进制 DoH)
                let resolve_ipv6 = task.prefer_ipv6.unwrap_or(false);
                match resolve_a_aaaa_record(
                    &dns_client,
                    &task.doh_url,
                    &task.doh_resolve_domain,
                    resolve_ipv6,
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
                // 直接模式，跳过DNS解析
                println!("    -> 跳过DNS解析，使用直接IP模式");
            }
            _ => {
                eprintln!("    [!] 不支持的解析模式: {}", task.resolve_mode);
            }
        }
    }

    // 等待所有测试完成
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
