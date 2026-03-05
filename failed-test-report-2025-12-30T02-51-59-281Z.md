# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:51:59
- **数据来源**: connectivity_results-20251230-025158.json
- **总测试数**: 633
- **失败测试数**: 203
- **成功测试数**: 430
- **失败率**: 32.07%
- **平均延迟**: 107.76ms
- **最小延迟**: 52ms
- **最大延迟**: 683ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:51:59
- **IP地址**: 20.49.61.54
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 200 次 (98.5%)
- **连接超时: I/O超时**: 3 次 (1.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (200 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 7    | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 8    | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 11   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 12   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 15   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 16   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 20   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 21   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 22   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 25   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 26   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 41   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 42   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 49   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 50   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 51   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 52   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 53   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 56   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 57   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 62   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 65   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 66   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 98   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 99   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 100  | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 104  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 105  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 106  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 111  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 112  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 113  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 116  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 119  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 120  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 124  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 125  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 126  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 137  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 138  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 139  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 143  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 144  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 145  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 149  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 150  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 157  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 158  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 165  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 166  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 167  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 171  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 172  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 173  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 178  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 179  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 180  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 184  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 185  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 186  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 190  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 191  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 192  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 196  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 197  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 198  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 203  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 204  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 205  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 208  | freeyx.cloudflare88.eu.org              | 2606:4700:3009::83da:e4f:b8a5           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009::83da:e4f:b8a5]:443: connect: network is unreachable           |
| 209  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:7dab:39d5:a339:79b6:3c1c]:443: connect: network is unreachable |
| 210  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:100d:4fdb:d8a1]:443: connect: network is unreachable |
| 212  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 215  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 216  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 219  | bestcf.030101.xyz                       | 2606:4700:19:e3ed:311e:da65:ac7a:3d52   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:19:e3ed:311e:da65:ac7a:3d52]:443: connect: network is unreachable   |
| 220  | bestcf.030101.xyz                       | 2606:4700:0:72:e1e6:5546:888c:67d1      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:72:e1e6:5546:888c:67d1]:443: connect: network is unreachable      |
| 226  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 227  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 229  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 232  | cf.zhetengsha.eu.org                    | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 233  | cf.zhetengsha.eu.org                    | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 237  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 238  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 239  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 242  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 243  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 246  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 247  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 251  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 252  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 253  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 259  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 260  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 261  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 265  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 266  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 267  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 274  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 275  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 276  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 280  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 281  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 282  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 286  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 287  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 288  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 296  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 297  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 301  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 302  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 303  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 307  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 308  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 309  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 312  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 313  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 315  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 318  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 319  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 322  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 323  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 325  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 330  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 331  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 332  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 337  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 338  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 339  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 341  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 345  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 346  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 347  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 352  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 353  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 354  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 357  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 358  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 362  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 363  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 364  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 365  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 368  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 369  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 380  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 381  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 382  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 386  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 387  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 389  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 393  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 394  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 395  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 399  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 400  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 401  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 404  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 423  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 424  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 425  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 430  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 431  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 432  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 439  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 440  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 442  | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable   |
| 451  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 452  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 456  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 457  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 458  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 488  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 489  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 490  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 491  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 492  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 493  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 494  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 495  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 496  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 497  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 538  | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 540  | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable   |
| 541  | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable   |
| 542  | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable    |
| 543  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable  |
| 544  | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable   |
| 545  | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 546  | 2803:f800:51:6a7b:7c95:5585:9678:1549   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable   |
| 547  | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable    |
| 586  | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable   |
| 607  | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable   |
| 609  | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable   |
| 610  | 2a06:98c1:3105:0:db:557f:8a53:2469      | 2a06:98c1:3105:0:db:557f:8a53:2469      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable      |
| 611  | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable      |
| 612  | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable       |
| 613  | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 614  | 2a06:98c1:3100:0:a3:1339:d974:e2c       | 2a06:98c1:3100:0:a3:1339:d974:e2c       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable       |
| 615  | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable   |
| 616  | 2a06:98c1:3106::c5:5d39:736d            | 2a06:98c1:3106::c5:5d39:736d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable            |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 411  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 486  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 633  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 200 次 (98.5%)
- **连接超时**: 3 次 (1.5%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 203 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 200 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 430 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 114  | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 383  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 417  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 571  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 573  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 598  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 517  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 316  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 461  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 376  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 379  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 415  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 579  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 103  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 283  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 372  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 384  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 623  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 271  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 373  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 370  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 436  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 577  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 61   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 593  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 298  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 446  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 600  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 367  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 574  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 285  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 469  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 356  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 485  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 526  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 199  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 513  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 559  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 473  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 6    | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 549  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 624  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 207  | freeyx.cloudflare88.eu.org            | 141.101.120.39  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 516  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 133  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 273  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 329  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 477  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 177  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 507  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 583  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 24   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 95   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 152  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 214  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 89   | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 254  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 270  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 570  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 581  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 115  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 183  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 223  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 284  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 459  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 474  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 498  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 530  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 589  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 594  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 604  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 5    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 195  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 206  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 269  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 575  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 30   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 181  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 201  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 340  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 511  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 293  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 390  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 466  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 476  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 536  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 567  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 73   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 87   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 132  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 441  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 509  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 584  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 17   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 175  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 202  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 228  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 351  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 472  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 519  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 555  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 47   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 244  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 311  | cf.090227.xyz                         | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 317  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 349  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 409  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 449  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 470  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 40   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 146  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 169  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 471  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 512  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 551  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 45   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 92   | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 131  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 161  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 258  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 268  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 304  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 333  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 462  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 464  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 27   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 70   | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 129  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 170  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 189  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 256  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 299  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 350  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 500  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 564  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 578  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 603  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 433  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 479  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 528  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 529  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 565  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 28   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 46   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 63   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 64   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 85   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 107  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 127  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 255  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 263  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 289  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 403  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 502  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 535  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 97   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 108  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 160  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 221  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 225  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 355  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 418  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 426  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 531  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 563  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 628  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 86   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 336  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 419  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 427  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 437  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 450  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 522  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 562  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 54   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 72   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 241  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 300  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 335  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 515  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 597  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 620  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 23   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 391  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 407  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 463  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 482  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 121  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 291  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 326  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 334  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 385  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 487  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 499  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 13   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 110  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 142  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 176  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 378  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 460  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 568  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 88   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 90   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 595  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 48   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 130  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 135  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 558  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 588  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 14   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 80   | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 235  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 467  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 468  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 518  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 32   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 123  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 154  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 421  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 422  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 525  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 548  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 77   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 234  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 290  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 371  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 478  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 608  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 94   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 140  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 200  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 217  | bestcf.030101.xyz                     | 104.17.60.95    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 222  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 240  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 306  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 396  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 402  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 410  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 454  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 576  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 1    | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 447  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 480  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 521  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 596  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 93   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 148  | cf.877771.xyz                         | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 314  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 374  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 524  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 532  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 560  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 580  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 599  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 76   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 91   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 156  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 159  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 388  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 465  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 484  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 569  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 601  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 2    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 9    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 43   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 187  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 295  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 348  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 360  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 435  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 503  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 591  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 68   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 109  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 122  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 230  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 3    | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 151  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 164  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 321  | ip.gs                                 | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 481  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 257  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 320  | ip.gs                                 | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 413  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 561  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 38   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 71   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 101  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 262  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 392  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 397  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 622  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 60   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 245  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 572  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 84   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 155  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 305  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 429  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 443  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 504  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 585  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 342  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 408  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 557  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 602  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 618  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 248  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 250  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 505  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 514  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 629  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 74   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 278  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 406  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 510  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 523  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 527  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 75   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 141  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 162  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 592  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 39   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 55   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 188  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 218  | bestcf.030101.xyz                     | 104.17.55.198   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 324  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 455  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 556  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 566  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 606  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 117  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 279  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 501  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 533  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 627  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 520  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 625  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 118  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 294  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 428  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 224  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 359  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 537  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 605  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 231  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 264  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 416  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 587  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 590  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 134  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 292  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 343  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 534  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 621  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 31   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 182  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 194  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 213  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 236  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 310  | cf.090227.xyz                         | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 617  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 10   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 277  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 483  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 4    | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 552  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 153  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 438  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 58   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 67   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 147  | cf.877771.xyz                         | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 272  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 136  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 554  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 550  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 398  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 508  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 366  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 444  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 475  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 553  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 412  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 582  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 619  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 44   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 193  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 328  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 420  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 327  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 448  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 414  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 163  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 453  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 375  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 445  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 506  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 626  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 344  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 96   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 539  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 168  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 102  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 18   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 434  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 128  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 630  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 29   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 405  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 191      | cloudflare |
| 249  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 631  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 226      | cloudflare |
| 361  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 59   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 377  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 398      | cloudflare |
| 174  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 529      | cloudflare |
| 632  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 533      | cloudflare |
| 211  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 683      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 205 条记录
- **正常 (100-200ms)**: 217 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 200 次

### 按协议统计

- **none**: 203 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
