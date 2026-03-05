你提出了一个非常专业的、针对 CDN/WAF 或优化网络连接测试的关键点。在测试优选 IP
时，**TLS SNI (Server Name Indication)** 和 **HTTP Host Header**
确实必须独立于实际连接的 IP 地址进行设置。

这是因为 Cloudflare 这类 CDN 服务器在同一个 IP 地址上托管了成千上万的域名。

- **SNI (TLS ServerName):** 告诉服务器应该使用哪个域名的证书进行 TLS 握手。如果
  SNI 错误，握手就会失败（证书不匹配）。
- **Host Header:** 告诉服务器或后端程序将请求路由到哪个虚拟主机。

我将对程序进行升级，使这三个关键参数（**DoH 解析域名**、**SNI 域名**、**Host
Header 值**）可以独立配置。

### **1\. 更新 InputTask 结构**

我们在 JSON 输入中增加两个明确的字段来控制 SNI 和 Host Header。

Rust

// src/main.rs \- 修改 InputTask\
\#\[derive(Debug, Deserialize, Clone)\]\
struct InputTask {\
doh_resolve_domain: String, // 1\. 用于 DoH 查询，获取 IP Hint 的域名\
test_sni_host: String, // 2\. 用于 TLS SNI 的域名\
test_host_header: String, // 3\. 用于 HTTP Host Header 的值\
doh_url: String,\
port: u16,\
prefer_ipv6: Option\<bool\>,\
}

### **2\. 更新 test_connectivity 逻辑**

关键在于：

1. **URL 构造**: reqwest (以及其底层的 rustls) 会将 URL 中的域名作为 **SNI**
   的值发送出去。因此，我们用 test_sni_host 来构造 URL。
2. **Host Header**: 显式地使用 .header("Host", ...) 来覆盖 reqwest 默认生成的
   Host Header。
3. **IP 绑定**: 继续使用 resolve_to_addrs 强制连接到 DoH 解析到的 IP。

#### **完整更新后的 src/main.rs**

（hickory-proto 和 DNS 解析逻辑保持不变，只更新了 InputTask 和 test_connectivity
函数）

Rust

use anyhow::{Context, Result};\
use reqwest::{Client, Version};\
use serde::{Deserialize, Serialize};\
use std::net::{IpAddr, SocketAddr};\
use std::time::Instant;\
use std::str::FromStr;

use hickory_proto::op::{Message, Query, ResponseCode};\
use hickory_proto::rr::{Name, RecordType, RData};\
use hickory_proto::rr::rdata::svcb::{SvcParamKey, IpHint};

// \--- 1\. 输入配置 (已更新) \---\
\#\[derive(Debug, Deserialize, Clone)\]\
struct InputTask {\
doh_resolve_domain: String, // 1\. 用于 DoH 查询，获取 IP Hint 的域名\
test_sni_host: String, // 2\. 用于 TLS SNI 的域名\
test_host_header: String, // 3\. 用于 HTTP Host Header 的值\
doh_url: String,\
port: u16,\
prefer_ipv6: Option\<bool\>,\
}

// \--- 2\. 输出结果 (保持不变) \---\
\#\[derive(Debug, Serialize)\]\
struct TestResult {\
domain_used: String, // DoH 解析的域名\
target_ip: String,\
ip_version: String,\
sni_host: String, // 实际使用的 SNI\
host_header: String, // 实际使用的 Host header\
success: bool,\
status_code: Option\<u16\>,\
protocol: String,\
latency_ms: Option\<u64\>,\
server_header: Option\<String\>,\
error_msg: Option\<String\>,\
}

// \--- 3\. DoH 二进制查询与解析 (保持不变) \---\
async fn resolve_https_binary(client: \&Client, doh_url: &str, domain: &str)
\-\> Result\<Vec\<IpAddr\>\> {\
let mut query_msg \= Message::new();\
let name \= Name::from_str(domain).context("Invalid domain name format")?;\
let query \= Query::query(name.clone(), RecordType::HTTPS);\
query_msg.add_query(query);\
let query_bytes \= query_msg.to_vec().context("Failed to serialize DNS query")?;

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
    let dns\_msg \= Message::from\_vec(\&resp\_bytes).context("Failed to parse DNS wire format")?;

    if dns\_msg.response\_code() \!= ResponseCode::NoError {
        return Err(anyhow::anyhow\!("DNS query returned error code: {}", dns\_msg.response\_code()));
    }

    let mut ips \= Vec::new();
    for record in dns\_msg.answers() {
        if let Some(RData::HTTPS(svcb)) \= record.data() {
            for param in svcb.svc\_params() {
                match param {
                    SvcParamKey::Ipv4Hint(IpHint(hints)) \=\> {
                        for ip in hints { ips.push(IpAddr::V4(\*ip)); }
                    }
                    SvcParamKey::Ipv6Hint(IpHint(hints)) \=\> {
                        for ip in hints { ips.push(IpAddr::V6(\*ip)); }
                    }
                    \_ \=\> {}
                }
            }
        }
    }
    Ok(ips)

}

// \--- 4\. HTTP/3 连通性测试 (已更新) \---\
async fn test_connectivity(task: InputTask, ip: IpAddr) \-\> TestResult {\
// 关键点 1: URL 决定 SNI。我们使用 test_sni_host 构造 URL。\
let url \= format\!("https://{}:{}/", task.test_sni_host, task.port);

    let socket\_addr \= SocketAddr::new(ip, task.port);
    let ip\_ver \= if ip.is\_ipv6() { "IPv6" } else { "IPv4" };

    let client\_build \= Client::builder()
        // 关键点 2: 强制 IP 连接
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

// \--- 5\. 主程序 \---\
\#\[tokio::main\]\
async fn main() {\
let doh_http_client \= Client::builder().use_rustls_tls().build().expect("Failed
to create DoH client");

    // 示例输入:
    // 1\. 使用 A 域名获取优选 IP
    // 2\. 使用 B 域名作为 SNI/Host 进行测试 (确保证书/路由匹配)
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
        // 可以在这里添加测试不同 SNI/Host 的任务
        /\*
        ,{
            "doh\_resolve\_domain": "local-aria2-webui.masx200.ddns-ip.net",
            "test\_sni\_host": "local-aria2-webui.masx200.ddns-ip.net",
            "test\_host\_header": "local-aria2-webui.masx200.ddns-ip.net",
            "doh\_url": "...",
            "port": 443,
            "prefer\_ipv6": true
        }
        \*/
    \]
    "\#;

    // ... (后续的解析和运行逻辑保持不变)

    let tasks: Vec\<InputTask\> \= serde\_json::from\_str(input\_json).expect("Invalid JSON");
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

    println\!("\\n=== 测试结果 \===");
    println\!("{}", serde\_json::to\_string\_pretty(\&results).unwrap());

}

现在，你的测试工具拥有了完整的、专业的网络身份伪装和连通性测试能力，可以精确地测试
Cloudflare 等 CDN 提供的优选 IP 上的特定虚拟主机服务。
