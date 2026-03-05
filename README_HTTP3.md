# HTTP/3 测试功能实现

## 概述

本项目已成功集成了完整的 HTTP/3 测试功能，支持以下特性：

- 🚀 **原生 HTTP/3 测试**: 使用 `h3` 和 `h3-quinn` 库进行真正的 HTTP/3 连接测试
- 🔗 **协议协商和回退**: 自动尝试 HTTP/3，失败后回退到 HTTP/2 和 HTTP/1.1
- 🌍 **DNS 解析集成**: 支持 RFC 8484 DNS over HTTPS (DoH)、传统 DNS 和直接 IP
  模式
- 📊 **综合测试报告**: 详细的测试结果、协议分布、延迟统计和错误分析
- ⚙️ **并发测试**: 支持多域名、多 IP 的并发 HTTP/3 测试
- 📝️ **多种输出格式**: JSON、表格和文本报告

## 快速开始

### 1. 构建项目

```bash
# 启用 HTTP/3 实验特性
export RUSTFLAGS='--cfg reqwest_unstable'

# 构建发布版本
cargo build --release

# 或者直接运行
cargo run --bin rust-http3-cloudflare-test-tool
```

### 2. 基本使用

```bash
# 运行所有测试模式
./rust-http3-cloudflare-test-tool

# 仅测试 HTTP/3 (原生 h3)
./rust-http3-cloudflare-test-tool --mode native_h3

# 测试特定域名
./rust-http3-cloudflare-test-tool --domains local-aria2-webui.masx200.ddns-ip.net,google.com

# 启用 IPv6 测试
./rust-http3-cloudflare-test-tool --ipv6

# 仅表格输出
./rust-http3-cloudflare-test-tool --output table
```

### 3. 使用配置文件

```bash
# 使用配置文件
./rust-http3-cloudflare-test-tool --config http3_config.json
```

## 测试模式说明

### 1. 原生 HTTP/3 测试 (`native_h3`)

使用 `h3` 库和 `h3-quinn` QUIC 传输进行真正的 HTTP/3 连接：

- ✅ 支持 HTTP/3 特性：0-RTT、QPACK、Server Push
- 🔍 详细的协议信息：连接 ID、流 ID、ALPN 协议
- ⚡️ 高级配置：字段段大小限制、grease、数据报支持

```bash
# 原生 HTTP/3 测试
./rust-http3-cloudflare-test-tool --mode native_h3 --domains local-aria2-webui.masx200.ddns-ip.net
```

### 2. 集成测试 (`integration` / `reqwest_h3`)

使用 `reqwest` 库进行 HTTP/3 测试，包含协议协商：

- 🔄 自动回退：HTTP/3 → HTTP/2 → HTTP/1.1
- 🔍 协议检测：通过响应头和版本检测实际使用的协议
- 📡 兼容性测试：确保与现有服务器的兼容性

```bash
# 集成测试（包含回退机制）
./rust-http3-cloudflare-test-tool --mode integration --domains google.com
```

### 3. 全面测试 (`all`)

运行所有测试模式，提供完整的 HTTP/3 兼容性分析：

```bash
# 运行所有测试模式
./rust-http3-cloudflare-test-tool --mode all --domains local-aria2-webui.masx200.ddns-ip.net,google.com
```

## 命令行选项

| 选项             | 短选项 | 默认值                                                          | 说明                                                      |
| ---------------- | ------ | --------------------------------------------------------------- | --------------------------------------------------------- |
| `--mode`         | `-m`   | `all`                                                           | 测试模式：`native_h3`、`reqwest_h3`、`integration`、`all` |
| `--domains`      | `-d`   | `local-aria2-webui.masx200.ddns-ip.net,google.com,facebook.com` | 目标域名（逗号分隔）                                      |
| `--output`       | `-o`   | `all`                                                           | 输出格式：`json`、`table`、`all`                          |
| `--timeout`      | `-t`   | `30`                                                            | 请求超时时间（秒）                                        |
| `--config`       | `-c`   | -                                                               | 配置文件路径（JSON 格式）                                 |
| `--ipv6`         | `-6`   | `false`                                                         | 启用 IPv6 测试                                            |
| `--resolve-mode` | `-r`   | `https`                                                         | DNS 解析模式：`https`、`a_aaaa`、`direct`                 |
| `--doh-server`   | `-s`   | 预设 DoH 服务器                                                 | DNS over HTTPS 服务器 URL                                 |
| `--help`         | `-h`   | -                                                               | 显示帮助信息                                              |
| `--version`      | `-V`   | -                                                               | 显示版本信息                                              |

## DNS 解析模式

### 1. RFC 8484 DNS over HTTPS (`https`)

使用 RFC 8484 标准的 DNS over HTTPS：

```bash
# 使用 DoH
./rust-http3-cloudflare-test-tool --resolve-mode https --domains local-aria2-webui.masx200.ddns-ip.net
```

### 2. 传统 DNS (`a_aaaa`)

传统的 A 和 AAAA 记录查询：

```bash
# 使用传统 DNS
./rust-http3-cloudflare-test-tool --resolve-mode a_aaaa --domains google.com
```

### 3. 直接 IP (`direct`)

绕过 DNS，直接使用指定的 IP 地址：

```bash
# 使用配置文件中的直接 IP
./rust-http3-cloudflare-test-tool --config http3_config.json
```

## 输出格式

### 1. JSON 输出

结构化的 JSON 结果，包含所有测试细节：

```json
[
  {
    "test_mode": "native_h3",
    "target_domain": "local-aria2-webui.masx200.ddns-ip.net",
    "target_ip": "104.16.123.64",
    "ip_version": "IPv4",
    "test_path": "/cdn-cgi/trace",
    "success": true,
    "status_code": 200,
    "protocol_detected": "HTTP/3",
    "latency_ms": 245,
    "response_size": 1024,
    "alpn_protocol": "h3",
    "additional_metrics": {
      "connection_id": "abc123",
      "stream_id": 4
    }
  }
]
```

### 2. 表格输出

简洁的表格格式，便于快速查看：

```
域名                 IP地址           版本      协议          状态     延迟    大小    ALPN     测试方法   错误
========================================================================================================================================================
local-aria2-webui.masx200.ddns-ip.net       104.16.123.64   IPv4     HTTP/3         成功     245ms    1024B    h3       native_h3
google.com           142.250.196.206 IPv4     HTTP/3         成功     189ms    2048B    h3       native_h3
```

### 3. 综合报告

详细的文本报告，包含统计和分析：

```
=== HTTP/3 综合测试报告 ===

总测试数: 15
成功: 12
失败: 3
成功率: 80.00%

📡 按域名统计:
  local-aria2-webui.masx200.ddns-ip.net: 4/4 (100.00% 成功)
  google.com: 4/5 (80.00% 成功)
  facebook.com: 4/6 (66.67% 成功)

🔗 协议分布:
  HTTP/3: 8 (66.67%)
  h2: 4 (33.33%)

🔐 ALPN 协议分布:
  h3: 8 (66.67%)
  h2: 4 (33.33%)

⏱️  延迟统计 (ms):
  平均: 234.50
  最小: 89
  最大: 567
  中位数: 198

❌ 错误统计:
  连接失败: 2
  超时: 1
```

## 配置文件

配置文件使用 JSON 格式，可以覆盖所有命令行选项：

```json
{
  "test_mode": "all",
  "target_domains": ["local-aria2-webui.masx200.ddns-ip.net", "google.com"],
  "output_format": "all",
  "max_concurrent_tests": 10,
  "timeout_seconds": 30,
  "enable_ipv6": false,
  "dns_resolve_mode": "https",
  "doh_server": "https://deno-dns-over-https-server.g18uibxgnb.de5.net/",
  "test_paths": ["/", "/api/v1/test"],
  "use_fallback": true,
  "max_field_section_size": 8192
}
```

## 高级功能

### 1. HTTP/3 特性测试

- **0-RTT 数据传输**: 测试 HTTP/3 的 0-RTT 功能
- **QPACK 压缩**: 测试 QPACK 头场压缩
- **Server Push**: 检测服务器推送功能
- **Grease**: 测试协议的健壮性

### 2. 连接池和复用

- **连接复用**: 复用 HTTP/3 连接以提高性能
- **连接池**: 管理多个并发连接
- **健康检查**: 监控连接健康状态

### 3. 错误分析

- **详细错误分类**: 连接错误、协议错误、超时等
- **错误统计**: 错误类型分布和趋势分析
- **故障排除**: 提供解决建议和调试信息

## 故障排除

### 1. 常见问题

**构建错误**:

```bash
# 确保启用了必要的特性
export RUSTFLAGS='--cfg reqwest_unstable'
cargo build --release
```

**连接失败**:

```bash
# 检查网络连接和防火墙设置
./rust-http3-cloudflare-test-tool --timeout 60 --resolve-mode a_aaaa
```

**DNS 解析失败**:

```bash
# 使用不同的 DoH 服务器
./rust-http3-cloudflare-test-tool --doh-server https://deno-dns-over-https-server.g18uibxgnb.de5.net/
```

### 2. 调试和日志

启用详细输出进行调试：

```bash
# 使用 RUST_LOG 启用详细日志
RUST_LOG=debug ./rust-http3-cloudflare-test-tool --mode native_h3 --domains test.com
```

### 3. 性能优化

- **并发限制**: 调整 `max_concurrent_tests` 参数
- **超时设置**: 根据网络环境调整 `timeout_seconds`
- **连接池**: 启用连接复用提高性能

## 技术架构

### 1. 模块结构

```
src/
├── main.rs                     # 主程序入口
├── main_comprehensive_h3.rs   # 综合测试控制器
├── h3_direct_test.rs          # 原生 HTTP/3 测试
├── main_h3_test.rs            # 集成测试 (reqwest)
├── http3_test.rs               # DNS 解析和基础连接测试
└── Cargo.toml                  # 依赖配置
```

### 2. 核心依赖

- **h3**: HTTP/3 协议实现
- **h3-quinn**: QUIC 传输层实现
- **reqwest**: HTTP 客户端库（支持 HTTP/3）
- **quinn**: QUIC 协议实现
- **tokio**: 异步运行时
- **serde**: 序列化/反序列化
- **chrono**: 时间处理
- **clap**: 命令行解析

### 3. 协议支持

- **HTTP/3**: 基于 QUIC 的下一代 HTTP 协议
- **HTTP/2**: 基于 TCP/ TLS 的二进制协议
- **HTTP/1.1**: 传统的文本协议
- **DNS over HTTPS**: RFC 8484 安全 DNS 查询
- **ALPN**: 应用层协议协商

## 贡献和支持

### 1. 报告问题

如果遇到问题，请提供以下信息：

- 操作系统和版本
- Rust 版本 (`rustc --version`)
- 命令行参数
- 错误信息和堆栈跟踪
- 网络环境描述

### 2. 功能请求

欢迎提交功能请求和改进建议：

- 新的测试模式
- 额外的协议支持
- 性能优化
- 用户体验改进

### 3. 开发和测试

```bash
# 运行测试
cargo test

# 运行特定测试
cargo test native_h3

# 启用调试
cargo test -- --nocapture
```

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。
