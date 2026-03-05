好的，这是根据您的所有高级需求（包括二进制 DoH 解析、H3 测试，以及独立的
SNI/Host Header 设置）整合而成的完整版 main.rs 文件。

### **1\. 确保 Cargo.toml 配置正确**

在运行此代码之前，请确保您的 Cargo.toml 文件包含以下关键依赖项：

Ini, TOML

\[package\]\
name \= "cf-h3-scanner"\
version \= "0.1.0"\
edition \= "2021"

\[dependencies\]\
\# HTTP 客户端 (包含 H3, rustls, json 特性)\
reqwest \= { version \= "0.12", features \= \["json", "rustls-tls", "http3",
"gzip", "brotli"\] }\
\# 异步运行时\
tokio \= { version \= "1", features \= \["full"\] }\
\# 序列化/反序列化\
serde \= { version \= "1", features \= \["derive"\] }\
serde_json \= "1"\
\# 错误处理\
anyhow \= "1"\
\# 专业 DNS 协议库 (用于解析二进制 DNS 消息)\
hickory-proto \= "0.24"

### **2\. 完整 src/main.rs 代码**

Rust

use anyhow::{Context, Result};\
use reqwest::{Client, Version};\
use serde::{Deserialize, Serialize};\
use std::net::{IpAddr, SocketAddr};\
use std::time::Instant;\
use std::str::FromStr;\
use std::collections::HashSet;

// 引入 hickory-proto 处理二进制 DNS\
use hickory_proto::op::{Message, Query, ResponseCode};\
use hickory_proto::rr::{Name, RecordType, RData};\
use hickory_proto::rr::rdata::svcb::{SvcParamKey, IpHint};

// \--- 1\. 输入配置 (关键: 三个独立域名参数) \---\
\#\[derive(Debug, Deserialize, Clone)\]\
struct InputTask {\
doh_resolve_domain: String, // 1\. 用于 DoH 查询，获取 IP Hint 的域名\
test_sni_host: String, // 2\. 用于 TLS SNI 的域名\
test_host_header: String, // 3\. 用于 HTTP Host Header 的值\
doh_url: String, // DoH 解析服务的 URL\
port: u16, // 目标端口，通常是 443\
prefer_ipv6: Option\<bool\>, // 仅测试 IPv6 Hint (true) 或 IPv4 Hint (false)\
}

// \--- 2\. 输出结果 \---\
\#\[derive(Debug, Serialize)\]\
struct TestResult {\
domain_used: String, // DoH 解析的域名\
target_ip: String,\
ip_version: String,\
sni_host: String, // 实际使用的 SNI\
host_header: String, // 实际使用的 Host header\
success: bool,\
status_code: Option\<u16\>,\
protocol: String, // 实际协商的协议 (h3, h2, http/1.1)\
latency_ms: Option\<u64\>,\
server_header: Option\<String\>,\
error_msg: Option\<String\>,\
}

// \--- 3\. 核心：DoH 二进制查询与解析 \---\
async fn resolve_https_binary(client: \&Client, doh_url: &str, domain: &str)
\-\> Result\<Vec\<IpAddr\>\> {\
// 3.1 构建 DNS 查询消息 (Question: Type 65 \- HTTPS)\
let mut query_msg \= Message::new();\
let name \= Name::from_str(domain).context("Invalid domain name format")?;\
let query \= Query::query(name.clone(), RecordType::HTTPS);\
query_msg.add_query(query);

    // 序列化为二进制
    let query\_bytes \= query\_msg.to\_vec().context("Failed to serialize DNS query")?;

    // 3.2 发送 DoH 请求 (POST application/dns-message)
    let resp \= client.post(doh\_url)
        .header("Content-Type", "application/dns-message")
        .header("Accept", "application/dns-message")
        .body(query\_bytes)
        .send()
        .await
        .context("DoH request failed")?;

    if \!resp.status().is\_success() {
        return Err(anyhow::anyhow\!("DoH server returned status: {}", resp.status()));
    }

    let resp\_bytes \= resp.bytes().await.context("Failed to read DoH body")?;

    // 3.3 解析二进制响应
    let dns\_msg \= Message::from\_vec(\&resp\_bytes).context("Failed to parse DNS wire format")?;

    if dns\_msg.response\_code() \!= ResponseCode::NoError {
        return Err(anyhow::anyhow\!("DNS query returned error code: {}", dns\_msg.response\_code()));
    }

    let mut ips \= Vec::new();

    // 3.4 提取 HTTPS 记录中的 ipv4hint 和 ipv6hint
    for record in dns\_msg.answers() {
        if let Some(RData::HTTPS(svcb)) \= record.data() {
            for param in svcb.svc\_params() {
                match param {
                    // 提取 ipv4hint
                    SvcParamKey::Ipv4Hint(IpHint(hints)) \=\> {
                        for ip in hints { ips.push(IpAddr::V4(\*ip)); }
                    }
                    // 提取 ipv6hint
                    SvcParamKey::Ipv6Hint(IpHint(hints)) \=\> {
                        for ip in hints { ips.push(IpAddr::V6(\*ip)); }
                    }
                    \_ \=\> {} // 忽略其他参数
                }
            }
        }
    }

    // 去重
    let unique\_ips: HashSet\<IpAddr\> \= ips.into\_iter().collect();
    Ok(unique\_ips.into\_iter().collect())

}

// \--- 4\. HTTP/3 连通性测试 \---\
async fn test_connectivity(task: InputTask, ip: IpAddr) \-\> TestResult {\
// 关键点 1: URL 决定 SNI。reqwest 将使用 test_sni_host 作为 SNI。\
let url \= format\!("https://{}:{}/", task.test_sni_host, task.port);

    let socket\_addr \= SocketAddr::new(ip, task.port);
    let ip\_ver \= if ip.is\_ipv6() { "IPv6" } else { "IPv4" };

    // 客户端构建：强制绑定 IP，并设置 H3
    let client\_build \= Client::builder()
        // 关键点 2: 强制 IP 连接 (将 test\_sni\_host 解析到特定的 IP)
        .resolve\_to\_addrs(\&task.test\_sni\_host, &\[socket\_addr\])
        .danger\_accept\_invalid\_certs(true)
        .http3\_prior\_knowledge()
        .timeout(std::time::Duration::from\_secs(5))
        .no\_proxy()
        .build();

    let client \= match client\_build {
        Ok(c) \=\> c,
        Err(e) \=\> return TestResult::fail(\&task, \&ip.to\_string(), ip\_ver, e.to\_string()),
    };

    let start \= Instant::now();

    // 发送请求
    let req \= client.get(\&url)
        // 关键点 3: 覆盖 Host Header
        .header("Host", \&task.test\_host\_header)
        .header("User-Agent", "curl/8.12.1")
        .send();

    match req.await {
        Ok(res) \=\> {
            let latency \= start.elapsed().as\_millis() as u64;
            let status \= res.status().as\_u16();
            let server \= res.headers().get("server").and\_then(|v| v.to\_str().ok()).map(|s| s.to\_string());

            let protocol \= match res.version() {
                Version::HTTP\_3 \=\> "h3",
                Version::HTTP\_2 \=\> "h2",
                Version::HTTP\_11 \=\> "http/1.1",
                \_ \=\> "unknown",
            };

            TestResult {
                domain\_used: task.doh\_resolve\_domain,
                target\_ip: ip.to\_string(),
                ip\_version: ip\_ver.to\_string(),
                sni\_host: task.test\_sni\_host,
                host\_header: task.test\_host\_header,
                success: status \< 500,
                status\_code: Some(status),
                protocol: protocol.to\_string(),
                latency\_ms: Some(latency),
                server\_header: server,
                error\_msg: None,
            }
        }
        Err(e) \=\> TestResult::fail(\&task, \&ip.to\_string(), ip\_ver, e.to\_string()),
    }

}

impl TestResult {\
fn fail(task: \&InputTask, ip: &str, ver: &str, msg: String) \-\> Self {\
TestResult {\
domain_used: task.doh_resolve_domain.clone(),\
target_ip: ip.to_string(),\
ip_version: ver.to_string(),\
sni_host: task.test_sni_host.clone(),\
host_header: task.test_host_header.clone(),\
success: false,\
status_code: None,\
protocol: "none".to_string(),\
latency_ms: None,\
server_header: None,\
error_msg: Some(msg),\
}\
}\
}

// \--- 5\. 主程序入口 \---\
\#\[tokio::main\]\
async fn main() {\
// 用于 DoH 请求的客户端\
let doh_http_client \= Client::builder().use_rustls_tls().build().expect("Failed
to create DoH client");

    // 示例输入 JSON：演示了如何使用不同的域名进行解析和测试
    let input\_json \= r\#"
    \[
        {
            "doh\_resolve\_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test\_sni\_host": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "test\_host\_header": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "doh\_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer\_ipv6": null
        }
        // 另一个测试场景：使用 Cloudflare IP 优选域名获取 IP，但用另一个域名进行 SNI/Host 验证
        /\*
        ,{
            "doh\_resolve\_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test\_sni\_host": "some-other-domain.com",
            "test\_host\_header": "some-other-domain.com",
            "doh\_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer\_ipv6": false
        }
        \*/
    \]
    "\#;

    let tasks: Vec\<InputTask\> \= serde\_json::from\_str(input\_json).expect("Invalid JSON format in input");
    let mut futures \= Vec::new();

    for task in tasks {
        let client\_ref \= doh\_http\_client.clone();

        println\!("\>\>\> 正在通过 {} 解析 {} 的 HTTPS 记录...", task.doh\_url, task.doh\_resolve\_domain);

        match resolve\_https\_binary(\&client\_ref, \&task.doh\_url, \&task.doh\_resolve\_domain).await {
            Ok(ips) \=\> {
                if ips.is\_empty() {
                    println\!("    \[\!\] 未找到 IP Hint");
                    continue;
                }
                println\!("    \-\> 解析成功，获取到 {} 个 IP Hint: {:?}", ips.len(), ips);

                for ip in ips {
                    let is\_v6 \= ip.is\_ipv6();
                    // 过滤器
                    if let Some(pref) \= task.prefer\_ipv6 {
                        if pref \!= is\_v6 { continue; }
                    }

                    let task\_clone \= task.clone();
                    futures.push(tokio::spawn(async move {
                        test\_connectivity(task\_clone, ip).await
                    }));
                }
            },
            Err(e) \=\> {
                eprintln\!("    \[X\] DNS 解析失败: {:?}", e);
            }
        }
    }

    let mut results \= Vec::new();
    for f in futures {
        if let Ok(res) \= f.await {
            results.push(res);
        }
    }

    println\!("\\n=== 最终测试结果 (JSON) \===");
    println\!("{}", serde\_json::to\_string\_pretty(\&results).unwrap());

}
