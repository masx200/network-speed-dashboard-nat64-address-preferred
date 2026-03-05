这是一个非常专业且正确的建议！使用正则表达式解析 DNS
确实是不严谨的，尤其是在处理 HTTPS (SVCB) 这种包含复杂二进制结构（TLV
格式：Type-Length-Value）的记录时。

使用 **RFC 8484 (DNS over HTTPS)** 标准的二进制格式 (application/dns-message)
是最稳健的方法。Rust 拥有强大的 DNS 库 hickory-dns (原名
trust-dns)，我们可以利用其底层的 hickory-proto 来完美解析二进制 DNS 消息。

以下是重构后的最终版代码。

### **核心变更点**

1. **引入 hickory-proto**: 用于构建标准的二进制 DNS 查询请求，并解析二进制响应。
2. **放弃 JSON API**: 改为发送 POST 请求，Content-Type 为
   application/dns-message，Body 为二进制 DNS 包。
3. **动态 DoH URL**: doh_url 现在是 JSON 输入的一部分，你可以随意指定
   Google、Cloudflare 或你的 xget 代理地址。

### **1\. Cargo.toml 配置**

我们需要添加 hickory-proto 库。

Ini, TOML

\[package\]\
name \= "cf-h3-scanner"\
version \= "0.1.0"\
edition \= "2021"

\[dependencies\]\
\# HTTP 客户端\
reqwest \= { version \= "0.12", features \= \["json", "rustls-tls", "http3",
"gzip", "brotli"\] }\
\# 异步运行时\
tokio \= { version \= "1", features \= \["full"\] }\
\# 序列化\
serde \= { version \= "1", features \= \["derive"\] }\
serde_json \= "1"\
\# 错误处理\
anyhow \= "1"\
\# 专业 DNS 协议库 (用于解析二进制)\
hickory-proto \= "0.24"

### **2\. src/main.rs 完整代码**

这个版本完全抛弃了字符串正则匹配，转而使用结构化的二进制解析，能够准确提取
ipv4hint 和 ipv6hint。

Rust

use anyhow::{Context, Result};\
use reqwest::{Client, Version};\
use serde::{Deserialize, Serialize};\
use std::net::{IpAddr, SocketAddr};\
use std::time::Instant;\
use std::str::FromStr;

// 引入 hickory-proto 处理二进制 DNS\
use hickory_proto::op::{Message, Query, ResponseCode};\
use hickory_proto::rr::{Name, RecordType, RData};\
use hickory_proto::rr::rdata::svcb::{SvcParamKey, IpHint};

// \--- 1\. 输入配置 \---\
\#\[derive(Debug, Deserialize, Clone)\]\
struct InputTask {\
target_domain: String, // 要测试的域名\
doh_url: String, // 自定义的 DoH 地址\
port: u16,\
prefer_ipv6: Option\<bool\>,\
}

// \--- 2\. 输出结果 \---\
\#\[derive(Debug, Serialize)\]\
struct TestResult {\
domain: String,\
target_ip: String,\
ip_version: String,\
success: bool,\
status_code: Option\<u16\>,\
protocol: String,\
latency_ms: Option\<u64\>,\
server_header: Option\<String\>,\
error_msg: Option\<String\>,\
}

// \--- 3\. 核心：DoH 二进制查询与解析 \---\
// 发送 RFC 8484 标准的 POST 请求，并解析二进制响应\
async fn resolve_https_binary(client: \&Client, doh_url: &str, domain: &str)
\-\> Result\<Vec\<IpAddr\>\> {\
// 3.1 构建 DNS 查询消息 (Question)\
let mut query_msg \= Message::new();\
let name \= Name::from_str(domain).context("Invalid domain name format")?;\
// 查询类型为 HTTPS (Type 65\)\
let query \= Query::query(name.clone(), RecordType::HTTPS);\
query_msg.add_query(query);

    // 序列化为二进制
    let query\_bytes \= query\_msg.to\_vec().context("Failed to serialize DNS query")?;

    // 3.2 发送 DoH 请求 (POST application/dns-message)
    // 注意：有些代理如果不支持 POST，可能需要改用 GET base64 方式，但标准 DoH 推荐 POST
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

    // 3.4 提取 HTTPS 记录中的 hint
    for record in dns\_msg.answers() {
        if let Some(RData::HTTPS(svcb)) \= record.data() {
            // HTTPS 记录包含多个参数 (SvcParam)
            for param in svcb.svc\_params() {
                match param {
                    // 提取 ipv4hint
                    SvcParamKey::Ipv4Hint(IpHint(hints)) \=\> {
                        for ip in hints {
                            ips.push(IpAddr::V4(\*ip));
                        }
                    }
                    // 提取 ipv6hint
                    SvcParamKey::Ipv6Hint(IpHint(hints)) \=\> {
                        for ip in hints {
                            ips.push(IpAddr::V6(\*ip));
                        }
                    }
                    \_ \=\> {} // 忽略 ALPN, ECH 等其他参数
                }
            }
        }
    }

    Ok(ips)

}

// \--- 4\. HTTP/3 连通性测试 (保持之前的高效逻辑) \---\
async fn test_connectivity(task: InputTask, ip: IpAddr) \-\> TestResult {\
let url \= format\!("https://{}:{}/", task.target_domain, task.port);\
let socket_addr \= SocketAddr::new(ip, task.port);\
let ip_ver \= if ip.is_ipv6() { "IPv6" } else { "IPv4" };

    // 创建独立的 Client 实例以绑定特定的 IP
    let client\_build \= Client::builder()
        .resolve\_to\_addrs(\&task.target\_domain, &\[socket\_addr\])
        .danger\_accept\_invalid\_certs(true)
        .http3\_prior\_knowledge() // 强制 H3
        .timeout(std::time::Duration::from\_secs(5))
        .no\_proxy()
        .build();

    let client \= match client\_build {
        Ok(c) \=\> c,
        Err(e) \=\> return TestResult::fail(\&task.target\_domain, \&ip.to\_string(), ip\_ver, e.to\_string()),
    };

    let start \= Instant::now();
    // 伪装 User-Agent
    match client.get(\&url).header("User-Agent", "curl/8.12.1").send().await {
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
                domain: task.target\_domain,
                target\_ip: ip.to\_string(),
                ip\_version: ip\_ver.to\_string(),
                success: status \< 500,
                status\_code: Some(status),
                protocol: protocol.to\_string(),
                latency\_ms: Some(latency),
                server\_header: server,
                error\_msg: None,
            }
        }
        Err(e) \=\> TestResult::fail(\&task.target\_domain, \&ip.to\_string(), ip\_ver, e.to\_string()),
    }

}

impl TestResult {\
fn fail(domain: &str, ip: &str, ver: &str, msg: String) \-\> Self {\
TestResult {\
domain: domain.to_string(),\
target_ip: ip.to_string(),\
ip_version: ver.to_string(),\
success: false,\
status_code: None,\
protocol: "none".to_string(),\
latency_ms: None,\
server_header: None,\
error_msg: Some(msg),\
}\
}\
}

// \--- 5\. 主流程 \---\
\#\[tokio::main\]\
async fn main() {\
// 基础 HTTP 客户端用于 DoH 请求\
let doh_http_client \= Client::builder()\
.use_rustls_tls()\
.build()\
.expect("Failed to create DoH client");

    // JSON 输入
    let input\_json \= r\#"
    \[
        {
            "target\_domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "doh\_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
            "port": 443,
            "prefer\_ipv6": null
        }
    \]
    "\#;

    let tasks: Vec\<InputTask\> \= serde\_json::from\_str(input\_json).expect("Invalid JSON");
    let mut futures \= Vec::new();

    for task in tasks {
        let client\_ref \= doh\_http\_client.clone();

        // 步骤 1: 解析
        println\!("\>\>\> 正在通过 {} 解析 {} 的 HTTPS 记录 (二进制模式)...", task.doh\_url, task.target\_domain);

        match resolve\_https\_binary(\&client\_ref, \&task.doh\_url, \&task.target\_domain).await {
            Ok(ips) \=\> {
                if ips.is\_empty() {
                    println\!("    \[\!\] 未找到 HTTPS 记录或 Hint 为空");
                    continue;
                }
                println\!("    \-\> 解析成功，获取到 {} 个 IP Hint: {:?}", ips.len(), ips);

                // 步骤 2: 生成测试任务
                for ip in ips {
                    let is\_v6 \= ip.is\_ipv6();
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

    // 步骤 3: 收集并输出结果
    let mut results \= Vec::new();
    for f in futures {
        if let Ok(res) \= f.await {
            results.push(res);
        }
    }

    println\!("\\n=== 测试结果 \===");
    println\!("{}", serde\_json::to\_string\_pretty(\&results).unwrap());

}

### **代码改进详解**

1. **二进制安全性 (hickory-proto)**:
   - 旧方法（正则）：如果 DNS 返回的 JSON 格式稍有变动（比如 Google
     改变了字段顺序，或者 Cloudflare 返回了不同的 JSON 结构），正则就会失效。
   - 新方法（二进制）：直接解析符合 RFC 标准的 Wire Format 数据。无论服务器是
     Google 还是 AdGuard，只要它遵循 DNS 标准，hickory-proto 就能精确地将
     ipv4hint 和 ipv6hint 提取出来，不会误判，也不会漏掉。
2. **通用性**:
   - 现在的 resolve_https_binary 函数是一个通用的 DoH
     解析器。它可以解析任何支持标准 RFC 8484 的 DoH 服务器返回的数据。
   - 即使用户提供的 URL 是代理 (xget...)，只要该代理能够透传
     application/dns-message 类型的 POST 请求，这个程序就能正常工作。
3. **流程优化**:
   - JSON 输入中明确区分了 target_domain (你想连的网站) 和 doh_url (你想用的 DNS
     服务器)。
   - 程序会打印解析过程的日志，让你知道是 DNS 挂了还是后续连接挂了。

### **预期运行输出**

Plaintext

\>\>\> 正在通过 https://deno-dns-over-https-server.g18uibxgnb.de5.net/解析
hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io 的 HTTPS 记录
(二进制模式)...\
\-\> 解析成功，获取到 3 个 IP Hint: \[104.21.33.118, 172.67.162.86,
2606:4700:3030::ac43:a256\]

\=== 测试结果 \===\
\[\
{\
"domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",\
"target_ip": "104.21.33.118",\
"ip_version": "IPv4",\
"success": true,\
"status_code": 200,\
"protocol": "h3",\
"latency_ms": 125,\
"server_header": "cloudflare",\
"error_msg": null\
},\
...\
\]
