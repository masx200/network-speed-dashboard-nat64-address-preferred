# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 15:12:47
- **数据来源**: connectivity_results-20251214-151246.json
- **总测试数**: 446
- **失败测试数**: 2
- **成功测试数**: 444
- **失败率**: 0.45%
- **平均延迟**: 91.94ms
- **最小延迟**: 70ms
- **最大延迟**: 823ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 15:12:47
- **IP地址**: 2a09:bac5:d2a1:1c64::2d4:7a
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1446, -104.8116
- **时区**: America/Denver
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
| 84   | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 231  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

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

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                                | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 29   | whatismyipaddress.com                 | 2606:4700::6813:de4f                  | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 267  | www.okcupid.com                       | 104.18.160.63                         | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 16   | palera.in                             | 2606:4700:3032::ac43:9d7a             | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 27   | whatismyipaddress.com                 | 104.19.223.79                         | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 44   | silkbook.com                          | 2606:4700:20::681a:9a0                | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 50   | singapore.com                         | 2606:4700:20::681a:c8c                | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 108  | icook.tw                              | 2606:4700:10::ac42:9e73               | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 178  | www.ipget.net                         | 2606:4700:3036::6815:fd4              | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 181  | www.pcmag.com                         | 2606:4700::6810:1476                  | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 182  | www.pcmag.com                         | 2606:4700::6810:1576                  | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 361  | www.whatismyip.com                    | 2606:4700:20::681a:d17                | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 22   | ip.sb                                 | 104.26.13.31                          | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 103  | 104.18.37.40                          | 104.18.37.40                          | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 205  | iplocation.io                         | 104.26.11.222                         | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 246  | www.4chan.org                         | 104.16.229.229                        | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 288  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70             | IPv6   | h2   | ✅ 成功 | 72       | cloudflare |
| 420  | cf.090227.xyz                         | 104.18.43.174                         | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 421  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052             | IPv6   | h2   | ✅ 成功 | 72       | cloudflare |
| 12   | comicabc.com                          | 2a06:98c1:3121::3                     | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 20   | ip.gs                                 | 2606:4700:3035::ac43:a01c             | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 42   | silkbook.com                          | 2606:4700:20::681a:8a0                | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 49   | singapore.com                         | 2606:4700:20::ac43:4bc2               | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 83   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be             | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 88   | tasteatlas.com                        | 2606:4700::6811:2569                  | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 97   | www.udemy.com                         | 104.16.143.237                        | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 98   | www.udemy.com                         | 2606:4700::6810:8eed                  | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 99   | www.udemy.com                         | 2606:4700::6810:8fed                  | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 143  | ifconfig.co                           | 2606:4700:3030::ac43:a86a             | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 204  | iplocation.io                         | 104.26.10.222                         | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 208  | iplocation.io                         | 2606:4700:20::ac43:4664               | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 234  | cfip.xxxxxxxx.tk                      | 104.16.232.223                        | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 296  | na.877774.xyz                         | 104.18.187.25                         | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 343  | 104.16.223.179                        | 104.16.223.179                        | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 403  | xn--b6gac.eu.org                      | 104.21.90.78                          | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 412  | fbi.gov                               | 104.16.149.244                        | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 413  | fbi.gov                               | 104.16.148.244                        | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 26   | ip.sb                                 | 2606:4700:20::681a:d1f                | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 43   | silkbook.com                          | 2606:4700:20::ac43:4bd0               | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 56   | www.visa.com.hk                       | 104.18.20.69                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 95   | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304             | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 106  | icook.tw                              | 104.20.28.74                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 215  | 104.18.37.13                          | 104.18.37.13                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 216  | ct.877774.xyz                         | 172.64.229.185                        | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 223  | ct.877774.xyz                         | 172.64.229.174                        | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 275  | www.hugedomains.com                   | 2606:4700:20::681a:725                | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 302  | cu.877774.xyz                         | 104.26.4.112                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 367  | cloudflare-ip.mofashi.ltd             | 172.67.155.172                        | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 383  | cmcc.877774.xyz                       | 104.16.148.7                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 385  | cmcc.877774.xyz                       | 104.16.148.9                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 406  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e             | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 409  | cf.zhetengsha.eu.org                  | 2606:4700:440a::ac40:98f1             | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 436  | time.is                               | 104.26.12.54                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 8    | 104.17.68.85                          | 104.17.68.85                          | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 53   | 172.67.106.26                         | 172.67.106.26                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 55   | www.visa.com.hk                       | 104.18.21.69                          | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 116  | www.digitalocean.com                  | 2606:4700::6813:ad44                  | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 140  | ifconfig.co                           | 172.67.168.106                        | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 141  | ifconfig.co                           | 104.21.54.91                          | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 144  | www.wto.org                           | 172.64.146.66                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 145  | www.wto.org                           | 104.18.41.190                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 164  | japan.com                             | 2606:4700:20::681a:43c                | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 173  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91             | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 179  | www.pcmag.com                         | 104.16.20.118                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 180  | www.pcmag.com                         | 104.16.21.118                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 207  | iplocation.io                         | 2606:4700:20::681a:bde                | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 209  | steamdb.info                          | 104.20.34.212                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 211  | steamdb.info                          | 2606:4700:10::6814:22d4               | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 242  | 103.160.204.59                        | 103.160.204.59                        | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 250  | icook.hk                              | 2606:4700:3031::6815:5ad2             | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 264  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7             | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 266  | www.okcupid.com                       | 104.17.48.63                          | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 268  | www.okcupid.com                       | 104.16.239.254                        | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 270  | www.hugedomains.com                   | 104.26.7.37                           | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 274  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf               | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 287  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90             | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 326  | toy-people.com                        | 104.26.2.36                           | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 328  | toy-people.com                        | 2606:4700:20::ac43:4812               | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 357  | www.whatismyip.com                    | 172.67.69.129                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 360  | www.whatismyip.com                    | 2606:4700:20::ac43:4581               | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 386  | cmcc.877774.xyz                       | 104.16.148.10                         | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 438  | time.is                               | 2606:4700:20::681a:d36                | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 444  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94               | IPv6   | h2   | ✅ 成功 | 75       | cloudflare |
| 3    | www.7749tv.com                        | 104.19.133.4                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 13   | palera.in                             | 104.21.58.72                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 15   | palera.in                             | 2606:4700:3035::6815:3a48             | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 17   | ip.gs                                 | 104.21.14.176                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 24   | ip.sb                                 | 2606:4700:20::681a:c1f                | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 69   | dnschecker.org                        | 2606:4700:20::ac43:49d8               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 85   | tasteatlas.com                        | 104.17.36.105                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 107  | icook.tw                              | 2606:4700:10::6814:1c4a               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 122  | eur.877774.xyz                        | 104.21.26.150                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 134  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 160  | japan.com                             | 104.26.5.60                           | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 170  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 172  | cf.0sm.com                            | 172.67.187.145                        | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 176  | www.ipget.net                         | 172.67.207.26                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 222  | ct.877774.xyz                         | 172.64.229.173                        | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 256  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 261  | cf.877771.xyz                         | 104.21.80.180                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 273  | www.hugedomains.com                   | 2606:4700:20::681a:625                | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 295  | na.877774.xyz                         | 104.18.38.235                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 325  | toy-people.com                        | 172.67.72.18                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 340  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf               | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 344  | 104.17.79.11                          | 104.17.79.11                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 351  | zread.ai                              | 172.67.202.78                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 366  | cloudflare-ip.mofashi.ltd             | 104.21.72.233                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 372  | bestcf.030101.xyz                     | 104.17.211.46                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 379  | cmcc.877774.xyz                       | 104.16.148.3                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 389  | cmcc.877774.xyz                       | 104.16.148.244                        | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 402  | saas.sin.fan                          | 162.159.36.20                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 416  | www.ipchicken.com                     | 104.26.6.112                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 419  | cf.090227.xyz                         | 172.64.144.82                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 440  | time.is                               | 2606:4700:20::681a:c36                | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 4    | 104.16.61.163                         | 104.16.61.163                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 5    | www.glassdoor.com                     | 104.16.25.46                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 21   | ip.sb                                 | 104.26.12.31                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 23   | ip.sb                                 | 172.67.75.172                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 30   | whatismyipaddress.com                 | 2606:4700::6813:df4f                  | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 48   | singapore.com                         | 2606:4700:20::681a:d8c                | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 57   | 162.159.133.85                        | 162.159.133.85                        | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 58   | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06             | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 87   | tasteatlas.com                        | 2606:4700::6811:2469                  | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 100  | 104.17.142.12                         | 104.17.142.12                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 102  | 198.62.62.4                           | 198.62.62.4                           | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 111  | 104.26.13.31                          | 104.26.13.31                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 121  | eur.877774.xyz                        | 104.21.47.209                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 195  | ipv4.ip.sb                            | 104.26.12.31                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 199  | ipinfo.in                             | 104.21.21.129                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 206  | iplocation.io                         | 2606:4700:20::681a:ade                | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 226  | www.gov.ua                            | 104.21.23.72                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 243  | 104.18.254.88                         | 104.18.254.88                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 249  | icook.hk                              | 172.67.161.104                        | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 258  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc               | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 263  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4             | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 283  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1               | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 298  | cu.877774.xyz                         | 104.26.4.117                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 300  | cu.877774.xyz                         | 104.26.4.118                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 311  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7               | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 353  | zread.ai                              | 2606:4700:3032::ac43:ca4e             | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 370  | 172.64.151.55                         | 172.64.151.55                         | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 374  | bestcf.030101.xyz                     | 2606:4700:23:e182:db7d:103e:afc5:c316 | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 390  | cmcc.877774.xyz                       | 104.16.149.1                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 393  | cmcc.877774.xyz                       | 104.16.149.4                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 396  | cmcc.877774.xyz                       | 104.16.149.7                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 398  | cmcc.877774.xyz                       | 104.16.149.9                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 401  | saas.sin.fan                          | 162.159.36.5                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 404  | xn--b6gac.eu.org                      | 172.67.153.253                        | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 407  | cf.zhetengsha.eu.org                  | 172.64.152.241                        | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 415  | fbi.gov                               | 2606:4700::6810:95f4                  | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 417  | www.ipchicken.com                     | 104.26.7.112                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 426  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53               | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 25   | ip.sb                                 | 2606:4700:20::ac43:4bac               | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 38   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 41   | silkbook.com                          | 172.67.75.208                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 52   | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 68   | dnschecker.org                        | 2606:4700:20::681a:759                | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 71   | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 78   | gamer.com.tw                          | 104.18.2.197                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 79   | gamer.com.tw                          | 104.18.3.197                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 82   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 86   | tasteatlas.com                        | 104.17.37.105                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 101  | 172.67.79.211                         | 172.67.79.211                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 177  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 210  | steamdb.info                          | 172.66.175.250                        | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 214  | 172.67.75.172                         | 172.67.75.172                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 225  | 172.67.110.232                        | 172.67.110.232                        | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 228  | www.gov.ua                            | 2606:4700:3031::6815:1748             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 229  | www.gov.ua                            | 2606:4700:3033::ac43:d17f             | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 237  | cfip.xxxxxxxx.tk                      | 188.114.96.125                        | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 269  | www.okcupid.com                       | 104.16.223.254                        | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 272  | www.hugedomains.com                   | 172.67.70.191                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 297  | na.877774.xyz                         | 104.19.74.233                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 349  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb               | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 358  | www.whatismyip.com                    | 104.26.12.23                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 365  | asia.877774.xyz                       | 104.16.211.153                        | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 395  | cmcc.877774.xyz                       | 104.16.149.6                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 399  | cmcc.877774.xyz                       | 104.16.149.10                         | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 428  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353               | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 437  | time.is                               | 104.26.13.54                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 446  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394               | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 46   | singapore.com                         | 104.26.12.140                         | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 70   | dnschecker.org                        | 2606:4700:20::681a:659                | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 105  | icook.tw                              | 172.66.158.115                        | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 118  | 104.19.223.58                         | 104.19.223.58                         | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 149  | stock.hostmonit.com                   | 172.67.187.251                        | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 162  | japan.com                             | 2606:4700:20::681a:53c                | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 200  | ipinfo.in                             | 172.67.198.203                        | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 240  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:bf:5dba:fa1d:5993:9cf8 | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 245  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e             | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 251  | icook.hk                              | 2606:4700:3037::ac43:a168             | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 291  | craig.ns.cloudflare.com               | 172.64.35.192                         | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 306  | cu.877774.xyz                         | 104.26.4.115                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 330  | toy-people.com                        | 2606:4700:20::681a:224                | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 339  | kyree.ns.cloudflare.com               | 172.64.35.207                         | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 354  | zread.ai                              | 2606:4700:3033::6815:4cf0             | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 376  | cmcc.877774.xyz                       | 104.16.149.244                        | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 377  | cmcc.877774.xyz                       | 104.16.148.1                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 391  | cmcc.877774.xyz                       | 104.16.149.2                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 439  | time.is                               | 2606:4700:20::ac43:449d               | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 442  | rustam.ns.cloudflare.com              | 162.159.44.148                        | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
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
