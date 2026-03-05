# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:23:36
- **数据来源**: connectivity_results-20251229-172335.json
- **总测试数**: 494
- **失败测试数**: 2
- **成功测试数**: 492
- **失败率**: 0.40%
- **平均延迟**: 100.98ms
- **最小延迟**: 77ms
- **最大延迟**: 823ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:23:36
- **IP地址**: 2a09:bac5:d2a6:15d7::22d:b2
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
| 36   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 379  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 267  | cf.zhetengsha.eu.org                    | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 391  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 178  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 307  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 42   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 117  | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 185  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 359  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 463  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 33   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 60   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 76   | ipinfo.in                               | 172.67.198.203                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 84   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 132  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 176  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 227  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 294  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 299  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 350  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 374  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 390  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 461  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 476  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 477  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 483  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 47   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 56   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 83   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 131  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 135  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 171  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 197  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 198  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 206  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 265  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 325  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 349  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 389  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 421  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 452  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 454  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 469  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 482  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 7    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 38   | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 44   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 80   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 232  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 233  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 236  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 271  | silkbook.com                            | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 287  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 293  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 346  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 439  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 447  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 489  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 20   | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 48   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 75   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 96   | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 119  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 145  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 148  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:0:fb:e00f:f23d:42c6      | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 151  | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 153  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 162  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 184  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 212  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 238  | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 247  | palera.in                               | 188.114.96.3                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 256  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 268  | cf.zhetengsha.eu.org                    | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 292  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 312  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 330  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 366  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 380  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 406  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 413  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 433  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 436  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 451  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 457  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 11   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 15   | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 26   | trevor.ns.cloudflare.com                | 108.162.195.154                         | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 41   | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 43   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 57   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 66   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 79   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 94   | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 115  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 121  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 122  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 139  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 161  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 180  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 194  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 196  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 201  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 218  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 231  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 239  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 242  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 257  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 261  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 262  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 288  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 297  | braden.ns.cloudflare.com                | 108.162.195.169                         | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 305  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 317  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 321  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 323  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 370  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 384  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 394  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 402  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 420  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 442  | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 473  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 85   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 93   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 99   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 124  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 126  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 137  | sullivan.ns.cloudflare.com              | 172.64.35.161                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 181  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 182  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 189  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 200  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 210  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 211  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 215  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 216  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 230  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 237  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 245  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 250  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 258  | singapore.com                           | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 259  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 273  | silkbook.com                            | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 283  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 289  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 300  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 347  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 352  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 365  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 378  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 385  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 396  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 404  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 407  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 449  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 462  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 465  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 29   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 73   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 87   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 88   | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 91   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 101  | icook.hk                                | 188.114.96.3                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 134  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 248  | palera.in                               | 188.114.97.3                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 255  | bestcf.030101.xyz                       | 2606:4700::fffd:819d:acda               | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 264  | ip.gs                                   | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 277  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 302  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 304  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 308  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 309  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 320  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 328  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 336  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 343  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 387  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 388  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 392  | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 405  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 408  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 411  | www.7749tv.com                          | 104.19.133.4                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 430  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 441  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 450  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 478  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 486  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 488  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 14   | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 46   | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 53   | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 55   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 62   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 70   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 77   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 97   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |

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
