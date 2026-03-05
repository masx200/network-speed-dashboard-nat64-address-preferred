# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:28:34
- **数据来源**: connectivity_results-20251229-172833.json
- **总测试数**: 502
- **失败测试数**: 5
- **成功测试数**: 497
- **失败率**: 1.00%
- **平均延迟**: 64.48ms
- **最小延迟**: 33ms
- **最大延迟**: 635ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:28:34
- **IP地址**: 2a09:bac1:76a1:6298::23d:bc
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 38.6877, -77.8369
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 4 次 (80.0%)
- **DNS解析错误: 其他DNS错误**: 1 次 (20.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 44   | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.110:443: i/o timeout |
| 232  | dylan.ns.cloudflare.com  | 162.159.44.187  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.187:443: i/o timeout  |
| 373  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 501  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                      |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------- |
| 76   | 168.138.165.174 | 168.138.165.174 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": EOF |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 4 次 (80.0%)
- **DNS解析错误**: 1 次 (20.0%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 50   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 33       | cloudflare |
| 52   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 34       | cloudflare |
| 428  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 38       | cloudflare |
| 166  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 318  | singapore.com                           | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 429  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 39       | cloudflare |
| 431  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 39       | cloudflare |
| 304  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 427  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 478  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 70   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 89   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 103  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 130  | cu.877774.xyz                           | 104.26.4.117                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 170  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 41       | cloudflare |
| 243  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 374  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 413  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 459  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 56   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 57   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 112  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 133  | cu.877774.xyz                           | 104.26.4.111                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 169  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 254  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 266  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 282  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 328  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 341  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 363  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 384  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 389  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 406  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 432  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 443  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 485  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 486  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 48   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 80   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 84   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 185  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 200  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 241  | cf.zhetengsha.eu.org                    | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 280  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 355  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 382  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 391  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 424  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 458  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 54   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 73   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 77   | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 81   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 111  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 158  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 195  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 237  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 242  | cf.zhetengsha.eu.org                    | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 260  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 310  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 326  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 372  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 433  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 479  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 41   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 51   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 128  | cu.877774.xyz                           | 104.26.4.115                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 137  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 162  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 236  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 238  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 249  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 334  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 383  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 387  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 392  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 475  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 30   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 49   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 58   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 62   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 167  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 192  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 197  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 239  | cf.zhetengsha.eu.org                    | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 245  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 308  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 319  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 338  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 381  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 393  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 394  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 444  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 460  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 465  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 472  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 473  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 498  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 9    | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 31   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 33   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 59   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 87   | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 115  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 129  | cu.877774.xyz                           | 104.26.4.116                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 174  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 191  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 193  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 286  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 307  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 335  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 351  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 398  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 448  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 470  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 5    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 16   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 42   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 164  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 168  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 188  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 207  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 211  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 253  | cf.090227.xyz                           | 104.18.42.98                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 263  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 321  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 336  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 337  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 343  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 362  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 364  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 380  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 396  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 404  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 425  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 450  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 454  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 455  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 462  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 492  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 13   | comicabc.com                            | 2a06:98c1:3120::3                       | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 53   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 60   | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 83   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 88   | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 90   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 102  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 108  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 165  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 283  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 289  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 305  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 327  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 352  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 401  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 405  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 430  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 493  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 34   | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 55   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 68   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 101  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 106  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 114  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 123  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 202  | bestcf.030101.xyz                       | 104.19.55.80                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 204  | bestcf.030101.xyz                       | 2606:4700:0:c5:4803:8845:8bde:1897      | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 228  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 230  | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 240  | cf.zhetengsha.eu.org                    | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 244  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 246  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 324  | silkbook.com                            | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 332  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 342  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 361  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 371  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 388  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 441  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 449  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 474  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 480  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 487  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 67   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 71   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 78   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 82   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 98   | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 134  | cu.877774.xyz                           | 104.26.4.112                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 135  | cu.877774.xyz                           | 104.26.4.113                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 161  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 186  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 212  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 224  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 164 条记录
- **快 (50-100ms)**: 36 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 5 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
