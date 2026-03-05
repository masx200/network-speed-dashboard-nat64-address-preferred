这是一个非常专业且深入的问题。你不仅关注了底层的 QUIC
协议库，还涉及到具体的网络连通性测试和 Rust 的实战开发。

下面我将分三部分回答：首先介绍 Cloudflare 的 quiche 及 Rust 的 HTTP/3
现状，其次分析你提供的 curl 输出（其中有一个关键问题），最后提供你需要的 Rust
代码实现。

---

### **1\. cloudflare/quiche 与 Rust 的 HTTP/3 支持现状**

#### **什么是 quiche?**

quiche 是 Cloudflare 开源的一个 **QUIC 传输协议** 和 **HTTP/3** 的实现。

- **底层库 (Low-level):** 它处理 QUIC
  的连接握手、包的加密解密、拥塞控制和丢包恢复。
- **IO 无关 (IO-free):** 这是一个非常重要的特性。quiche 不直接处理 socket (UDP
  套接字)。你需要自己负责从 UDP socket 读取数据，喂给 quiche，然后把 quiche
  生成的数据写入 socket。这使得它可以轻松集成到现有的事件循环（如 Nginx、Envoy
  或 Rust 的 Tokio）中。
- **FFI 支持:** 它提供了 C 语言绑定，因此 curl、Nginx 等非 Rust 软件也能调用它。

#### **Rust HTTP/3 支持现状**

目前 Rust 在 HTTP/3 领域的生态主要有两大支柱：

1. **quiche (Cloudflare):**
   - **优点:** 经过 Cloudflare 边缘网络的实战考验，性能极其强悍，非常稳定。
   - **缺点:** 抽象程度低。如果你只是想发起一个 HTTP 请求，直接用 quiche
     会非常痛苦（需要几百行代码来处理 UDP socket 和 Event Loop）。
2. **quinn \+ h3 (社区驱动):**
   - quinn: 也是一个纯 Rust 的 QUIC 实现，比 quiche 更贴合 Rust 的 Future/Async
     模型。
   - reqwest: Rust 最流行的 HTTP 客户端。**目前的 reqwest (v0.12+) 的 HTTP/3
     功能主要基于 quinn 或 h3，而不是 quiche。**

**总结:** 如果你要造轮子（开发网关、代理），用
quiche；如果你只是想写个工具测试网站（就像你的需求），使用集成好的 reqwest
或基于 quinn 的库会更高效。

---

### **2\. 分析你的 Curl 命令与输出**

**关键发现：你的 Curl 测试实际上并没有走 HTTP/3。**

请看你提供的日志：

- ALPN: curl offers h2,http/1.1\
  ...
- ALPN: server accepted http/1.1
- using HTTP/1.x

原因分析：\
尽管你加了 \--http3 参数，但你的 curl
版本（curl-8.12.1_4-win64-mingw）似乎并没有正确编译/链接 HTTP/3 的后端库（如
ngtcp2+quiche 或 ngtcp2+openssl-quic），或者握手阶段降级了。它只提供了 h2 和
http/1.1 给服务器，服务器选择了 http/1.1。\
**这也解释了为什么你需要用 Rust 写一个测试工具：**
它可以让你更精准地控制协议握手，而不依赖系统中可能配置不当的 curl。

---

### **3\. Rust 实战：Cloudflare 连通性测试工具**

为了满足你的需求（测试特定 IP、强制 ALPN、输出 JSON），我将使用 Rust 最强大的
HTTP 客户端框架 reqwest。虽然它底层默认不用
quiche，但它能完美实现“测试连通性”和“强制解析
IP”的功能，且代码可读性远高于直接操作 UDP。

#### **项目设置 (Cargo.toml)**

你需要配置 http3 特性。

Ini, TOML

\[package\]\
name \= "cf-tester"\
version \= "0.1.0"\
edition \= "2021"

\[dependencies\]\
reqwest \= { version \= "0.12", features \= \["json", "rustls-tls", "http3"\] }\
tokio \= { version \= "1", features \= \["full"\] }\
serde \= { version \= "1", features \= \["derive"\] }\
serde_json \= "1"\
anyhow \= "1"

#### **Rust 代码实现 (main.rs)**

这段代码实现了你的核心需求：

1. **输入解析**: 接受 JSON 数组。
2. **DNS 劫持 (--resolve)**: 使用 resolve_to_addrs 强制将域名指向特定 IP。
3. **协议强制**: 尝试协商 HTTP/3，并检查结果。
4. **JSON 输出**: 包含延迟、实际使用的协议版本、状态码等。

Rust

use reqwest::{Client, Version};\
use serde::{Deserialize, Serialize};\
use std::net::SocketAddr;\
use std::time::Instant;\
use std::sync::Arc;\
use anyhow::Result;

// \--- 1\. 定义输入数据结构 \---\
\#\[derive(Debug, Deserialize)\]\
struct TestTarget {\
domain: String,\
ip: String, // 目标 IP (IPv4 或 IPv6)\
port: u16, // 端口，通常是 443\
alpn: String, // "h3" 或 "h2"\
}

// \--- 2\. 定义输出结果结构 \---\
\#\[derive(Debug, Serialize)\]\
struct TestResult {\
domain: String,\
target_ip: String,\
success: bool,\
status_code: Option\<u16\>,\
protocol_version: String, // 实际协商的协议 (HTTP/1.1, h2, h3)\
latency_ms: Option\<u64\>,\
error_msg: Option\<String\>,\
}

// \--- 3\. 核心测试函数 \---\
async fn test_connectivity(target: \&TestTarget) \-\> TestResult {\
let url \= format\!("https://{}:{}/meta", target.domain, target.port); // 使用
/meta 端点，正如你 curl 命令中用的

    // 解析 IP 地址
    let addr\_str \= format\!("{}:{}", target.ip, target.port);
    let socket\_addr: SocketAddr \= match addr\_str.parse() {
        Ok(addr) \=\> addr,
        Err(e) \=\> return TestResult::fail(target, format\!("IP Parse Error: {}", e)),
    };

    // 构建 Client
    // reqwest 的 http3 支持是“尝试性”的。如果服务器支持，它会尝试升级，
    // 或者我们可以通过预先知道服务器支持 h3 来加速 (ClientBuilder 配置)。
    let mut builder \= Client::builder()
        .resolve\_to\_addrs(\&target.domain, &\[socket\_addr\]) // 对应 curl 的 \--resolve
        .danger\_accept\_invalid\_certs(true) // 为了测试方便，忽略证书域名匹配问题（因为我们可能强制解析了 IP）
        .timeout(std::time::Duration::from\_secs(5));

    // 根据需求开启 HTTP/3
    if target.alpn.eq\_ignore\_ascii\_case("h3") {
        builder \= builder.http3\_prior\_knowledge(); // 强制尝试 HTTP/3
    } else {
        // 如果是 h2，reqwest 默认会自动协商，但我们可以关闭 h3
        builder \= builder.http2\_prior\_knowledge();
    }

    let client \= match builder.build() {
        Ok(c) \=\> c,
        Err(e) \=\> return TestResult::fail(target, format\!("Client Build Error: {}", e)),
    };

    let start \= Instant::now();

    // 发起请求
    match client.get(\&url).send().await {
        Ok(response) \=\> {
            let latency \= start.elapsed().as\_millis() as u64;
            let status \= response.status().as\_u16();
            let version \= response.version();

            // 将 Version 枚举转换为字符串
            let version\_str \= match version {
                Version::HTTP\_09 \=\> "HTTP/0.9",
                Version::HTTP\_10 \=\> "HTTP/1.0",
                Version::HTTP\_11 \=\> "HTTP/1.1",
                Version::HTTP\_2 \=\> "h2",
                Version::HTTP\_3 \=\> "h3",
                \_ \=\> "Unknown",
            };

            TestResult {
                domain: target.domain.clone(),
                target\_ip: target.ip.clone(),
                success: status \== 200, // 假设 200 才算完全成功
                status\_code: Some(status),
                protocol\_version: version\_str.to\_string(),
                latency\_ms: Some(latency),
                error\_msg: None,
            }
        }
        Err(e) \=\> TestResult::fail(target, format\!("Request Error: {}", e)),
    }

}

impl TestResult {\
fn fail(target: \&TestTarget, msg: String) \-\> Self {\
TestResult {\
domain: target.domain.clone(),\
target_ip: target.ip.clone(),\
success: false,\
status_code: None,\
protocol_version: "None".to_string(),\
latency_ms: None,\
error_msg: Some(msg),\
}\
}\
}

// \--- 4\. 主程序入口 \---\
\#\[tokio::main\]\
async fn main() {\
// 模拟输入 JSON (这里你可以改为从文件读取或标准输入读取)\
// 注意：local-aria2-webui.masx200.ddns-ip.net 的 IPv6 需要真实可用\
let input_json \= r\#"\
\[\
{\
"domain": "local-aria2-webui.masx200.ddns-ip.net",\
"ip": "2606:4700:7::da",\
"port": 443,\
"alpn": "h3"\
},\
{\
"domain": "local-aria2-webui.masx200.ddns-ip.net",\
"ip": "162.159.140.220",\
"port": 443,\
"alpn": "h2"\
},\
{\
"domain": "local-aria2-webui.masx200.ddns-ip.net",\
"ip": "1.1.1.1",\
"port": 443,\
"alpn": "h3"\
}\
\]\
"\#;

    let targets: Vec\<TestTarget\> \= serde\_json::from\_str(input\_json).expect("JSON format error");
    let mut results \= Vec::new();

    // 并发测试所有目标
    let mut handles \= Vec::new();
    for target in targets {
        // 为了所有权转移，这里 clone 一下，实际生产中可以用 Arc
        // 这里为了简单直接传值
        handles.push(tokio::spawn(async move {
            test\_connectivity(\&target).await
        }));
    }

    for handle in handles {
        if let Ok(res) \= handle.await {
            results.push(res);
        }
    }

    // 输出最终 JSON
    let output\_json \= serde\_json::to\_string\_pretty(\&results).unwrap();
    println\!("{}", output\_json);

}

### **4\. 代码解析与使用说明**

1. **http3_prior_knowledge()**: 这个方法非常关键。标准的 HTTP/3 通常需要先建立
   HTTP/1.1 或 H2 连接，收到 Alt-Svc
   头后才升级。但因为你要直接测试连通性，我们假设服务器支持，直接强制使用 HTTP/3
   握手。
2. **resolve_to_addrs**: 这完全对应了 curl 的 \--resolve 参数。它绕过了本地 DNS
   解析，直接向你指定的 IP 发送 SNI 为 local-aria2-webui.masx200.ddns-ip.net
   的握手包。
3. **结果验证**: 输出中的 protocol_version
   字段会告诉你实际使用了什么协议。如果输出是 h3，说明 UDP 连通且 QUIC
   握手成功；如果是 h2 或 HTTP/1.1，说明 H3 失败降级了。

#### **如何运行**

1. 确保安装了 Rust。
2. 创建一个新项目：cargo new cf_tester。
3. 将上面的 Cargo.toml 和 main.rs 内容复制进去。
4. 运行：cargo run。

#### **预期输出示例**

JSON

\[\
{\
"domain": "local-aria2-webui.masx200.ddns-ip.net",\
"target_ip": "2606:4700:7::da",\
"success": true,\
"status_code": 200,\
"protocol_version": "h3",\
"latency_ms": 145,\
"error_msg": null\
},\
{\
"domain": "local-aria2-webui.masx200.ddns-ip.net",\
"target_ip": "1.1.1.1",\
"success": false,\
"status_code": null,\
"protocol_version": "None",\
"latency_ms": null,\
"error_msg": "Request Error: error sending request for url
(https://local-aria2-webui.masx200.ddns-ip.net:443/meta)"\
}\
\]

### **下一步建议**

你可以根据这个代码基础，添加读取 stdin 的功能，这样你就可以通过管道将 IP
列表传给这个工具了：

PowerShell

\# Windows PowerShell 示例\
Get-Content ip_list.json | cargo run \-\-release
