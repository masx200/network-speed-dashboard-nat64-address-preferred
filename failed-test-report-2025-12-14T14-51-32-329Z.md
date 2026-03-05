# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:51:32
- **数据来源**: connectivity_results-20251214-145131.json
- **总测试数**: 450
- **失败测试数**: 4
- **成功测试数**: 446
- **失败率**: 0.89%
- **平均延迟**: 69.83ms
- **最小延迟**: 40ms
- **最大延迟**: 713ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:51:32
- **IP地址**: 2a09:bac5:9f27:2828::400:63
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

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 32   | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |
| 91   | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 270  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 331  | kyree.ns.cloudflare.com  | 172.64.35.207   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.207:443: i/o timeout   |

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

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                                | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 126  | whatismyipaddress.com                 | 104.19.223.79                         | IPv4   | h2   | ✅ 成功 | 40       | cloudflare |
| 315  | cloudflare-ip.mofashi.ltd             | 188.114.97.3                          | IPv4   | h2   | ✅ 成功 | 41       | cloudflare |
| 157  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc               | IPv6   | h2   | ✅ 成功 | 42       | cloudflare |
| 268  | www.udemy.com                         | 2606:4700::6810:8fed                  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 351  | fbi.gov                               | 2606:4700::6810:94f4                  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 49   | steamdb.info                          | 2606:4700:10::ac42:affa               | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 346  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e             | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 437  | ifconfig.co                           | 2606:4700:3030::ac43:a86a             | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 30   | ipv4.ip.sb                            | 104.26.13.31                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 42   | ct.877774.xyz                         | 172.64.229.217                        | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 55   | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394               | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 58   | 172.67.75.172                         | 172.67.75.172                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 138  | icook.hk                              | 2606:4700:3031::6815:5ad2             | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 186  | 172.67.106.26                         | 172.67.106.26                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 215  | cmcc.877774.xyz                       | 104.16.148.4                          | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 256  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70             | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 260  | na.877774.xyz                         | 104.18.187.25                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 269  | www.udemy.com                         | 2606:4700::6810:8eed                  | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 369  | www.digitalocean.com                  | 104.19.174.68                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 409  | www.wto.org                           | 104.18.41.190                         | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 447  | japan.com                             | 104.26.4.60                           | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 8    | www.pcmag.com                         | 2606:4700::6810:1576                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 105  | ip.sb                                 | 2606:4700:20::ac43:4bac               | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 123  | 104.26.6.112                          | 104.26.6.112                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 145  | www.4chan.org                         | 104.16.229.229                        | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 151  | dnschecker.org                        | 2606:4700:20::681a:759                | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 174  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be             | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 199  | cu.877774.xyz                         | 104.26.4.118                          | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 234  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7             | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 264  | tasteatlas.com                        | 2606:4700::6811:2469                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 371  | www.digitalocean.com                  | 2606:4700::6813:ad44                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 372  | www.digitalocean.com                  | 2606:4700::6813:ae44                  | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 12   | ipinfo.in                             | 2606:4700:3031::6815:1581             | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 13   | www.ipget.net                         | 104.21.15.212                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 57   | shopify.com                           | 23.227.38.33                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 67   | www.ipchicken.com                     | 172.67.68.101                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 72   | iplocation.io                         | 2606:4700:20::681a:ade                | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 131  | 103.160.204.59                        | 103.160.204.59                        | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 177  | www.hugedomains.com                   | 104.26.6.37                           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 197  | cu.877774.xyz                         | 104.26.4.116                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 198  | cu.877774.xyz                         | 104.26.4.117                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 209  | cmcc.877774.xyz                       | 104.16.149.11                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 221  | cmcc.877774.xyz                       | 104.16.148.10                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 235  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4             | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 246  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06             | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 258  | gamer.com.tw                          | 104.18.2.197                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 292  | toy-people.com                        | 2606:4700:20::681a:224                | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 295  | zread.ai                              | 188.114.96.3                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 307  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 336  | saas.sin.fan                          | 162.159.36.20                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 337  | saas.sin.fan                          | 162.159.36.5                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 374  | 104.18.37.40                          | 104.18.37.40                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 401  | time.is                               | 172.67.68.157                         | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 404  | time.is                               | 2606:4700:20::681a:d36                | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 407  | www.csgo.com                          | 195.85.59.95                          | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 442  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f               | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 6    | www.pcmag.com                         | 104.16.21.118                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 11   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb             | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 16   | www.ipget.net                         | 2606:4700:3036::6815:fd4              | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 19   | comicabc.com                          | 2a06:98c1:3120::3                     | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.161                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 44   | ct.877774.xyz                         | 172.64.229.44                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 61   | www.gov.ua                            | 2606:4700:3033::ac43:d17f             | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 62   | www.gov.ua                            | 2606:4700:3031::6815:1748             | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 108  | singapore.com                         | 104.26.13.140                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 110  | singapore.com                         | 2606:4700:20::681a:c8c                | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 114  | ip.gs                                 | 2606:4700:3036::6815:eb0              | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 133  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 137  | icook.hk                              | 2606:4700:3037::ac43:a168             | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 181  | www.okcupid.com                       | 104.16.223.254                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 196  | cu.877774.xyz                         | 104.26.4.115                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 203  | www.visa.cn                           | 162.159.152.2                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 229  | cmcc.877774.xyz                       | 104.16.149.5                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 277  | freeyx.cloudflare88.eu.org            | 141.101.121.19                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 322  | bestcf.030101.xyz                     | 2606:4700:0:26e6:9bb0:8293:779:dc27   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 327  | www.whatismyip.com                    | 2606:4700:20::ac43:4581               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 354  | icook.tw                              | 2606:4700:10::6814:1c4a               | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 381  | cfip.1323123.xyz                      | 104.16.133.220                        | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 421  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1              | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 425  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47              | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 427  | eur.877774.xyz                        | 104.21.26.150                         | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 446  | japan.com                             | 172.67.70.92                          | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 450  | japan.com                             | 2606:4700:20::681a:43c                | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 15   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 17   | comicabc.com                          | 172.67.174.21                         | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 46   | steamdb.info                          | 172.66.175.250                        | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 50   | 104.16.45.84                          | 104.16.45.84                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 64   | www.glassdoor.com                     | 104.16.25.46                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 65   | www.glassdoor.com                     | 104.17.64.70                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 88   | cfip.xxxxxxxx.tk                      | 188.114.97.144                        | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 124  | 172.67.110.232                        | 172.67.110.232                        | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 160  | 104.18.42.26                          | 104.18.42.26                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 178  | www.hugedomains.com                   | 2606:4700:20::681a:625                | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 183  | www.okcupid.com                       | 104.18.160.63                         | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 192  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 244  | www.visa.com.sg                       | 104.18.12.229                         | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 266  | www.udemy.com                         | 104.16.142.237                        | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 279  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:0:72:9d2c:ac0d:3727    | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 280  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:bf:5dba:fa1d:5993:9cf8 | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 288  | toy-people.com                        | 104.26.3.36                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 291  | toy-people.com                        | 2606:4700:20::681a:324                | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 294  | zread.ai                              | 188.114.97.3                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 308  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 347  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 350  | fbi.gov                               | 2606:4700::6810:95f4                  | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 355  | icook.tw                              | 2606:4700:10::ac42:9e73               | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 359  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 360  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 410  | www.wto.org                           | 2606:4700:4406::ac40:9242             | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 423  | 104.26.13.31                          | 104.26.13.31                          | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 445  | japan.com                             | 104.26.5.60                           | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 29   | ipv4.ip.sb                            | 104.26.12.31                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 34   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 71   | iplocation.io                         | 104.26.10.222                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 74   | iplocation.io                         | 2606:4700:20::681a:bde                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 109  | singapore.com                         | 2606:4700:20::681a:d8c                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 119  | silkbook.com                          | 2606:4700:20::681a:8a0                | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 147  | 172.67.49.134                         | 172.67.49.134                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 152  | dnschecker.org                        | 2606:4700:20::ac43:49d8               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 163  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e             | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 173  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242             | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 217  | cmcc.877774.xyz                       | 104.16.148.6                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 227  | cmcc.877774.xyz                       | 104.16.149.3                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 261  | na.877774.xyz                         | 104.19.74.233                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 285  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 297  | zread.ai                              | 2606:4700:3032::ac43:ca4e             | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 301  | 104.17.79.11                          | 104.17.79.11                          | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 318  | cloudflare-ip.mofashi.ltd             | 2a06:98c1:3121::3                     | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 321  | bestcf.030101.xyz                     | 2606:4700:0:b21c:66ba:fef:2ef4:2658   | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 379  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 390  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 428  | eur.877774.xyz                        | 104.21.29.164                         | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 432  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb               | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 438  | ifconfig.co                           | 2606:4700:3037::6815:365b             | IPv6   | h2   | ✅ 成功 | 50       | cloudflare |
| 28   | ipv4.ip.sb                            | 172.67.75.172                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.174                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 41   | ct.877774.xyz                         | 172.64.229.195                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 59   | www.gov.ua                            | 172.67.209.127                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 66   | www.ipchicken.com                     | 104.26.7.112                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 69   | iplocation.io                         | 172.67.70.100                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 77   | palera.in                             | 172.67.157.122                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 92   | cfip.xxxxxxxx.tk                      | 198.41.214.141                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 101  | ip.sb                                 | 172.67.75.172                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 104  | ip.sb                                 | 2606:4700:20::681a:d1f                | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 107  | singapore.com                         | 172.67.75.194                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 112  | ip.gs                                 | 104.21.14.176                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 134  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6             | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 150  | dnschecker.org                        | 104.26.6.89                           | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 175  | www.hugedomains.com                   | 172.67.70.191                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 205  | cmcc.877774.xyz                       | 104.16.149.7                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 213  | cmcc.877774.xyz                       | 104.16.148.2                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 225  | cmcc.877774.xyz                       | 104.16.149.1                          | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 232  | cf.877771.xyz                         | 104.21.80.180                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 252  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 320  | bestcf.030101.xyz                     | 104.19.153.222                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 339  | cf.zhetengsha.eu.org                  | 172.64.145.158                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 370  | www.digitalocean.com                  | 104.19.173.68                         | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 419  | stock.hostmonit.com                   | 172.67.187.251                        | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 449  | japan.com                             | 2606:4700:20::ac43:465c               | IPv6   | h2   | ✅ 成功 | 51       | cloudflare |
| 10   | ipinfo.in                             | 104.21.21.129                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 45   | 104.18.39.196                         | 104.18.39.196                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 89   | cfip.xxxxxxxx.tk                      | 190.93.246.67                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 102  | ip.sb                                 | 104.26.13.31                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 121  | silkbook.com                          | 2606:4700:20::681a:9a0                | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 122  | 104.18.37.13                          | 104.18.37.13                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 176  | www.hugedomains.com                   | 104.26.7.37                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 194  | cu.877774.xyz                         | 104.26.4.113                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 202  | cu.877774.xyz                         | 104.26.4.112                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 207  | cmcc.877774.xyz                       | 104.16.149.9                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 210  | cmcc.877774.xyz                       | 104.16.149.12                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 214  | cmcc.877774.xyz                       | 104.16.148.3                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 219  | cmcc.877774.xyz                       | 104.16.148.8                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 222  | cmcc.877774.xyz                       | 104.16.148.11                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 228  | cmcc.877774.xyz                       | 104.16.149.4                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 233  | cf.877771.xyz                         | 172.67.152.183                        | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 242  | 172.67.243.218                        | 172.67.243.218                        | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 253  | yx-auto.pages.dev                     | 172.66.44.144                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 257  | gamer.com.tw                          | 104.18.3.197                          | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 289  | toy-people.com                        | 104.26.2.36                           | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 335  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 367  | 104.17.142.12                         | 104.17.142.12                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 396  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9               | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 408  | www.wto.org                           | 172.64.146.66                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 420  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb             | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 422  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 426  | eur.877774.xyz                        | 104.21.47.209                         | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 3    | cf.0sm.com                            | 104.21.7.133                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 4    | cf.0sm.com                            | 172.67.187.145                        | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 5    | www.pcmag.com                         | 104.16.20.118                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 24   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 70   | iplocation.io                         | 104.26.11.222                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 75   | 104.16.61.163                         | 104.16.61.163                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 83   | cfip.xxxxxxxx.tk                      | 104.18.228.35                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 85   | cfip.xxxxxxxx.tk                      | 104.21.91.19                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 113  | ip.gs                                 | 172.67.160.28                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 132  | 104.18.254.88                         | 104.18.254.88                         | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 135  | icook.hk                              | 172.67.161.104                        | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 167  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1               | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 187  | 172.67.120.0                          | 172.67.120.0                          | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 243  | 162.159.133.85                        | 162.159.133.85                        | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 111 条记录
- **快 (50-100ms)**: 89 条记录
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
