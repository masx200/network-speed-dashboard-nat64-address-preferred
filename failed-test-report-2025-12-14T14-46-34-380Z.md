# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:46:34
- **数据来源**: connectivity_results-20251214-144633.json
- **总测试数**: 444
- **失败测试数**: 2
- **成功测试数**: 442
- **失败率**: 0.45%
- **平均延迟**: 76.76ms
- **最小延迟**: 41ms
- **最大延迟**: 952ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:46:34
- **IP地址**: 2a09:bac5:9f27:1678::23d:7d
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

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 203  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 233  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

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

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 298  | saas.sin.fan                          | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 334  | 172.67.79.211                         | 172.67.79.211                           | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 299  | saas.sin.fan                          | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 283  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 304  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 308  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 337  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 341  | cfip.1323123.xyz                      | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 366  | eur.877774.xyz                        | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 297  | 104.26.13.31                          | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 317  | cf.090227.xyz                         | 104.18.35.15                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 351  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 415  | cmcc.877774.xyz                       | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 287  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 309  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 335  | www.digitalocean.com                  | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 338  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 420  | cmcc.877774.xyz                       | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 429  | japan.com                             | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 282  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 303  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 319  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 336  | www.digitalocean.com                  | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 372  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 378  | 104.19.175.123                        | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 414  | cmcc.877774.xyz                       | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 270  | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 292  | 172.64.151.55                         | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 293  | xn--b6gac.eu.org                      | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 294  | xn--b6gac.eu.org                      | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 318  | cf.090227.xyz                         | 172.64.152.241                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 324  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 343  | www.ipchicken.com                     | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 348  | otto.ns.cloudflare.com                | 162.159.44.135                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 361  | time.is                               | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 398  | ifconfig.co                           | 104.21.54.91                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 412  | cmcc.877774.xyz                       | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 413  | cmcc.877774.xyz                       | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 424  | cmcc.877774.xyz                       | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 433  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 264  | 104.17.79.11                          | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 266  | asia.877774.xyz                       | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 281  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 295  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 305  | fbi.gov                               | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 307  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 320  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 383  | 172.67.181.209                        | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 426  | cmcc.877774.xyz                       | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 271  | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 276  | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 286  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 296  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 367  | eur.877774.xyz                        | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 418  | cmcc.877774.xyz                       | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 430  | japan.com                             | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 434  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 435  | www.wto.org                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 437  | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 267  | asia.877774.xyz                       | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 300  | cf.zhetengsha.eu.org                  | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 301  | cf.zhetengsha.eu.org                  | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 368  | eur.877774.xyz                        | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 382  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 393  | lewis.ns.cloudflare.com               | 108.162.195.159                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 290  | bestcf.030101.xyz                     | 2606:4700:23:3674:16f8:2b7a:34cb:d251   | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 291  | bestcf.030101.xyz                     | 2606:4700:23:e182:db7d:103e:afc5:c316   | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 346  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 362  | time.is                               | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 399  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 416  | cmcc.877774.xyz                       | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 425  | cmcc.877774.xyz                       | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 428  | www.csgo.com                          | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 444  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 188  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 289  | bestcf.030101.xyz                     | 104.17.211.46                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 356  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 364  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 400  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 411  | cmcc.877774.xyz                       | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 422  | cmcc.877774.xyz                       | 104.16.149.7                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 438  | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 279  | www.whatismyip.com                    | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 312  | braden.ns.cloudflare.com              | 108.162.195.169                         | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 333  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 340  | 104.18.37.40                          | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 342  | 172.64.35.24                          | 172.64.35.24                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 352  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 375  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 277  | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 323  | moura.ns.cloudflare.com               | 172.64.35.217                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 339  | 198.62.62.4                           | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 344  | www.ipchicken.com                     | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 357  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 363  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 423  | cmcc.877774.xyz                       | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 275  | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 345  | www.ipchicken.com                     | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 354  | damien.ns.cloudflare.com              | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 359  | 104.18.78.214                         | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 373  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 374  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 395  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 397  | ifconfig.co                           | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 432  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 64   | www.visa.cn                           | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 84   | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 139  | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 358  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 376  | 104.19.223.58                         | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 381  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 431  | japan.com                             | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 436  | www.wto.org                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 306  | fbi.gov                               | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 322  | moura.ns.cloudflare.com               | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 327  | 104.17.142.12                         | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 21   | 104.18.254.88                         | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 102  | www.glassdoor.com                     | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 234  | cfip.xxxxxxxx.tk                      | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 331  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 360  | time.is                               | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 365  | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 392  | lewis.ns.cloudflare.com               | 172.64.35.159                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 410  | cmcc.877774.xyz                       | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 419  | cmcc.877774.xyz                       | 104.16.149.4                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 10   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 67   | www.okcupid.com                       | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 75   | yx-auto.pages.dev                     | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 143  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 185  | cf.877774.xyz                         | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 206  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 226  | steamdb.info                          | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 229  | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 273  | iplocation.io                         | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 302  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 406  | cmcc.877774.xyz                       | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 417  | cmcc.877774.xyz                       | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 29   | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 30   | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 83   | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 119  | cu.877774.xyz                         | 104.26.4.115                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 140  | whatismyipaddress.com                 | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 149  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 192  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 195  | tasteatlas.com                        | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 246  | ct.877774.xyz                         | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 268  | zread.ai                              | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 380  | stock.hostmonit.com                   | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 28   | ip.sb                                 | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 40   | toy-people.com                        | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 69   | www.okcupid.com                       | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 81   | www.hugedomains.com                   | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 118  | cu.877774.xyz                         | 104.26.4.114                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 147  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 148  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 154  | silkbook.com                          | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 225  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 261  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 15   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 20   | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 41   | toy-people.com                        | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 59   | www.4chan.org                         | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 112  | cu.877774.xyz                         | 104.26.4.117                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 117  | cu.877774.xyz                         | 104.26.4.113                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 127  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 160  | 172.67.106.26                         | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 196  | tasteatlas.com                        | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 208  | www.pcmag.com                         | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 213  | cf.0sm.com                            | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 220  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 223  | ipinfo.in                             | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 236  | ipv4.ip.sb                            | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 242  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 427  | www.csgo.com                          | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 439  | abdullah.ns.cloudflare.com            | 172.64.35.203                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 39   | toy-people.com                        | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 76   | yx-auto.pages.dev                     | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 77   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 80   | www.hugedomains.com                   | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 133  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 137  | ip.gs                                 | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 156  | silkbook.com                          | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 170  | dnschecker.org                        | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 186  | cf.877774.xyz                         | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 222  | ipinfo.in                             | 172.67.198.203                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 237  | ipv4.ip.sb                            | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 238  | ipv4.ip.sb                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 256  | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 391  | lewis.ns.cloudflare.com               | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 402  | cmcc.877774.xyz                       | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 16   | 103.160.204.59                        | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 50   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 57   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 98   | na.877774.xyz                         | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 110  | freeyx.cloudflare88.eu.org            | 141.101.121.173                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 111  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa73:82e3:b098:334a:b5b2 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 115  | cu.877774.xyz                         | 104.26.4.111                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 135  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 138  | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 153  | silkbook.com                          | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 40 条记录
- **快 (50-100ms)**: 160 条记录
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
