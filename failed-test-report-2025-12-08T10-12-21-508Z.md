# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 10:12:21
- **数据来源**: connectivity_results.json
- **总测试数**: 495
- **失败测试数**: 183
- **成功测试数**: 312
- **失败率**: 36.97%
- **平均延迟**: 72.82ms
- **最小延迟**: 33ms
- **最大延迟**: 883ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 174 次 (95.1%)
- **连接超时: I/O超时**: 9 次 (4.9%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (174 次测试)

| 序号 | 主机/域名                                                                           | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ----------------------------------------------------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 6    | palera.in                                                                           | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 7    | palera.in                                                                           | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 11   | fbi.gov                                                                             | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 12   | fbi.gov                                                                             | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 16   | singapore.com                                                                       | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 17   | singapore.com                                                                       | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 18   | singapore.com                                                                       | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 22   | uriah.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 23   | uriah.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 24   | uriah.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 29   | cris.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 30   | cris.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 31   | cris.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 34   | icook.hk                                                                            | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 35   | icook.hk                                                                            | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 36   | [2606:4700:83bd::7d8:2b47]                                                          | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 43   | www.wto.org                                                                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 44   | www.wto.org                                                                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 48   | dnschecker.org                                                                      | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 49   | dnschecker.org                                                                      | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 50   | dnschecker.org                                                                      | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 54   | xn--b6gac.eu.org                                                                    | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 55   | xn--b6gac.eu.org                                                                    | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 59   | rustam.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 60   | rustam.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 61   | rustam.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 64   | www.udemy.com                                                                       | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 65   | www.udemy.com                                                                       | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 66   | [2606:4700:9add::880:52fc]                                                          | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 72   | www.pcmag.com                                                                       | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 73   | www.pcmag.com                                                                       | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 79   | bestcf.030101.xyz                                                                   | 2606:4700:0:bfb9:840c:e0cf:682e:d6f0    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:bfb9:840c:e0cf:682e:d6f0]:443: connect: network is unreachable    |
| 80   | bestcf.030101.xyz                                                                   | 2606:4700:0:bffe:110b:db0f:7987:8a53    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:bffe:110b:db0f:7987:8a53]:443: connect: network is unreachable    |
| 87   | cf.877771.xyz                                                                       | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 88   | cf.877771.xyz                                                                       | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 102  | comicabc.com                                                                        | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 103  | comicabc.com                                                                        | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 107  | silkbook.com                                                                        | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 108  | silkbook.com                                                                        | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 109  | silkbook.com                                                                        | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 113  | braden.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 114  | braden.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 115  | braden.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 122  | [2606:4700:83be::11:74f]                                                            | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 126  | toy-people.com                                                                      | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 127  | toy-people.com                                                                      | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 128  | toy-people.com                                                                      | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 131  | stock.hostmonit.com                                                                 | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 132  | stock.hostmonit.com                                                                 | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 136  | benedict.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 137  | benedict.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 138  | benedict.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 145  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2fb3               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable               |
| 146  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2c4d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable               |
| 150  | sullivan.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 151  | sullivan.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 152  | sullivan.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 159  | tasteatlas.com                                                                      | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 160  | tasteatlas.com                                                                      | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 164  | ip.gs                                                                               | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 165  | ip.gs                                                                               | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 171  | japan.com                                                                           | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 172  | japan.com                                                                           | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 173  | japan.com                                                                           | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 174  | [2606:4700:4403::7357:544f]                                                         | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 176  | [2606:4700:4409::5b5b:7758]                                                         | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 179  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 180  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 185  | craig.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 186  | craig.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 187  | craig.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 189  | [2606:4700:440f::53aa:4126]                                                         | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 193  | www.hugedomains.com                                                                 | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 194  | www.hugedomains.com                                                                 | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 195  | www.hugedomains.com                                                                 | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 196  | [2606:4700:964f::6e2c:588e]                                                         | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 209  | huxley.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 210  | huxley.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 211  | huxley.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 216  | lewis.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 217  | lewis.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 218  | lewis.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 223  | dylan.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 224  | dylan.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 225  | dylan.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 229  | damien.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 230  | damien.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 231  | damien.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 235  | cf.877774.xyz                                                                       | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 236  | cf.877774.xyz                                                                       | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 240  | www.whatismyip.com                                                                  | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 241  | www.whatismyip.com                                                                  | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 242  | www.whatismyip.com                                                                  | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 248  | iplocation.io                                                                       | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 249  | iplocation.io                                                                       | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 250  | iplocation.io                                                                       | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 253  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2fb3               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable               |
| 254  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2c4d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable               |
| 259  | julio.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 260  | julio.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 261  | julio.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 271  | trevor.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 272  | trevor.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 273  | trevor.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 280  | ifconfig.co                                                                         | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 281  | ifconfig.co                                                                         | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 284  | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 285  | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 289  | ip.sb                                                                               | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 290  | ip.sb                                                                               | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 291  | ip.sb                                                                               | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 295  | otto.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 296  | otto.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 297  | otto.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 302  | www.gov.ua                                                                          | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 303  | www.gov.ua                                                                          | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 306  | cf.0sm.com                                                                          | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 307  | cf.0sm.com                                                                          | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 311  | moura.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 312  | moura.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 313  | moura.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 316  | www.digitalocean.com                                                                | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 317  | www.digitalocean.com                                                                | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 321  | time.is                                                                             | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 322  | time.is                                                                             | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 323  | time.is                                                                             | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 328  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3009:aa59:4b67:cdf7:b91:1a47  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cdf7:b91:1a47]:443: connect: network is unreachable  |
| 329  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3010:3392:350c:968b:1f41:32b0 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:3392:350c:968b:1f41:32b0]:443: connect: network is unreachable |
| 333  | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3036::ac43:d4c5               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::ac43:d4c5]:443: connect: network is unreachable               |
| 334  | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3034::6815:3db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:3db6]:443: connect: network is unreachable               |
| 338  | www.ipget.net                                                                       | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 339  | www.ipget.net                                                                       | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 343  | decker.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 344  | decker.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 345  | decker.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 347  | [2606:4700:4408::18c5:3304]                                                         | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 353  | pranab.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 354  | pranab.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 355  | pranab.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 363  | icook.tw                                                                            | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 364  | icook.tw                                                                            | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 369  | wilson.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 370  | wilson.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 371  | wilson.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 374  | whatismyipaddress.com                                                               | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 375  | whatismyipaddress.com                                                               | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 379  | ashton.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 380  | ashton.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 381  | ashton.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 411  | abdullah.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 412  | abdullah.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 413  | abdullah.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 416  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 417  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 423  | yx-auto.pages.dev                                                                   | 2606:4700:3034::6815:9e6                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable                |
| 424  | yx-auto.pages.dev                                                                   | 2606:4700:3033::ac43:a162               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable               |
| 429  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 430  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 437  | bowen.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 438  | bowen.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 439  | bowen.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 443  | kyree.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 444  | kyree.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 445  | kyree.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 460  | [2606:4700:440b::3e6e:5f06]                                                         | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 465  | zread.ai                                                                            | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 466  | zread.ai                                                                            | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 470  | ipinfo.in                                                                           | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 471  | ipinfo.in                                                                           | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 478  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3034::6815:2ffc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:2ffc]:443: connect: network is unreachable               |
| 479  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3037::ac43:aedb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:aedb]:443: connect: network is unreachable               |
| 483  | [2606:4700:8de6::5fa2:799e]                                                         | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 486  | steamdb.info                                                                        | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 487  | steamdb.info                                                                        | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 447  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 455  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 461  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 481  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 482  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 488  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 491  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 494  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 495  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 174 次 (95.1%)
- **连接超时**: 9 次 (4.9%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 183 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 9 次，IPv6失败 174 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：singapore.com (3次),
uriah.ns.cloudflare.com (3次), cris.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 422 |
yx-auto.pages.dev | 104.21.9.230 | IPv4 | h3 | ✅ 成功 | 33 | cloudflare | | 292
| otto.ns.cloudflare.com | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 34 | cloudflare
| | 42 | www.wto.org | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 35 | cloudflare | |
166 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare | |
403 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare | |
425 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare |
| 472 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅ 成功 | 36 | cloudflare
| | 82 | eur.877774.xyz | 104.21.47.209 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 247 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 358 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 37 | cloudflare
| | 421 | yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 | 37 |
cloudflare | | 140 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 143 | cf.090227.xyz | 172.66.44.77 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 163 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 | 38 |
cloudflare | | 267 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 38
| cloudflare | | 283 | cloudflare-ip.mofashi.ltd | 172.67.155.172 | IPv4 | h3 |
✅ 成功 | 38 | cloudflare | | 480 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 |
✅ 成功 | 38 | cloudflare | | 208 | huxley.ns.cloudflare.com | 172.64.35.188 |
IPv4 | h3 | ✅ 成功 | 39 | cloudflare | | 264 | www.okcupid.com | 104.16.144.63
| IPv4 | h3 | ✅ 成功 | 39 | cloudflare | | 101 | comicabc.com | 172.67.174.21 |
IPv4 | h3 | ✅ 成功 | 40 | cloudflare | | 177 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 40 |
cloudflare | | 373 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功
| 40 | cloudflare | | 401 | cmcc.877774.xyz | 104.16.149.1 | IPv4 | h3 | ✅ 成功
| 40 | cloudflare | | 463 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 40
| cloudflare | | 468 | ipinfo.in | 104.21.21.129 | IPv4 | h3 | ✅ 成功 | 40 |
cloudflare | | 243 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 41 |
cloudflare | | 365 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 | 41 |
cloudflare | | 117 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 42
| cloudflare | | 256 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 |
✅ 成功 | 42 | cloudflare | | 346 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅
成功 | 42 | cloudflare | | 448 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 |
✅ 成功 | 42 | cloudflare | | 63 | www.udemy.com | 104.16.142.237 | IPv4 | h3 |
✅ 成功 | 43 | cloudflare | | 99 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅
成功 | 43 | cloudflare | | 148 | sullivan.ns.cloudflare.com | 172.64.35.161 |
IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 268 | trevor.ns.cloudflare.com |
108.162.195.154 | IPv4 | h3 | ✅ 成功 | 43 | cloudflare | | 477 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 104.21.47.252 | IPv4 |
h3 | ✅ 成功 | 43 | cloudflare | | 426 | www.glassdoor.com | 104.17.64.70 | IPv4
| h3 | ✅ 成功 | 44 | cloudflare | | 427 | local-aria2-webui.masx200.ddns-ip.net
| 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 44 | cloudflare | | 266 | www.okcupid.com
| 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 431 | ipv4.ip.sb |
104.26.12.31 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 456 | cfip.xxxxxxxx.tk
| 104.16.241.229 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 86 | cf.877771.xyz
| 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 142 | na.877774.xyz
| 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 332 |
72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re
| 104.21.61.182 | IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 350 |
pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h3 | ✅ 成功 | 46 |
cloudflare | | 356 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 46 |
cloudflare | | 397 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 46
| cloudflare | | 415 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 |
46 | cloudflare | | 13 | singapore.com | 172.67.75.194 | IPv4 | h3 | ✅ 成功 |
47 | cloudflare | | 37 | www.visa.com.hk | 104.18.20.69 | IPv4 | h3 | ✅ 成功 |
47 | cloudflare | | 319 | time.is | 172.67.68.157 | IPv4 | h3 | ✅ 成功 | 47 |
cloudflare | | 336 | www.ipget.net | 172.67.207.26 | IPv4 | h3 | ✅ 成功 | 47 |
cloudflare | | 68 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 | ✅ 成功 | 48
| cloudflare | | 94 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 48 |
cloudflare | | 95 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 48 |
cloudflare | | 366 | wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅
成功 | 48 | cloudflare | | 402 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅
成功 | 48 | cloudflare | | 118 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 |
✅ 成功 | 49 | cloudflare | | 129 | stock.hostmonit.com | 172.67.187.251 | IPv4
| h3 | ✅ 成功 | 49 | cloudflare | | 139 | cfip.1323123.xyz | 104.16.133.220 |
IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 154 | 172.64.147.73 | 172.64.147.73 |
IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 232 | 172.67.243.218 | 172.67.243.218
| IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 263 | www.okcupid.com |
104.18.160.63 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 434 |
bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare
| | 89 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 181 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 192 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 278 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 50 |
cloudflare | | 314 | www.digitalocean.com | 104.19.174.68 | IPv4 | h3 | ✅ 成功
| 50 | cloudflare | | 404 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h3 | ✅ 成功
| 50 | cloudflare | | 5 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 9 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 71 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 144 | cf.090227.xyz | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 239 | www.whatismyip.com | 172.67.69.129 | IPv4 | h3 | ✅ 成功 |
51 | cloudflare | | 464 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅ 成功 | 51 |
cloudflare | | 8 | 172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 74 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 91 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 96 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 120 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 52
| cloudflare | | 178 | ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4
| h3 | ✅ 成功 | 52 | cloudflare | | 182 | craig.ns.cloudflare.com |
108.162.195.192 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 190 |
www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | |
203 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | |
215 | lewis.ns.cloudflare.com | 172.64.35.159 | IPv4 | h3 | ✅ 成功 | 52 |
cloudflare | | 251 | cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功
| 52 | cloudflare | | 385 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅
成功 | 52 | cloudflare | | 451 | cfip.xxxxxxxx.tk | 190.93.246.67 | IPv4 | h3 |
✅ 成功 | 52 | cloudflare | | 21 | uriah.ns.cloudflare.com | 172.64.35.194 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 38 | www.visa.com.hk | 104.18.21.69 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 90 | cu.877774.xyz | 104.26.4.114 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 233 | cf.877774.xyz | 104.18.41.190 |
IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 238 | www.whatismyip.com |
104.26.13.23 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 246 | iplocation.io |
104.26.11.222 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 294 |
otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare
| | 318 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 383
| cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | | 454
| cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | |
46 | dnschecker.org | 104.26.7.89 | IPv4 | h3 | ✅ 成功 | 54 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 64 条记录
- **快 (50-100ms)**: 36 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 9 次
- **IPv6 失败**: 174 次

### 按协议统计

- **none**: 183 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
