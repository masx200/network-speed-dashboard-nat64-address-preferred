# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/22 10:21:38
- **数据来源**: connectivity_results-20251222-102128.json
- **总测试数**: 462
- **失败测试数**: 2
- **成功测试数**: 460
- **失败率**: 0.43%
- **平均延迟**: 544.23ms
- **最小延迟**: 379ms
- **最大延迟**: 1625ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/22 10:21:38
- **IP地址**: 2a09:bac5:1f0a:1232::1d0:b0
- **国家/地区**: China (CN)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: Asia (AS)
- **地理坐标**: 31.2222, 121.4581
- **时区**: Asia/Shanghai
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
| 106  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 402  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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

| 序号 | 主机/域名                             | 目标IP                             | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ---------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 83   | iplocation.io                         | 2606:4700:20::ac43:4664            | IPv6   | h2   | ✅ 成功 | 379      | cloudflare |
| 87   | icook.hk                              | 2606:4700:3037::ac43:a168          | IPv6   | h2   | ✅ 成功 | 380      | cloudflare |
| 345  | zread.ai                              | 172.67.202.78                      | IPv4   | h2   | ✅ 成功 | 380      | cloudflare |
| 203  | cmcc.877774.xyz                       | 104.16.148.244                     | IPv4   | h2   | ✅ 成功 | 381      | cloudflare |
| 284  | www.visa.com.hk                       | 104.18.20.69                       | IPv4   | h2   | ✅ 成功 | 381      | cloudflare |
| 341  | www.udemy.com                         | 104.16.142.237                     | IPv4   | h2   | ✅ 成功 | 382      | cloudflare |
| 86   | icook.hk                              | 2606:4700:3031::6815:5ad2          | IPv6   | h2   | ✅ 成功 | 383      | cloudflare |
| 85   | icook.hk                              | 104.21.90.210                      | IPv4   | h2   | ✅ 成功 | 384      | cloudflare |
| 111  | www.okcupid.com                       | 104.16.239.254                     | IPv4   | h2   | ✅ 成功 | 384      | cloudflare |
| 175  | cf.090227.xyz                         | 104.18.42.98                       | IPv4   | h2   | ✅ 成功 | 384      | cloudflare |
| 214  | cmcc.877774.xyz                       | 104.16.149.11                      | IPv4   | h2   | ✅ 成功 | 384      | cloudflare |
| 51   | 104.18.39.196                         | 104.18.39.196                      | IPv4   | h2   | ✅ 成功 | 385      | cloudflare |
| 145  | cu.877774.xyz                         | 104.26.4.112                       | IPv4   | h2   | ✅ 成功 | 385      | cloudflare |
| 154  | yx-auto.pages.dev                     | 172.66.44.144                      | IPv4   | h2   | ✅ 成功 | 385      | cloudflare |
| 163  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd          | IPv6   | h2   | ✅ 成功 | 385      | cloudflare |
| 176  | cf.090227.xyz                         | 172.64.145.158                     | IPv4   | h2   | ✅ 成功 | 385      | cloudflare |
| 257  | ip.gs                                 | 2606:4700:3035::ac43:a01c          | IPv6   | h2   | ✅ 成功 | 385      | cloudflare |
| 172  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7            | IPv6   | h2   | ✅ 成功 | 386      | cloudflare |
| 276  | whatismyipaddress.com                 | 2606:4700::6813:de4f               | IPv6   | h2   | ✅ 成功 | 386      | cloudflare |
| 338  | tasteatlas.com                        | 104.17.36.105                      | IPv4   | h2   | ✅ 成功 | 386      | cloudflare |
| 376  | www.whatismyip.com                    | 104.26.13.23                       | IPv4   | h2   | ✅ 成功 | 386      | cloudflare |
| 109  | cfip.xxxxxxxx.tk                      | 104.27.21.118                      | IPv4   | h2   | ✅ 成功 | 387      | cloudflare |
| 123  | www.hugedomains.com                   | 2606:4700:20::681a:625             | IPv6   | h2   | ✅ 成功 | 387      | cloudflare |
| 144  | cu.877774.xyz                         | 104.26.4.111                       | IPv4   | h2   | ✅ 成功 | 387      | cloudflare |
| 76   | 104.18.254.88                         | 104.18.254.88                      | IPv4   | h2   | ✅ 成功 | 388      | cloudflare |
| 77   | 172.67.49.134                         | 172.67.49.134                      | IPv4   | h2   | ✅ 成功 | 388      | cloudflare |
| 244  | time.is                               | 104.26.13.54                       | IPv4   | h2   | ✅ 成功 | 388      | cloudflare |
| 335  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca            | IPv6   | h2   | ✅ 成功 | 388      | cloudflare |
| 52   | 104.16.45.84                          | 104.16.45.84                       | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 81   | iplocation.io                         | 2606:4700:20::681a:ade             | IPv6   | h2   | ✅ 成功 | 389      | cloudflare |
| 138  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0            | IPv6   | h2   | ✅ 成功 | 389      | cloudflare |
| 151  | cu.877774.xyz                         | 104.26.4.118                       | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 161  | xn--b6gac.eu.org                      | 172.67.153.253                     | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 287  | cf.877774.xyz                         | 104.18.41.190                      | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 330  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:fb:e00f:f23d:42c6 | IPv6   | h2   | ✅ 成功 | 389      | cloudflare |
| 350  | toy-people.com                        | 104.26.2.36                        | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 351  | toy-people.com                        | 104.26.3.36                        | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 405  | 104.18.78.214                         | 104.18.78.214                      | IPv4   | h2   | ✅ 成功 | 389      | cloudflare |
| 71   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc           | IPv6   | h2   | ✅ 成功 | 390      | cloudflare |
| 82   | iplocation.io                         | 2606:4700:20::681a:bde             | IPv6   | h2   | ✅ 成功 | 390      | cloudflare |
| 118  | www.hugedomains.com                   | 104.26.7.37                        | IPv4   | h2   | ✅ 成功 | 390      | cloudflare |
| 127  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4          | IPv6   | h2   | ✅ 成功 | 390      | cloudflare |
| 259  | ip.sb                                 | 172.67.75.172                      | IPv4   | h2   | ✅ 成功 | 390      | cloudflare |
| 326  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052          | IPv6   | h2   | ✅ 成功 | 390      | cloudflare |
| 434  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387            | IPv6   | h2   | ✅ 成功 | 390      | cloudflare |
| 56   | www.gov.ua                            | 2606:4700:3031::6815:1748          | IPv6   | h2   | ✅ 成功 | 391      | cloudflare |
| 72   | 103.160.204.59                        | 103.160.204.59                     | IPv4   | h2   | ✅ 成功 | 391      | cloudflare |
| 164  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126          | IPv6   | h2   | ✅ 成功 | 391      | cloudflare |
| 205  | cmcc.877774.xyz                       | 104.16.149.2                       | IPv4   | h2   | ✅ 成功 | 391      | cloudflare |
| 293  | dnschecker.org                        | 104.26.6.89                        | IPv4   | h2   | ✅ 成功 | 391      | cloudflare |
| 328  | freeyx.cloudflare88.eu.org            | 141.101.121.124                    | IPv4   | h2   | ✅ 成功 | 391      | cloudflare |
| 143  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758          | IPv6   | h2   | ✅ 成功 | 392      | cloudflare |
| 182  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd            | IPv6   | h2   | ✅ 成功 | 392      | cloudflare |
| 204  | cmcc.877774.xyz                       | 104.16.149.1                       | IPv4   | h2   | ✅ 成功 | 392      | cloudflare |
| 113  | www.okcupid.com                       | 104.18.160.63                      | IPv4   | h2   | ✅ 成功 | 393      | cloudflare |
| 262  | ip.sb                                 | 2606:4700:20::ac43:4bac            | IPv6   | h2   | ✅ 成功 | 393      | cloudflare |
| 156  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90          | IPv6   | h2   | ✅ 成功 | 394      | cloudflare |
| 218  | cmcc.877774.xyz                       | 104.16.148.2                       | IPv4   | h2   | ✅ 成功 | 394      | cloudflare |
| 354  | toy-people.com                        | 2606:4700:20::ac43:4812            | IPv6   | h2   | ✅ 成功 | 394      | cloudflare |
| 114  | www.okcupid.com                       | 104.16.144.63                      | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 134  | 172.67.243.218                        | 172.67.243.218                     | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 141  | www.visa.com.sg                       | 104.18.12.229                      | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 211  | cmcc.877774.xyz                       | 104.16.149.8                       | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 342  | www.udemy.com                         | 104.16.143.237                     | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 415  | eur.877774.xyz                        | 104.21.47.209                      | IPv4   | h2   | ✅ 成功 | 395      | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.173                     | IPv4   | h2   | ✅ 成功 | 396      | cloudflare |
| 153  | yx-auto.pages.dev                     | 172.66.47.112                      | IPv4   | h2   | ✅ 成功 | 396      | cloudflare |
| 272  | singapore.com                         | 2606:4700:20::ac43:4bc2            | IPv6   | h2   | ✅ 成功 | 396      | cloudflare |
| 365  | cloudflare-ip.mofashi.ltd             | 104.21.72.233                      | IPv4   | h2   | ✅ 成功 | 396      | cloudflare |
| 377  | www.whatismyip.com                    | 172.67.69.129                      | IPv4   | h2   | ✅ 成功 | 396      | cloudflare |
| 460  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb            | IPv6   | h2   | ✅ 成功 | 396      | cloudflare |
| 226  | www.ipchicken.com                     | 172.67.68.101                      | IPv4   | h2   | ✅ 成功 | 397      | cloudflare |
| 250  | 104.17.68.85                          | 104.17.68.85                       | IPv4   | h2   | ✅ 成功 | 397      | cloudflare |
| 366  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9          | IPv6   | h2   | ✅ 成功 | 397      | cloudflare |
| 372  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf            | IPv6   | h2   | ✅ 成功 | 397      | cloudflare |
| 126  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7          | IPv6   | h2   | ✅ 成功 | 398      | cloudflare |
| 236  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9            | IPv6   | h2   | ✅ 成功 | 398      | cloudflare |
| 273  | whatismyipaddress.com                 | 104.19.222.79                      | IPv4   | h2   | ✅ 成功 | 398      | cloudflare |
| 296  | dnschecker.org                        | 2606:4700:20::681a:759             | IPv6   | h2   | ✅ 成功 | 398      | cloudflare |
| 409  | icook.tw                              | 104.20.28.74                       | IPv4   | h2   | ✅ 成功 | 398      | cloudflare |
| 166  | fbi.gov                               | 104.16.148.244                     | IPv4   | h2   | ✅ 成功 | 399      | cloudflare |
| 243  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394            | IPv6   | h2   | ✅ 成功 | 399      | cloudflare |
| 159  | saas.sin.fan                          | 162.159.36.20                      | IPv4   | h2   | ✅ 成功 | 401      | cloudflare |
| 160  | xn--b6gac.eu.org                      | 104.21.90.78                       | IPv4   | h2   | ✅ 成功 | 401      | cloudflare |
| 263  | ip.sb                                 | 2606:4700:20::681a:c1f             | IPv6   | h2   | ✅ 成功 | 401      | cloudflare |
| 139  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0            | IPv6   | h2   | ✅ 成功 | 402      | cloudflare |
| 157  | 172.64.151.55                         | 172.64.151.55                      | IPv4   | h2   | ✅ 成功 | 402      | cloudflare |
| 158  | saas.sin.fan                          | 162.159.36.5                       | IPv4   | h2   | ✅ 成功 | 402      | cloudflare |
| 213  | cmcc.877774.xyz                       | 104.16.149.10                      | IPv4   | h2   | ✅ 成功 | 402      | cloudflare |
| 237  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9            | IPv6   | h2   | ✅ 成功 | 402      | cloudflare |
| 292  | dnschecker.org                        | 172.67.73.216                      | IPv4   | h2   | ✅ 成功 | 402      | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.29.164                      | IPv4   | h2   | ✅ 成功 | 402      | cloudflare |
| 93   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc            | IPv6   | h2   | ✅ 成功 | 403      | cloudflare |
| 152  | cu.877774.xyz                         | 104.26.4.119                       | IPv4   | h2   | ✅ 成功 | 403      | cloudflare |
| 440  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f            | IPv6   | h2   | ✅ 成功 | 403      | cloudflare |
| 162  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e          | IPv6   | h2   | ✅ 成功 | 404      | cloudflare |
| 269  | singapore.com                         | 104.26.12.140                      | IPv4   | h2   | ✅ 成功 | 404      | cloudflare |
| 407  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47           | IPv6   | h2   | ✅ 成功 | 404      | cloudflare |
| 325  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae          | IPv6   | h2   | ✅ 成功 | 405      | cloudflare |
| 343  | www.udemy.com                         | 2606:4700::6810:8eed               | IPv6   | h2   | ✅ 成功 | 405      | cloudflare |
| 5    | comicabc.com                          | 2606:4700:3036::6815:400a          | IPv6   | h2   | ✅ 成功 | 407      | cloudflare |
| 201  | cmcc.877774.xyz                       | 104.16.148.11                      | IPv4   | h2   | ✅ 成功 | 407      | cloudflare |
| 358  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b            | IPv6   | h2   | ✅ 成功 | 408      | cloudflare |
| 167  | fbi.gov                               | 2606:4700::6810:94f4               | IPv6   | h2   | ✅ 成功 | 409      | cloudflare |
| 270  | singapore.com                         | 2606:4700:20::681a:c8c             | IPv6   | h2   | ✅ 成功 | 409      | cloudflare |
| 280  | silkbook.com                          | 104.26.8.160                       | IPv4   | h2   | ✅ 成功 | 409      | cloudflare |
| 337  | tasteatlas.com                        | 104.17.37.105                      | IPv4   | h2   | ✅ 成功 | 409      | cloudflare |
| 67   | cloudflare.182682.xyz                 | 2a06:98c1:3120::5692:61a4          | IPv6   | h2   | ✅ 成功 | 410      | cloudflare |
| 392  | bestcf.030101.xyz                     | 172.67.191.116                     | IPv4   | h2   | ✅ 成功 | 410      | cloudflare |
| 189  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9            | IPv6   | h2   | ✅ 成功 | 411      | cloudflare |
| 344  | www.udemy.com                         | 2606:4700::6810:8fed               | IPv6   | h2   | ✅ 成功 | 411      | cloudflare |
| 278  | silkbook.com                          | 104.26.9.160                       | IPv4   | h2   | ✅ 成功 | 412      | cloudflare |
| 378  | www.whatismyip.com                    | 2606:4700:20::681a:d17             | IPv6   | h2   | ✅ 成功 | 412      | cloudflare |
| 379  | www.whatismyip.com                    | 2606:4700:20::ac43:4581            | IPv6   | h2   | ✅ 成功 | 412      | cloudflare |
| 165  | fbi.gov                               | 104.16.149.244                     | IPv4   | h2   | ✅ 成功 | 413      | cloudflare |
| 265  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182                     | IPv4   | h2   | ✅ 成功 | 413      | cloudflare |
| 69   | 172.67.110.232                        | 172.67.110.232                     | IPv4   | h2   | ✅ 成功 | 414      | cloudflare |
| 84   | icook.hk                              | 172.67.161.104                     | IPv4   | h2   | ✅ 成功 | 414      | cloudflare |
| 277  | 104.18.42.26                          | 104.18.42.26                       | IPv4   | h2   | ✅ 成功 | 414      | cloudflare |
| 353  | toy-people.com                        | 2606:4700:20::681a:224             | IPv6   | h2   | ✅ 成功 | 414      | cloudflare |
| 120  | www.hugedomains.com                   | 172.67.70.191                      | IPv4   | h2   | ✅ 成功 | 415      | cloudflare |
| 235  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9            | IPv6   | h2   | ✅ 成功 | 415      | cloudflare |
| 321  | www.digitalocean.com                  | 2606:4700::6813:ad44               | IPv6   | h2   | ✅ 成功 | 415      | cloudflare |
| 361  | 104.16.223.179                        | 104.16.223.179                     | IPv4   | h2   | ✅ 成功 | 415      | cloudflare |
| 346  | zread.ai                              | 104.21.76.240                      | IPv4   | h2   | ✅ 成功 | 416      | cloudflare |
| 398  | gamer.com.tw                          | 104.18.2.197                       | IPv4   | h2   | ✅ 成功 | 416      | cloudflare |
| 7    | cf.0sm.com                            | 104.21.7.133                       | IPv4   | h2   | ✅ 成功 | 417      | cloudflare |
| 79   | iplocation.io                         | 172.67.70.100                      | IPv4   | h2   | ✅ 成功 | 418      | cloudflare |
| 451  | japan.com                             | 104.26.4.60                        | IPv4   | h2   | ✅ 成功 | 418      | cloudflare |
| 227  | www.ipchicken.com                     | 104.26.6.112                       | IPv4   | h2   | ✅ 成功 | 419      | cloudflare |
| 391  | 198.62.62.4                           | 198.62.62.4                        | IPv4   | h2   | ✅ 成功 | 419      | cloudflare |
| 396  | 172.67.79.211                         | 172.67.79.211                      | IPv4   | h2   | ✅ 成功 | 419      | cloudflare |
| 256  | ip.gs                                 | 104.21.14.176                      | IPv4   | h2   | ✅ 成功 | 420      | cloudflare |
| 359  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b            | IPv6   | h2   | ✅ 成功 | 420      | cloudflare |
| 416  | ifconfig.co                           | 172.67.168.106                     | IPv4   | h2   | ✅ 成功 | 420      | cloudflare |
| 6    | comicabc.com                          | 2606:4700:3030::ac43:ae15          | IPv6   | h2   | ✅ 成功 | 421      | cloudflare |
| 59   | cloudflare.182682.xyz                 | 104.18.185.26                      | IPv4   | h2   | ✅ 成功 | 421      | cloudflare |
| 207  | cmcc.877774.xyz                       | 104.16.149.4                       | IPv4   | h2   | ✅ 成功 | 422      | cloudflare |
| 282  | silkbook.com                          | 2606:4700:20::ac43:4bd0            | IPv6   | h2   | ✅ 成功 | 423      | cloudflare |
| 406  | 104.18.37.40                          | 104.18.37.40                       | IPv4   | h2   | ✅ 成功 | 423      | cloudflare |
| 108  | cfip.xxxxxxxx.tk                      | 104.20.255.53                      | IPv4   | h2   | ✅ 成功 | 424      | cloudflare |
| 229  | www.glassdoor.com                     | 104.16.25.46                       | IPv4   | h2   | ✅ 成功 | 424      | cloudflare |
| 254  | palera.in                             | 2606:4700:3035::6815:3a48          | IPv6   | h2   | ✅ 成功 | 424      | cloudflare |
| 401  | 104.26.13.31                          | 104.26.13.31                       | IPv4   | h2   | ✅ 成功 | 424      | cloudflare |
| 347  | zread.ai                              | 2606:4700:3032::ac43:ca4e          | IPv6   | h2   | ✅ 成功 | 425      | cloudflare |
| 61   | cloudflare.182682.xyz                 | 104.21.227.134                     | IPv4   | h2   | ✅ 成功 | 426      | cloudflare |
| 178  | cf.090227.xyz                         | 2a06:98c1:3101::ac40:919e          | IPv6   | h2   | ✅ 成功 | 426      | cloudflare |
| 224  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353            | IPv6   | h2   | ✅ 成功 | 426      | cloudflare |
| 340  | tasteatlas.com                        | 2606:4700::6811:2569               | IPv6   | h2   | ✅ 成功 | 426      | cloudflare |
| 168  | fbi.gov                               | 2606:4700::6810:95f4               | IPv6   | h2   | ✅ 成功 | 427      | cloudflare |
| 66   | cloudflare.182682.xyz                 | 2606:4700:8ca0::3dc4:21a2          | IPv6   | h2   | ✅ 成功 | 428      | cloudflare |
| 195  | cmcc.877774.xyz                       | 104.16.148.5                       | IPv4   | h2   | ✅ 成功 | 428      | cloudflare |
| 352  | toy-people.com                        | 2606:4700:20::681a:324             | IPv6   | h2   | ✅ 成功 | 428      | cloudflare |
| 339  | tasteatlas.com                        | 2606:4700::6811:2469               | IPv6   | h2   | ✅ 成功 | 429      | cloudflare |
| 124  | cf.877771.xyz                         | 172.67.152.183                     | IPv4   | h2   | ✅ 成功 | 430      | cloudflare |
| 142  | www.visa.com.sg                       | 104.18.13.229                      | IPv4   | h2   | ✅ 成功 | 430      | cloudflare |
| 410  | icook.tw                              | 2606:4700:10::6814:1c4a            | IPv6   | h2   | ✅ 成功 | 430      | cloudflare |
| 400  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304          | IPv6   | h2   | ✅ 成功 | 433      | cloudflare |
| 183  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd            | IPv6   | h2   | ✅ 成功 | 434      | cloudflare |
| 452  | japan.com                             | 104.26.5.60                        | IPv4   | h2   | ✅ 成功 | 435      | cloudflare |
| 45   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb          | IPv6   | h2   | ✅ 成功 | 436      | cloudflare |
| 247  | time.is                               | 2606:4700:20::ac43:449d            | IPv6   | h2   | ✅ 成功 | 436      | cloudflare |
| 302  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad            | IPv6   | h2   | ✅ 成功 | 436      | cloudflare |
| 371  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf            | IPv6   | h2   | ✅ 成功 | 436      | cloudflare |
| 374  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06          | IPv6   | h2   | ✅ 成功 | 437      | cloudflare |
| 258  | ip.gs                                 | 2606:4700:3036::6815:eb0           | IPv6   | h2   | ✅ 成功 | 438      | cloudflare |
| 271  | singapore.com                         | 2606:4700:20::681a:d8c             | IPv6   | h2   | ✅ 成功 | 438      | cloudflare |
| 260  | ip.sb                                 | 104.26.12.31                       | IPv4   | h2   | ✅ 成功 | 439      | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.174                     | IPv4   | h2   | ✅ 成功 | 440      | cloudflare |
| 290  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242          | IPv6   | h2   | ✅ 成功 | 440      | cloudflare |
| 348  | zread.ai                              | 2606:4700:3033::6815:4cf0          | IPv6   | h2   | ✅ 成功 | 440      | cloudflare |
| 147  | cu.877774.xyz                         | 104.26.4.114                       | IPv4   | h2   | ✅ 成功 | 442      | cloudflare |
| 248  | time.is                               | 2606:4700:20::681a:c36             | IPv6   | h2   | ✅ 成功 | 443      | cloudflare |
| 274  | whatismyipaddress.com                 | 104.19.223.79                      | IPv4   | h2   | ✅ 成功 | 443      | cloudflare |
| 103  | cfip.xxxxxxxx.tk                      | 104.25.105.1                       | IPv4   | h2   | ✅ 成功 | 444      | cloudflare |
| 251  | palera.in                             | 104.21.58.72                       | IPv4   | h2   | ✅ 成功 | 445      | cloudflare |
| 285  | www.visa.com.hk                       | 104.18.21.69                       | IPv4   | h2   | ✅ 成功 | 445      | cloudflare |
| 360  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b            | IPv6   | h2   | ✅ 成功 | 445      | cloudflare |
| 336  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca            | IPv6   | h2   | ✅ 成功 | 446      | cloudflare |
| 403  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f             | IPv6   | h2   | ✅ 成功 | 446      | cloudflare |
| 177  | cf.090227.xyz                         | 2a06:98c1:3108::6812:2a62          | IPv6   | h2   | ✅ 成功 | 447      | cloudflare |
| 385  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb            | IPv6   | h2   | ✅ 成功 | 447      | cloudflare |
| 13   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a          | IPv6   | h2   | ✅ 成功 | 448      | cloudflare |
| 446  | www.wto.org                           | 2606:4700:4406::ac40:9242          | IPv6   | h2   | ✅ 成功 | 448      | cloudflare |
| 97   | cfip.xxxxxxxx.tk                      | 190.93.246.67                      | IPv4   | h2   | ✅ 成功 | 449      | cloudflare |
| 364  | cloudflare-ip.mofashi.ltd             | 172.67.155.172                     | IPv4   | h2   | ✅ 成功 | 450      | cloudflare |
| 412  | 104.19.223.58                         | 104.19.223.58                      | IPv4   | h2   | ✅ 成功 | 450      | cloudflare |
| 255  | ip.gs                                 | 172.67.160.28                      | IPv4   | h2   | ✅ 成功 | 451      | cloudflare |
| 455  | japan.com                             | 2606:4700:20::ac43:465c            | IPv6   | h2   | ✅ 成功 | 451      | cloudflare |
| 46   | ipinfo.in                             | 2606:4700:3031::6815:1581          | IPv6   | h2   | ✅ 成功 | 452      | cloudflare |
| 447  | stock.hostmonit.com                   | 172.67.187.251                     | IPv4   | h2   | ✅ 成功 | 452      | cloudflare |
| 310  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1            | IPv6   | h2   | ✅ 成功 | 453      | cloudflare |
| 323  | cf.zhetengsha.eu.org                  | 172.64.145.158                     | IPv4   | h2   | ✅ 成功 | 453      | cloudflare |
| 17   | www.pcmag.com                         | 2606:4700::6810:1576               | IPv6   | h2   | ✅ 成功 | 454      | cloudflare |
| 300  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad            | IPv6   | h2   | ✅ 成功 | 454      | cloudflare |
| 50   | steamdb.info                          | 2606:4700:10::6814:22d4            | IPv6   | h2   | ✅ 成功 | 455      | cloudflare |
| 283  | silkbook.com                          | 2606:4700:20::681a:9a0             | IPv6   | h2   | ✅ 成功 | 455      | cloudflare |
| 363  | 162.159.133.85                        | 162.159.133.85                     | IPv4   | h2   | ✅ 成功 | 456      | cloudflare |
| 442  | www.csgo.com                          | 195.85.59.95                       | IPv4   | h2   | ✅ 成功 | 457      | cloudflare |
| 9    | cf.0sm.com                            | 2606:4700:3032::6815:785           | IPv6   | h2   | ✅ 成功 | 458      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 200 条记录
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
