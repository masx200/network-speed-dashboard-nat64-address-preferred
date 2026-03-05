# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 09:50:45
- **数据来源**: connectivity_results.json
- **总测试数**: 500
- **失败测试数**: 188
- **成功测试数**: 312
- **失败率**: 37.60%
- **平均延迟**: 106.82ms
- **最小延迟**: 67ms
- **最大延迟**: 546ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 175 次 (93.1%)
- **连接超时: I/O超时**: 10 次 (5.3%)
- **连接被拒绝: 通用连接拒绝**: 2 次 (1.1%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (175 次测试)

| 序号 | 主机/域名                                                                           | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ----------------------------------------------------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 13   | www.wto.org                                                                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 14   | www.wto.org                                                                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 18   | silkbook.com                                                                        | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 19   | silkbook.com                                                                        | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 20   | silkbook.com                                                                        | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 21   | [2606:4700:4403::7357:544f]                                                         | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 25   | abdullah.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 26   | abdullah.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 27   | abdullah.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 31   | bowen.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 32   | bowen.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 33   | bowen.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 37   | julio.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 38   | julio.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 39   | julio.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 43   | braden.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 44   | braden.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 45   | braden.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 46   | [2606:4700:964f::6e2c:588e]                                                         | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 49   | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3034::6815:3db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:3db6]:443: connect: network is unreachable               |
| 50   | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3036::ac43:d4c5               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::ac43:d4c5]:443: connect: network is unreachable               |
| 54   | lewis.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 55   | lewis.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 56   | lewis.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 64   | uriah.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 65   | uriah.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 66   | uriah.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 77   | cf.877771.xyz                                                                       | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 78   | cf.877771.xyz                                                                       | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 82   | ashton.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 83   | ashton.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 84   | ashton.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 96   | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 97   | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 102  | dnschecker.org                                                                      | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 103  | dnschecker.org                                                                      | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 104  | dnschecker.org                                                                      | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 108  | benedict.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 109  | benedict.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 110  | benedict.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 117  | pranab.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 118  | pranab.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 119  | pranab.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 122  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2fb3               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable               |
| 123  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2c4d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable               |
| 126  | ifconfig.co                                                                         | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 127  | ifconfig.co                                                                         | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 131  | dylan.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 132  | dylan.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 133  | dylan.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 136  | palera.in                                                                           | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 137  | palera.in                                                                           | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 141  | craig.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 142  | craig.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 143  | craig.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 148  | decker.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 149  | decker.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 150  | decker.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 152  | [2606:4700:440f::53aa:4126]                                                         | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 155  | whatismyipaddress.com                                                               | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 156  | whatismyipaddress.com                                                               | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 161  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 162  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 165  | www.udemy.com                                                                       | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 166  | www.udemy.com                                                                       | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 177  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3037::ac43:aedb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:aedb]:443: connect: network is unreachable               |
| 178  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3034::6815:2ffc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:2ffc]:443: connect: network is unreachable               |
| 179  | [2606:4700:9add::880:52fc]                                                          | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 181  | [2606:4700:83be::11:74f]                                                            | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 184  | icook.tw                                                                            | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 185  | icook.tw                                                                            | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 188  | xn--b6gac.eu.org                                                                    | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 189  | xn--b6gac.eu.org                                                                    | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 195  | sullivan.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 196  | sullivan.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 197  | sullivan.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 200  | [2606:4700:4409::5b5b:7758]                                                         | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 205  | damien.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 206  | damien.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 207  | damien.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 214  | cris.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 215  | cris.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 216  | cris.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 220  | toy-people.com                                                                      | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 221  | toy-people.com                                                                      | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 222  | toy-people.com                                                                      | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 234  | ip.sb                                                                               | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 235  | ip.sb                                                                               | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 236  | ip.sb                                                                               | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 241  | cf.877774.xyz                                                                       | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 242  | cf.877774.xyz                                                                       | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 247  | icook.hk                                                                            | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 248  | icook.hk                                                                            | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 253  | singapore.com                                                                       | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 254  | singapore.com                                                                       | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 255  | singapore.com                                                                       | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 256  | [2606:4700:8de6::5fa2:799e]                                                         | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 257  | [2606:4700:4408::18c5:3304]                                                         | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 260  | comicabc.com                                                                        | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 261  | comicabc.com                                                                        | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 264  | ip.gs                                                                               | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 265  | ip.gs                                                                               | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 270  | japan.com                                                                           | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 271  | japan.com                                                                           | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 272  | japan.com                                                                           | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 275  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 276  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 284  | otto.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 285  | otto.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 286  | otto.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 289  | ipinfo.in                                                                           | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 290  | ipinfo.in                                                                           | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 294  | time.is                                                                             | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 295  | time.is                                                                             | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 296  | time.is                                                                             | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 303  | steamdb.info                                                                        | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 304  | steamdb.info                                                                        | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 308  | stock.hostmonit.com                                                                 | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 309  | stock.hostmonit.com                                                                 | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 310  | [2606:4700:440b::3e6e:5f06]                                                         | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 343  | www.hugedomains.com                                                                 | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 344  | www.hugedomains.com                                                                 | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 345  | www.hugedomains.com                                                                 | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 350  | www.ipget.net                                                                       | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 351  | www.ipget.net                                                                       | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 361  | cf.0sm.com                                                                          | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 362  | cf.0sm.com                                                                          | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 372  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2c4d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable               |
| 373  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2fb3               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable               |
| 376  | bestcf.030101.xyz                                                                   | 2606:4700::e81a:f8c6:467d:2d94          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::e81a:f8c6:467d:2d94]:443: connect: network is unreachable          |
| 377  | bestcf.030101.xyz                                                                   | 2606:4700:0:db:66c:f662:2aa0:bf33       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:db:66c:f662:2aa0:bf33]:443: connect: network is unreachable       |
| 378  | [2606:4700:83bd::7d8:2b47]                                                          | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 384  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 385  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 388  | fbi.gov                                                                             | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 389  | fbi.gov                                                                             | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 392  | www.gov.ua                                                                          | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 393  | www.gov.ua                                                                          | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 396  | www.digitalocean.com                                                                | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 397  | www.digitalocean.com                                                                | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 400  | tasteatlas.com                                                                      | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 401  | tasteatlas.com                                                                      | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 403  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3009:7dab:39d5:a339:79b6:3c1c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:7dab:39d5:a339:79b6:3c1c]:443: connect: network is unreachable |
| 404  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3009:aa59:4b67:100d:4fdb:d8a1 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:100d:4fdb:d8a1]:443: connect: network is unreachable |
| 405  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3009::83da:e4f:b8a5           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009::83da:e4f:b8a5]:443: connect: network is unreachable           |
| 410  | kyree.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 411  | kyree.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 412  | kyree.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 416  | wilson.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 417  | wilson.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 418  | wilson.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 422  | huxley.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 423  | huxley.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 424  | huxley.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 428  | www.pcmag.com                                                                       | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 429  | www.pcmag.com                                                                       | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 433  | zread.ai                                                                            | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 434  | zread.ai                                                                            | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 449  | iplocation.io                                                                       | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 450  | iplocation.io                                                                       | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 451  | iplocation.io                                                                       | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 455  | rustam.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 456  | rustam.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 457  | rustam.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 461  | www.whatismyip.com                                                                  | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 462  | www.whatismyip.com                                                                  | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 463  | www.whatismyip.com                                                                  | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 467  | trevor.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 468  | trevor.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 469  | trevor.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 473  | moura.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 474  | moura.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 475  | moura.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 478  | yx-auto.pages.dev                                                                   | 2606:4700:3033::ac43:a162               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable               |
| 479  | yx-auto.pages.dev                                                                   | 2606:4700:3034::6815:9e6                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable                |

#### 连接超时: I/O超时 (10 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 379  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 444  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 480  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 481  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 482  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 483  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 484  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 485  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 486  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 498  | cfip.xxxxxxxx.tk | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |

#### 连接被拒绝: 通用连接拒绝 (2 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                  |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------- |
| 201  | 222.105.131.225 | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: connect: connection refused |
| 346  | 138.2.18.82     | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: connect: connection refused     |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名     | 目标IP        | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                                                              |
| ---- | ------------- | ------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------- |
| 266  | 158.101.77.33 | 158.101.77.33 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": read tcp 10.1.1.23:47500->158.101.77.33:443: read: connection reset by peer |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 175 次 (93.1%)
- **连接超时**: 10 次 (5.3%)
- **连接被拒绝**: 2 次 (1.1%)
- **DNS解析错误**: 1 次 (0.5%)

#### 错误模式分析

**超时集中度分析**: 共有 10 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 187 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 13 次，IPv6失败 175 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：silkbook.com (3次),
abdullah.ns.cloudflare.com (3次), bowen.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 113 |
104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 204 |
damien.ns.cloudflare.com | 172.64.35.168 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare
| | 280 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare
| | 352 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare |
| 81 | ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 70 |
cloudflare | | 154 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h3 | ✅ 成功
| 71 | cloudflare | | 173 | asia.877774.xyz | 104.16.211.153 | IPv4 | h3 | ✅
成功 | 71 | cloudflare | | 438 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅
成功 | 71 | cloudflare | | 252 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅
成功 | 72 | cloudflare | | 395 | www.digitalocean.com | 104.19.174.68 | IPv4 |
h3 | ✅ 成功 | 72 | cloudflare | | 495 | cfip.xxxxxxxx.tk | 190.93.247.169 |
IPv4 | h3 | ✅ 成功 | 72 | cloudflare | | 86 | www.okcupid.com | 104.17.48.63 |
IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 320 | cmcc.877774.xyz | 104.16.149.10
| IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 30 | bowen.ns.cloudflare.com |
172.64.35.83 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 176 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 172.67.174.219 | IPv4 |
h3 | ✅ 成功 | 74 | cloudflare | | 306 | stock.hostmonit.com | 104.21.7.193 |
IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 342 | www.hugedomains.com |
104.26.6.37 | IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 383 |
ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 | 74 |
cloudflare | | 15 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅ 成功 | 75 |
cloudflare | | 288 | ipinfo.in | 172.67.198.203 | IPv4 | h3 | ✅ 成功 | 75 |
cloudflare | | 323 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 75
| cloudflare | | 85 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 |
76 | cloudflare | | 315 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 |
76 | cloudflare | | 500 | cfip.xxxxxxxx.tk | 104.17.127.110 | IPv4 | h3 | ✅
成功 | 76 | cloudflare | | 73 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅
成功 | 77 | cloudflare | | 366 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅
成功 | 77 | cloudflare | | 59 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3 |
✅ 成功 | 78 | cloudflare | | 232 | ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 |
78 | cloudflare | | 292 | time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 334 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h3 | ✅ 成功 | 78
| cloudflare | | 347 | www.7749tv.com | 104.19.133.4 | IPv4 | h3 | ✅ 成功 | 78
| cloudflare | | 357 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 60 | www.ipchicken.com | 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 79
| cloudflare | | 94 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 134 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅
成功 | 79 | cloudflare | | 159 | local-aria2-webui.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 233 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 311 | 172.64.35.24 |
172.64.35.24 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 374 | bestcf.030101.xyz
| 104.17.54.201 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare | | 421 |
huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 79 | cloudflare
| | 454 | rustam.ns.cloudflare.com | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 79 |
cloudflare | | 231 | ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 249 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 399 | tasteatlas.com | 104.17.36.105 | IPv4 | h3 | ✅ 成功 | 80 |
cloudflare | | 477 | yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 |
80 | cloudflare | | 17 | silkbook.com | 104.26.9.160 | IPv4 | h3 | ✅ 成功 | 81
| cloudflare | | 63 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅
成功 | 81 | cloudflare | | 208 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 |
✅ 成功 | 81 | cloudflare | | 251 | singapore.com | 172.67.75.194 | IPv4 | h3 |
✅ 成功 | 81 | cloudflare | | 262 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功
| 81 | cloudflare | | 267 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 300 | gamer.com.tw | 104.18.3.197 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 359 | cf.0sm.com | 104.21.7.133 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 364 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 427 | www.pcmag.com | 104.16.21.118 | IPv4 | h3 | ✅ 成功 | 81 |
cloudflare | | 472 | moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅
成功 | 81 | cloudflare | | 494 | cfip.xxxxxxxx.tk | 104.25.105.1 | IPv4 | h3 |
✅ 成功 | 81 | cloudflare | | 9 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 34 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4
| h3 | ✅ 成功 | 82 | cloudflare | | 163 | www.udemy.com | 104.16.143.237 | IPv4
| h3 | ✅ 成功 | 82 | cloudflare | | 298 | 104.16.45.84 | 104.16.45.84 | IPv4 |
h3 | ✅ 成功 | 82 | cloudflare | | 327 | cmcc.877774.xyz | 104.16.148.4 | IPv4 |
h3 | ✅ 成功 | 82 | cloudflare | | 7 | 104.18.78.214 | 104.18.78.214 | IPv4 | h3
| ✅ 成功 | 83 | cloudflare | | 16 | silkbook.com | 172.67.75.208 | IPv4 | h3 |
✅ 成功 | 83 | cloudflare | | 71 | 172.67.181.209 | 172.67.181.209 | IPv4 | h3 |
✅ 成功 | 83 | cloudflare | | 90 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3
| ✅ 成功 | 83 | cloudflare | | 91 | www.glassdoor.com | 104.16.25.46 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 151 | 104.16.223.179 | 104.16.223.179 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 191 | www.visa.com.hk | 104.18.21.69 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 202 | damien.ns.cloudflare.com |
108.162.195.168 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 305 | 162.159.133.85
| 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 322 |
cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | | 333
| cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功 | 83 | cloudflare | |
413 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 439 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 83 |
cloudflare | | 74 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 99 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 105 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 |
✅ 成功 | 84 | cloudflare | | 128 | dylan.ns.cloudflare.com | 108.162.195.187 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 239 | cf.877774.xyz | 104.18.41.190 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 250 | singapore.com | 104.26.13.140 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 381 | 172.64.146.16 | 172.64.146.16 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 431 | zread.ai | 172.67.202.78 | IPv4
| h3 | ✅ 成功 | 84 | cloudflare | | 446 | iplocation.io | 104.26.10.222 | IPv4
| h3 | ✅ 成功 | 84 | cloudflare | | 492 | cfip.xxxxxxxx.tk | 190.93.244.201 |
IPv4 | h3 | ✅ 成功 | 84 | cloudflare | | 69 | saas.sin.fan | 162.159.36.20 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 72 | na.877774.xyz | 104.19.74.233 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 76 | cf.877771.xyz | 172.67.152.183 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 111 | 172.64.148.15 | 172.64.148.15 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 112 | 104.18.254.88 | 104.18.254.88 |
IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 138 | craig.ns.cloudflare.com |
108.162.195.192 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 243 | 103.160.204.59
| 103.160.204.59 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 299 | gamer.com.tw
| 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 321 | cmcc.877774.xyz
| 104.16.149.11 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | | 341 |
www.hugedomains.com | 104.26.7.37 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
353 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
368 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare |
| 445 | 172.67.110.232 | 172.67.110.232 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare
| | 61 | uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 86 |
cloudflare | | 124 | ifconfig.co | 104.21.54.91 | IPv4 | h3 | ✅ 成功 | 86 |
cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 13 次
- **IPv6 失败**: 175 次

### 按协议统计

- **none**: 187 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
