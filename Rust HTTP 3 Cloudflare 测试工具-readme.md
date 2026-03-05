# Rust HTTP/3 Cloudflare 测试工具

这是一个用于测试 Cloudflare HTTP/3 服务的 Rust 项目。

## 功能特性

- 使用 HTTP/3 协议与 Cloudflare 服务进行通信
- 支持 QUIC 协议进行快速安全的网络传输
- 可用于测试 Cloudflare Workers 和 Pages 等服务的 HTTP/3 支持

## 使用要求

- Rust 编程语言环境
- Cargo 构建工具
- 支持 HTTP/3 和 QUIC 协议的网络环境

## 安装使用

1. 克隆项目仓库:

```bash
git clone https://gitee.com/masx200/golang-http3-cloudflare-test-tool.git
```

2. 进入项目目录:

```bash
cd golang-http3-cloudflare-test-tool
```

3. 构建项目:

```bash
cargo build --release
```

4. 运行测试:

```bash
cargo test
```

## 贡献指南

欢迎贡献代码和改进。请遵循以下步骤:

1. Fork 项目仓库
2. 创建新分支
3. 提交代码更改
4. 创建 Pull Request

## 许可协议

本项目采用 MIT 许可协议。详情请查看 LICENSE 文件。
