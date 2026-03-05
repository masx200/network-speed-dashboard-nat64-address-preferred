# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:14:04
- **数据来源**: connectivity_results-20251212-121404.json
- **总测试数**: 444
- **失败测试数**: 4
- **成功测试数**: 440
- **失败率**: 0.90%
- **平均延迟**: 75.61ms
- **最小延迟**: 49ms
- **最大延迟**: 661ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:14:04
- **IP地址**: 2a09:bac5:9f26:188c::272:ec
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 36.4766, -78.1847
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 4 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名               | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ----------------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 37   | cfip.xxxxxxxx.tk        | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 133  | 172.64.201.25           | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 151  | uriah.ns.cloudflare.com | 162.159.44.194 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.194:443: i/o timeout |
| 152  | uriah.ns.cloudflare.com | 172.64.35.194  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.194:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 4 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.64（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 182  | www.wto.org                | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 86   | silkbook.com               | 2606:4700:20::681a:9a0    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 183  | www.wto.org                | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 63   | ip.sb                      | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 264  | www.gov.ua                 | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 15   | time.is                    | 2606:4700:20::681a:d36    | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 24   | www.glassdoor.com          | 104.16.25.46              | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 52   | benedict.ns.cloudflare.com | 108.162.195.205           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 60   | ip.sb                      | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 301  | www.okcupid.com            | 104.16.144.63             | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 320  | yx-auto.pages.dev          | 2606:4700:310c::ac42:2c90 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 354  | toy-people.com             | 104.26.2.36               | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 14   | time.is                    | 2606:4700:20::681a:c36    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 27   | trevor.ns.cloudflare.com   | 162.159.44.154            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 38   | cfip.xxxxxxxx.tk           | 104.16.241.229            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 79   | singapore.com              | 172.67.75.194             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 81   | singapore.com              | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 87   | silkbook.com               | 2606:4700:20::681a:8a0    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 144  | www.digitalocean.com       | 104.19.174.68             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 226  | cmcc.877774.xyz            | 104.16.149.10             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 253  | ipinfo.in                  | 2606:4700:3031::6815:1581 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 284  | icook.hk                   | 2606:4700:3037::ac43:a168 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 294  | www.visa.com.sg            | 104.18.13.229             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 304  | www.okcupid.com            | 104.18.160.63             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 339  | cu.877774.xyz              | 104.26.4.118              | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 19   | rustam.ns.cloudflare.com   | 172.64.35.148             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 100  | ashton.ns.cloudflare.com   | 162.159.44.173            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 112  | julio.ns.cloudflare.com    | 108.162.195.209           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 150  | uriah.ns.cloudflare.com    | 108.162.195.194           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 175  | 172.67.181.209             | 172.67.181.209            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 202  | japan.com                  | 2606:4700:20::681a:43c    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 234  | abdullah.ns.cloudflare.com | 2606:4700:58::a29f:2ccb   | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 251  | ipinfo.in                  | 104.21.21.129             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 295  | www.visa.com.sg            | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 297  | cf.877771.xyz              | 104.21.80.180             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 310  | www.hugedomains.com        | 2606:4700:20::ac43:46bf   | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 311  | www.hugedomains.com        | 2606:4700:20::681a:725    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 358  | toy-people.com             | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 6    | comicabc.com               | 2606:4700:3036::6815:400a | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 10   | www.ipchicken.com          | 172.67.68.101             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 59   | ip.sb                      | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 68   | palera.in                  | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 71   | ip.gs                      | 2606:4700:3036::6815:eb0  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 77   | singapore.com              | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 82   | singapore.com              | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 132  | www.udemy.com              | 2606:4700::6810:8eed      | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 138  | icook.tw                   | 104.20.28.74              | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 140  | icook.tw                   | 2606:4700:10::6814:1c4a   | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 186  | lewis.ns.cloudflare.com    | 162.159.44.159            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 210  | cmcc.877774.xyz            | 104.16.148.7              | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 261  | 104.26.6.112               | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 282  | icook.hk                   | 104.21.90.210             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 291  | huxley.ns.cloudflare.com   | 2606:4700:58::a29f:2cbc   | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 307  | www.hugedomains.com        | 172.67.70.191             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 360  | asia.877774.xyz            | 104.16.211.153            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 366  | zread.ai                   | 188.114.97.3              | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 398  | bestcf.030101.xyz          | 162.159.133.251           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 16   | time.is                    | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 22   | rustam.ns.cloudflare.com   | 2803:f800:50::6ca2:c394   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 80   | singapore.com              | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 105  | dnschecker.org             | 104.26.7.89               | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 115  | julio.ns.cloudflare.com    | 2803:f800:50::6ca2:c3d1   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 145  | www.digitalocean.com       | 104.19.173.68             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 146  | www.digitalocean.com       | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 161  | eur.877774.xyz             | 104.21.47.209             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 179  | ifconfig.co                | 172.67.168.106            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 191  | lewis.ns.cloudflare.com    | 2803:f800:50::6ca2:c39f   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 200  | japan.com                  | 104.26.5.60               | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 214  | cmcc.877774.xyz            | 104.16.148.11             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 238  | 104.16.45.84               | 104.16.45.84              | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 240  | ipv4.ip.sb                 | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 242  | cf.0sm.com                 | 104.21.7.133              | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 260  | 104.18.37.13               | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 380  | kyree.ns.cloudflare.com    | 2606:4700:58::a29f:2ccf   | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 388  | cloudflare-ip.mofashi.ltd  | 172.67.155.172            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 23   | www.glassdoor.com          | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 32   | wilson.ns.cloudflare.com   | 108.162.195.110           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 36   | wilson.ns.cloudflare.com   | 2606:4700:58::a29f:2c6e   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 70   | ip.gs                      | 104.21.14.176             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 76   | whatismyipaddress.com      | 2606:4700::6813:df4f      | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 114  | julio.ns.cloudflare.com    | 2606:4700:58::a29f:2cd1   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 128  | tasteatlas.com             | 2606:4700::6811:2569      | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 130  | www.udemy.com              | 104.16.142.237            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 156  | 104.18.78.214              | 104.18.78.214             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 173  | damien.ns.cloudflare.com   | 2606:4700:58::a29f:2ca8   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 185  | www.wto.org                | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 213  | cmcc.877774.xyz            | 104.16.148.10             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 215  | cmcc.877774.xyz            | 104.16.148.12             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 230  | cmcc.877774.xyz            | 104.16.148.1              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 248  | www.pcmag.com              | 2606:4700::6810:1576      | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 259  | steamdb.info               | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 299  | cf.877771.xyz              | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 321  | yx-auto.pages.dev          | 2606:4700:310c::ac42:2f70 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 337  | cu.877774.xyz              | 104.26.4.111              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 340  | cu.877774.xyz              | 104.26.4.114              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 342  | pranab.ns.cloudflare.com   | 172.64.35.199             | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 355  | toy-people.com             | 104.26.3.36               | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 365  | zread.ai                   | 188.114.96.3              | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 370  | decker.ns.cloudflare.com   | 108.162.195.155           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 7    | comicabc.com               | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 99 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
