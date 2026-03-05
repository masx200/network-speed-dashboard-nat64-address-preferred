# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## 项目概述

这是一个用Rust开发的HTTP/3连接测试工具，主要用于测试Cloudflare和其他支持HTTP/3的服务的连通性和性能。项目已成功迁移到使用本地下载的第三方库进行开发。

## 核心功能

### DNS解析功能

- **Hickory-DNS (v0.25.2)**: 使用下载的hickory-dns库替代trust-dns，支持RFC
  8484标准的完整DNS解析
  - DoH (DNS over HTTPS): 使用Hickory-DNS进行符合RFC 8484的DoH查询
  - HTTPS记录解析: 支持HTTPS (类型65) DNS记录以获取SVCB参数
  - A/AAAA记录解析: 支持传统的IPv4和IPv6地址解析
  - 直接IP模式: 支持手动指定IP地址进行测试
- **Reqwest (v0.12.24)**: 使用下载的reqwest库，支持HTTP/3协议
  - HTTP/3支持: 使用quinn协议进行HTTP/3连接
  - 完整的TLS支持: 使用本地rustls库
  - 回退兼容: 保留JSON API作为Hickory-DNS失败时的回退方案

### HTTP/3连接测试

- 使用reqwest库进行HTTP/3协议通信
- 支持SNI (Server Name Indication) 配置
- 可配置目标端口 (默认443)
- 收集连接指标：延迟、协议版本、响应码等

## 关键架构决策

### 第三方库集成

**重要**: 项目已成功从crates.io版本迁移到本地下载的第三方库：

1. **Hickory-DNS替代**:
   - 从 `trust-dns-resolver = "0.23.0-alpha.5"` 迁移到
     `hickory-resolver = { path = "./hickory-dns-main/crates/resolver", features = ["tokio"] }`
   - 支持完整的RFC 8484 DNS解析器
   - DoH查询使用标准的DNS-over-HTTPS协议

2. **Reqwest本地化**:
   - 从 `reqwest = { version = "0.12", features = [...] }` 迁移到
     `reqwest = { path = "./reqwest-master/reqwest-master", features = [...] }`
   - 支持最新的HTTP/3特性（需要`RUSTFLAGS='--cfg reqwest_unstable'`）

### 依赖管理更新

**关键依赖变更**:

```toml
[dependencies]
# HTTP 客户端 - 使用本地 reqwest 库
reqwest = { path = "./reqwest-master/reqwest-master", features = ["json", "rustls-tls", "http3", "gzip", "brotli"] }

# Hickory-DNS - 使用本地 hickory-dns 库
hickory-resolver = { path = "./hickory-dns-main/crates/resolver", features = ["tokio"] }
hickory-client = { path = "./hickory-dns-main/crates/client" }
hickory-proto = { path = "./hickory-dns-main/crates/proto" }

# 其他必需依赖
rustls = { version = "0.23", default-features = false, features = ["std", "tls12"] }
webpki-roots = "1"
ring = "0.17"
futures = "0.3"
```

## 开发指南

### HTTP/3连接测试

要启用HTTP/3支持，必须使用以下环境变量：

```bash
# 启用reqwest不稳定特性以获得HTTP/3功能
export RUSTFLAGS='--cfg reqwest_unstable'

# 构建项目
cargo build --release

# 运行程序
cargo run --release
```

### DNS解析模式

程序支持三种解析模式：

1. **https模式**: 使用Hickory-DNS进行DoH查询（推荐）
   - 符合RFC 8484标准
   - 支持完整的DNS over HTTPS协议

2. **a_aaaa模式**: 使用Hickory-DNS进行传统A/AAAA记录查询
   - 兼容性更好
   - 适用于不需要DoH的场景

3. **direct模式**: 使用预定义的IP地址
   - 最快的连接方式
   - 跳过DNS解析直接连接

## 构建说明

### 成功标准

项目能够成功构建并运行，输出JSON格式的测试结果，包含：

- 域名解析结果
- IP地址信息
- HTTP/3连接测试结果
- 协议版本 (http/1.1, http/2, h3)
- 连接延迟
- 服务器响应信息

### 故障排除

#### 常见构建问题

1. **RUSTFLAGS未设置**: HTTP/3功能需要`--cfg reqwest_unstable`

   ```bash
   # 解决方案
   export RUSTFLAGS='--cfg reqwest_unstable'
   cargo build --release
   ```

2. **路径依赖错误**: 确保本地库路径正确

   ```bash
   # 检查路径
   ls -la hickory-dns-main/ reqwest-master/
   ```

3. **版本不兼容**: 本地库版本可能与API不兼容
   ```bash
   # 清理并重新构建
   cargo clean
   cargo build --release
   ```

## 版本历史

### v0.2.0 (当前版本)

- 成功迁移到本地第三方库
- 支持Hickory-DNS v0.25.2完整DNS解析
- 支持Reqwest v0.12.24与HTTP/3
- 改进的错误处理和日志记录
- 三种DNS解析模式支持

## 部署注意事项

### 开发环境要求

- Rust 1.64.0+
- Tokio运行时
- 网络连接（用于DoH查询）
- 目标服务器的HTTP/3支持

### 生产环境

- 确保使用有效的TLS证书
- 配置适当的超时设置
- 监控连接池和资源使用

## 许可证

本项目采用MIT许可证。详见LICENSE文件。

## 项目概述

这是一个用Rust开发的HTTP/3连接测试工具，主要用于测试Cloudflare和其他支持HTTP/3的服务的连通性和性能。项目使用DNS
over HTTPS (DoH)解析目标域名的IP地址，并进行HTTP/3连接测试。

## 核心功能

### DNS解析功能

- **DoH JSON API**: 调用Google DNS、Cloudflare DNS等DoH服务
- **HTTPS记录解析**: 解析HTTPS (类型65) DNS记录以获取SVCB参数
- **A/AAAA记录解析**: 支持传统的IPv4和IPv6地址解析
- **直接IP模式**: 支持手动指定IP地址进行测试

### HTTP/3连接测试

- 使用reqwest库进行HTTP/3协议通信
- 支持SNI (Server Name Indication) 配置
- 可配置目标端口 (默认443)
- 收集连接指标：延迟、协议版本、响应码等

## 关键架构决策

### DNS解析器演进

**重要**: 项目已从正则表达式DNS解析迁移到符合RFC 8484标准的解析方式：

1. **原始实现**: 使用简单正则表达式匹配 `ipv4hint=` 和 `ipv6hint=` 模式
2. **当前实现**: 优先使用RFC 8484标准，保留正则表达式作为回退方案
3. **未来计划**: 完整的trust-dns库集成以支持完整的二进制DNS消息解析

### 错误处理策略

- 使用anyhow库进行统一的错误处理
- DNS解析失败时提供清晰的错误信息
- 网络连接错误时记录详细信息

## 常用命令

### 构建和测试

```bash
# 构建项目
cargo build --release

# 运行测试
pnpx cross-env RUSTFLAGS='--cfg reqwest_unstable' powershell -c '&"C:\Program Files\Rust stable MSVC 1.91\bin\cargo.EXE" test --package golang-http3-cloudflare-test-tool --bin golang-http3-cloudflare-test-tool -- doh_docs_test::doh_docs_integration_tests test_test::tests --nocapture'
# 运行程序
cargo run --release
```

### 开发环境变量

```bash
# 启用HTTP/3不稳定特性 (如需要)
RUSTFLAGS='--cfg reqwest_unstable' cargo build

# 使用特定DNS解析器
RUSTFLAGS='--cfg reqwest_unstable' cargo run
```

## 代码结构

```
src/
├── main.rs              # 主程序入口和核心逻辑
├── dns_parser.rs       # DNS解析相关功能 (未来模块化)
├── http_client.rs      # HTTP/3客户端封装 (未来模块化)
└── types.rs           # 数据结构定义 (未来模块化)
```

## 配置文件格式

程序接受JSON格式的配置，支持以下字段：

```json
[
  {
    "doh_resolve_domain": "example.com",
    "test_sni_host": "example.com",
    "test_host_header": "example.com",
    "doh_url": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
    "port": 443,
    "prefer_ipv6": false,
    "resolve_mode": "https", // "https", "a_aaaa", "direct"
    "direct_ips": ["1.1.1.1", "2606:4700:4700::1"] // 仅用于direct模式
  }
]
```

## 开发指南

### 添加新的DoH服务提供商

1. 在 `InputTask` 结构中添加新的URL模板
2. 确保URL格式支持正确的查询参数
3. 测试新服务的响应格式兼容性

### 添加新的DNS记录类型支持

1. 在 `parse_https_hints` 函数中添加新的解析逻辑
2. 更新RFC 8484兼容的解析器
3. 添加相应的测试用例

### HTTP/3协议扩展

1. 在reqwest配置中添加新的HTTP/3设置
2. 实现特定的ALPN协议协商
3. 添加性能测试和基准测试

## 测试策略

### 单元测试

- 运行 `cargo test` 执行所有单元测试
- 重点测试DNS解析逻辑的正确性
- 验证HTTP/3连接建立过程

### 集成测试

- 使用真实HTTP/3服务进行端到端测试
- 测试不同网络条件下的性能
- 验证与主流Cloudflare服务的兼容性

### 性能基准

- 使用内置的延迟测量功能
- 对比不同DoH服务的响应时间
- 分析HTTP/3 vs HTTP/2 vs HTTP/1.1性能差异

## 部署注意事项

### 生产环境

- 确保使用TLS证书验证
- 配置适当的超时设置
- 监控连接池和资源使用

### 安全考虑

- 不在日志中记录敏感信息
- 验证DNS响应的完整性
- 使用安全的HTTP客户端配置

## 故障排除

### 常见问题

1. **DNS解析失败**: 检查DoH服务URL和网络连接
2. **HTTP/3握手失败**: 确认目标服务器支持HTTP/3
3. **证书错误**: 验证SNI配置和证书链
4. **性能问题**: 检查网络延迟和服务器响应时间

### 调试技巧

- 使用 `-vv` 参数增加详细日志输出
- 先用简单的A/AAA记录测试连通性
- 逐步增加复杂性：先测试HTTP/1.1，再HTTP/2，最后HTTP/3

## 贡献指南

### 代码风格

- 使用 `cargo fmt` 进行代码格式化
- 使用 `cargo clippy` 进行静态分析
- 遵循Rust官方命名约定

### 提交流程

1. 创建功能分支：`git checkout -b feature/new-feature`
2. 提交流码：`git commit -m "feat: add new feature"`
3. 推送分支：`git push origin feature/new-feature`
4. 创建Pull Request并描述变更内容

### 文档更新

- 更新README.md中的新功能说明
- 添加代码注释解释复杂逻辑
- 更新CHANGELOG.md记录重要变更

## 版本历史

### v0.1.0 (当前)

- 基本的HTTP/3连接测试功能
- 支持DoH JSON API DNS解析
- 正则表达式HTTPS记录解析

### v0.2.0 (计划中)

- RFC 8484兼容的DNS解析器
- 改进的错误处理和日志
- 模块化的代码结构

## 许可证

本项目采用MIT许可证。详见LICENSE文件。

## 联系方式

- 项目主页: [GitHub仓库地址]
- 问题反馈: [GitHub Issues]
- 文档: [文档链接]

---

**注意**: 此文档应与代码实现保持同步。当添加新功能时，请相应更新此文件。
