这是一个非常棒的发现！你提到的 **HTTPS 记录 (Type 65\)** 是现代 DNS
的一个重要特性（SVCB/HTTPS 记录）。

相比于传统的 A (IPv4) 和 AAAA (IPv6) 记录，HTTPS
记录的价值在于它\*\*“一站式”\*\*提供了所有连接信息：

1. **协议支持 (alpn=h3,h2)**: 明确告诉你服务端支持 HTTP/3，不需要去猜。
2. **IP 线索 (ipv4hint, ipv6hint)**: 直接在 DNS 响应里给了你 IP
   地址列表，省去了二次查询。
3. **ECH (Encrypted Client Hello)**: 提供了加密 Client Hello
   的密钥，用于更隐私的握手（这是对抗 DPI 干扰的关键）。

基于你提供的新数据，我更新了 Rust 工具。现在的逻辑更加智能：**直接请求 Type 65
记录，解析其中的 hint IP，然后进行 HTTP/3 连通性测试。**

### **新增能力：解析 HTTPS (Type 65\) 记录并测试 HTTP/3**

#### **Cargo.toml (保持不变)**

需要 reqwest (带有 http3), tokio, serde, anyhow, regex (新增正则用于解析复杂的
DNS 字符串)。

Ini, TOML

\[package\]\
name \= "cf-scanner-v2"\
version \= "0.1.0"\
edition \= "2021"

\[dependencies\]\
reqwest \= { version \= "0.12", features \= \["json", "rustls-tls", "http3",
"gzip", "brotli"\] }\
tokio \= { version \= "1", features \= \["full"\] }\
serde \= { version \= "1", features \= \["derive"\] }\
serde_json \= "1"\
anyhow \= "1"\
regex \= "1" \# 新增：用于解析 DNS 字符串

#### **src/main.rs (完整升级版)**

这段代码实现了：

1. 请求 DoH 接口获取 type=HTTPS。
2. 使用正则提取 ipv4hint 和 ipv6hint 中的 IP 列表。
3. 针对解析出的每一个 IP，强制使用 HTTP/3 进行握手测试。

Rust

use anyhow::{Context, Result};\
use reqwest::{Client, Version};\
use serde::{Deserialize, Serialize};\
use std::net::SocketAddr;\
use std::time::Instant;\
use regex::Regex;\
use std::collections::HashSet;

// \--- 1\. 输入任务结构 \---\
\#\[derive(Debug, Deserialize, Clone)\]\
struct InputTask {\
domain: String,\
port: u16,\
// 如果为 true，只测试 ipv6hint，否则测试 ipv4hint\
// 如果为 null (Option::None)，则两者都测试\
prefer_ipv6: Option\<bool\>,\
}

// \--- 2\. Google DoH 响应结构 \---\
\#\[derive(Debug, Deserialize)\]\
struct DoHResponse {\
\#\[serde(rename \= "Status")\]\
status: i32,\
\#\[serde(rename \= "Answer")\]\
answer: Option\<Vec\<DoHAnswer\>\>,\
}

\#\[derive(Debug, Deserialize)\]\
struct DoHAnswer {\
name: String,\
\#\[serde(rename \= "type")\]\
record_type: u16, // 我们关注 65 (HTTPS)\
data: String, // 包含 alpn, ipv4hint, ipv6hint 的长字符串\
}

// \--- 3\. 测试结果输出结构 \---\
\#\[derive(Debug, Serialize)\]\
struct TestResult {\
domain: String,\
target_ip: String,\
ip_version: String, // "IPv4" or "IPv6"\
success: bool,\
status_code: Option\<u16\>,\
protocol: String, // "h3", "h2", "http/1.1"\
latency_ms: Option\<u64\>,\
headers_server: Option\<String\>, // 获取 Server 头，看是否是 cloudflare\
error_msg: Option\<String\>,\
}

// \--- 4\. 核心逻辑：解析 HTTPS 记录字符串 \---\
// 输入示例: "1 . alpn=h3,h2 ipv4hint=104.21.33.118,172.67.162.86 ipv6hint=..."\
fn parse_https_hints(data: &str) \-\> (Vec\<String\>, Vec\<String\>) {\
let mut v4_ips \= Vec::new();\
let mut v6_ips \= Vec::new();

    // 正则匹配 ipv4hint=... 和 ipv6hint=...
    // 简单的按空格分割然后查找 key=value 也是一种方法，但正则更稳健
    let re\_v4 \= Regex::new(r"ipv4hint=(\[0-9\\.,\]+)").unwrap();
    let re\_v6 \= Regex::new(r"ipv6hint=(\[0-9a-fA-F:\\.,\]+)").unwrap();

    if let Some(caps) \= re\_v4.captures(data) {
        if let Some(match\_str) \= caps.get(1) {
            for ip in match\_str.as\_str().split(',') {
                v4\_ips.push(ip.trim().to\_string());
            }
        }
    }

    if let Some(caps) \= re\_v6.captures(data) {
        if let Some(match\_str) \= caps.get(1) {
            for ip in match\_str.as\_str().split(',') {
                v6\_ips.push(ip.trim().to\_string());
            }
        }
    }

    (v4\_ips, v6\_ips)

}

// \--- 5\. 获取 DNS HTTPS 记录 \---\
async fn resolve_https_record(client: \&Client, domain: &str) \-\>
Result\<Vec\<String\>\> {\
// 使用 Type 65 (HTTPS) 查询\
let dns_url \= format\!(\
"https://deno-dns-over-https-server.g18uibxgnb.de5.net/?name={}\&type=HTTPS",\
domain\
);

    let resp \= client.get(\&dns\_url).send().await?.json::\<DoHResponse\>().await?;

    if resp.status \!= 0 {
        return Err(anyhow::anyhow\!("DNS Error Status: {}", resp.status));
    }

    let mut all\_ips \= Vec::new();

    if let Some(answers) \= resp.answer {
        for ans in answers {
            if ans.record\_type \== 65 {
                let (v4, v6) \= parse\_https\_hints(\&ans.data);
                all\_ips.extend(v4);
                all\_ips.extend(v6);
            }
        }
    }

    // 去重
    let unique\_ips: HashSet\<String\> \= all\_ips.into\_iter().collect();
    Ok(unique\_ips.into\_iter().collect())

}

// \--- 6\. 连通性测试 (支持 H3) \---\
async fn test_ip(domain: String, port: u16, ip: String) \-\> TestResult {\
let url \= format\!("https://{}:{}/", domain, port); // 测试根路径\
let addr_str \= format\!("{}:{}", ip, port);

    // 判断 IP 版本
    let ip\_ver \= if ip.contains(':') { "IPv6" } else { "IPv4" };

    let socket\_addr: SocketAddr \= match addr\_str.parse() {
        Ok(a) \=\> a,
        Err(e) \=\> return TestResult::fail(\&domain, \&ip, ip\_ver, format\!("IP Parse Error: {}", e)),
    };

    // 构建 Client: 强制绑定 IP，强制 H3
    let client\_res \= Client::builder()
        .resolve\_to\_addrs(\&domain, &\[socket\_addr\])
        .danger\_accept\_invalid\_certs(true) // 忽略证书验证，专注于连通性
        .http3\_prior\_knowledge() // 核心：强制使用 HTTP/3，不进行 H1/H2 升级协商
        .timeout(std::time::Duration::from\_secs(4))
        .build();

    let client \= match client\_res {
        Ok(c) \=\> c,
        Err(e) \=\> return TestResult::fail(\&domain, \&ip, ip\_ver, format\!("Client Build Error: {}", e)),
    };

    let start \= Instant::now();
    // 模拟真实浏览器 Header
    let req \= client.get(\&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send();

    match req.await {
        Ok(res) \=\> {
            let latency \= start.elapsed().as\_millis() as u64;
            let status \= res.status().as\_u16();
            let server\_header \= res.headers()
                .get("server")
                .and\_then(|v| v.to\_str().ok())
                .map(|s| s.to\_string());

            let protocol \= match res.version() {
                Version::HTTP\_3 \=\> "h3",
                Version::HTTP\_2 \=\> "h2",
                Version::HTTP\_11 \=\> "http/1.1",
                \_ \=\> "unknown",
            };

            TestResult {
                domain,
                target\_ip: ip,
                ip\_version: ip\_ver.to\_string(),
                success: status \< 500, // 只要不是本地连接错误或严重服务器错误都算连通成功
                status\_code: Some(status),
                protocol: protocol.to\_string(),
                latency\_ms: Some(latency),
                headers\_server: server\_header,
                error\_msg: None,
            }
        }
        Err(e) \=\> TestResult::fail(\&domain, \&ip, ip\_ver, e.to\_string()),
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
headers_server: None,\
error_msg: Some(msg),\
}\
}\
}

// \--- 7\. 主程序 \---\
\#\[tokio::main\]\
async fn main() {\
let dns_client \= Client::new();

    // 输入：包含你的目标域名
    let input\_json \= r\#"
    \[
        {
            "domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",
            "port": 443,
            "prefer\_ipv6": null
        }
    \]
    "\#;

    let tasks: Vec\<InputTask\> \= serde\_json::from\_str(input\_json).unwrap();
    let mut results \= Vec::new();
    let mut futures \= Vec::new();

    for task in tasks {
        let dns\_c \= dns\_client.clone();

        // 步骤 1: 解析 DNS (Type 65\)
        println\!("正在解析 HTTPS 记录: {} ...", task.domain);
        match resolve\_https\_record(\&dns\_c, \&task.domain).await {
            Ok(ips) \=\> {
                println\!("  \-\> 找到 {} 个候选 IP: {:?}", ips.len(), ips);

                // 步骤 2: 生成测试任务
                for ip in ips {
                    let is\_v6 \= ip.contains(':');

                    // 简单的过滤器：如果指定了 prefer\_ipv6，则跳过不符合的版本
                    if let Some(want\_v6) \= task.prefer\_ipv6 {
                        if want\_v6 \!= is\_v6 { continue; }
                    }

                    let domain\_clone \= task.domain.clone();
                    futures.push(tokio::spawn(async move {
                        test\_ip(domain\_clone, task.port, ip).await
                    }));
                }
            },
            Err(e) \=\> {
                eprintln\!("DNS 解析失败 \[{}\]: {}", task.domain, e);
            }
        }
    }

    // 等待所有测试完成
    for f in futures {
        if let Ok(res) \= f.await {
            results.push(res);
        }
    }

    // 输出最终 JSON
    println\!("{}", serde\_json::to\_string\_pretty(\&results).unwrap());

}

### **代码亮点分析**

1. **解析 type=HTTPS 的黑魔法**:
   - 代码中的 parse_https_hints 函数使用正则表达式 (ipv4hint=(\[0-9\\.,\]+))
     专门从 Google DoH 返回的杂乱字符串中提取干净的 IP 列表。
   - 这完美适配你提供的 DNS 响应格式：data: "1 . alpn=h3,h2 ipv4hint=...
     ipv6hint=..."。
2. **http3_prior_knowledge()**:
   - 因为 DNS 记录 (alpn=h3) 已经明确告诉我们支持 H3，我们在 Rust 代码中使用了
     prior_knowledge。这意味着客户端**不会**先发 TCP SYN，而是直接发 UDP QUIC
     数据包。这是测试 UDP 连通性最直接的方法。
3. **结果验证**:
   - 输出中包含 headers_server。Cloudflare 的节点通常会在 Header 里返回 Server:
     cloudflare。这可以作为辅助验证，证明你确实连接到了 Cloudflare
     的边缘节点，而不是被中间人劫持。

### **预期输出**

运行后，你将得到类似下面的 JSON。注意 protocol 字段如果是 "h3"，说明 UDP QUIC
连接成功；如果失败或降级，会显示错误或 h2。

JSON

\[\
{\
"domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",\
"target_ip": "104.21.33.118",\
"ip_version": "IPv4",\
"success": true,\
"status_code": 200,\
"protocol": "h3",\
"latency_ms": 128,\
"headers_server": "cloudflare",\
"error_msg": null\
},\
{\
"domain": "hello-world-deno-deploy.a1u06h9fe9y5bozbmgz3.qzz.io",\
"target_ip": "2606:4700:3030::ac43:a256",\
"ip_version": "IPv6",\
"success": true,\
"status_code": 200,\
"protocol": "h3",\
"latency_ms": 115,\
"headers_server": "cloudflare",\
"error_msg": null\
}\
\]

现在你可以输入一系列托管在 Cloudflare 上的域名，程序会自动找出其所有优选 IP
(IPv4 和 IPv6 hint)，并逐个测试 HTTP/3 的真实连通性。
