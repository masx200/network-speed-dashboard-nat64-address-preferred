# NAT64/DNS64 服务测试工具

这是一个用于测试公共 NAT64/DNS64 服务的性能和可用性的工具集。

## 项目简介

NAT64 是一种 IPv6 过渡技术，允许仅有 IPv6 连接的客户端访问 IPv4 资源。本项目通过测试各个公共 NAT64 服务的：

- **DNS64 解析能力** - 正确合成 IPv6 地址
- **NAT64 连通性** - 成功访问 IPv4 资源
- **性能指标** - DNS 查询延迟、HTTP 连接延迟

来帮助用户选择最适合的 NAT64 服务提供商。

## 项目结构

```
.
├── nat64_test.go          # NAT64 测试主程序
├── nat64-services.json    # NAT64 服务配置列表
├── test-targets.json      # 测试目标配置
├── go.mod                 # Go 依赖管理
└── README.md              # 本文件
```

## 快速开始

### 1. 安装依赖

```bash
go mod init nat64-tester
go get github.com/miekg/dns
```

### 2. 编译程序

```bash
go build -o nat64-tester nat64_test.go
```

### 3. 运行测试

```bash
# 基本测试
./nat64-tester -verbose

# 自定义并发和超时
./nat64-tester -verbose -concurrency 10 -timeout 15

# 使用自定义配置文件
./nat64-tester -services my-services.json -targets my-targets.json
```

### 4. 查看结果

测试完成后会生成 JSON 报告文件：

```bash
nat64-test-report-20240305-153045.json
```

报告包含：
- 每个服务的详细测试结果
- DNS64 解析成功率和延迟
- 连接成功率和延迟
- 按地理位置和服务提供商的统计数据

## 配置文件说明

### nat64-services.json

定义要测试的 NAT64 服务列表：

```json
{
  "services": [
    {
      "provider": "服务提供商名称",
      "location": "地理位置",
      "website": "https://example.com",
      "dns64_servers": ["2001:67c:2960::64"],
      "nat64_prefixes": ["2001:67c:2960:6464::/96"],
      "dot": "dot.example.com"
    }
  ]
}
```

### test-targets.json

定义要测试的 IPv4 目标：

```json
{
  "targets": [
    {
      "name": "Google",
      "host": "www.google.com",
      "expected_ip": "142.250.185.36",
      "description": "Google 搜索"
    }
  ]
}
```

## 命令行选项

| 选项 | 默认值 | 说明 |
|------|--------|------|
| `-verbose` | false | 显示详细测试输出 |
| `-concurrency` | 5 | 并发测试数量 |
| `-timeout` | 10 | 超时时间（秒） |
| `-services` | nat64-services.json | NAT64 服务配置文件 |
| `-targets` | test-targets.json | 测试目标配置文件 |
| `-output` | (自动生成) | 输出文件名 |

## 测试流程

对于每个 NAT64 服务和测试目标组合：

1. **DNS64 解析测试**
   - 查询 DNS64 服务器获取 AAAA 记录
   - 验证返回的 IPv6 地址是否在 NAT64 前缀范围内
   - 记录解析延迟

2. **NAT64 连接测试**
   - 使用合成的 IPv6 地址建立 HTTPS 连接
   - 验证 HTTP 状态码
   - 记录连接延迟

3. **结果汇总**
   - 计算成功率
   - 计算平均延迟
   - 生成 JSON 报告

## 当前支持的 NAT64 服务

- **Kasper Dupont** (德国纽伦堡、英国伦敦、芬兰赫尔辛基)
- **level66.services** (德国 Anycast)
- **Trex** (芬兰坦佩雷)
- **ZTVI** (美国弗里蒙特、芝加哥)

## 技术细节

### NAT64 地址合成

NAT64 使用以下格式合成 IPv6 地址：

```
[NAT64 前缀]:[IPv4 地址嵌入]
```

例如，使用 NAT64 前缀 `2001:67c:2960:6464::/96` 访问 `8.8.8.8`：

```
2001:67c:2960:6464::808:808
```

### DNS64 工作原理

1. 客户端向 DNS64 服务器查询 AAAA 记录
2. DNS64 服务器查询 A 记录（IPv4 地址）
3. DNS64 服务器将 IPv4 地址嵌入 NAT64 前缀
4. 返回合成的 IPv6 地址给客户端

## 输出格式

### 单个测试结果

```json
{
  "timestamp": "2024-03-05T15:30:45Z",
  "service_provider": "Kasper Dupont",
  "service_location": "Germany / Nürnberg",
  "dns_server": "2a00:1098:2b::1",
  "nat64_prefix": "2a00:1098:2b::/96",
  "target_host": "www.google.com",
  "original_ipv4": "142.250.185.36",
  "synthesized_ipv6": "2a00:1098:2b::8efa:b924",
  "dns64_success": true,
  "dns64_latency_ms": 45,
  "connect_success": true,
  "connect_latency_ms": 123,
  "http_status_code": 200
}
```

## 后续开发计划

- [ ] 添加前端可视化仪表板
- [ ] 支持测试历史记录和趋势分析
- [ ] 添加邮件/短信告警功能
- [ ] 支持自定义测试间隔和持续监控
- [ ] 添加更多 NAT64 服务提供商

## 参考资源

- [nat64.xyz - 公共 NAT64 服务列表](https://nat64.xyz/)
- [RFC 6147 - DNS64](https://datatracker.ietf.org/doc/html/rfc6147)
- [RFC 6146 - NAT64](https://datatracker.ietf.org/doc/html/rfc6146)

## 许可证

MIT License

## 贡献

欢迎提交 Pull Request 或 Issue！
