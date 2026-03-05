# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 16:36:31
- **数据来源**: connectivity_results.json
- **总测试数**: 486
- **失败测试数**: 184
- **成功测试数**: 302
- **失败率**: 37.86%
- **平均延迟**: 108.43ms
- **最小延迟**: 45ms
- **最大延迟**: 4832ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 173 次 (94.0%)
- **连接超时: I/O超时**: 9 次 (4.9%)
- **连接被拒绝: 通用连接拒绝**: 2 次 (1.1%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (173 次测试)

| 序号 | 主机/域名                                                                           | 目标IP                               | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                             |
| ---- | ----------------------------------------------------------------------------------- | ------------------------------------ | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------ |
| 9    | bowen.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c53              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable              |
| 10   | bowen.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable              |
| 11   | bowen.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:2353              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable              |
| 15   | julio.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable              |
| 16   | julio.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable              |
| 17   | julio.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable              |
| 21   | silkbook.com                                                                        | 2606:4700:20::681a:8a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable               |
| 22   | silkbook.com                                                                        | 2606:4700:20::681a:9a0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable               |
| 23   | silkbook.com                                                                        | 2606:4700:20::ac43:4bd0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable              |
| 27   | abdullah.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable              |
| 28   | abdullah.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable              |
| 29   | abdullah.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable              |
| 33   | ip.sb                                                                               | 2606:4700:20::681a:d1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable               |
| 34   | ip.sb                                                                               | 2606:4700:20::681a:c1f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable               |
| 35   | ip.sb                                                                               | 2606:4700:20::ac43:4bac              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable              |
| 37   | [2606:4700:4403::7357:544f]                                                         | 2606:4700:4403::7357:544f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable            |
| 42   | braden.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable              |
| 43   | braden.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable              |
| 44   | braden.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable              |
| 47   | www.wto.org                                                                         | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 48   | www.wto.org                                                                         | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 51   | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3034::6815:3db6            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:3db6]:443: connect: network is unreachable            |
| 52   | 72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re | 2606:4700:3036::ac43:d4c5            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::ac43:d4c5]:443: connect: network is unreachable            |
| 53   | [2606:4700:964f::6e2c:588e]                                                         | 2606:4700:964f::6e2c:588e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable            |
| 60   | cf.877771.xyz                                                                       | 2606:4700:3033::ac43:98b7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable            |
| 61   | cf.877771.xyz                                                                       | 2606:4700:3033::6815:50b4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable            |
| 71   | uriah.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable              |
| 72   | uriah.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable              |
| 73   | uriah.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable              |
| 79   | ashton.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable              |
| 80   | ashton.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable              |
| 81   | ashton.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23ad              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable              |
| 96   | dnschecker.org                                                                      | 2606:4700:20::681a:659               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable               |
| 97   | dnschecker.org                                                                      | 2606:4700:20::681a:759               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable               |
| 98   | dnschecker.org                                                                      | 2606:4700:20::ac43:49d8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable              |
| 102  | benedict.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ccd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable              |
| 103  | benedict.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable              |
| 104  | benedict.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23cd              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable              |
| 112  | pranab.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cc7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable              |
| 113  | pranab.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable              |
| 114  | pranab.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23c7              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable              |
| 117  | ifconfig.co                                                                         | 2606:4700:3037::6815:365b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable            |
| 118  | ifconfig.co                                                                         | 2606:4700:3030::ac43:a86a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable            |
| 122  | dylan.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cbb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable              |
| 123  | dylan.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable              |
| 124  | dylan.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23bb              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable              |
| 127  | palera.in                                                                           | 2606:4700:3032::ac43:9d7a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable            |
| 128  | palera.in                                                                           | 2606:4700:3035::6815:3a48            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable            |
| 132  | craig.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cc0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable              |
| 133  | craig.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable              |
| 134  | craig.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23c0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable              |
| 137  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2fb3            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable            |
| 138  | cf.090227.xyz                                                                       | 2606:4700:310c::ac42:2c4d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable            |
| 144  | decker.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable              |
| 145  | decker.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable              |
| 146  | decker.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239b              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable              |
| 148  | [2606:4700:440f::53aa:4126]                                                         | 2606:4700:440f::53aa:4126            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable            |
| 151  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3030::6815:e29             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable             |
| 152  | local-aria2-webui.masx200.ddns-ip.net                                               | 2606:4700:3031::ac43:9db6            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable            |
| 157  | whatismyipaddress.com                                                               | 2606:4700::6813:de4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                 |
| 158  | whatismyipaddress.com                                                               | 2606:4700::6813:df4f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                 |
| 164  | www.udemy.com                                                                       | 2606:4700::6810:8eed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                 |
| 165  | www.udemy.com                                                                       | 2606:4700::6810:8fed                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                 |
| 171  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3037::ac43:aedb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:aedb]:443: connect: network is unreachable            |
| 172  | 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re                              | 2606:4700:3034::6815:2ffc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:2ffc]:443: connect: network is unreachable            |
| 173  | [2606:4700:9add::880:52fc]                                                          | 2606:4700:9add::880:52fc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable             |
| 176  | icook.tw                                                                            | 2606:4700:10::ac42:9e73              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable              |
| 177  | icook.tw                                                                            | 2606:4700:10::6814:1c4a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable              |
| 178  | [2606:4700:83be::11:74f]                                                            | 2606:4700:83be::11:74f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable               |
| 185  | xn--b6gac.eu.org                                                                    | 2606:4700:3035::6815:5a4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable            |
| 186  | xn--b6gac.eu.org                                                                    | 2606:4700:3037::ac43:99fd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable            |
| 190  | sullivan.ns.cloudflare.com                                                          | 2606:4700:58::a29f:2ca1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable              |
| 191  | sullivan.ns.cloudflare.com                                                          | 2803:f800:50::6ca2:c3a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable              |
| 192  | sullivan.ns.cloudflare.com                                                          | 2a06:98c1:50::ac40:23a1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable              |
| 196  | [2606:4700:4409::5b5b:7758]                                                         | 2606:4700:4409::5b5b:7758            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable            |
| 201  | damien.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2ca8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable              |
| 202  | damien.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable              |
| 203  | damien.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23a8              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable              |
| 216  | cris.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2cca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable              |
| 217  | cris.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c3ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable              |
| 218  | cris.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:23ca              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable              |
| 223  | icook.hk                                                                            | 2606:4700:3037::ac43:a168            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable            |
| 224  | icook.hk                                                                            | 2606:4700:3031::6815:5ad2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable            |
| 229  | cf.877774.xyz                                                                       | 2606:4700:4406::ac40:9242            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable            |
| 230  | cf.877774.xyz                                                                       | 2a06:98c1:3102::6812:29be            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable            |
| 235  | singapore.com                                                                       | 2606:4700:20::681a:c8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable               |
| 236  | singapore.com                                                                       | 2606:4700:20::681a:d8c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable               |
| 237  | singapore.com                                                                       | 2606:4700:20::ac43:4bc2              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable              |
| 238  | [2606:4700:8de6::5fa2:799e]                                                         | 2606:4700:8de6::5fa2:799e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable            |
| 239  | [2606:4700:4408::18c5:3304]                                                         | 2606:4700:4408::18c5:3304            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable            |
| 242  | comicabc.com                                                                        | 2606:4700:3030::ac43:ae15            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable            |
| 243  | comicabc.com                                                                        | 2606:4700:3036::6815:400a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable            |
| 246  | ip.gs                                                                               | 2606:4700:3035::ac43:a01c            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable            |
| 247  | ip.gs                                                                               | 2606:4700:3036::6815:eb0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable             |
| 252  | japan.com                                                                           | 2606:4700:20::681a:43c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable               |
| 253  | japan.com                                                                           | 2606:4700:20::ac43:465c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable              |
| 254  | japan.com                                                                           | 2606:4700:20::681a:53c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable               |
| 257  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2f70            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable            |
| 258  | yx-auto.pages.dev                                                                   | 2606:4700:310c::ac42:2c90            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable            |
| 266  | time.is                                                                             | 2606:4700:20::ac43:449d              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable              |
| 267  | time.is                                                                             | 2606:4700:20::681a:d36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable               |
| 268  | time.is                                                                             | 2606:4700:20::681a:c36               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable               |
| 272  | otto.ns.cloudflare.com                                                              | 2606:4700:58::a29f:2c87              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable              |
| 273  | otto.ns.cloudflare.com                                                              | 2803:f800:50::6ca2:c387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable              |
| 274  | otto.ns.cloudflare.com                                                              | 2a06:98c1:50::ac40:2387              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable              |
| 278  | ipinfo.in                                                                           | 2606:4700:3031::6815:1581            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable            |
| 279  | ipinfo.in                                                                           | 2606:4700:3037::ac43:c6cb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable            |
| 287  | stock.hostmonit.com                                                                 | 2606:4700:3033::ac43:bbfb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable            |
| 288  | stock.hostmonit.com                                                                 | 2606:4700:3037::6815:7c1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable             |
| 289  | [2606:4700:440b::3e6e:5f06]                                                         | 2606:4700:440b::3e6e:5f06            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable            |
| 292  | steamdb.info                                                                        | 2606:4700:10::6814:22d4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable              |
| 293  | steamdb.info                                                                        | 2606:4700:10::ac42:affa              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable              |
| 300  | www.hugedomains.com                                                                 | 2606:4700:20::681a:725               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable               |
| 301  | www.hugedomains.com                                                                 | 2606:4700:20::681a:625               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable               |
| 302  | www.hugedomains.com                                                                 | 2606:4700:20::ac43:46bf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable              |
| 306  | www.ipget.net                                                                       | 2606:4700:3031::ac43:cf1a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable            |
| 307  | www.ipget.net                                                                       | 2606:4700:3036::6815:fd4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable             |
| 345  | toy-people.com                                                                      | 2606:4700:20::ac43:4812              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable              |
| 346  | toy-people.com                                                                      | 2606:4700:20::681a:324               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable               |
| 347  | toy-people.com                                                                      | 2606:4700:20::681a:224               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable               |
| 351  | cf.0sm.com                                                                          | 2606:4700:3032::6815:785             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable             |
| 352  | cf.0sm.com                                                                          | 2606:4700:3037::ac43:bb91            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable            |
| 360  | trevor.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c9a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable              |
| 361  | trevor.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c39a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable              |
| 362  | trevor.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:239a              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable              |
| 364  | [2606:4700:83bd::7d8:2b47]                                                          | 2606:4700:83bd::7d8:2b47             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable             |
| 367  | bestcf.030101.xyz                                                                   | 2606:4700:0:5d3:8010:fe57:6ca8:4452  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:5d3:8010:fe57:6ca8:4452]:443: connect: network is unreachable  |
| 368  | bestcf.030101.xyz                                                                   | 2606:4700::69f4:ed8c:2f05:1c41       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::69f4:ed8c:2f05:1c41]:443: connect: network is unreachable       |
| 373  | fbi.gov                                                                             | 2606:4700::6810:94f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                 |
| 374  | fbi.gov                                                                             | 2606:4700::6810:95f4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                 |
| 380  | www.gov.ua                                                                          | 2606:4700:3031::6815:1748            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable            |
| 381  | www.gov.ua                                                                          | 2606:4700:3033::ac43:d17f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable            |
| 384  | www.digitalocean.com                                                                | 2606:4700::6813:ae44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                 |
| 385  | www.digitalocean.com                                                                | 2606:4700::6813:ad44                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                 |
| 388  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3030::6815:e29             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable             |
| 389  | ae8a9c24-83de.masx200.ddns-ip.net                                                   | 2606:4700:3031::ac43:9db6            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable            |
| 393  | tasteatlas.com                                                                      | 2606:4700::6811:2469                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                 |
| 394  | tasteatlas.com                                                                      | 2606:4700::6811:2569                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                 |
| 397  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2c4d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable            |
| 398  | cf.zhetengsha.eu.org                                                                | 2606:4700:310c::ac42:2fb3            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable            |
| 402  | kyree.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2ccf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable              |
| 403  | kyree.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable              |
| 404  | kyree.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23cf              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable              |
| 408  | wilson.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c6e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable              |
| 409  | wilson.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c36e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable              |
| 410  | wilson.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:236e              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable              |
| 413  | zread.ai                                                                            | 2606:4700:3033::6815:4cf0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable            |
| 414  | zread.ai                                                                            | 2606:4700:3032::ac43:ca4e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable            |
| 418  | iplocation.io                                                                       | 2606:4700:20::681a:ade               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable               |
| 419  | iplocation.io                                                                       | 2606:4700:20::681a:bde               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable               |
| 420  | iplocation.io                                                                       | 2606:4700:20::ac43:4664              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable              |
| 425  | rustam.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2c94              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable              |
| 426  | rustam.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable              |
| 427  | rustam.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:2394              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable              |
| 431  | www.whatismyip.com                                                                  | 2606:4700:20::ac43:4581              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable              |
| 432  | www.whatismyip.com                                                                  | 2606:4700:20::681a:d17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable               |
| 433  | www.whatismyip.com                                                                  | 2606:4700:20::681a:c17               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable               |
| 435  | freeyx.cloudflare88.eu.org                                                          | 2606:4700:3010:0:f5e8:7af2:1260:72f1 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:f5e8:7af2:1260:72f1]:443: connect: network is unreachable |
| 440  | moura.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2cd9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable              |
| 441  | moura.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c3d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable              |
| 442  | moura.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:23d9              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable              |
| 445  | yx-auto.pages.dev                                                                   | 2606:4700:3034::6815:9e6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable             |
| 446  | yx-auto.pages.dev                                                                   | 2606:4700:3033::ac43:a162            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable            |
| 450  | huxley.ns.cloudflare.com                                                            | 2606:4700:58::a29f:2cbc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable              |
| 451  | huxley.ns.cloudflare.com                                                            | 2803:f800:50::6ca2:c3bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable              |
| 452  | huxley.ns.cloudflare.com                                                            | 2a06:98c1:50::ac40:23bc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable              |
| 456  | www.pcmag.com                                                                       | 2606:4700::6810:1476                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                 |
| 457  | www.pcmag.com                                                                       | 2606:4700::6810:1576                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                 |
| 461  | lewis.ns.cloudflare.com                                                             | 2606:4700:58::a29f:2c9f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable              |
| 462  | lewis.ns.cloudflare.com                                                             | 2803:f800:50::6ca2:c39f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable              |
| 463  | lewis.ns.cloudflare.com                                                             | 2a06:98c1:50::ac40:239f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable              |
| 478  | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::6815:48e9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable            |
| 479  | cloudflare-ip.mofashi.ltd                                                           | 2606:4700:3037::ac43:9bac            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable            |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 363  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 375  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 390  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 481  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 482  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 483  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 484  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 485  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 486  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |

#### 连接被拒绝: 通用连接拒绝 (2 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                  |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------- |
| 75   | 138.2.18.82     | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: connect: connection refused     |
| 82   | 222.105.131.225 | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: connect: connection refused |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 173 次 (94.0%)
- **连接超时**: 9 次 (4.9%)
- **连接被拒绝**: 2 次 (1.1%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 119.194（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 184 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 11 次，IPv6失败 173 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：bowen.ns.cloudflare.com (3次),
julio.ns.cloudflare.com (3次), silkbook.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 147 |
104.16.223.179 | 104.16.223.179 | IPv4 | h3 | ✅ 成功 | 45 | cloudflare | | 170
| 04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 104.21.47.252 | IPv4
| h3 | ✅ 成功 | 45 | cloudflare | | 211 | ct.877774.xyz | 172.64.229.161 | IPv4
| h3 | ✅ 成功 | 45 | cloudflare | | 334 | 172.64.156.195 | 172.64.156.195 |
IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 369 | 104.17.68.85 | 104.17.68.85 |
IPv4 | h3 | ✅ 成功 | 46 | cloudflare | | 67 | 172.64.153.172 | 172.64.153.172 |
IPv4 | h3 | ✅ 成功 | 47 | cloudflare | | 14 | julio.ns.cloudflare.com |
172.64.35.209 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 226 | 172.64.49.165 |
172.64.49.165 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 371 | fbi.gov |
104.16.148.244 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | | 443 |
yx-auto.pages.dev | 172.67.161.98 | IPv4 | h3 | ✅ 成功 | 48 | cloudflare | |
339 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | | 430
| www.whatismyip.com | 104.26.12.23 | IPv4 | h3 | ✅ 成功 | 49 | cloudflare | |
87 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅ 成功 | 50 | cloudflare |
| 338 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅ 成功 | 51 | cloudflare | | 65
| saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 74 |
172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 90 |
www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 250
| japan.com | 172.67.70.92 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 261 |
eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 265 |
time.is | 104.26.13.54 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 308 |
cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 52 | cloudflare | | 150
| local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h3 | ✅ 成功 |
53 | cloudflare | | 326 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h3 | ✅ 成功 |
53 | cloudflare | | 335 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 | 53
| cloudflare | | 336 | www.csgo.com | 195.85.59.161 | IPv4 | h3 | ✅ 成功 | 53 |
cloudflare | | 434 | freeyx.cloudflare88.eu.org | 141.101.121.254 | IPv4 | h3 |
✅ 成功 | 53 | cloudflare | | 59 | cf.877771.xyz | 172.67.152.183 | IPv4 | h3 |
✅ 成功 | 54 | cloudflare | | 366 | bestcf.030101.xyz | 104.19.152.12 | IPv4 |
h3 | ✅ 成功 | 54 | cloudflare | | 169 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re | 172.67.174.219 | IPv4 |
h3 | ✅ 成功 | 55 | cloudflare | | 213 | cris.ns.cloudflare.com |
108.162.195.202 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 259 | 172.67.243.218
| 172.67.243.218 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 310 |
cmcc.877774.xyz | 104.16.149.2 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 327 |
cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 343 |
toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare | | 407 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare
| | 464 | cu.877774.xyz | 104.26.4.118 | IPv4 | h3 | ✅ 成功 | 55 | cloudflare |
| 36 | 104.17.142.12 | 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 56 | cloudflare | |
70 | uriah.ns.cloudflare.com | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 56 |
cloudflare | | 181 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 | ✅ 成功 | 56
| cloudflare | | 195 | www.visa.com.hk | 104.18.21.69 | IPv4 | h3 | ✅ 成功 | 56
| cloudflare | | 299 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功
| 56 | cloudflare | | 313 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功
| 56 | cloudflare | | 355 | www.visa.com.sg | 104.18.12.229 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 356 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 |
✅ 成功 | 56 | cloudflare | | 412 | zread.ai | 172.67.202.78 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 472 | cu.877774.xyz | 104.26.4.117 | IPv4 | h3 | ✅
成功 | 56 | cloudflare | | 115 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅
成功 | 57 | cloudflare | | 180 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 207 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 209 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 241 | comicabc.com | 172.67.174.21 | IPv4 | h3 |
✅ 成功 | 57 | cloudflare | | 295 | 162.159.36.104 | 162.159.36.104 | IPv4 | h3
| ✅ 成功 | 57 | cloudflare | | 447 | huxley.ns.cloudflare.com | 108.162.195.188
| IPv4 | h3 | ✅ 成功 | 57 | cloudflare | | 471 | cu.877774.xyz | 104.26.4.116 |
IPv4 | h3 | ✅ 成功 | 57 | cloudflare | | 66 | saas.sin.fan | 162.159.36.20 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 85 | www.okcupid.com | 104.17.48.63 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 93 | dnschecker.org | 104.26.6.89 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 153 | www.visa.cn | 162.159.153.2 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 260 | eur.877774.xyz | 104.21.26.150 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 317 | cmcc.877774.xyz | 104.16.149.9 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 330 | cmcc.877774.xyz | 104.16.148.9 |
IPv4 | h3 | ✅ 成功 | 58 | cloudflare | | 39 | braden.ns.cloudflare.com |
108.162.195.169 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 291 | steamdb.info |
172.66.175.250 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 294 | 172.64.35.24 |
172.64.35.24 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 309 | cmcc.877774.xyz |
104.16.149.1 | IPv4 | h3 | ✅ 成功 | 59 | cloudflare | | 88 | www.okcupid.com |
104.16.239.254 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 131 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 166 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 275 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare
| | 168 | 104.19.223.58 | 104.19.223.58 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare
| | 227 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare
| | 231 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 466 | cu.877774.xyz | 104.26.4.111 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | |
470 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare | | 8
| bowen.ns.cloudflare.com | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare
| | 107 | 104.18.254.88 | 104.18.254.88 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare
| | 348 | shopify.com | 23.227.38.33 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | |
353 | www.4chan.org | 104.16.228.229 | IPv4 | h3 | ✅ 成功 | 62 | cloudflare | |
382 | www.digitalocean.com | 104.19.173.68 | IPv4 | h3 | ✅ 成功 | 62 |
cloudflare | | 154 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 63 |
cloudflare | | 284 | 162.159.133.85 | 162.159.133.85 | IPv4 | h3 | ✅ 成功 | 63
| cloudflare | | 340 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 63 |
cloudflare | | 458 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅
成功 | 63 | cloudflare | | 50 |
72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re
| 172.67.212.197 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 62 | na.877774.xyz
| 104.19.74.233 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 232 | singapore.com
| 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 276 | ipinfo.in |
104.21.21.129 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 304 | www.ipget.net |
172.67.207.26 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 314 | cmcc.877774.xyz
| 104.16.149.6 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | | 439 |
moura.ns.cloudflare.com | 172.64.35.217 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare
| | 5 | 172.67.106.26 | 172.67.106.26 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare |
| 175 | icook.tw | 172.66.158.115 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | |
256 | yx-auto.pages.dev | 172.66.44.144 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare
| | 324 | cmcc.877774.xyz | 104.16.148.3 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare
| | 333 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 65 |
cloudflare | | 386 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 |
h3 | ✅ 成功 | 65 | cloudflare | | 6 | bowen.ns.cloudflare.com | 108.162.195.83
| IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 162 | www.udemy.com | 104.16.142.237
| IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 200 | damien.ns.cloudflare.com |
172.64.35.168 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare | | 318 | cmcc.877774.xyz
| 104.16.149.10 | IPv4 | h3 | ✅ 成功 | 66 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 12 条记录
- **快 (50-100ms)**: 88 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 11 次
- **IPv6 失败**: 173 次

### 按协议统计

- **none**: 184 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
