# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 12:55:02
- **数据来源**: connectivity_results-20251214-125502.json
- **总测试数**: 454
- **失败测试数**: 168
- **成功测试数**: 286
- **失败率**: 37.00%
- **平均延迟**: 103.66ms
- **最小延迟**: 57ms
- **最大延迟**: 520ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 12:55:02
- **IP地址**: 172.212.169.145
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: cloud
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.6015, -93.6127
- **时区**: America/Chicago
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 164 次 (97.6%)
- **连接超时: I/O超时**: 4 次 (2.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名                             | 目标IP                               | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                             |
| ---- | ------------------------------------- | ------------------------------------ | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------ |
| 3    | steamdb.info                          | 2606:4700:10::ac42:affa              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable              |
| 4    | steamdb.info                          | 2606:4700:10::6814:22d4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable              |
| 7    | ipinfo.in                             | 2606:4700:3037::ac43:c6cb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable            |
| 8    | ipinfo.in                             | 2606:4700:3031::6815:1581            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable            |
| 12   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable              |
| 13   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable              |
| 14   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable              |
| 17   | www.pcmag.com                         | 2606:4700::6810:1576                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                 |
| 18   | www.pcmag.com                         | 2606:4700::6810:1476                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                 |
| 21   | www.gov.ua                            | 2606:4700:3031::6815:1748            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable            |
| 22   | www.gov.ua                            | 2606:4700:3033::ac43:d17f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable            |
| 28   | iplocation.io                         | 2606:4700:20::681a:ade               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable               |
| 29   | iplocation.io                         | 2606:4700:20::681a:bde               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable               |
| 30   | iplocation.io                         | 2606:4700:20::ac43:4664              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable              |
| 43   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable            |
| 44   | cf.0sm.com                            | 2606:4700:3032::6815:785             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable             |
| 47   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable             |
| 50   | comicabc.com                          | 2606:4700:3036::6815:400a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable            |
| 51   | comicabc.com                          | 2606:4700:3030::ac43:ae15            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable            |
| 55   | www.ipget.net                         | 2606:4700:3036::6815:fd4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable             |
| 56   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable            |
| 74   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable            |
| 79   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable              |
| 80   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable              |
| 81   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable              |
| 85   | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable              |
| 86   | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable              |
| 87   | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable              |
| 91   | www.hugedomains.com                   | 2606:4700:20::ac43:46bf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable              |
| 92   | www.hugedomains.com                   | 2606:4700:20::681a:625               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable               |
| 93   | www.hugedomains.com                   | 2606:4700:20::681a:725               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable               |
| 103  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable            |
| 104  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable            |
| 109  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable              |
| 110  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable              |
| 111  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable              |
| 116  | icook.hk                              | 2606:4700:3031::6815:5ad2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable            |
| 117  | icook.hk                              | 2606:4700:3037::ac43:a168            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable            |
| 120  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable            |
| 121  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable            |
| 125  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable              |
| 126  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable              |
| 127  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable              |
| 134  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable              |
| 135  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable              |
| 136  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable              |
| 140  | toy-people.com                        | 2606:4700:20::681a:324               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable               |
| 141  | toy-people.com                        | 2606:4700:20::ac43:4812              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable              |
| 142  | toy-people.com                        | 2606:4700:20::681a:224               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable               |
| 146  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable              |
| 147  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable              |
| 148  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable              |
| 152  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable              |
| 153  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable              |
| 154  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable              |
| 161  | zread.ai                              | 2606:4700:3032::ac43:ca4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable            |
| 162  | zread.ai                              | 2606:4700:3033::6815:4cf0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable            |
| 166  | www.whatismyip.com                    | 2606:4700:20::ac43:4581              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable              |
| 167  | www.whatismyip.com                    | 2606:4700:20::681a:c17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable               |
| 168  | www.whatismyip.com                    | 2606:4700:20::681a:d17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable               |
| 172  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable              |
| 173  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable              |
| 174  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable              |
| 175  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable            |
| 178  | bestcf.030101.xyz                     | 2606:4700:0:f0:8071:a554:1555:a780   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:f0:8071:a554:1555:a780]:443: connect: network is unreachable   |
| 179  | bestcf.030101.xyz                     | 2606:4700:b:4d5f:ec98:3ef5:721:4971  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:b:4d5f:ec98:3ef5:721:4971]:443: connect: network is unreachable  |
| 185  | fbi.gov                               | 2606:4700::6810:95f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                 |
| 186  | fbi.gov                               | 2606:4700::6810:94f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                 |
| 189  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable            |
| 190  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable            |
| 192  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable            |
| 196  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable              |
| 197  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable              |
| 198  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable              |
| 202  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable              |
| 203  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable              |
| 204  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable              |
| 207  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable            |
| 208  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable            |
| 213  | cf.090227.xyz                         | 2a06:98c1:3101::ac40:919e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable            |
| 214  | cf.090227.xyz                         | 2a06:98c1:3108::6812:2a62            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable            |
| 218  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable              |
| 219  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable              |
| 220  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable              |
| 224  | time.is                               | 2606:4700:20::681a:c36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable               |
| 225  | time.is                               | 2606:4700:20::681a:d36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable               |
| 226  | time.is                               | 2606:4700:20::ac43:449d              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable              |
| 230  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable              |
| 231  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable              |
| 232  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable              |
| 239  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable              |
| 240  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable              |
| 241  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable              |
| 245  | ip.sb                                 | 2606:4700:20::681a:c1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable               |
| 246  | ip.sb                                 | 2606:4700:20::681a:d1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable               |
| 247  | ip.sb                                 | 2606:4700:20::ac43:4bac              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable              |
| 250  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable            |
| 251  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable            |
| 259  | palera.in                             | 2606:4700:3035::6815:3a48            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable            |
| 260  | palera.in                             | 2606:4700:3032::ac43:9d7a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable            |
| 264  | singapore.com                         | 2606:4700:20::ac43:4bc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable              |
| 265  | singapore.com                         | 2606:4700:20::681a:d8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable               |
| 266  | singapore.com                         | 2606:4700:20::681a:c8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable               |
| 269  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:f5e8:7af2:12d8:5d82 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable |
| 273  | silkbook.com                          | 2606:4700:20::681a:9a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable               |
| 274  | silkbook.com                          | 2606:4700:20::ac43:4bd0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable              |
| 275  | silkbook.com                          | 2606:4700:20::681a:8a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable               |
| 278  | ip.gs                                 | 2606:4700:3036::6815:eb0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable             |
| 279  | ip.gs                                 | 2606:4700:3035::ac43:a01c            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable            |
| 282  | whatismyipaddress.com                 | 2606:4700::6813:de4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                 |
| 283  | whatismyipaddress.com                 | 2606:4700::6813:df4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                 |
| 285  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable            |
| 289  | dnschecker.org                        | 2606:4700:20::681a:659               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable               |
| 290  | dnschecker.org                        | 2606:4700:20::681a:759               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable               |
| 291  | dnschecker.org                        | 2606:4700:20::ac43:49d8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable              |
| 292  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable            |
| 297  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable              |
| 298  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable              |
| 299  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable              |
| 303  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable            |
| 307  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable              |
| 308  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable              |
| 309  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable              |
| 315  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable              |
| 316  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable              |
| 317  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable              |
| 318  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable            |
| 321  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 322  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 325  | tasteatlas.com                        | 2606:4700::6811:2569                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                 |
| 326  | tasteatlas.com                        | 2606:4700::6811:2469                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                 |
| 327  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable            |
| 330  | www.udemy.com                         | 2606:4700::6810:8fed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                 |
| 331  | www.udemy.com                         | 2606:4700::6810:8eed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                 |
| 342  | icook.tw                              | 2606:4700:10::ac42:9e73              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable              |
| 343  | icook.tw                              | 2606:4700:10::6814:1c4a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable              |
| 344  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable               |
| 348  | www.digitalocean.com                  | 2606:4700::6813:ad44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                 |
| 349  | www.digitalocean.com                  | 2606:4700::6813:ae44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                 |
| 351  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable             |
| 355  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable              |
| 356  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable              |
| 357  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable              |
| 361  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable              |
| 362  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable              |
| 363  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable              |
| 368  | ifconfig.co                           | 2606:4700:3037::6815:365b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable            |
| 369  | ifconfig.co                           | 2606:4700:3030::ac43:a86a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable            |
| 373  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable              |
| 374  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable              |
| 375  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable              |
| 383  | japan.com                             | 2606:4700:20::681a:43c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable               |
| 384  | japan.com                             | 2606:4700:20::681a:53c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable               |
| 385  | japan.com                             | 2606:4700:20::ac43:465c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable              |
| 390  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable              |
| 391  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable              |
| 392  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable              |
| 395  | www.wto.org                           | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 396  | www.wto.org                           | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 401  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable            |
| 402  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable             |
| 409  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable              |
| 410  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable              |
| 411  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable              |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 376  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 445  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 451  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 454  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (97.6%)
- **连接超时**: 4 次 (2.4%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 168 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
iplocation.io (3次), huxley.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 270  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 366  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 272  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 133  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 128  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 421  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 201  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 405  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 416  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 449  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 170  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 222  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 364  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 94   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 301  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 258  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 52   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 427  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 286  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 346  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 58   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 68   | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 253  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 284  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 413  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 359  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 48   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 157  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 199  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 267  | freeyx.cloudflare88.eu.org            | 141.101.120.207 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 276  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 195  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 261  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 419  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 426  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 95   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 182  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 249  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 332  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 418  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 446  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 145  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 159  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 236  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 256  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 49   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 67   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 164  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 205  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 212  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 280  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 300  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 340  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 372  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 386  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 447  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 75   | www.7749tv.com                        | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 118  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 119  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 243  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 302  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 328  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 335  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 336  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 341  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 377  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 425  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 440  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 444  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 452  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 99   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 155  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 216  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 235  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 293  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 337  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 338  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 378  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 393  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 450  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 27   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 45   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 73   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 137  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 163  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 370  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 432  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 435  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 437  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 26   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 59   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 77   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 234  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 277  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 164 次

### 按协议统计

- **none**: 168 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
