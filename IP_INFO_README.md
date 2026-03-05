# IP地址信息集成功能说明

本项目已成功集成IP地址信息获取功能，现在在运行测试时会自动获取当前的网络环境信息并包含在失败测试报告中。

## 功能特性

### 🌐 IP地址信息获取

- **多API支持**: 使用 `ipinfo.io` 和 `ifconfig.co` 两个API服务
- **容错机制**: 如果一个API失败，会自动尝试另一个
- **丰富信息**: 获取IP地址、地理位置、网络组织、时区等详细信息
- **安全认证**: 使用Bearer Token进行API认证

### 📊 报告集成

- **Markdown报告**: 在测试失败报告中显示详细的网络环境信息
- **JSON报告**: 在结构化数据中包含IP信息
- **命令行输出**: 在控制台显示IP信息摘要

## 修改的脚本

### npm run check

```bash
# 修改前
go run -v host_connectivity_check.go -verbose

# 修改后
node ip-info.js && go run -v host_connectivity_check.go -verbose
```

### npm run start

```bash
# 修改前
node generate-test-report.js

# 修改后
node ip-info.js && node generate-test-report.js
```

## 新增文件

### ip-info.js

IP地址信息获取工具，支持：

- 异步获取IP信息
- 多API容错机制
- 格式化输出（Markdown/JSON）
- 错误处理和重试

### generate-test-report.js (增强)

测试报告生成器已增强，新增功能：

- 集成IP信息获取
- 在报告中包含网络环境信息
- 新增命令行选项 `--no-ip-info` 禁用IP信息

## 使用示例

### 获取IP信息

```bash
# 直接运行IP信息获取
node ip-info.js

# 示例输出
🔍 开始获取当前IP地址信息...
正在从 ipinfo.io 获取IP信息...
✅ ipinfo.io 获取成功: 240e:b8e:35cd:a601:111d:b508:1568:acd5 (China)
正在从 ifconfig.co 获取IP信息...
✅ ifconfig.co 获取成功: 240e:b8e:35cd:a601:111d:b508:1568:acd5 (China)
✅ IP信息获取完成
   IP地址: 240e:b8e:35cd:a601:111d:b508:1568:acd5
   国家: China (CN)
   ASN: AS4134
   组织: Chinanet
   坐标: 34.7732, 113.722
   时区: Asia/Shanghai
```

### 生成包含IP信息的测试报告

```bash
npm run start
```

### 生成不包含IP信息的测试报告

```bash
node generate-test-report.js --no-ip-info
```

## 报告中的IP信息示例

### Markdown报告

```
## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 14:50:37
- **IP地址**: 240e:b8e:35cd:a601:111d:b508:1568:acd5
- **国家/地区**: China (CN)
- **ASN**: AS4134
- **网络组织**: Chinanet
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined
```

### JSON报告

```json
{
  "test_environment": {
    "timestamp": "2025-12-12T06:50:37.465Z",
    "ip_info": {
      "ip": "240e:b8e:35cd:a601:111d:b508:1568:acd5",
      "asn": "AS4134",
      "as_name": "Chinanet",
      "country_code": "CN",
      "country": "China",
      "latitude": 34.7732,
      "longitude": 113.722,
      "time_zone": "Asia/Shanghai",
      "source": "combined"
    }
  }
}
```

## API配置

### ipinfo.io

- **URL**: `https://api.ipinfo.io/lite/me`
- **认证**: Bearer Token
- **信息**: IP、ASN、网络组织、地理位置

### ifconfig.co

- **URL**: `https://ifconfig.co/json`
- **认证**: 无需认证
- **信息**: IP、地理位置、坐标、时区、User-Agent

## 错误处理

- 如果API调用失败，会设置默认值避免报告生成中断
- 如果两个API都失败，会在报告中记录错误信息
- 网络连接问题不会影响主要的测试功能

## 技术实现

- **语言**: JavaScript (Node.js)
- **HTTP请求**: 使用 `child_process.exec` 调用 curl
- **JSON解析**: 原生 `JSON.parse()`
- **错误处理**: try-catch + 默认值机制
- **异步支持**: async/await 模式

## 注意事项

1. 需要系统安装 `curl` 命令
2. 需要能访问外网（国际互联网）
3. IP信息仅供参考，具体位置可能因VPN等原因有偏差
4. 某些网络环境下API可能不稳定，已实现容错机制
