# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:42:43
- **数据来源**: connectivity_results-20251230-014243.json
- **总测试数**: 506
- **失败测试数**: 3
- **成功测试数**: 503
- **失败率**: 0.59%
- **平均延迟**: 72.03ms
- **最小延迟**: 40ms
- **最大延迟**: 728ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:42:43
- **IP地址**: 2a09:bac5:6196:1173::1bd:134
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 33.4475, -112.0866
- **时区**: America/Phoenix
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 3 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 128  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 345  | 198.41.194.162   | 198.41.194.162 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.194.162:443: i/o timeout |
| 506  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 3 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 3 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 503 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 163  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 229  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 121  | cfip.xxxxxxxx.tk                        | 190.93.247.169                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 178  | cu.877774.xyz                           | 104.26.4.119                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 419  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 384  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 33   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 162  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 312  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 365  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 475  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 147  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 149  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 224  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 257  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 267  | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 352  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 383  | cf.090227.xyz                           | 104.18.42.98                            | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 7    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 14   | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 15   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 137  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 152  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 292  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 306  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 385  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 401  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 403  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 420  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 463  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 477  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 504  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 16   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 42   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 60   | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 64   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 71   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 93   | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 182  | freeyx.cloudflare88.eu.org              | 141.101.120.246                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 225  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 258  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 269  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 270  | cloudflare-ip.mofashi.ltd               | 2a06:98c1:3120::3                       | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 332  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 361  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 407  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 474  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 484  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 493  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 500  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 37   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 45   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 70   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 76   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 81   | cf.zhetengsha.eu.org                    | 172.66.47.179                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 82   | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 169  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 173  | cu.877774.xyz                           | 104.26.4.114                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 177  | cu.877774.xyz                           | 104.26.4.118                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 208  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 213  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 255  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 285  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 287  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 288  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 295  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 325  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 353  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 366  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 368  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 374  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 375  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 376  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 410  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 418  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 422  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 427  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 433  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 442  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 451  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 10   | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 12   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 34   | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 63   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 83   | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 84   | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 103  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 122  | cfip.xxxxxxxx.tk                        | 104.18.228.35                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 130  | cfip.xxxxxxxx.tk                        | 104.20.255.53                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 183  | freeyx.cloudflare88.eu.org              | 141.101.120.10                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 196  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 200  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 205  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 217  | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 226  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 263  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 265  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 277  | bestcf.030101.xyz                       | 104.19.156.139                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 281  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 290  | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 302  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 308  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 377  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 426  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 444  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 449  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 450  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 459  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 461  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 494  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 499  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 55       | cloudflare |
| 36   | ipinfo.in                               | 172.67.198.203                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 40   | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 61   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 72   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 78   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 104  | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 115  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 127  | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 134  | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 143  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 161  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 175  | cu.877774.xyz                           | 104.26.4.116                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 176  | cu.877774.xyz                           | 104.26.4.117                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 179  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 245  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 280  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 311  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 313  | www.7749tv.com                          | 104.16.115.36                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 317  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 326  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 327  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 341  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 370  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 378  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 416  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 428  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 455  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 472  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 485  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 501  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 6    | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 8    | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 19   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 25   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 38   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 43   | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 54   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 77   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 80   | cf.zhetengsha.eu.org                    | 172.66.44.77                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 96   | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 120  | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 148  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 150  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 153  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 190  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 202  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 203  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 207  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 209  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 211  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 248  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 259  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 274  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 315  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 329  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 334  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 347  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 363  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 367  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 402  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 424  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 430  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 448  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 464  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 473  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 503  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 31   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 44   | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 52   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 95   | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 102  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 125  | cfip.xxxxxxxx.tk                        | 104.25.105.1                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 144  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 145  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 146  | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 151  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 158  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 180  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 239  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 310  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 318  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 328  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 331  | japan.com                               | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 364  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 382  | cf.090227.xyz                           | 172.64.145.158                          | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 400  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 414  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 434  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 438  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 454  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 476  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 489  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 47   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 62   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 89   | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 91   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 109  | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 116  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 185  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b79:c9c1:cab1:57d6 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 204  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 221  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 237  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 286  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 291  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 296  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 307  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 337  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 346  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 348  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 379  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 391  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 423  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 435  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 445  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 453  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 502  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 18   | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 53   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 56   | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 59   | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 65   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 66   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 74   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 85   | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 105  | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 132  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 133  | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 141  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 198  | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 212  | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 234  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 240  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 249  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 264  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 278  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:ab56:90d6:b7ab    | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 284  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 354  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 471  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 482  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 11   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 17   | comicabc.com                            | 172.67.174.21                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 26   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 51   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 88   | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 119  | cfip.xxxxxxxx.tk                        | 190.93.246.67                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 139  | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 172  | cu.877774.xyz                           | 104.26.4.113                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 227  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 236  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 247  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 272  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 289  | eur.877774.xyz                          | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 299  | otto.ns.cloudflare.com                  | 172.64.35.135                           | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 301  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 333  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 369  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 398  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 425  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 458  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 496  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 497  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 24   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 39   | steamdb.info                            | 104.20.34.212                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 50   | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 79   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 110  | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 123  | cfip.xxxxxxxx.tk                        | 104.21.91.19                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 131  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 138  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 171  | cu.877774.xyz                           | 104.26.4.112                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 201  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 206  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.7                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 314  | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 316  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 350  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 362  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 429  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 432  | ip.gs                                   | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 447  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 483  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 68   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 87   | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 90   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 107  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 108  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 111  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 117  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 140  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 191  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 199  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 268  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 283  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 343  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 344  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 351  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 371  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 443  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 495  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 13   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 41   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 86   | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 106  | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 124  | cfip.xxxxxxxx.tk                        | 188.114.97.144                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 164  | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 399  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 412  | benedict.ns.cloudflare.com              | 108.162.195.205                         | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 421  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 441  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 462  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 35   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 181  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 189  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 228  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 233  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 238  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 250  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 273  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 305  | damien.ns.cloudflare.com                | 172.64.35.168                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 373  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 395  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 409  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 498  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 505  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 49   | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 58   | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 75   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 136  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 262  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 271  | cloudflare-ip.mofashi.ltd               | 2a06:98c1:3121::3                       | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 282  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 293  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 309  | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 342  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 381  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 431  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 436  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 142  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 157  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 160  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 261  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 266  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 335  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 349  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 437  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 440  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 469  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 73   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 129  | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 184  | freeyx.cloudflare88.eu.org              | 141.101.121.31                          | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 260  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 275  | 172.64.35.24                            | 172.64.35.24                            | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 380  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 390  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 394  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 439  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 492  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 5    | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 276  | bestcf.030101.xyz                       | 104.19.157.251                          | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 20   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 94   | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 155  | sullivan.ns.cloudflare.com              | 162.159.44.161                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 336  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 392  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 456  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 67   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 71       | cloudflare |
| 97   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 135  | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 170  | cu.877774.xyz                           | 104.26.4.111                            | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 194  | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 297  | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h2   | ✅ 成功 | 71       | cloudflare |
| 55   | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 72       | cloudflare |
| 126  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 411  | benedict.ns.cloudflare.com              | 172.64.35.205                           | IPv4   | h2   | ✅ 成功 | 72       | cloudflare |
| 57   | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 22   | trevor.ns.cloudflare.com                | 108.162.195.154                         | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 46   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 355  | abdullah.ns.cloudflare.com              | 108.162.195.203                         | IPv4   | h2   | ✅ 成功 | 74       | cloudflare |
| 470  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 74       | cloudflare |
| 486  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h2   | ✅ 成功 | 75       | cloudflare |
| 174  | cu.877774.xyz                           | 104.26.4.115                            | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 320  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 356  | abdullah.ns.cloudflare.com              | 162.159.44.203                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 452  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 251  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 466  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 357  | abdullah.ns.cloudflare.com              | 172.64.35.203                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 413  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 193  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 231  | decker.ns.cloudflare.com                | 162.159.44.155                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 241  | kyree.ns.cloudflare.com                 | 172.64.35.207                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 252  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 393  | moura.ns.cloudflare.com                 | 172.64.35.217                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 478  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 118  | cfip.xxxxxxxx.tk                        | 104.17.127.110                          | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 457  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 48   | ct.877774.xyz                           | 172.64.229.236                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 243  | kyree.ns.cloudflare.com                 | 162.159.44.207                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 460  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 479  | julio.ns.cloudflare.com                 | 108.162.195.209                         | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 9    | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 166  | craig.ns.cloudflare.com                 | 108.162.195.192                         | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 187  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 465  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 165  | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 338  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 99   | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 467  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 491  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 244  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 298  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 386  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 246  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 408  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 468  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 30   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 159  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 322  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 389  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 396  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h2   | ✅ 成功 | 100      | cloudflare |
| 415  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 100      | cloudflare |
| 1    | 172.64.154.18                           | 172.64.154.18                           | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 32   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 102      | cloudflare |
| 101  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h2   | ✅ 成功 | 102      | cloudflare |
| 210  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 279  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:bd32:7275:fc25    | IPv6   | h2   | ✅ 成功 | 102      | cloudflare |
| 197  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 103      | cloudflare |
| 397  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h2   | ✅ 成功 | 103      | cloudflare |
| 2    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 3    | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 29   | wilson.ns.cloudflare.com                | 108.162.195.110                         | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 256  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h2   | ✅ 成功 | 106      | cloudflare |
| 324  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 106      | cloudflare |
| 358  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 106      | cloudflare |
| 446  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |
| 168  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h2   | ✅ 成功 | 107      | cloudflare |
| 195  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 107      | cloudflare |
| 388  | bowen.ns.cloudflare.com                 | 172.64.35.83                            | IPv4   | h2   | ✅ 成功 | 107      | cloudflare |
| 192  | cris.ns.cloudflare.com                  | 108.162.195.202                         | IPv4   | h2   | ✅ 成功 | 108      | cloudflare |
| 360  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h2   | ✅ 成功 | 108      | cloudflare |
| 319  | lewis.ns.cloudflare.com                 | 172.64.35.159                           | IPv4   | h2   | ✅ 成功 | 110      | cloudflare |
| 359  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 111      | cloudflare |
| 300  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 112      | cloudflare |
| 304  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 113      | cloudflare |
| 481  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h2   | ✅ 成功 | 113      | cloudflare |
| 23   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h2   | ✅ 成功 | 115      | cloudflare |
| 100  | braden.ns.cloudflare.com                | 172.64.35.169                           | IPv4   | h2   | ✅ 成功 | 115      | cloudflare |
| 167  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h2   | ✅ 成功 | 116      | cloudflare |
| 339  | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h2   | ✅ 成功 | 116      | cloudflare |
| 242  | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h2   | ✅ 成功 | 117      | cloudflare |
| 254  | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h2   | ✅ 成功 | 117      | cloudflare |
| 487  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h2   | ✅ 成功 | 117      | cloudflare |
| 490  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 117      | cloudflare |
| 186  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h2   | ✅ 成功 | 118      | cloudflare |
| 188  | pranab.ns.cloudflare.com                | 172.64.35.199                           | IPv4   | h2   | ✅ 成功 | 118      | cloudflare |
| 113  | huxley.ns.cloudflare.com                | 108.162.195.188                         | IPv4   | h2   | ✅ 成功 | 119      | cloudflare |
| 405  | rustam.ns.cloudflare.com                | 172.64.35.148                           | IPv4   | h2   | ✅ 成功 | 121      | cloudflare |
| 321  | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h2   | ✅ 成功 | 122      | cloudflare |
| 230  | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h2   | ✅ 成功 | 123      | cloudflare |
| 114  | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4   | h2   | ✅ 成功 | 124      | cloudflare |
| 387  | bowen.ns.cloudflare.com                 | 162.159.44.83                           | IPv4   | h2   | ✅ 成功 | 125      | cloudflare |
| 480  | julio.ns.cloudflare.com                 | 162.159.44.209                          | IPv4   | h2   | ✅ 成功 | 125      | cloudflare |
| 253  | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h2   | ✅ 成功 | 126      | cloudflare |
| 303  | damien.ns.cloudflare.com                | 108.162.195.168                         | IPv4   | h2   | ✅ 成功 | 126      | cloudflare |
| 417  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h2   | ✅ 成功 | 127      | cloudflare |
| 156  | sullivan.ns.cloudflare.com              | 172.64.35.161                           | IPv4   | h2   | ✅ 成功 | 128      | cloudflare |
| 323  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 128      | cloudflare |
| 488  | uriah.ns.cloudflare.com                 | 108.162.195.194                         | IPv4   | h2   | ✅ 成功 | 129      | cloudflare |
| 28   | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h2   | ✅ 成功 | 131      | cloudflare |
| 235  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h2   | ✅ 成功 | 131      | cloudflare |
| 112  | huxley.ns.cloudflare.com                | 172.64.35.188                           | IPv4   | h2   | ✅ 成功 | 132      | cloudflare |
| 98   | braden.ns.cloudflare.com                | 108.162.195.169                         | IPv4   | h2   | ✅ 成功 | 134      | cloudflare |
| 406  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h2   | ✅ 成功 | 135      | cloudflare |
| 21   | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h2   | ✅ 成功 | 136      | cloudflare |
| 27   | wilson.ns.cloudflare.com                | 162.159.44.110                          | IPv4   | h2   | ✅ 成功 | 140      | cloudflare |
| 340  | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h2   | ✅ 成功 | 145      | cloudflare |
| 154  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h2   | ✅ 成功 | 151      | cloudflare |
| 330  | 172.64.52.127                           | 172.64.52.127                           | IPv4   | h2   | ✅ 成功 | 157      | cloudflare |
| 404  | rustam.ns.cloudflare.com                | 162.159.44.148                          | IPv4   | h2   | ✅ 成功 | 162      | cloudflare |
| 232  | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h2   | ✅ 成功 | 172      | cloudflare |
| 372  | 141.147.185.63                          | 141.147.185.63                          | IPv4   | h2   | ✅ 成功 | 304      | cloudflare |
| 4    | 34.143.159.175                          | 34.143.159.175                          | IPv4   | h2   | ✅ 成功 | 453      | cloudflare |
| 294  | 3.0.50.69                               | 3.0.50.69                               | IPv4   | h2   | ✅ 成功 | 515      | cloudflare |
| 92   | 168.138.165.174                         | 168.138.165.174                         | IPv4   | h2   | ✅ 成功 | 728      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 6 条记录
- **快 (50-100ms)**: 432 条记录
- **正常 (100-200ms)**: 61 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 2 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 3 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
