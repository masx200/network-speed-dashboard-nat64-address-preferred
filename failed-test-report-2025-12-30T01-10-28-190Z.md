# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:10:28
- **数据来源**: connectivity_results-20251230-011027.json
- **总测试数**: 507
- **失败测试数**: 183
- **成功测试数**: 324
- **失败率**: 36.09%
- **平均延迟**: 121.52ms
- **最小延迟**: 50ms
- **最大延迟**: 748ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:10:28
- **IP地址**: 48.211.212.212
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 36.6694, -78.3877
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 179 次 (97.8%)
- **连接超时: I/O超时**: 4 次 (2.2%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (179 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 5    | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 6    | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 9    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 10   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 13   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 14   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 18   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 19   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 20   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 23   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 24   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 30   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 31   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 32   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 33   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 34   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 38   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 39   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 40   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 43   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 44   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 47   | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 48   | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 66   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 67   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 69   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 77   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 78   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 79   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 82   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 83   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 86   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 87   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 116  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 122  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 123  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 124  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 134  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 135  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 136  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 140  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 141  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 142  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 146  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 147  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 148  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 152  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 153  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 157  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 158  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 159  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 165  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 166  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 173  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 174  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 175  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 182  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 183  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 184  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 188  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 189  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 190  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 194  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 195  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 196  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 201  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 202  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 203  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 208  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 209  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 210  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 214  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:0:51:d1e4:65f0:5ed1      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:51:d1e4:65f0:5ed1]:443: connect: network is unreachable      |
| 215  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cdf7:f50d:f763 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cdf7:f50d:f763]:443: connect: network is unreachable |
| 216  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 219  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:ab56:90d6:b7ab    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:7809:f54e:ab56:90d6:b7ab]:443: connect: network is unreachable    |
| 220  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:bd32:7275:fc25    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:7809:f54e:bd32:7275:fc25]:443: connect: network is unreachable    |
| 226  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 227  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 230  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 231  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 233  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 237  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 238  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 239  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 242  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 243  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 246  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 247  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 251  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 252  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 253  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 256  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 257  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 261  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 262  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 263  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 269  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 270  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 271  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 275  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 276  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 277  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 280  | cf.zhetengsha.eu.org                    | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 281  | cf.zhetengsha.eu.org                    | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 288  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 289  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 290  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 299  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 300  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 301  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 304  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 305  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 309  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 310  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 311  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 315  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 316  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 317  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 320  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 321  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 324  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 325  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 327  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 329  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 333  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 334  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 335  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 341  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 342  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 343  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 345  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 350  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 351  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 352  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 356  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 357  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 358  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 359  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 363  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 368  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 369  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 374  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 375  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 380  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 385  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 386  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 387  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 389  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 391  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 396  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 397  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 398  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 401  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 402  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 405  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 406  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 409  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 410  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 415  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 416  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 417  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 421  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 422  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 423  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 426  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 427  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 461  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 462  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 463  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 465  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 476  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 477  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 478  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 479  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 480  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 481  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 482  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 483  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 362  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 500  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 505  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 507  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 179 次 (97.8%)
- **连接超时**: 4 次 (2.2%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 183 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 179 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                  | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | -------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 71   | 103.160.204.59             | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 127  | www.okcupid.com            | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 420  | abdullah.ns.cloudflare.com | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 488  | eur.877774.xyz             | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 278  | cf.zhetengsha.eu.org       | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 399  | www.wto.org                | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 489  | 104.16.65.1                | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 439  | 173.245.49.194             | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 411  | 172.67.181.209             | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 470  | 104.17.69.244              | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 331  | ashton.ns.cloudflare.com   | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 279  | cf.zhetengsha.eu.org       | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 370  | 104.17.142.12              | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 438  | 104.18.255.167             | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 471  | 104.31.16.158              | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 403  | stock.hostmonit.com        | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 390  | ifconfig.co                | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 451  | 104.18.81.19               | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 255  | cf.090227.xyz              | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 365  | gamer.com.tw               | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 292  | 162.159.36.104             | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 55   | 104.18.37.13               | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 28   | cloudflare.182682.xyz      | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 197  | 104.16.223.179             | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 445  | 104.26.5.134               | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 57   | ct.877774.xyz              | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 371  | 172.67.79.211              | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 117  | www.4chan.org              | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 395  | japan.com                  | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 63   | ct.877774.xyz              | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 340  | dnschecker.org             | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 498  | cfip.xxxxxxxx.tk           | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 100  | cmcc.877774.xyz            | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 139  | iplocation.io              | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 302  | palera.in                  | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 355  | uriah.ns.cloudflare.com    | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 298  | ip.sb                      | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 376  | 198.62.62.4                | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 487  | eur.877774.xyz             | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 49   | 104.18.39.196              | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 440  | www.csgo.com               | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 81   | icook.hk                   | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 393  | japan.com                  | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 26   | cloudflare.182682.xyz      | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 113  | cmcc.877774.xyz            | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 433  | 162.159.140.85             | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 474  | 104.17.142.212             | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 493  | cfip.xxxxxxxx.tk           | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 497  | cfip.xxxxxxxx.tk           | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 499  | cfip.xxxxxxxx.tk           | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 132  | sullivan.ns.cloudflare.com | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 228  | zread.ai                   | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 37   | wilson.ns.cloudflare.com   | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 60   | ct.877774.xyz              | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 108  | cmcc.877774.xyz            | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 145  | craig.ns.cloudflare.com    | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 434  | 172.64.52.127              | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 101  | cmcc.877774.xyz            | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 130  | 172.67.120.0               | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 295  | 104.17.68.85               | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 372  | cf.877774.xyz              | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 25   | cloudflare.182682.xyz      | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 56   | ct.877774.xyz              | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 249  | bowen.ns.cloudflare.com    | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 268  | time.is                    | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 436  | 172.64.48.226              | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 485  | 104.18.189.153             | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 503  | cfip.xxxxxxxx.tk           | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 42   | ipinfo.in                  | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 72   | shopify.com                | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 96   | cmcc.877774.xyz            | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 109  | cmcc.877774.xyz            | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 467  | 104.19.154.200             | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 88   | cmcc.877774.xyz            | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 180  | cris.ns.cloudflare.com     | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 204  | 104.17.79.11               | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 443  | 162.159.136.89             | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 496  | cfip.xxxxxxxx.tk           | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 4    | www.pcmag.com              | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 52   | ipv4.ip.sb                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | www.visa.cn                | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 192  | toy-people.com             | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 449  | 162.159.128.253            | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 469  | 104.19.212.207             | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 7    | comicabc.com               | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 8    | comicabc.com               | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 54   | ipv4.ip.sb                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 91   | cmcc.877774.xyz            | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 137  | iplocation.io              | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 286  | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 373  | cf.877774.xyz              | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 45   | www.udemy.com              | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 62   | ct.877774.xyz              | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 90   | cmcc.877774.xyz            | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 98   | cmcc.877774.xyz            | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 236  | braden.ns.cloudflare.com   | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 450  | 104.26.3.162               | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 46   | www.udemy.com              | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 313  | singapore.com              | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 383  | damien.ns.cloudflare.com   | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 492  | cfip.xxxxxxxx.tk           | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 58   | ct.877774.xyz              | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 110  | cmcc.877774.xyz            | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 388  | 104.18.78.214              | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 392  | 104.19.223.58              | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 441  | www.csgo.com               | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 464  | 104.17.162.3               | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 490  | www.7749tv.com             | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 191  | toy-people.com             | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 407  | icook.tw                   | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 495  | cfip.xxxxxxxx.tk           | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 154  | pranab.ns.cloudflare.com   | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 283  | www.ipchicken.com          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 129  | www.okcupid.com            | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 171  | www.whatismyip.com         | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 404  | stock.hostmonit.com        | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 413  | otto.ns.cloudflare.com     | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 446  | 172.64.229.7               | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 455  | 104.18.166.129             | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 486  | eur.877774.xyz             | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 73   | 104.18.254.88              | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 95   | cmcc.877774.xyz            | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 102  | cmcc.877774.xyz            | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 126  | www.okcupid.com            | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 128  | www.okcupid.com            | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 121  | www.hugedomains.com        | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 133  | sullivan.ns.cloudflare.com | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 408  | icook.tw                   | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 456  | 198.41.208.224             | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 466  | 104.18.151.172             | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 61   | ct.877774.xyz              | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 92   | cmcc.877774.xyz            | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 273  | rustam.ns.cloudflare.com   | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 297  | ip.sb                      | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 425  | www.digitalocean.com       | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 506  | cfip.xxxxxxxx.tk           | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 17   | trevor.ns.cloudflare.com   | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 50   | 104.16.45.84               | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 89   | cmcc.877774.xyz            | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 112  | cmcc.877774.xyz            | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 115  | cu.877774.xyz              | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 119  | www.hugedomains.com        | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 323  | whatismyipaddress.com      | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 143  | craig.ns.cloudflare.com    | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 458  | lewis.ns.cloudflare.com    | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 504  | cfip.xxxxxxxx.tk           | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 36   | wilson.ns.cloudflare.com   | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 51   | 172.67.75.172              | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 307  | silkbook.com               | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 308  | silkbook.com               | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 384  | damien.ns.cloudflare.com   | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 460  | lewis.ns.cloudflare.com    | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 468  | 104.19.220.22              | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 3    | www.pcmag.com              | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 107  | cmcc.877774.xyz            | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 224  | xn--b6gac.eu.org           | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 332  | ashton.ns.cloudflare.com   | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 457  | 104.19.148.121             | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 12   | www.ipget.net              | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 200  | decker.ns.cloudflare.com   | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 225  | xn--b6gac.eu.org           | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 338  | dnschecker.org             | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 125  | www.okcupid.com            | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 360  | www.visa.com.hk            | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 378  | 172.64.35.24               | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 381  | 104.26.13.31               | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 412  | otto.ns.cloudflare.com     | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 59   | ct.877774.xyz              | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 84   | www.gov.ua                 | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 94   | cmcc.877774.xyz            | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 104  | cmcc.877774.xyz            | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 114  | cu.877774.xyz              | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 138  | iplocation.io              | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 502  | cfip.xxxxxxxx.tk           | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 53   | ipv4.ip.sb                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 240  | fbi.gov                    | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 432  | 198.41.208.15              | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 22   | cf.0sm.com                 | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 156  | pranab.ns.cloudflare.com   | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 211  | freeyx.cloudflare88.eu.org | 141.101.120.156 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 232  | 172.64.151.55              | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 64   | steamdb.info               | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 118  | www.4chan.org              | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 162  | www.visa.com.sg            | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 193  | toy-people.com             | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 400  | www.wto.org                | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 428  | 104.19.175.123             | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 21   | cf.0sm.com                 | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 172  | www.whatismyip.com         | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 177  | na.877774.xyz              | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 293  | www.glassdoor.com          | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 452  | 104.26.8.117               | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 99   | cmcc.877774.xyz            | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 223  | asia.877774.xyz            | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 254  | cf.090227.xyz              | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 435  | 162.159.61.183             | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 178  | na.877774.xyz              | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 418  | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 41   | ipinfo.in                  | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 105  | cmcc.877774.xyz            | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 111 条记录
- **正常 (100-200ms)**: 89 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 179 次

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
