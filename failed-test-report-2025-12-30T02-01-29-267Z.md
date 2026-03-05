# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:01:29
- **数据来源**: connectivity_results-20251230-020128.json
- **总测试数**: 506
- **失败测试数**: 181
- **成功测试数**: 325
- **失败率**: 35.77%
- **平均延迟**: 123.16ms
- **最小延迟**: 64ms
- **最大延迟**: 1500ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:01:29
- **IP地址**: 52.173.163.131
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6015, -93.6127
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
| 1    | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 2    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 8    | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 9    | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 10   | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 22   | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 23   | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 24   | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 33   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 34   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 38   | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 39   | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 43   | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 44   | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 45   | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 49   | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 50   | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 51   | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 58   | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 59   | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 60   | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 64   | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 65   | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 66   | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 72   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 73   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 74   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 75   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 76   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 81   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 82   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 85   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 86   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 90   | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 91   | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 92   | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 94   | freeyx.cloudflare88.eu.org              | 2606:4700:3010:e070:5d82:552a:2069:936a | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:e070:5d82:552a:2069:936a]:443: connect: network is unreachable |
| 100  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 101  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 102  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 106  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 107  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 108  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 111  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 114  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 115  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 118  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 119  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 122  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:ab56:90d6:b7ab    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:7809:f54e:ab56:90d6:b7ab]:443: connect: network is unreachable    |
| 123  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:bd32:7275:fc25    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:7809:f54e:bd32:7275:fc25]:443: connect: network is unreachable    |
| 126  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 127  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 129  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 133  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 134  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 135  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 164  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 165  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 168  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 169  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 173  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 174  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 175  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 178  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 179  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 188  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 189  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 190  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 194  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 195  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 196  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 203  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 204  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 205  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 211  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 212  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 213  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 220  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 221  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 222  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 225  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 226  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 230  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 231  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 232  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 235  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 236  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 239  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 240  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 244  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 245  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 246  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 248  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 249  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 256  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 257  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 258  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 262  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 263  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 264  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 265  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 270  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 271  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 272  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 275  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 279  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 280  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 281  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 284  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 285  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 286  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 289  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 290  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 297  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 298  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 301  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 302  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 319  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 320  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 321  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 324  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 325  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 329  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 330  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 331  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 336  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 337  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 338  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 343  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 344  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 347  | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 348  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 351  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 352  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 354  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 362  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 363  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 366  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 371  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 372  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 373  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 375  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 378  | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 379  | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 384  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 385  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 387  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 391  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 392  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 393  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 398  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 399  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 400  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 406  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 407  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 418  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 419  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 431  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 432  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 433  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 462  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 467  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 468  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 469  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 470  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 471  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 472  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 473  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 474  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 488  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 489  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 490  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 494  | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 495  | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 496  | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 500  | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 501  | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 502  | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 314  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 505  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 506  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 178 次 (98.3%)
- **连接超时**: 3 次 (1.7%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 178 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
sullivan.ns.cloudflare.com (3次), craig.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 325 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 78   | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 128  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 498  | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 316  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 267  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 481  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 295  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 130  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 199  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 287  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 408  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 452  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 95   | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 409  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 332  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 374  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 167  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 427  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 202  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 69   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 209  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 185  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 223  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 247  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 250  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 83   | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 251  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 278  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 430  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 104  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 224  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 310  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 454  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 487  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 183  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 233  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 436  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 451  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 20   | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 117  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 476  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 455  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 482  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 497  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 46   | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 277  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 411  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 96   | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 313  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 401  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 426  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 447  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 12   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 28   | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 305  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 364  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 365  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 381  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 423  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 14   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 37   | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 237  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 386  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 448  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 31   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 367  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 380  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 405  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 449  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 26   | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 61   | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 79   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 200  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 266  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 268  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 274  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 424  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 18   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 27   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 68   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 162  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 253  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 311  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 328  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 493  | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 124  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 361  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 453  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 11   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 339  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 358  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 410  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 29   | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 40   | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 52   | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 181  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 182  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 243  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 464  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 25   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 166  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 177  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 293  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 341  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 54   | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 137  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 463  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 479  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 63   | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 67   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 112  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 206  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 282  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 283  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 296  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 340  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 403  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 439  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 450  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 62   | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 197  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 273  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 346  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 356  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 376  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 417  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 437  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 484  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 87   | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 120  | bestcf.030101.xyz                     | 104.19.156.139  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 163  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 335  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 383  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 397  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 446  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 457  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 486  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 47   | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 55   | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 77   | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 229  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 395  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 443  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 53   | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 139  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 170  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 322  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 370  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 414  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 131  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 215  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 306  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 416  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 228  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 425  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 485  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 105  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 210  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 219  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 440  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 456  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 459  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 460  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 36   | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 71   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 136  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 172  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 184  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 260  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 394  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 466  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 478  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 499  | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 180  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 309  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 415  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 441  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 110  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 360  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 377  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 413  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 15   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 80   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 84   | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 176  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 191  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 207  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 353  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 56   | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 113  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 208  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 216  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 299  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 357  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 429  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 242  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 269  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 428  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 483  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 70   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 368  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 420  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 477  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 30   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 255  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 369  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 444  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 186  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 254  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 359  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 458  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 492  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 57   | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 125  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 214  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 227  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 307  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 421  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 461  | www.7749tv.com                        | 104.16.115.36   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 252  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 259  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 294  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 326  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 480  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 5    | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 97   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 300  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 315  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 442  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 17   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 291  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 438  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 3    | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 19   | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 88   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 308  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 312  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 323  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 350  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 342  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 98   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 121  | bestcf.030101.xyz                     | 104.19.157.251  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 193  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 241  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 304  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 412  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 32   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 42   | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 93   | freeyx.cloudflare88.eu.org            | 141.101.120.159 | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 292  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 355  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 389  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 404  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 422  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 89   | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 132  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 41   | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 201  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 334  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 171  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 382  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 4    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 13   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 48   | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 116  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 198  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 218  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 317  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 434  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 16   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 238  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 396  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 318  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 349  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 21   | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 475  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 402  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 491  | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 435  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 445  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 7    | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 261  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 234  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 99   | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 187  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 35   | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 333  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 388  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 345  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 192  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 103  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 109  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 176      | cloudflare |
| 390  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 276  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 197      | cloudflare |
| 288  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 201      | cloudflare |
| 6    | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 254      | cloudflare |
| 327  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 332      | cloudflare |
| 303  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 339      | cloudflare |
| 217  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 492      | cloudflare |
| 465  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 502      | cloudflare |
| 503  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 698      | cloudflare |
| 504  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 1500     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 81 条记录
- **正常 (100-200ms)**: 236 条记录
- **慢 (200-500ms)**: 5 条记录
- **很慢 (>500ms)**: 3 条记录

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
