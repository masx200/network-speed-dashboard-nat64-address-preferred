# NAT64 测试项目 - 快速开始

## 📦 项目文件清单

已创建的核心文件：

```
├── nat64_test.go              # NAT64 测试主程序
├── nat64-services.json        # NAT64 服务配置（7个服务提供商）
├── test-targets.json          # 测试目标列表（7个常用服务）
├── NAT64-README.md            # 详细文档
├── go.mod                     # Go 依赖（已有）
├── run-nat64-test.bat         # Windows 启动脚本
└── run-nat64-test.sh          # Linux/macOS 启动脚本
```

## 🚀 三步开始使用

### 步骤 1: 编译程序

```bash
go build -o nat64-tester nat64_test.go
```

### 步骤 2: 运行测试

```bash
# Windows
nat64-tester.exe -verbose

# Linux/macOS
./nat64-tester -verbose
```

### 步骤 3: 查看结果

测试会生成 JSON 报告：
```
nat64-test-report-20240305-153045.json
```

## 📊 测试覆盖范围

### NAT64 服务提供商（7个）

| 提供商 | 位置 | DNS64 服务器 | NAT64 前缀 |
|--------|------|--------------|------------|
| Kasper Dupont | 德国纽伦堡 | 2a00:1098:2b::1 | 2a00:1098:2b::/96 |
| Kasper Dupont | 英国伦敦 | 2a00:1098:2b::1 | 2a00:1098:2b::/96 |
| Kasper Dupont | 芬兰赫尔辛基 | 2a00:1098:2b::1 | 2a00:1098:2b::/96 |
| level66.services | 德国 Anycast | 2001:67c:2960::64 | 2001:67c:2960:6464::/96 |
| Trex | 芬兰坦佩雷 | 2001:67c:2b0::4 | 2001:67c:2b0:db32:0:1::/96 |
| ZTVI | 美国弗里蒙特 | 2602:fc59:b0:9e::64 | 2602:fc59:b0:64::/96 |
| ZTVI | 美国芝加哥 | 2602:fc59:11:1::64 | 2602:fc59:11:64::/96 |

### 测试目标（7个）

- Google DNS (8.8.8.8)
- Cloudflare DNS (1.1.1.1)
- Google 搜索
- Cloudflare 官网
- Example 域名
- GitHub
- Wikipedia

## 🔧 命令行选项

```bash
# 基本测试
nat64-tester -verbose

# 调整并发数（默认 5）
nat64-tester -verbose -concurrency 10

# 调整超时时间（默认 10 秒）
nat64-tester -verbose -timeout 15

# 使用自定义配置文件
nat64-tester -services my-services.json -targets my-targets.json

# 指定输出文件名
nat64-tester -output my-report.json
```

## 📈 测试指标

测试程序会输出以下指标：

1. **DNS64 解析测试**
   - 解析成功率
   - 平均解析延迟

2. **NAT64 连接测试**
   - 连接成功率
   - 平均连接延迟
   - HTTP 状态码

3. **汇总统计**
   - 总测试数量
   - 成功/失败数量
   - 按服务提供商的统计

## 📝 输出示例

```
成功加载 7 个 NAT64 服务
成功加载 7 个测试目标

开始测试...
✅ [Germany / Nürnberg] dns.google -> 2a00:1098:2b::1
✅ [Germany / Anycast] one.one.one.one -> 2001:67c:2960::64
❌ [U.S.A. / Fremont] www.google.com -> 2602:fc59:b0:9e::64
...
完成 98 个测试

✅ 测试完成！结果已保存到 nat64-test-report-20240305-153045.json

=== 测试摘要 ===
总测试数: 98
成功: 85 | 失败: 13
DNS64 成功率: 96.94%
连接成功率: 86.73%
平均 DNS 延迟: 52.34 ms
平均连接延迟: 145.67 ms
```

## 🎯 下一步

### 添加更多 NAT64 服务

编辑 `nat64-services.json`：

```json
{
  "services": [
    {
      "provider": "你的服务商",
      "location": "城市",
      "dns64_servers": ["2001:db8::64"],
      "nat64_prefixes": ["2001:db8:64::/96"]
    }
  ]
}
```

### 添加更多测试目标

编辑 `test-targets.json`：

```json
{
  "targets": [
    {
      "name": "服务名称",
      "host": "example.com",
      "expected_ip": "93.184.216.34",
      "description": "描述"
    }
  ]
}
```

### 前端可视化仪表板

可以基于现有的 Vue.js 仪表板改造：

1. 读取 NAT64 测试报告 JSON
2. 显示服务提供商、地理位置、成功率等
3. 添加延迟图表和趋势分析

## 🐛 故障排除

### 编译错误

```
go: cannot find module
```

解决方法：
```bash
go mod init nat64-tester
go get github.com/miekg/dns
```

### 运行时错误

```
listen tcp :0: bind: address already in use
```

这是正常的，程序会自动重试。

### DNS 解析失败

检查你的网络连接，确保可以访问 IPv6 网络。

## 📚 相关资源

- [nat64.xyz](https://nat64.xyz/) - 公共 NAT64 服务列表
- [RFC 6147 - DNS64](https://datatracker.ietf.org/doc/html/rfc6147)
- [RFC 6146 - NAT64](https://datatracker.ietf.org/doc/html/rfc6146)
- [当前项目目录](./)
