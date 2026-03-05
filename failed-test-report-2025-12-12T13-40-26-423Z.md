# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 13:40:26
- **数据来源**: connectivity_results-20251212-134026.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 59.31ms
- **最小延迟**: 40ms
- **最大延迟**: 791ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 13:40:26
- **IP地址**: 2a09:bac5:6217:28::4:356
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 37.1835, -121.7714
- **时区**: America/Los_Angeles
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 39   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 375  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                   | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 158  | cris.ns.cloudflare.com      | 2803:f800:50::6ca2:c3ca   | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 319  | singapore.com               | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 12   | www.pcmag.com               | 104.16.20.118             | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 167  | decker.ns.cloudflare.com    | 162.159.44.155            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 157  | cris.ns.cloudflare.com      | 108.162.195.202           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 159  | cris.ns.cloudflare.com      | 2a06:98c1:50::ac40:23ca   | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 185  | kyree.ns.cloudflare.com     | 2a06:98c1:50::ac40:23cf   | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 239  | cf.zhetengsha.eu.org        | 104.18.35.15              | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 328  | silkbook.com                | 104.26.8.160              | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 339  | ashton.ns.cloudflare.com    | 2606:4700:58::a29f:2cad   | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 344  | dnschecker.org              | 172.67.73.216             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 374  | tasteatlas.com              | 2606:4700::6811:2569      | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 431  | www.wto.org                 | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 49   | 172.67.75.172               | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 61   | steamdb.info                | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 64   | ipinfo.in                   | 2606:4700:3037::ac43:c6cb | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 72   | www.gov.ua                  | 2606:4700:3033::ac43:d17f | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 111  | www.hugedomains.com         | 104.26.7.37               | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 114  | www.hugedomains.com         | 2606:4700:20::681a:725    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 165  | toy-people.com              | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 199  | www.whatismyip.com          | 172.67.69.129             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 201  | www.whatismyip.com          | 2606:4700:20::681a:c17    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 261  | braden.ns.cloudflare.com    | 2a06:98c1:50::ac40:23a9   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 275  | www.glassdoor.com           | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 288  | rustam.ns.cloudflare.com    | 2a06:98c1:50::ac40:2394   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 293  | palera.in                   | 172.67.157.122            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 301  | benedict.ns.cloudflare.com  | 2803:f800:50::6ca2:c3cd   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 306  | ip.sb                       | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 309  | ip.gs                       | 104.21.14.176             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 311  | ip.gs                       | 2606:4700:3035::ac43:a01c | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 317  | singapore.com               | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 333  | [2606:4700:964f::6e2c:588e] | 2606:4700:964f::6e2c:588e | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 372  | tasteatlas.com              | 104.17.36.105             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 373  | tasteatlas.com              | 2606:4700::6811:2469      | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 9    | www.ipget.net               | 2606:4700:3031::ac43:cf1a | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 44   | ipv4.ip.sb                  | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 73   | www.gov.ua                  | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 81   | iplocation.io               | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 113  | www.hugedomains.com         | 2606:4700:20::ac43:46bf   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 153  | pranab.ns.cloudflare.com    | 2a06:98c1:50::ac40:23c7   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 154  | pranab.ns.cloudflare.com    | 2606:4700:58::a29f:2cc7   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 171  | decker.ns.cloudflare.com    | 2803:f800:50::6ca2:c39b   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 173  | 104.16.223.179              | 104.16.223.179            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 174  | zread.ai                    | 104.21.76.240             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 179  | asia.877774.xyz             | 104.17.139.62             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 203  | www.whatismyip.com          | 2606:4700:20::ac43:4581   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 233  | cmcc.877774.xyz             | 104.16.149.9              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 247  | fbi.gov                     | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 250  | fbi.gov                     | 2606:4700::6810:95f4      | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 254  | cf.090227.xyz               | 2606:4700:4407::ac40:9052 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 266  | bowen.ns.cloudflare.com     | 2803:f800:50::6ca2:c353   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 271  | icook.tw                    | 104.20.28.74              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 321  | whatismyipaddress.com       | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 327  | silkbook.com                | 104.26.9.160              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 337  | ashton.ns.cloudflare.com    | 162.159.44.173            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 346  | dnschecker.org              | 104.26.7.89               | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 350  | cf.877774.xyz               | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 358  | julio.ns.cloudflare.com     | 2803:f800:50::6ca2:c3d1   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 383  | 104.18.37.40                | 104.18.37.40              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 390  | www.digitalocean.com        | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 412  | damien.ns.cloudflare.com    | 108.162.195.168           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 418  | 104.19.175.123              | 104.19.175.123            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 419  | ifconfig.co                 | 104.21.54.91              | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 45   | ipv4.ip.sb                  | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 68   | 172.67.110.232              | 172.67.110.232            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 74   | 103.160.204.59              | 103.160.204.59            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 97   | huxley.ns.cloudflare.com    | 2803:f800:50::6ca2:c3bc   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 99   | www.visa.cn                 | 162.159.153.2             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 101  | www.okcupid.com             | 104.16.239.254            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 122  | sullivan.ns.cloudflare.com  | 2a06:98c1:50::ac40:23a1   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 133  | craig.ns.cloudflare.com     | 2a06:98c1:50::ac40:23c0   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 134  | cu.877774.xyz               | 104.26.4.112              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 136  | cu.877774.xyz               | 104.26.4.119              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 139  | cu.877774.xyz               | 104.26.4.116              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 145  | na.877774.xyz               | 104.18.187.25             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 150  | pranab.ns.cloudflare.com    | 162.159.44.199            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 164  | toy-people.com              | 2606:4700:20::ac43:4812   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 166  | toy-people.com              | 2606:4700:20::681a:324    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 175  | zread.ai                    | 172.67.202.78             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 178  | asia.877774.xyz             | 104.16.211.153            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 182  | kyree.ns.cloudflare.com     | 172.64.35.207             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 196  | cloudflare-ip.mofashi.ltd   | 2606:4700:3037::6815:48e9 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 208  | 172.64.151.55               | 172.64.151.55             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 214  | cmcc.877774.xyz             | 104.16.149.2              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 226  | cmcc.877774.xyz             | 104.16.148.3              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 232  | cmcc.877774.xyz             | 104.16.149.10             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 236  | saas.sin.fan                | 104.26.0.111              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 237  | saas.sin.fan                | 104.26.1.111              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 240  | cf.zhetengsha.eu.org        | 172.64.152.241            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 242  | cf.zhetengsha.eu.org        | 2a06:98c1:3108::6812:2a62 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 248  | fbi.gov                     | 104.16.148.244            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 260  | braden.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a9   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 285  | rustam.ns.cloudflare.com    | 172.64.35.148             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 286  | rustam.ns.cloudflare.com    | 162.159.44.148            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 287  | rustam.ns.cloudflare.com    | 108.162.195.148           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 291  | 104.17.68.85                | 104.17.68.85              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 295  | palera.in                   | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 299  | benedict.ns.cloudflare.com  | 2606:4700:58::a29f:2ccd   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 305  | ip.sb                       | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 355  | julio.ns.cloudflare.com     | 108.162.195.209           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
