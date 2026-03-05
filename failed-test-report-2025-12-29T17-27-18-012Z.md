# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:27:18
- **数据来源**: connectivity_results-20251229-172717.json
- **总测试数**: 492
- **失败测试数**: 2
- **成功测试数**: 490
- **失败率**: 0.41%
- **平均延迟**: 69.47ms
- **最小延迟**: 40ms
- **最大延迟**: 3745ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:27:18
- **IP地址**: 2a09:bac5:6216:28::4:338
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
| 94   | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 411  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
| 189  | freeyx.cloudflare88.eu.org              | 141.101.121.109                         | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 474  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 36   | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 308  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 42       | cloudflare |
| 75   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 242  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 261  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 310  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 458  | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 484  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 33   | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 128  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 178  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 198  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 258  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 264  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 270  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 317  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 372  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 456  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 466  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 15   | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 28   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 38   | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 46   | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 57   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 107  | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 135  | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 156  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 165  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 168  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 180  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 234  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 253  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 254  | cf.zhetengsha.eu.org                    | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 280  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 284  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 296  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 320  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 332  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 333  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 342  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 347  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 353  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 390  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 396  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 431  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 439  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 457  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 459  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 460  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 22   | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 31   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 32   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 39   | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 58   | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 63   | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 68   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 74   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 76   | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 105  | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 150  | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 174  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 179  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 183  | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 208  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 236  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 246  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 265  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 287  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 294  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 303  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 314  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 321  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 334  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 349  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 395  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 428  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 444  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 447  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 453  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 461  | www.7749tv.com                          | 104.17.196.161                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 462  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 465  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 21   | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 24   | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 37   | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 47   | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 56   | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 60   | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 67   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 90   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 111  | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 122  | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 142  | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 171  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 177  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 181  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 206  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 210  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 241  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 250  | bestcf.030101.xyz                       | 104.19.147.41                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 257  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 295  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 298  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 315  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 363  | silkbook.com                            | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 376  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 377  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 393  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 402  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 410  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 416  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 419  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 427  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 430  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 470  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 471  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 486  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 16   | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 34   | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 83   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 87   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 110  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 113  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 130  | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 133  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 155  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 158  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 159  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 161  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 176  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 194  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 203  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 222  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 247  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 259  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 260  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 293  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 305  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 312  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 343  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 351  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 361  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 385  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 401  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 403  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 425  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 450  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 488  | abdullah.ns.cloudflare.com              | 162.159.44.203                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 17   | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 27   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 35   | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 48   | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 54   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 66   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 88   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 91   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 93   | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 117  | ct.877774.xyz                           | 172.64.229.236                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 136  | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 152  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 166  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 204  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 207  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 231  | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 262  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 263  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 268  | cf.090227.xyz                           | 104.18.42.98                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 323  | moura.ns.cloudflare.com                 | 172.64.35.217                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 327  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 337  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 341  | benedict.ns.cloudflare.com              | 172.64.35.205                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 346  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 386  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 389  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 418  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 423  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 429  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 433  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 441  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 464  | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 483  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 13   | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 18   | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 23   | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 51   | trevor.ns.cloudflare.com                | 108.162.195.154                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 65   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 79   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 80   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 96   | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 99   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 120  | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 126  | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 131  | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 132  | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 137  | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 149  | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 163  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 167  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 183 条记录
- **快 (50-100ms)**: 17 条记录
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
