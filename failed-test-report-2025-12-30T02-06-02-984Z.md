# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:06:02
- **数据来源**: connectivity_results-20251230-020602.json
- **总测试数**: 501
- **失败测试数**: 5
- **成功测试数**: 496
- **失败率**: 1.00%
- **平均延迟**: 73.32ms
- **最小延迟**: 40ms
- **最大延迟**: 999ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:06:02
- **IP地址**: 2a09:bac1:7681:5e78::401:63
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

- **连接超时: I/O超时**: 5 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (5 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 274  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 309  | cris.ns.cloudflare.com   | 108.162.195.202 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.202:443: i/o timeout |
| 318  | decker.ns.cloudflare.com | 162.159.44.155  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.155:443: i/o timeout  |
| 332  | kyree.ns.cloudflare.com  | 162.159.44.207  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.207:443: i/o timeout  |
| 485  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 5 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 5 次超时，主要集中在IP段 162.159（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 5 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 496 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 339  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 462  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 349  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 81   | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 140  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 234  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 348  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 352  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 74   | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 85   | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 90   | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 137  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 242  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 257  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 314  | bestcf.030101.xyz                       | 104.19.157.251                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 385  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 412  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 437  | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 42   | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 52   | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 58   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 99   | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 145  | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 166  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 270  | cfip.xxxxxxxx.tk                        | 188.114.97.144                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 317  | bestcf.030101.xyz                       | 2606:4700::fffd:819d:acda               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 359  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 408  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 457  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 459  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 460  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 461  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 488  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 8    | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 17   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 18   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 21   | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 50   | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 83   | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 155  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 198  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 200  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 205  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 230  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 238  | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 307  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 334  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 356  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 374  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 411  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 427  | silkbook.com                            | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 12   | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 57   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 63   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 65   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 100  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 133  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 152  | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 188  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 194  | ct.877774.xyz                           | 172.64.229.236                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 232  | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 237  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 280  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 281  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 295  | zread.ai                                | 2a06:98c1:3121::3                       | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 296  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 297  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 298  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 327  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 344  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 383  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 386  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 431  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 455  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 458  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 478  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 492  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 56   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 59   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 79   | www.7749tv.com                          | 104.19.133.4                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 84   | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 88   | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 89   | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 98   | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 103  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 107  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 161  | ipinfo.in                               | 172.67.198.203                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 180  | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 181  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 184  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 239  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 247  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 287  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 329  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 347  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 354  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 422  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 425  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 430  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 463  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 489  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 491  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 499  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 22   | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 24   | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 25   | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 37   | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 43   | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 87   | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 138  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 139  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 148  | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 151  | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 176  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 182  | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 196  | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 207  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 225  | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 283  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 286  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 291  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 294  | zread.ai                                | 2a06:98c1:3120::3                       | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 337  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 370  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 423  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 444  | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 496  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 7    | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 9    | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 13   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 20   | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 72   | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 120  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 130  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 163  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 167  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 190  | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 213  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 268  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 316  | bestcf.030101.xyz                       | 2606:4700::8d:f082:8938:66d8            | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 335  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 396  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 400  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 407  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 416  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 432  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 483  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 51   | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 78   | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 82   | japan.com                               | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 106  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 117  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 121  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 146  | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 153  | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 165  | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 186  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 189  | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 211  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 292  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 299  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 312  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 321  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 323  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 338  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 340  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 357  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 360  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 364  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 399  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 418  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 434  | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 435  | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 453  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 475  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 498  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 6    | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 34   | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 35   | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 49   | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 54   | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 71   | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 75   | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 94   | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 102  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 127  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 135  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 136  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 208  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 217  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 221  | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 236  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 263  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 271  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 275  | cfip.xxxxxxxx.tk                        | 190.93.247.169                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 324  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 328  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 345  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 413  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 417  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 428  | silkbook.com                            | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 433  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 443  | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 495  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 500  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 5    | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 11   | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 112  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 128  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 149  | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 160  | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 187  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 199  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 246  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 262  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 384  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 451  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 452  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 486  | icook.hk                                | 188.114.96.3                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 501  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 15   | comicabc.com                            | 172.67.174.21                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 36   | eur.877774.xyz                          | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 64   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 172  | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 191  | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 201  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 212  | cmcc.877774.xyz                         | 104.16.149.7                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 224  | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 231  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 240  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 243  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 260  | freeyx.cloudflare88.eu.org              | 141.101.120.144                         | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 265  | cfip.xxxxxxxx.tk                        | 104.17.127.110                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 267  | cfip.xxxxxxxx.tk                        | 104.25.105.1                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 273  | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 293  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 330  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 351  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 355  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 358  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 395  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 421  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 26   | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 27   | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 60   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 73   | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 143  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 154  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 158  | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 169  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 229  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 284  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 285  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 290  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 300  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 301  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 311  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 315  | bestcf.030101.xyz                       | 104.19.156.139                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 341  | cf.zhetengsha.eu.org                    | 172.64.152.241                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 350  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 373  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 397  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 424  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 426  | silkbook.com                            | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 429  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 436  | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 441  | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 10   | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 76   | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 119  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 178  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 202  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 226  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 261  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cd47:f6f7:aaa7 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 276  | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 313  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 371  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 387  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 409  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 487  | icook.hk                                | 188.114.97.3                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 62   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 77   | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 118  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 124  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 132  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 147  | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 157  | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 164  | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 195  | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 203  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 206  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 209  | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 266  | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 326  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 343  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 375  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 379  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 382  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 393  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 398  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 449  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 456  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 468  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 497  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 61   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 95   | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 123  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 142  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 156  | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 159  | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 162  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 179  | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 204  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 233  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 245  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 410  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 476  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 477  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 70   | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 126  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 168  | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 210  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 228  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 342  | cf.zhetengsha.eu.org                    | 104.18.35.15                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 415  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 33   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 108  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 125  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 150  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 325  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 448  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 454  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 16   | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 105  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 264  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 272  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 414  | ip.gs                                   | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 490  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 19   | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 104  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 170  | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 289  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 392  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 467  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 494  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 177  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 193  | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 235  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 91   | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 122  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 197  | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 419  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 493  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 420  | singapore.com                           | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 470  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 92   | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 115  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 183  | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 80   | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 269  | cfip.xxxxxxxx.tk                        | 190.93.246.67                           | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 14   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 241  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 73       | cloudflare |
| 174  | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 66   | lewis.ns.cloudflare.com                 | 172.64.35.159                           | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 116  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 39   | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 175  | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 346  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 131  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 288  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 77       | cloudflare |
| 129  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 378  | moura.ns.cloudflare.com                 | 172.64.35.217                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 30   | trevor.ns.cloudflare.com                | 108.162.195.154                         | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 101  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 44   | damien.ns.cloudflare.com                | 108.162.195.168                         | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 93   | 172.64.52.127                           | 172.64.52.127                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 192  | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 302  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 68   | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 96   | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 185  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 249  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 310  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 473  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 110  | abdullah.ns.cloudflare.com              | 162.159.44.203                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 173  | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 97   | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 111  | abdullah.ns.cloudflare.com              | 172.64.35.203                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 244  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 45   | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 388  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 47   | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 252  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 253  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 369  | bowen.ns.cloudflare.com                 | 172.64.35.83                            | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 305  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 92       | cloudflare |
| 376  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 31   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 93       | cloudflare |
| 259  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h2   | ✅ 成功 | 93       | cloudflare |
| 372  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 93       | cloudflare |
| 464  | julio.ns.cloudflare.com                 | 162.159.44.209                          | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 69   | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 336  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 405  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 465  | julio.ns.cloudflare.com                 | 108.162.195.209                         | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 484  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 113  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 440  | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 469  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 32   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 96       | cloudflare |
| 366  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 96       | cloudflare |
| 390  | rustam.ns.cloudflare.com                | 172.64.35.148                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 391  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 96       | cloudflare |
| 277  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 282  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 306  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 368  | bowen.ns.cloudflare.com                 | 162.159.44.83                           | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 403  | benedict.ns.cloudflare.com              | 108.162.195.205                         | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 442  | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 41   | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 367  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 381  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 394  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h2   | ✅ 成功 | 99       | cloudflare |
| 450  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 474  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 40   | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h2   | ✅ 成功 | 100      | cloudflare |
| 48   | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 100      | cloudflare |
| 255  | craig.ns.cloudflare.com                 | 108.162.195.192                         | IPv4   | h2   | ✅ 成功 | 100      | cloudflare |
| 278  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 308  | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 322  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 101      | cloudflare |
| 404  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 101      | cloudflare |
| 439  | wilson.ns.cloudflare.com                | 108.162.195.110                         | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 480  | huxley.ns.cloudflare.com                | 172.64.35.188                           | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 1    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | h2   | ✅ 成功 | 102      | cloudflare |
| 134  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 446  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 466  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h2   | ✅ 成功 | 103      | cloudflare |
| 2    | 172.64.154.18                           | 172.64.154.18                           | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 29   | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 86   | 198.41.194.162                          | 198.41.194.162                          | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 363  | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 3    | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 380  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 402  | benedict.ns.cloudflare.com              | 172.64.35.205                           | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 406  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 438  | wilson.ns.cloudflare.com                | 162.159.44.110                          | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |
| 4    | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 107      | cloudflare |
| 251  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h2   | ✅ 成功 | 107      | cloudflare |
| 365  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 107      | cloudflare |
| 361  | braden.ns.cloudflare.com                | 172.64.35.169                           | IPv4   | h2   | ✅ 成功 | 108      | cloudflare |
| 377  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 108      | cloudflare |
| 256  | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h2   | ✅ 成功 | 110      | cloudflare |
| 38   | otto.ns.cloudflare.com                  | 172.64.35.135                           | IPv4   | h2   | ✅ 成功 | 112      | cloudflare |
| 248  | sullivan.ns.cloudflare.com              | 162.159.44.161                          | IPv4   | h2   | ✅ 成功 | 113      | cloudflare |
| 304  | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h2   | ✅ 成功 | 113      | cloudflare |
| 482  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | h2   | ✅ 成功 | 114      | cloudflare |
| 319  | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h2   | ✅ 成功 | 115      | cloudflare |
| 114  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 116      | cloudflare |
| 331  | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h2   | ✅ 成功 | 118      | cloudflare |
| 279  | pranab.ns.cloudflare.com                | 172.64.35.199                           | IPv4   | h2   | ✅ 成功 | 120      | cloudflare |
| 333  | kyree.ns.cloudflare.com                 | 172.64.35.207                           | IPv4   | h2   | ✅ 成功 | 122      | cloudflare |
| 55   | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h2   | ✅ 成功 | 127      | cloudflare |
| 481  | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4   | h2   | ✅ 成功 | 127      | cloudflare |
| 23   | 172.64.35.24                            | 172.64.35.24                            | IPv4   | h2   | ✅ 成功 | 128      | cloudflare |
| 479  | huxley.ns.cloudflare.com                | 108.162.195.188                         | IPv4   | h2   | ✅ 成功 | 128      | cloudflare |
| 447  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h2   | ✅ 成功 | 129      | cloudflare |
| 109  | abdullah.ns.cloudflare.com              | 108.162.195.203                         | IPv4   | h2   | ✅ 成功 | 130      | cloudflare |
| 303  | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h2   | ✅ 成功 | 130      | cloudflare |
| 254  | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h2   | ✅ 成功 | 132      | cloudflare |
| 46   | damien.ns.cloudflare.com                | 172.64.35.168                           | IPv4   | h2   | ✅ 成功 | 134      | cloudflare |
| 389  | rustam.ns.cloudflare.com                | 162.159.44.148                          | IPv4   | h2   | ✅ 成功 | 136      | cloudflare |
| 28   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h2   | ✅ 成功 | 137      | cloudflare |
| 471  | uriah.ns.cloudflare.com                 | 108.162.195.194                         | IPv4   | h2   | ✅ 成功 | 139      | cloudflare |
| 472  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h2   | ✅ 成功 | 140      | cloudflare |
| 67   | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h2   | ✅ 成功 | 142      | cloudflare |
| 445  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h2   | ✅ 成功 | 145      | cloudflare |
| 258  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h2   | ✅ 成功 | 147      | cloudflare |
| 401  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h2   | ✅ 成功 | 165      | cloudflare |
| 227  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 169      | cloudflare |
| 141  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 174      | cloudflare |
| 362  | braden.ns.cloudflare.com                | 108.162.195.169                         | IPv4   | h2   | ✅ 成功 | 178      | cloudflare |
| 320  | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h2   | ✅ 成功 | 186      | cloudflare |
| 250  | sullivan.ns.cloudflare.com              | 172.64.35.161                           | IPv4   | h2   | ✅ 成功 | 201      | cloudflare |
| 353  | 141.147.185.63                          | 141.147.185.63                          | IPv4   | h2   | ✅ 成功 | 410      | cloudflare |
| 144  | 34.143.159.175                          | 34.143.159.175                          | IPv4   | h2   | ✅ 成功 | 567      | cloudflare |
| 53   | 3.0.50.69                               | 3.0.50.69                               | IPv4   | h2   | ✅ 成功 | 811      | cloudflare |
| 171  | 168.138.165.174                         | 168.138.165.174                         | IPv4   | h2   | ✅ 成功 | 999      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 34 条记录
- **快 (50-100ms)**: 399 条记录
- **正常 (100-200ms)**: 58 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 5 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 5 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
