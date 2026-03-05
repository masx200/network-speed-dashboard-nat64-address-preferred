# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 13:30:41
- **数据来源**: connectivity_results-20251214-133040.json
- **总测试数**: 444
- **失败测试数**: 167
- **成功测试数**: 277
- **失败率**: 37.61%
- **平均延迟**: 107.60ms
- **最小延迟**: 45ms
- **最大延迟**: 818ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 13:30:41
- **IP地址**: 20.42.42.212
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名                             | 目标IP                               | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                             |
| ---- | ------------------------------------- | ------------------------------------ | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------ |
| 6    | www.gov.ua                            | 2606:4700:3031::6815:1748            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable            |
| 7    | www.gov.ua                            | 2606:4700:3033::ac43:d17f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable            |
| 10   | www.ipget.net                         | 2606:4700:3036::6815:fd4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable             |
| 11   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable            |
| 18   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable              |
| 19   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable              |
| 20   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable              |
| 24   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable              |
| 25   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable              |
| 26   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable              |
| 30   | www.pcmag.com                         | 2606:4700::6810:1476                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                 |
| 31   | www.pcmag.com                         | 2606:4700::6810:1576                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                 |
| 42   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable            |
| 43   | ipinfo.in                             | 2606:4700:3031::6815:1581            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable            |
| 46   | comicabc.com                          | 2606:4700:3030::ac43:ae15            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable            |
| 47   | comicabc.com                          | 2606:4700:3036::6815:400a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable            |
| 51   | steamdb.info                          | 2606:4700:10::ac42:affa              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable              |
| 52   | steamdb.info                          | 2606:4700:10::6814:22d4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable              |
| 53   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable             |
| 57   | iplocation.io                         | 2606:4700:20::681a:ade               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable               |
| 58   | iplocation.io                         | 2606:4700:20::681a:bde               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable               |
| 59   | iplocation.io                         | 2606:4700:20::ac43:4664              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable              |
| 90   | cf.0sm.com                            | 2606:4700:3032::6815:785             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable             |
| 91   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable            |
| 96   | singapore.com                         | 2606:4700:20::681a:d8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable               |
| 97   | singapore.com                         | 2606:4700:20::681a:c8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable               |
| 98   | singapore.com                         | 2606:4700:20::ac43:4bc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable              |
| 101  | icook.hk                              | 2606:4700:3031::6815:5ad2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable            |
| 102  | icook.hk                              | 2606:4700:3037::ac43:a168            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable            |
| 105  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable            |
| 106  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable              |
| 107  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable              |
| 108  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable              |
| 112  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable              |
| 113  | www.hugedomains.com                   | 2606:4700:20::681a:625               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable               |
| 114  | www.hugedomains.com                   | 2606:4700:20::681a:725               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable               |
| 128  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable              |
| 129  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable              |
| 130  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable              |
| 133  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable            |
| 134  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable            |
| 146  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable              |
| 147  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable              |
| 148  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable              |
| 153  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable            |
| 154  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable            |
| 158  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable              |
| 159  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable              |
| 160  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable              |
| 164  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable              |
| 165  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable              |
| 166  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable              |
| 173  | toy-people.com                        | 2606:4700:20::ac43:4812              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable              |
| 174  | toy-people.com                        | 2606:4700:20::681a:324               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable               |
| 175  | toy-people.com                        | 2606:4700:20::681a:224               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable               |
| 179  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable              |
| 180  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable              |
| 181  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable              |
| 186  | www.whatismyip.com                    | 2606:4700:20::681a:d17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable               |
| 187  | www.whatismyip.com                    | 2606:4700:20::681a:c17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable               |
| 188  | www.whatismyip.com                    | 2606:4700:20::ac43:4581              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable              |
| 192  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable            |
| 193  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable            |
| 198  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable              |
| 199  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable              |
| 200  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable              |
| 204  | zread.ai                              | 2606:4700:3032::ac43:ca4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable            |
| 205  | zread.ai                              | 2606:4700:3033::6815:4cf0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable            |
| 206  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable            |
| 210  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable              |
| 211  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable              |
| 212  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable              |
| 214  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:f5e8:7af2:12d8:5d82 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:f5e8:7af2:12d8:5d82]:443: connect: network is unreachable |
| 218  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable              |
| 219  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable              |
| 220  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable              |
| 223  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable            |
| 224  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable            |
| 226  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable            |
| 229  | fbi.gov                               | 2606:4700::6810:94f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                 |
| 230  | fbi.gov                               | 2606:4700::6810:95f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                 |
| 234  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable              |
| 235  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable              |
| 236  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable              |
| 239  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable            |
| 240  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable            |
| 243  | bestcf.030101.xyz                     | 2606:4700:0:f0:8071:a554:1555:a780   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:f0:8071:a554:1555:a780]:443: connect: network is unreachable   |
| 244  | bestcf.030101.xyz                     | 2606:4700:b:4d5f:ec98:3ef5:721:4971  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:b:4d5f:ec98:3ef5:721:4971]:443: connect: network is unreachable  |
| 251  | time.is                               | 2606:4700:20::681a:c36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable               |
| 252  | time.is                               | 2606:4700:20::681a:d36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable               |
| 253  | time.is                               | 2606:4700:20::ac43:449d              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable              |
| 257  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable              |
| 258  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable              |
| 259  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable              |
| 263  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable              |
| 264  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable              |
| 265  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable              |
| 272  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable              |
| 273  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable              |
| 274  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable              |
| 277  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable            |
| 280  | cf.090227.xyz                         | 2606:4700:4407::ac40:9052            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable            |
| 281  | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable            |
| 287  | palera.in                             | 2606:4700:3035::6815:3a48            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable            |
| 288  | palera.in                             | 2606:4700:3032::ac43:9d7a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable            |
| 295  | ip.sb                                 | 2606:4700:20::681a:d1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable               |
| 296  | ip.sb                                 | 2606:4700:20::681a:c1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable               |
| 297  | ip.sb                                 | 2606:4700:20::ac43:4bac              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable              |
| 303  | silkbook.com                          | 2606:4700:20::681a:8a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable               |
| 304  | silkbook.com                          | 2606:4700:20::681a:9a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable               |
| 305  | silkbook.com                          | 2606:4700:20::ac43:4bd0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable              |
| 308  | ip.gs                                 | 2606:4700:3036::6815:eb0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable             |
| 309  | ip.gs                                 | 2606:4700:3035::ac43:a01c            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable            |
| 313  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable              |
| 314  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable              |
| 315  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable              |
| 317  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable            |
| 319  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable            |
| 324  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable              |
| 325  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable              |
| 326  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable              |
| 331  | dnschecker.org                        | 2606:4700:20::681a:759               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable               |
| 332  | dnschecker.org                        | 2606:4700:20::ac43:49d8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable              |
| 333  | dnschecker.org                        | 2606:4700:20::681a:659               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable               |
| 335  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable            |
| 341  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable              |
| 342  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable              |
| 343  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable              |
| 353  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable            |
| 356  | whatismyipaddress.com                 | 2606:4700::6813:de4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                 |
| 357  | whatismyipaddress.com                 | 2606:4700::6813:df4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                 |
| 360  | tasteatlas.com                        | 2606:4700::6811:2569                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                 |
| 361  | tasteatlas.com                        | 2606:4700::6811:2469                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                 |
| 362  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable             |
| 365  | www.udemy.com                         | 2606:4700::6810:8eed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                 |
| 366  | www.udemy.com                         | 2606:4700::6810:8fed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                 |
| 370  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 371  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 379  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable               |
| 383  | www.digitalocean.com                  | 2606:4700::6813:ae44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                 |
| 384  | www.digitalocean.com                  | 2606:4700::6813:ad44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                 |
| 387  | icook.tw                              | 2606:4700:10::ac42:9e73              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable              |
| 388  | icook.tw                              | 2606:4700:10::6814:1c4a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable              |
| 395  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable              |
| 396  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable              |
| 397  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable              |
| 400  | ifconfig.co                           | 2606:4700:3030::ac43:a86a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable            |
| 401  | ifconfig.co                           | 2606:4700:3037::6815:365b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable            |
| 405  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable              |
| 406  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable              |
| 407  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable              |
| 411  | japan.com                             | 2606:4700:20::681a:43c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable               |
| 412  | japan.com                             | 2606:4700:20::681a:53c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable               |
| 413  | japan.com                             | 2606:4700:20::ac43:465c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable              |
| 417  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable              |
| 418  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable              |
| 419  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable              |
| 425  | www.wto.org                           | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 426  | www.wto.org                           | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 430  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable            |
| 431  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable             |
| 441  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable              |
| 442  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable              |
| 443  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable              |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 349  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 427  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 444  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 164 次 (98.2%)
- **连接超时**: 3 次 (1.8%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 167 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 164 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：wilson.ns.cloudflare.com (3次),
huxley.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 321  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 120  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 351  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 54   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 268  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 375  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 177  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 386  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 103  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 359  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 81   | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 8    | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 284  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 86   | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 318  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 346  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 3    | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 221  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 246  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 56   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 95   | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 424  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 73   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 215  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 354  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 416  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 433  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 40   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 44   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 48   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 310  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 415  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 16   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 29   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 61   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 66   | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 293  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 194  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 216  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 278  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 79   | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 83   | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 118  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 185  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 213  | freeyx.cloudflare88.eu.org            | 141.101.120.18  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 261  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 291  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 428  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 82   | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 222  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 78   | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 260  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 282  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 301  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 131  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 167  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 171  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 241  | bestcf.030101.xyz                     | 104.19.42.208   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 270  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 50   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 116  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 162  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 232  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 289  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 316  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 109  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 382  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 23   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 135  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 176  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 209  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 364  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 15   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 85   | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 145  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 157  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 191  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 163  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 45   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 276  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 298  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 369  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 372  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 119  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 201  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 334  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 338  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 403  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 94   | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 104  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 136  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 150  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 249  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 69   | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 87   | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 152  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 285  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 6 条记录
- **快 (50-100ms)**: 94 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 164 次

### 按协议统计

- **none**: 167 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
