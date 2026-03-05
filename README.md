# NAT64 网络测速仪表板

![Vue](https://img.shields.io/badge/Vue-3-green.svg)
![Vite](https://img.shields.io/badge/Vite-7.0+-blue.svg)
![Vuetify](https://img.shields.io/badge/Vuetify-3.0+-blueviolet.svg)
![Go](https://img.shields.io/badge/Go-1.25+-00ADD8.svg)

这是一个用于测试和可视化 **NAT64/DNS64 服务** 以及 **Cloudflare 优选 IP**
的网络测速仪表板项目。包含 Vue.js 前端仪表板和 Go 语言后端测试工具。

## 功能特性

### NAT64/DNS64 测试工具

- **DNS64 解析测试**: 通过 DoH 查询 A 记录获取 IPv4 地址
- **NAT64 地址合成**: 手动合成 IPv6 地址（前缀:96 + IPv4:4 字节）
- **连接测试**: 验证通过 NAT64 的 HTTPS 连接
- **性能指标**: 记录 DNS 延迟和连接延迟
- **并发测试**: 支持多并发测试加速
- **JSON 报告**: 自动生成详细测试报告

### Vue.js 前端仪表板

- **动态数据加载**: 自动加载所有测试报告文件
- **多维度排序**: 按延迟、成功率等指标排序
- **高级筛选**: 按国家、ASN、IP 版本、协议等筛选
- **统计概览**: 实时显示测试统计指标
- **响应式设计**: Material Design 风格

## 快速开始

### 前置要求

- [Node.js](https://nodejs.org/) v16+
- [Go](https://golang.org/) 1.21+
- IPv6 网络连接（用于 NAT64 测试）

### 安装

```bash
# 克隆项目
git clone <your-repository-url>
cd network-speed-dashboard-nat64-address-preferred

# 安装前端依赖
npm install
```

### 运行 NAT64 测试

```bash
# 运行 NAT64/DNS64 测试
go run nat64.go -verbose

# 使用自定义配置
go run nat64.go -services nat64-services.json -targets test-targets.json -concurrency 10

# 指定 DoH 服务器
go run nat64.go -dohip 104.21.95.9 -doh-url https://deno-dns-over-https-server.g18uibxgnb.de5.net/
```

### 运行 HTTP/3 连通性测试

```bash
# 运行连通性测试
go run host_connectivity_check.go -verbose -dohip 104.21.95.9 --doh-url https://deno-dns-over-https-server.g18uibxgnb.de5.net/

# 指定并发数和超时
go run host_connectivity_check.go -concurrency 20 -timeout 15
```

### 启动前端仪表板

```bash
# 开发模式
npm run dev
# 访问 http://localhost:5173

# 构建生产版本
npm run build

# 预览生产构建
npm run preview
```

## 项目结构

```
network-speed-dashboard-nat64-address-preferred/
├── nat64.go                    # NAT64/DNS64 测试工具
├── host_connectivity_check.go  # HTTP/3 连通性测试工具
├── main.go                     # HTTP/3 主测试程序
├── extract_hosts.go            # 主机提取工具
├── nat64-services.json         # NAT64 服务配置
├── test-targets.json           # 测试目标配置
├── hosts.json                  # 主机列表
├── src/
│   ├── components/
│   │   └── SpeedDashboard.vue  # 主仪表板组件
│   ├── plugins/
│   │   └── vuetify.js          # Vuetify 配置
│   ├── App.vue
│   └── main.js
├── public/                     # 静态资源
├── vite.config.js              # Vite 配置
└── package.json
```

## 配置说明

### NAT64 服务配置 (`nat64-services.json`)

```json
{
  "services": [
    {
      "provider": "Kasper Dupont",
      "location": "Germany / Nürnberg",
      "website": "https://nat64.net/public-providers",
      "nat64_prefixes": ["2a00:1098:2b::/96"],
      "dot": "dot.nat64.dk"
    }
  ]
}
```

### 测试目标配置 (`test-targets.json`)

```json
{
  "targets": [
    {
      "protocol": "https",
      "name": "IPLeak.net",
      "host": "ipv4.ipleak.net",
      "path": "/?mode=json",
      "description": "IPv4 地理位置查询"
    }
  ]
}
```

## 命令行选项

### nat64.go

| 选项           | 默认值                                              | 说明                       |
| -------------- | --------------------------------------------------- | -------------------------- |
| `-verbose`     | false                                               | 详细输出                   |
| `-concurrency` | 10                                                  | 并发测试数量               |
| `-timeout`     | 10                                                  | 超时时间(秒)               |
| `-services`    | nat64-services.json                                 | NAT64 服务配置文件         |
| `-targets`     | test-targets.json                                   | 测试目标配置文件           |
| `-output`      | 自动生成                                            | 输出文件名                 |
| `-doh-url`     | https://pngwczx94z.cloudflare-gateway.com/dns-query | DoH 服务 URL               |
| `-dohip`       | 162.159.36.20                                       | 强制解析 DoH 域名到指定 IP |

### host_connectivity_check.go

| 选项           | 默认值     | 说明                        |
| -------------- | ---------- | --------------------------- |
| `-verbose`     | false      | 详细输出                    |
| `-concurrency` | 10         | 并发测试数量                |
| `-timeout`     | 10         | 超时时间(秒)                |
| `-input`       | hosts.json | 输入文件路径                |
| `-ip-version`  | all        | IP 版本过滤 (ipv4/ipv6/all) |

## 测试报告格式

### NAT64 测试报告 (`nat64-test-report-*.json`)

```json
{
  "report_info": {
    "generated_at": "2024-03-05T15:30:45Z",
    "tool_version": "1.0.0"
  },
  "test_results": [
    {
      "timestamp": "2024-03-05T15:30:45Z",
      "service_provider": "Kasper Dupont",
      "service_location": "Germany / Nürnberg",
      "nat64_prefix": "2a00:1098:2b::/96",
      "target_host": "ipv4.ipleak.net",
      "original_ipv4": "104.21.95.9",
      "synthesized_ipv6": "2a00:1098:2b::6815:5f09",
      "dns64_success": true,
      "dns64_latency_ms": 45,
      "connect_success": true,
      "connect_latency_ms": 123,
      "http_status_code": 200
    }
  ],
  "summary": {
    "total_tests": 28,
    "success_tests": 25,
    "failed_tests": 3,
    "dns64_success_rate": 96.43,
    "connect_success_rate": 89.29,
    "avg_dns_latency_ms": 52.34,
    "avg_connect_latency_ms": 145.67
  }
}
```

### 连通性测试报告 (`failed-test-report-*.json`)

```json
{
  "report_info": { "generated_at": "timestamp" },
  "test_environment": {
    "ip_info": { "country": "China", "asn": "AS24400" }
  },
  "top_latency_records": [...],
  "failed_tests": [...]
}
```

## 技术架构

### NAT64 测试流程

```
┌─────────────────┐
│  加载配置文件    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  通过 DoH 查询   │
│  A 记录获取 IPv4 │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  手动合成 IPv6   │
│  前缀(12B)+IPv4(4B)│
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  测试 HTTPS 连接 │
│  使用合成 IPv6   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  生成 JSON 报告  │
└─────────────────┘
```

### 技术栈

**前端:**

- Vue 3 (Composition API)
- Vite 7
- Vuetify 3
- TypeScript

**后端:**

- Go 1.25
- miekg/dns (DNS 消息处理)
- HTTP/3 支持 (quic-go)

## npm 脚本

```bash
npm run dev          # 启动开发服务器
npm run build        # 构建生产版本
npm run preview      # 预览生产构建
npm run type-check   # TypeScript 类型检查
npm run fetch        # 获取 Cloudflare IPs
npm run extract      # 提取和合并 IP 信息
npm run clean        # 清理旧报告
npm run start        # 运行完整测试流程
```

## 支持的 NAT64 服务商

| 服务商           | 位置          | NAT64 前缀                 |
| ---------------- | ------------- | -------------------------- |
| Kasper Dupont    | 德国/纽伦堡   | 2a00:1098:2b::/96          |
| Kasper Dupont    | 英国/伦敦     | 2a00:1098:2c:1::/96        |
| Kasper Dupont    | 芬兰/赫尔辛基 | 2a01:4f8:c2c:123f:64::/96  |
| level66.services | 德国/Anycast  | 2001:67c:2960:6464::/96    |
| Trex             | 芬兰/坦佩雷   | 2001:67c:2b0:db32:0:1::/96 |
| ZTVI             | 美国/弗里蒙特 | 2602:fc59:b0:64::/96       |
| ZTVI             | 美国/芝加哥   | 2602:fc59:11:64::/96       |

## 参考资料

- [RFC 6147 - DNS64](https://datatracker.ietf.org/doc/html/rfc6147)
- [RFC 6146 - NAT64](https://datatracker.ietf.org/doc/html/rfc6146)
- [RFC 7050 - NAT64 前缀发现](https://datatracker.ietf.org/doc/html/rfc7050)
- [nat64.net](https://nat64.net/) - 公共 NAT64 服务

## 许可证

[MIT](LICENSE)
