# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:14:56
- **数据来源**: connectivity_results-20251214-141456.json
- **总测试数**: 441
- **失败测试数**: 167
- **成功测试数**: 274
- **失败率**: 37.87%
- **平均延迟**: 103.88ms
- **最小延迟**: 48ms
- **最大延迟**: 739ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:14:56
- **IP地址**: 172.172.119.100
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

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 6    | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 7    | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 8    | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 11   | www.pcmag.com                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 12   | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 15   | ipinfo.in                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 16   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 20   | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 21   | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 22   | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 25   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 26   | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 29   | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 30   | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 43   | www.gov.ua                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 44   | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 50   | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 51   | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 52   | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 55   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 60   | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 61   | icook.hk                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 66   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 67   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 68   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 77   | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 78   | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 79   | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 89   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 93   | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 94   | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 95   | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 99   | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 100  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 101  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 105  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 106  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 107  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 119  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 120  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 121  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 126  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 127  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 130  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 131  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 135  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 136  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 137  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 143  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 144  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 145  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 176  | toy-people.com                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 177  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 178  | toy-people.com                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 185  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 186  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 187  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 191  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 192  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 193  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 199  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 200  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 201  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 204  | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 205  | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 207  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa59:4b79:c9c1:cab1:57d6 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b79:c9c1:cab1:57d6]:443: connect: network is unreachable |
| 208  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 211  | bestcf.030101.xyz                     | 2606:4700::8d:f082:8938:66d8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::8d:f082:8938:66d8]:443: connect: network is unreachable            |
| 212  | bestcf.030101.xyz                     | 2606:4700::fffd:819d:acda               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::fffd:819d:acda]:443: connect: network is unreachable               |
| 216  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 217  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 218  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 221  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 222  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 229  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 230  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 231  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 234  | xn--b6gac.eu.org                      | 2a06:98c1:3121::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                       |
| 235  | xn--b6gac.eu.org                      | 2a06:98c1:3120::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                       |
| 238  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 239  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 244  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 245  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 252  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 253  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 254  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 258  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 259  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 260  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 264  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 265  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 266  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 271  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 272  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 277  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 278  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 279  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 282  | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 283  | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 285  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 288  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 289  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 294  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 295  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 299  | silkbook.com                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 300  | silkbook.com                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 301  | silkbook.com                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 303  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 307  | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 308  | dnschecker.org                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 309  | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 313  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 314  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 315  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 320  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 328  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 329  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 330  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 332  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 336  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 337  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 338  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 342  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 343  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 344  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 346  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 349  | www.ipget.net                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 350  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 353  | comicabc.com                          | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 354  | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 355  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 358  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 359  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 362  | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 363  | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 367  | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 368  | www.udemy.com                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 375  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 379  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 380  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 382  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 386  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 387  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 388  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 391  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 392  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 396  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 397  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 398  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 402  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 403  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 407  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 408  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 409  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 418  | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 419  | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 423  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 424  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 425  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 429  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 430  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 434  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 435  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 436  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 319  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 439  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 441  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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
trevor.ns.cloudflare.com (3次), iplocation.io
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 290  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 256  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 202  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 437  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 361  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 366  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 241  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 270  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 427  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 196  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 365  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 381  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 267  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 32   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 35   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 360  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 197  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 223  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 310  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 194  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 240  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 273  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 357  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 13   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 47   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 206  | freeyx.cloudflare88.eu.org            | 141.101.120.184 | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 356  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 38   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 394  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 9    | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 84   | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 113  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 215  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 49   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 70   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 181  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 341  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 372  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 10   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 195  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 324  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 369  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 385  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 2    | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 18   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 225  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 4    | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 90   | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 250  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 321  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 23   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 45   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 87   | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 91   | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 110  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 170  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 247  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 276  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 280  | ip.gs                                 | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 287  | cf.090227.xyz                         | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 401  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 108  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 214  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 255  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 411  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 138  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 189  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 213  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 243  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 304  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 306  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 376  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 36   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 59   | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 123  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 129  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 132  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 173  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 389  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 48   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 57   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 58   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 128  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 190  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 203  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 232  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 325  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 374  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 39   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 268  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 269  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 378  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 400  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 422  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 428  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 433  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 117  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 182  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 219  | cloudflare-ip.mofashi.ltd             | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 383  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 64   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 75   | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 111  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 226  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 237  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 275  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 371  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 236  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 364  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 421  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 34   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 71   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 73   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 134  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 281  | ip.gs                                 | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 323  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 412  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 37   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 76   | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 82   | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 142  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 284  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 296  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 302  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 405  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 28   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 41   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 72   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 124  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 242  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 318  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 414  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 438  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 88   | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 168  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 172  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 179  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 274  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 31   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 42   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 104  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 174  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 331  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 347  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 370  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 413  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 140  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 169  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 233  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 293  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 351  | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 415  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 74   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 118  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 257  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 96   | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 209  | bestcf.030101.xyz                     | 104.17.212.134  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 33   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 63   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 115  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 180  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 248  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 286  | cf.090227.xyz                         | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 395  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 404  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 114  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 262  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 335  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 416  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 27   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 56   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 86   | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 261  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 46   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 62   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 80   | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 125  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 431  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 19   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 83   | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 373  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 40   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 97   | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 2 条记录
- **快 (50-100ms)**: 179 条记录
- **正常 (100-200ms)**: 19 条记录
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
