# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:22:51
- **数据来源**: connectivity_results-20251230-022251.json
- **总测试数**: 573
- **失败测试数**: 192
- **成功测试数**: 381
- **失败率**: 33.51%
- **平均延迟**: 108.86ms
- **最小延迟**: 44ms
- **最大延迟**: 788ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:22:51
- **IP地址**: 172.203.7.53
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

- **网络不可达: 网络不可达**: 189 次 (98.4%)
- **连接超时: I/O超时**: 3 次 (1.6%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (189 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 4    | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 5    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 9    | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 10   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 13   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 14   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 17   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 18   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 21   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 22   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 28   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 29   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 30   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 33   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 37   | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 38   | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 39   | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 52   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 53   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 54   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 59   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 60   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 61   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 67   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 68   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 69   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 70   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 71   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 76   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 79   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 80   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 84   | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 85   | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 86   | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 96   | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 97   | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 98   | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 101  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 102  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 105  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 106  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 109  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 110  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 144  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 145  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 146  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 150  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 151  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 152  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 156  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 157  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 158  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 162  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 163  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 164  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 168  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 169  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 170  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 174  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 175  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 176  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 180  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 181  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 182  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 186  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 187  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 188  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 192  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 193  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 200  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 201  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 202  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 205  | bestcf.030101.xyz                       | 2606:4700:0:b21c:66ba:fef:2ef4:2658     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:b21c:66ba:fef:2ef4:2658]:443: connect: network is unreachable     |
| 206  | bestcf.030101.xyz                       | 2606:4700:0:26e6:9bb0:8293:779:dc27     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:26e6:9bb0:8293:779:dc27]:443: connect: network is unreachable     |
| 212  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 213  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 216  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 217  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 221  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 222  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 223  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 224  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 231  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 232  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 233  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 238  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 239  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 243  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 244  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 245  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 249  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 250  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 251  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 255  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 256  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 257  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 261  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 262  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 269  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 270  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 271  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 278  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 279  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 283  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 284  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 288  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 289  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 290  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 293  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cdf7:b91:1a47  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cdf7:b91:1a47]:443: connect: network is unreachable  |
| 294  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:3392:350c:968b:1f41:32b0 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:3392:350c:968b:1f41:32b0]:443: connect: network is unreachable |
| 298  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 299  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 300  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 302  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 316  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 322  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 323  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 324  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 328  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 329  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 330  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 336  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 337  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 338  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 339  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 344  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 345  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 346  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 347  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 350  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 351  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 354  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 355  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 356  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 361  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 362  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 367  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 368  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 373  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 374  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 375  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 382  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 383  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 384  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 388  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 389  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 394  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 395  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 396  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 398  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 402  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 403  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 404  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 407  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 414  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 415  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 416  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 419  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 420  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 424  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 425  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 426  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 443  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 444  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 456  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 457  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 465  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 466  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 467  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 485  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 486  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 487  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 488  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 489  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 490  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 491  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 492  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 493  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 494  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 536  | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 537  | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable   |
| 539  | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable   |
| 540  | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable   |
| 549  | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable    |
| 550  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable  |
| 551  | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable   |
| 552  | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 553  | 2803:f800:51:6a7b:7c95:5585:9678:1549   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable   |
| 554  | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable    |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 312  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 357  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 573  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 189 次 (98.4%)
- **连接超时**: 3 次 (1.6%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 192 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 189 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
wilson.ns.cloudflare.com (3次), ip.sb
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 381 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 555  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 363  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 468  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 214  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 167  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 263  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 326  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 349  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 391  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 512  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 421  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 272  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 291  | freeyx.cloudflare88.eu.org            | 141.101.121.109 | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 527  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 386  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 433  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 166  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 531  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 254  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 225  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 548  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 318  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 568  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 280  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 463  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 27   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 450  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 557  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 567  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 520  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 107  | icook.hk                              | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 358  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 439  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 305  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 509  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 189  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 265  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 277  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 529  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 148  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 235  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 369  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 445  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 122  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 208  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 341  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 281  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 234  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 264  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 282  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 36   | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 91   | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 108  | icook.hk                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 307  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 516  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 31   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 178  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 353  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 359  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 399  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 409  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 430  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 470  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 528  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 544  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 24   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 207  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 422  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 454  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 476  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 546  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 274  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 449  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 532  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 545  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 92   | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 103  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 104  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 118  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 127  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 198  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 437  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 482  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 497  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 88   | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 93   | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 211  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 452  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 460  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 495  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 561  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 117  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 136  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 218  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 475  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 227  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 406  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 19   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 48   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 65   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 133  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 297  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 319  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 523  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 534  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 42   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 43   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 63   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 94   | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 154  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 306  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 308  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 469  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 530  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 565  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 72   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 149  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 197  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 314  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 429  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 472  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 75   | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 99   | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 121  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 237  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 292  | freeyx.cloudflare88.eu.org            | 141.101.121.87  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 418  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 478  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 500  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 570  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 137  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 303  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 434  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 507  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 62   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 210  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 304  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 317  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 431  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 525  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 3    | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 155  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 179  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 273  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 366  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 503  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 524  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 325  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 343  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 447  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 473  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 506  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 526  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 191  | zread.ai                              | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 267  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 370  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 498  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 15   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 135  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 160  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 194  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 295  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 311  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 352  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 435  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 446  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 453  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 566  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 124  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 183  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 331  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 332  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 335  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 392  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 35   | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 64   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 129  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 195  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 266  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 412  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 428  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 442  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 480  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 484  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 12   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 46   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 47   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 73   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 83   | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 184  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 196  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 401  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 501  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 508  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 517  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 119  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 128  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 139  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 226  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 229  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 461  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 479  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 504  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 543  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 564  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 74   | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 241  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 242  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 315  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 365  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 371  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 515  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 23   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 51   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 111  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 247  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 252  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 286  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 287  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 380  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 438  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 513  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 560  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 100  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 130  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 161  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 313  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 379  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 397  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 474  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 2    | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 78   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 309  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 321  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 413  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 441  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 26   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 177  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 209  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 464  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 496  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 499  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 569  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 334  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 451  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 40   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 203  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 240  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 333  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 432  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 518  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 377  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 410  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 427  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 483  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 547  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 6    | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 34   | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 58   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 199  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 360  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 423  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 448  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 477  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 558  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 172  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 268  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 481  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 340  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 342  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 436  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 459  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 258  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 455  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 538  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 215  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 285  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 563  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 115  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 116  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 49   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 87   | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 417  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 562  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 11   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 57   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 131  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 134  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 364  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 522  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 66   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 90   | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 123  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 159  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 228  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 246  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 376  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 502  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 8    | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 41   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 204  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 514  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 542  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 7    | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 89   | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 126  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 521  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 50   | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 55   | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 112  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 141  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 185  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 440  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 113  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 405  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 510  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 45   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 408  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 559  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 44   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 132  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 171  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 533  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 535  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 125  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 310  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 411  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 378  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 505  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 471  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 32   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 77   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 275  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 327  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 259  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 296  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 301  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 1    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 348  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 372  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 260  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 16   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 95   | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 82   | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 220  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 236  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 390  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 462  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 230  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 147  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 276  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 143  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 248  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 541  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 381  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 511  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 393  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 400  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 556  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 190  | zread.ai                              | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 320  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 219  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 387  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 81   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 173      | cloudflare |
| 173  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 165  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 25   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 192      | cloudflare |
| 142  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 519  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 120  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 153  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 235      | cloudflare |
| 253  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 258      | cloudflare |
| 56   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 261      | cloudflare |
| 458  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 291      | cloudflare |
| 20   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 304      | cloudflare |
| 385  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 390      | cloudflare |
| 571  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 531      | cloudflare |
| 114  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 679      | cloudflare |
| 572  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 788      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 4 条记录
- **快 (50-100ms)**: 181 条记录
- **正常 (100-200ms)**: 184 条记录
- **慢 (200-500ms)**: 9 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 189 次

### 按协议统计

- **none**: 192 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
