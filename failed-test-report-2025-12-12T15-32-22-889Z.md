# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 15:32:22
- **数据来源**: connectivity_results-20251212-153222.json
- **总测试数**: 451
- **失败测试数**: 2
- **成功测试数**: 449
- **失败率**: 0.44%
- **平均延迟**: 83.59ms
- **最小延迟**: 59ms
- **最大延迟**: 1041ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 15:32:22
- **IP地址**: 2a09:bac5:7494:96::f:375
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6021, -93.6124
- **时区**: America/Chicago
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
| 145  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 324  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 63   | palera.in                             | 2a06:98c1:3120::3         | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 289  | www.hugedomains.com                   | 2606:4700:20::681a:625    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 353  | toy-people.com                        | 104.26.3.36               | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 19   | icook.tw                              | 2606:4700:10::6814:1c4a   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 72   | ip.gs                                 | 104.21.14.176             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 171  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 268  | iplocation.io                         | 172.67.70.100             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 278  | zread.ai                              | 104.21.76.240             | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 423  | cmcc.877774.xyz                       | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 42   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 64   | 104.17.68.85                          | 104.17.68.85              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 75   | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 141  | www.udemy.com                         | 104.16.143.237            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 183  | ifconfig.co                           | 2606:4700:3037::6815:365b | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 186  | www.wto.org                           | 104.18.41.190             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 190  | 104.18.37.13                          | 104.18.37.13              | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 192  | lewis.ns.cloudflare.com               | 172.64.35.159             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 244  | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 250  | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 267  | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 291  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 337  | na.877774.xyz                         | 104.18.38.235             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 386  | www.whatismyip.com                    | 172.67.69.129             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 391  | www.whatismyip.com                    | 2606:4700:20::ac43:4581   | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 438  | fbi.gov                               | 104.16.149.244            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 76   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 81   | 104.18.42.26                          | 104.18.42.26              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 84   | singapore.com                         | 172.67.75.194             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 85   | singapore.com                         | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 90   | www.visa.com.hk                       | 104.18.20.69              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 106  | dnschecker.org                        | 2606:4700:20::681a:759    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 111  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad   | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 112  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad   | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 150  | 104.18.78.214                         | 104.18.78.214             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 155  | www.digitalocean.com                  | 2606:4700::6813:ad44      | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 156  | 104.19.223.58                         | 104.19.223.58             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 189  | www.wto.org                           | 2a06:98c1:3102::6812:29be | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 233  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 242  | steamdb.info                          | 172.66.175.250            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 245  | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 248  | www.gov.ua                            | 104.21.23.72              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 269  | iplocation.io                         | 104.26.10.222             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 270  | iplocation.io                         | 104.26.11.222             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 274  | 172.67.120.0                          | 172.67.120.0              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 283  | www.okcupid.com                       | 104.16.144.63             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 290  | www.hugedomains.com                   | 2606:4700:20::681a:725    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 330  | cu.877774.xyz                         | 104.26.4.116              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 356  | toy-people.com                        | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 363  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b   | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 387  | www.whatismyip.com                    | 104.26.12.23              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 395  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 399  | bestcf.030101.xyz                     | 104.17.222.192            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 401  | 104.26.6.112                          | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 402  | cmcc.877774.xyz                       | 104.16.148.10             | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 439  | fbi.gov                               | 104.16.148.244            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 14   | www.ipget.net                         | 2606:4700:3036::6815:fd4  | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 29   | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 80   | whatismyipaddress.com                 | 2606:4700::6813:de4f      | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 86   | singapore.com                         | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 94   | silkbook.com                          | 2606:4700:20::ac43:4bd0   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 101  | dnschecker.org                        | 104.26.6.89               | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 110  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 119  | 104.26.13.31                          | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 129  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 164  | stock.hostmonit.com                   | 172.67.187.251            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 177  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 179  | 172.67.181.209                        | 172.67.181.209            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 181  | ifconfig.co                           | 104.21.54.91              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 187  | www.wto.org                           | 172.64.146.66             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 198  | japan.com                             | 104.26.4.60               | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 207  | icook.hk                              | 104.21.90.210             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 230  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 237  | ipv4.ip.sb                            | 104.26.12.31              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 246  | shopify.com                           | 23.227.38.33              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 251  | 104.18.254.88                         | 104.18.254.88             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 258  | ct.877774.xyz                         | 172.64.229.44             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 299  | www.visa.com.sg                       | 104.18.12.229             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 321  | cfip.xxxxxxxx.tk                      | 190.93.244.201            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 328  | cu.877774.xyz                         | 104.26.4.111              | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 340  | pranab.ns.cloudflare.com              | 108.162.195.199           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 348  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 365  | kyree.ns.cloudflare.com               | 162.159.44.207            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 382  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 389  | www.whatismyip.com                    | 2606:4700:20::681a:c17    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 390  | www.whatismyip.com                    | 2606:4700:20::681a:d17    | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 415  | cmcc.877774.xyz                       | 104.16.149.12             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 419  | cmcc.877774.xyz                       | 104.16.149.10             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 430  | xn--b6gac.eu.org                      | 172.67.153.253            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 444  | cf.090227.xyz                         | 2a06:98c1:3108::6812:2a62 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 6    | www.ipchicken.com                     | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 13   | www.ipget.net                         | 172.67.207.26             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 15   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 25   | time.is                               | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 62   | palera.in                             | 2a06:98c1:3121::3         | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 68   | ip.sb                                 | 2606:4700:20::681a:d1f    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 69   | ip.sb                                 | 2606:4700:20::ac43:4bac   | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 77   | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 79   | whatismyipaddress.com                 | 2606:4700::6813:df4f      | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 82   | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 91   | silkbook.com                          | 172.67.75.208             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
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
