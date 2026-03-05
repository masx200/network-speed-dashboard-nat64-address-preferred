# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/29 17:23:21
- **数据来源**: connectivity_results-20251229-172320.json
- **总测试数**: 495
- **失败测试数**: 181
- **成功测试数**: 314
- **失败率**: 36.57%
- **平均延迟**: 102.87ms
- **最小延迟**: 43ms
- **最大延迟**: 515ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/29 17:23:21
- **IP地址**: 64.236.160.17
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.8835, -87.6305
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 178 次 (98.3%)
- **连接超时: I/O超时**: 3 次 (1.7%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (178 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 5    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 6    | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 9    | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 10   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 13   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 14   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 18   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 19   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 20   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 26   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 27   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 28   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 29   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 30   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 34   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 35   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 38   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 39   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 43   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 44   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 58   | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 61   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 62   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 68   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 69   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 70   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 71   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 76   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 77   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 82   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 83   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 84   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 88   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 89   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 90   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 96   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 100  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 101  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 102  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 106  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 107  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 108  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 117  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 118  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 124  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 125  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 126  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 132  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 133  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 137  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 138  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 139  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 169  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 170  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 171  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 178  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 179  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 180  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 182  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cd47:f6a2:e5d6 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cd47:f6a2:e5d6]:443: connect: network is unreachable |
| 186  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 187  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 188  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 192  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 193  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 194  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 198  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 199  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 200  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 204  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 205  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 206  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 209  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 210  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 218  | bestcf.030101.xyz                       | 2606:4700:0:b21c:66ba:fef:2ef4:2658     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:b21c:66ba:fef:2ef4:2658]:443: connect: network is unreachable     |
| 219  | bestcf.030101.xyz                       | 2606:4700:0:26e6:9bb0:8293:779:dc27     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:26e6:9bb0:8293:779:dc27]:443: connect: network is unreachable     |
| 222  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 223  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 227  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 228  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 231  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 232  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 236  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 237  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 238  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 240  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 243  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 244  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 249  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 250  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 253  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 254  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 259  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 260  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 261  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 265  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 266  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 267  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 271  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 272  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 273  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 279  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 280  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 281  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 284  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 285  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 289  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 290  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 291  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 298  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 302  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 303  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 304  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 308  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 309  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 310  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 311  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 315  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 316  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 317  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 322  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 323  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 329  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 330  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 333  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 334  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 335  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 341  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 345  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 346  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 347  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 351  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 352  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 353  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 358  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 359  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 360  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 362  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 366  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 367  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 373  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 374  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 378  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 379  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 380  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 390  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 396  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 397  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 398  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 402  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 407  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 408  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 409  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 413  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 414  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 415  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 420  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 421  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 422  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 435  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 436  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 437  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 451  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 452  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 457  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 466  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 467  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 468  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 469  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 470  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 471  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 472  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 475  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 476  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 477  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 478  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 481  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 482  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 486  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 489  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 495  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
huxley.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 331  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 428  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 313  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 438  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 336  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 429  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 405  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 491  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 490  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 269  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 403  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 459  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 190  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 395  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 410  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 355  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 56   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 262  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 177  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 239  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 388  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 16   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 111  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 324  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 119  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 127  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 288  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 479  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 40   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 387  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 427  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 129  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 8    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 166  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 286  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 301  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 443  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 196  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 340  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 245  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 52   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 327  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 385  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 462  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 404  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 432  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 23   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 91   | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 377  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 423  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 474  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 15   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 22   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 24   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 202  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 246  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 406  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 275  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 458  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 36   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 131  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 197  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 297  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 364  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 80   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 110  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 123  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 208  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 319  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 389  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 418  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 461  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 64   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 365  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 381  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 53   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 251  | cf.090227.xyz                         | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 376  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 45   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 136  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 203  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 399  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 460  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 54   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 95   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 215  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 282  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 296  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 354  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 229  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 300  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 446  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 50   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 59   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 114  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 176  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 431  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 439  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 172  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 241  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 314  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 416  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 425  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 112  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 174  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 456  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 12   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 128  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 185  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 328  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 442  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 455  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 65   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 488  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 87   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 257  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 263  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 371  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 72   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 191  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 278  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 299  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 339  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 356  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 31   | cf.0sm.com                            | 104.18.122.66   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 48   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 73   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 105  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 109  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 220  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 274  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 338  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 42   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 81   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 453  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 4    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 337  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 66   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 98   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 207  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 268  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 368  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 32   | cf.0sm.com                            | 172.67.75.210   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 49   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 74   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 75   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 318  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 384  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 450  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 47   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 401  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 46   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 252  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 3    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 120  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 121  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 463  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 217  | bestcf.030101.xyz                     | 104.19.153.222  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 326  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 342  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 343  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 464  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 57   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 173  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 221  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 283  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 293  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 447  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 97   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 386  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 25   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 94   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 312  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 332  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 473  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 483  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 78   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 113  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 21   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 444  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 201  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 226  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 292  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 320  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 445  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 484  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 9 条记录
- **快 (50-100ms)**: 183 条记录
- **正常 (100-200ms)**: 8 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 178 次

### 按协议统计

- **none**: 181 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
