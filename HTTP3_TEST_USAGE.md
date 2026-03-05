# HTTP/3 测试工具使用指南

## 简介

这是一个基于 Rust h3 库的纯 HTTP/3 测试工具，可以直接与支持 HTTP/3 的服务器建立
QUIC 连接并发送请求。

## 构建程序

```bash
cargo build --release
```

## 使用方法

### 基本用法

```bash
# 测试默认域名 (local-aria2-webui.masx200.ddns-ip.net)
cargo run

# 测试指定域名
cargo run -- --domain example.com

# 测试指定路径
cargo run -- --domain local-aria2-webui.masx200.ddns-ip.net --path "/cdn-cgi/trace"

# 查看帮助
cargo run -- --help
```

### 参数说明

- `-d, --domain <DOMAIN>`: 测试域名 (默认:
  local-aria2-webui.masx200.ddns-ip.net)
- `-p, --port <PORT>`: 端口号 (默认: 443)
- `-t, --path <PATH>`: 请求路径 (默认: /)
- `--timeout <SECONDS>`: 超时时间 (默认: 10 秒)
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

### 示例

#### 1. 测试 Cloudflare HTTP/3 支持

```bash
cargo run -- --domain local-aria2-webui.masx200.ddns-ip.net --path "/cdn-cgi/trace"
```

#### 2. 测试其他支持 HTTP/3 的网站

```bash
cargo run -- --domain google.com
cargo run -- --domain facebook.com
```

#### 3. 设置环境变量查看详细日志

```bash
RUST_LOG=info cargo run -- --domain local-aria2-webui.masx200.ddns-ip.net
```

## 输出示例

成功运行的输出：

```
🚀 开始 HTTP/3 测试: local-aria2-webui.masx200.ddns-ip.net:443
✅ DNS 解析成功: local-aria2-webui.masx200.ddns-ip.net -> [2606:4700::6810:85e5]:443
✅ QUIC 连接建立成功，耗时: 1.4653407s
📡 发送 HTTP/3 请求: https://local-aria2-webui.masx200.ddns-ip.net/
📨 收到响应: 301 Moved Permanently HTTP/3.0
📋 响应头: { ... }
✅ HTTP/3 测试成功！状态码: 301 Moved Permanently, 响应大小: 167 字节

✅ HTTP/3 测试完成！
```

## 技术特性

### 核心库

- **h3**: HTTP/3 协议实现
- **h3-quinn**: QUIC 传输层实现
- **quinn**: QUIC 协议栈
- **rustls**: TLS 加密

### 实现细节

1. **DNS 解析**: 使用系统 DNS 解析域名
2. **QUIC 连接**: 建立基于 UDP 的 QUIC 连接
3. **TLS 加密**: 使用系统证书进行 TLS 握手
4. **ALPN 协商**: 协商使用 HTTP/3 协议 (ALPN: h3)
5. **HTTP/3 请求**: 发送 HTTP/3 请求并接收响应

### 支持的协议

- ✅ HTTP/3 over QUIC
- ✅ IPv4 和 IPv6
- ✅ TLS 1.3
- ✅ 连接复用
- ✅ 流控

## 故障排除

### 常见问题

1. **DNS 解析失败**
   - 检查域名是否正确
   - 检查网络连接

2. **连接超时**
   - 增加超时时间: `--timeout 30`
   - 检查防火墙设置

3. **TLS 握手失败**
   - 检查系统证书是否完整
   - 尝试访问其他支持 HTTP/3 的网站

4. **QUIC 连接失败**
   - 检查是否支持 UDP (某些网络环境可能阻止 UDP)
   - 尝试使用不同的网络

### 日志调试

```bash
# 启用详细日志
RUST_LOG=trace cargo run

# 仅查看应用日志
RUST_LOG=golang_http3_cloudflare_test_tool cargo run
```

## 注意事项

1. **网络要求**: 需要支持 UDP 的网络环境
2. **证书**: 程序使用系统根证书进行 TLS 验证
3. **IPv6**: 默认优先使用 IPv6，可以通过 DNS 解析控制
4. **超时**: 默认 10 秒超时，可根据网络环境调整

## 版本信息

- 版本: 1.0.0
- Rust: 2021 Edition
- 目标: HTTP/3 (RFC 9114)

## 许可证

MIT License
