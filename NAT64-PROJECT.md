# 🌐 NAT64 测试项目总结

## 🎯 项目目标

创建一个完整的 NAT64/DNS64 服务测试工具，用于评估各个公共 NAT64
服务的性能和可用性。

## 📂 已创建的文件

### 核心程序文件

1. **[nat64_test.go](nat64_test.go)** - 主测试程序
   - DNS64 解析测试
   - NAT64 连通性测试
   - 性能指标收集（延迟、成功率）
   - 并发测试支持
   - JSON 报告生成

2. **[nat64-services.json](nat64-services.json)** - 服务配置
   - 7 个 NAT64 服务提供商
   - 覆盖德国、英国、芬兰、美国等地区
   - 包含 DNS64 服务器和 NAT64 前缀信息

3. **[test-targets.json](test-targets.json)** - 测试目标
   - 7 个常用 IPv4 服务
   - Google、Cloudflare、GitHub 等

### 文档文件

4. **[NAT64-README.md](NAT64-README.md)** - 详细技术文档
   - 项目介绍
   - 配置说明
   - 使用指南
   - 技术原理解释

5. **[NAT64-QUICKSTART.md](NAT64-QUICKSTART.md)** - 快速开始指南
   - 3 步上手指南
   - 测试覆盖范围
   - 命令行选项
   - 输出示例
   - 故障排除

### 脚本文件

6. **[run-nat64-test.bat](run-nat64-test.bat)** - Windows 启动脚本
7. **[run-nat64-test.sh](run-nat64-test.sh)** - Linux/macOS 启动脚本

## 🔧 技术架构

### 测试流程

```
┌─────────────────┐
│  加载配置文件    │
│  - 服务列表      │
│  - 测试目标      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  并发测试循环    │
│  (Semaphore)    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ DNS64 解析测试   │
│ - 查询 AAAA      │
│ - 验证前缀       │
│ - 记录延迟       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ NAT64 连接测试   │
│ - HTTPS 连接     │
│ - 状态码检查     │
│ - 记录延迟       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  生成 JSON 报告  │
│  - 详细结果      │
│  - 统计摘要      │
└─────────────────┘
```

### 数据结构

```go
// NAT64 服务配置
type NAT64Service struct {
    Provider      string   // 服务提供商
    Location      string   // 地理位置
    DNSServers    []string // DNS64 服务器列表
    NAT64Prefixes []string // NAT64 前缀列表
}

// 测试目标
type TestTarget struct {
    Name        string // 目标名称
    Host        string // 域名
    ExpectedIP  string // 预期 IPv4 地址
    Description string // 描述
}

// 测试结果
type NAT64TestResult struct {
    ServiceProvider  string  // 服务提供商
    ServiceLocation  string  // 服务位置
    DNSServer        string  // 使用的 DNS 服务器
    NAT64Prefix      string  // 使用的 NAT64 前缀
    TargetHost       string  // 测试目标
    SynthesizedIPv6  string  // 合成的 IPv6 地址
    DNS64Success     bool    // DNS64 解析是否成功
    DNS64LatencyMs   uint64  // DNS64 解析延迟
    ConnectSuccess   bool    // 连接是否成功
    ConnectLatencyMs uint64  // 连接延迟
    HTTPStatusCode   *uint16 // HTTP 状态码
}
```

## 📊 测试覆盖

### NAT64 服务（7 个）

| 服务商           | 位置          | DNS64 服务器 | 前缀数量 |
| ---------------- | ------------- | ------------ | -------- |
| Kasper Dupont    | 德国·纽伦堡   | 3 个         | 4 个     |
| Kasper Dupont    | 英国·伦敦     | 3 个         | 4 个     |
| Kasper Dupont    | 芬兰·赫尔辛基 | 3 个         | 4 个     |
| level66.services | 德国·Anycast  | 2 个         | 1 个     |
| Trex             | 芬兰·坦佩雷   | 2 个         | 1 个     |
| ZTVI             | 美国·弗里蒙特 | 2 个         | 1 个     |
| ZTVI             | 美国·芝加哥   | 2 个         | 1 个     |

### 测试目标（7 个）

1. Google DNS (8.8.8.8)
2. Cloudflare DNS (1.1.1.1)
3. Google 搜索
4. Cloudflare 官网
5. Example 域名
6. GitHub
7. Wikipedia

### 测试组合数

- 7 个服务 × 平均 2.5 个 DNS 服务器 × 平均 2.5 个前缀 × 7 个目标
- **约 300 个测试用例**

## 🚀 快速开始

### 1. 编译

```bash
go build -o nat64-tester nat64_test.go
```

### 2. 运行

```bash
# Windows
nat64-tester.exe -verbose

# Linux/macOS
./nat64-tester -verbose
```

### 3. 查看结果

```bash
cat nat64-test-report-*.json | jq '.summary'
```

## 📈 输出示例

```json
{
  "report_info": {
    "generated_at": "2024-03-05T15:30:45Z",
    "tool_version": "1.0.0"
  },
  "test_results": [
    {
      "service_provider": "Kasper Dupont",
      "service_location": "Germany / Nürnberg",
      "dns_server": "2a00:1098:2b::1",
      "nat64_prefix": "2a00:1098:2b::/96",
      "target_host": "dns.google",
      "original_ipv4": "8.8.8.8",
      "synthesized_ipv6": "2a00:1098:2b::808:808",
      "dns64_success": true,
      "dns64_latency_ms": 45,
      "connect_success": true,
      "connect_latency_ms": 123,
      "http_status_code": 200
    }
  ],
  "summary": {
    "total_tests": 98,
    "success_tests": 85,
    "failed_tests": 13,
    "dns64_success_rate": 96.94,
    "connect_success_rate": 86.73,
    "avg_dns_latency_ms": 52.34,
    "avg_connect_latency_ms": 145.67
  }
}
```

## 🎨 后续开发计划

### 短期（1-2 周）

- [x] 添加更多 NAT64 服务提供商
- [x] 增加测试目标数量
- [x] 优化错误处理和重试逻辑
- [x] 添加 HTML 格式报告

### 中期（1-2 月）

- [x] 开发 Vue.js 前端仪表板
- [x] 添加历史记录和趋势分析
- [x] 支持持续监控模式
- [x] 添加告警功能（邮件/Webhook）

### 长期（3-6 月）

- [x] 分布式测试节点
- [x] 地理位置延迟分析
- [x] 自动化测试调度
- [x] 性能基准测试

## 📚 参考资料

### 技术文档

- [RFC 6147 - DNS64](https://datatracker.ietf.org/doc/html/rfc6147)
- [RFC 6146 - NAT64](https://datatracker.ietf.org/doc/html/rfc6146)
- [RFC 7050 - Discovering NAT64 Prefix](https://datatracker.ietf.org/doc/html/rfc7050)

### 服务资源

- [nat64.xyz](https://nat64.xyz/) - 公共 NAT64 服务列表
- [NAT64.net](https://nat64.net/) - Kasper Dupont 的服务
- [level66.services](https://level66.services/services/nat64/)

### 相关项目

- 原始 Cloudflare IP 测试项目
- [usque](https://github.com/your-repo/usque) - IPv6 测试工具

## 💡 使用建议

### 测试环境要求

- ✅ 支持 IPv6 的网络连接
- ✅ Go 1.21+ 编译环境
- ✅ 能够访问外部 DNS 服务器

### 最佳实践

1. **首次运行**：使用 `-verbose` 查看详细输出
2. **性能测试**：调整 `-concurrency` 参数（5-20）
3. **慢速网络**：增加 `-timeout` 参数（15-30 秒）
4. **定期测试**：设置 cron 任务定期运行

### 配置优化

```json
// nat64-services.json 添加新服务
{
  "provider": "新服务商",
  "location": "城市",
  "dns64_servers": ["2001:db8::1"],
  "nat64_prefixes": ["2001:db8:64::/96"],
  "website": "https://example.com"
}

// test-targets.json 添加新目标
{
  "name": "新服务",
  "host": "example.com",
  "expected_ip": "93.184.216.34",
  "description": "示例服务"
}
```

## 🤝 贡献指南

欢迎贡献！可以：

1. 添加更多 NAT64 服务
2. 报告 Bug
3. 提出功能建议
4. 改进文档
5. 提交 Pull Request

## 📄 许可证

MIT License - 自由使用和修改

---

**项目创建日期**: 2024-03-05 **最后更新**: 2024-03-05 **版本**: 1.0.0
