# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 12:11:16
- **数据来源**: connectivity_results-20251212-121115.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 79.38ms
- **最小延迟**: 34ms
- **最大延迟**: 2500ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 12:11:16
- **IP地址**: 172.184.173.244
- **国家/地区**: United States (US)
- **ASN**: AS8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **数据源**: ipinfo.io

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 164 次 (98.2%)
- **连接超时: I/O超时**: 3 次 (1.8%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (164 次测试)

| 序号 | 主机/域名                             | 目标IP                              | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | ------------------------------------- | ----------------------------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 8    | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable             |
| 9    | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable             |
| 10   | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable             |
| 16   | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable             |
| 17   | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable             |
| 18   | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable             |
| 21   | zread.ai                              | 2606:4700:3033::6815:4cf0           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable           |
| 22   | zread.ai                              | 2606:4700:3032::ac43:ca4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable           |
| 25   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 26   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |
| 27   | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable           |
| 31   | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable             |
| 32   | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable             |
| 33   | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable             |
| 37   | www.whatismyip.com                    | 2606:4700:20::681a:c17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable              |
| 38   | www.whatismyip.com                    | 2606:4700:20::681a:d17              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable              |
| 39   | www.whatismyip.com                    | 2606:4700:20::ac43:4581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable             |
| 42   | bestcf.030101.xyz                     | 2606:4700:0:26e6:9bb0:8293:779:dc27 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:26e6:9bb0:8293:779:dc27]:443: connect: network is unreachable |
| 43   | bestcf.030101.xyz                     | 2606:4700:0:b21c:66ba:fef:2ef4:2658 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:b21c:66ba:fef:2ef4:2658]:443: connect: network is unreachable |
| 46   | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable           |
| 53   | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable             |
| 54   | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable             |
| 55   | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable             |
| 58   | fbi.gov                               | 2606:4700::6810:95f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                |
| 59   | fbi.gov                               | 2606:4700::6810:94f4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                |
| 63   | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable             |
| 64   | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable             |
| 65   | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable             |
| 68   | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable           |
| 69   | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable           |
| 72   | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable           |
| 73   | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable           |
| 77   | time.is                               | 2606:4700:20::ac43:449d             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable             |
| 78   | time.is                               | 2606:4700:20::681a:c36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable              |
| 79   | time.is                               | 2606:4700:20::681a:d36              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable              |
| 86   | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable             |
| 87   | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable             |
| 88   | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable             |
| 92   | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable             |
| 93   | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable             |
| 94   | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable             |
| 97   | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable           |
| 98   | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable           |
| 102  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable             |
| 103  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable             |
| 104  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable             |
| 114  | palera.in                             | 2606:4700:3032::ac43:9d7a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable           |
| 115  | palera.in                             | 2606:4700:3035::6815:3a48           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable           |
| 119  | singapore.com                         | 2606:4700:20::681a:c8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable              |
| 120  | singapore.com                         | 2606:4700:20::681a:d8c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable              |
| 121  | singapore.com                         | 2606:4700:20::ac43:4bc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable             |
| 124  | ip.gs                                 | 2606:4700:3036::6815:eb0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable            |
| 125  | ip.gs                                 | 2606:4700:3035::ac43:a01c           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable           |
| 129  | silkbook.com                          | 2606:4700:20::681a:8a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable              |
| 130  | silkbook.com                          | 2606:4700:20::681a:9a0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable              |
| 131  | silkbook.com                          | 2606:4700:20::ac43:4bd0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable             |
| 133  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable           |
| 138  | ip.sb                                 | 2606:4700:20::681a:c1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable              |
| 139  | ip.sb                                 | 2606:4700:20::681a:d1f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable              |
| 140  | ip.sb                                 | 2606:4700:20::ac43:4bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable             |
| 145  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable             |
| 146  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable             |
| 147  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable             |
| 150  | whatismyipaddress.com                 | 2606:4700::6813:de4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                |
| 151  | whatismyipaddress.com                 | 2606:4700::6813:df4f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                |
| 154  | comicabc.com                          | 2606:4700:3030::ac43:ae15           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable           |
| 155  | comicabc.com                          | 2606:4700:3036::6815:400a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable           |
| 158  | www.ipget.net                         | 2606:4700:3036::6815:fd4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable            |
| 159  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable           |
| 163  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable             |
| 164  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable             |
| 165  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable             |
| 169  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable             |
| 170  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable             |
| 171  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable             |
| 175  | dnschecker.org                        | 2606:4700:20::ac43:49d8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable             |
| 176  | dnschecker.org                        | 2606:4700:20::681a:659              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable              |
| 177  | dnschecker.org                        | 2606:4700:20::681a:759              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable              |
| 180  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable           |
| 181  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable           |
| 185  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable             |
| 186  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable             |
| 187  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable             |
| 191  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable             |
| 192  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable             |
| 193  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable             |
| 196  | ipinfo.in                             | 2606:4700:3031::6815:1581           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable           |
| 197  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable           |
| 200  | steamdb.info                          | 2606:4700:10::6814:22d4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable             |
| 201  | steamdb.info                          | 2606:4700:10::ac42:affa             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable             |
| 213  | www.pcmag.com                         | 2606:4700::6810:1576                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                |
| 214  | www.pcmag.com                         | 2606:4700::6810:1476                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                |
| 221  | cf.0sm.com                            | 2606:4700:3032::6815:785            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable            |
| 222  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable           |
| 225  | www.gov.ua                            | 2606:4700:3033::ac43:d17f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable           |
| 226  | www.gov.ua                            | 2606:4700:3031::6815:1748           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable           |
| 232  | iplocation.io                         | 2606:4700:20::681a:bde              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable              |
| 233  | iplocation.io                         | 2606:4700:20::681a:ade              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable              |
| 234  | iplocation.io                         | 2606:4700:20::ac43:4664             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable             |
| 235  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable              |
| 266  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable            |
| 272  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable             |
| 273  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable             |
| 274  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable             |
| 276  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable           |
| 280  | icook.hk                              | 2606:4700:3031::6815:5ad2           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable           |
| 281  | icook.hk                              | 2606:4700:3037::ac43:a168           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable           |
| 285  | www.wto.org                           | 2606:4700:4406::ac40:9242           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable           |
| 286  | www.wto.org                           | 2a06:98c1:3102::6812:29be           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable           |
| 288  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable           |
| 291  | ifconfig.co                           | 2606:4700:3030::ac43:a86a           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable           |
| 292  | ifconfig.co                           | 2606:4700:3037::6815:365b           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable           |
| 296  | japan.com                             | 2606:4700:20::ac43:465c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable             |
| 297  | japan.com                             | 2606:4700:20::681a:43c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable              |
| 298  | japan.com                             | 2606:4700:20::681a:53c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable              |
| 302  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable           |
| 306  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable             |
| 307  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable             |
| 308  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable             |
| 310  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable           |
| 313  | www.udemy.com                         | 2606:4700::6810:8fed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                |
| 314  | www.udemy.com                         | 2606:4700::6810:8eed                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                |
| 319  | tasteatlas.com                        | 2606:4700::6811:2469                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                |
| 320  | tasteatlas.com                        | 2606:4700::6811:2569                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                |
| 323  | www.digitalocean.com                  | 2606:4700::6813:ae44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                |
| 324  | www.digitalocean.com                  | 2606:4700::6813:ad44                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                |
| 327  | icook.tw                              | 2606:4700:10::6814:1c4a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable             |
| 328  | icook.tw                              | 2606:4700:10::ac42:9e73             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable             |
| 332  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable             |
| 333  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable             |
| 334  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable             |
| 337  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable           |
| 340  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable            |
| 348  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable             |
| 349  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable             |
| 350  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable             |
| 353  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable           |
| 354  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable            |
| 358  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable             |
| 359  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable             |
| 360  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable             |
| 366  | www.hugedomains.com                   | 2606:4700:20::681a:725              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable              |
| 367  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable             |
| 368  | www.hugedomains.com                   | 2606:4700:20::681a:625              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable              |
| 384  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable             |
| 385  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable             |
| 386  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable             |
| 397  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable             |
| 398  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable             |
| 399  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable             |
| 402  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable           |
| 403  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable           |
| 406  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable           |
| 407  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable           |
| 415  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable             |
| 416  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable             |
| 417  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable             |
| 424  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable             |
| 425  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable             |
| 426  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable             |
| 431  | toy-people.com                        | 2606:4700:20::ac43:4812             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable             |
| 432  | toy-people.com                        | 2606:4700:20::681a:324              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable              |
| 433  | toy-people.com                        | 2606:4700:20::681a:224              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable              |
| 436  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:fb:e00f:f2d6:8f63  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:fb:e00f:f2d6:8f63]:443: connect: network is unreachable  |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 440  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 442  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 443  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：decker.ns.cloudflare.com (3次),
kyree.ns.cloudflare.com (3次), dylan.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 365  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 137  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 230  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 275  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 322  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 418  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 28   | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 81   | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 392  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 283  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 420  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 107  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 245  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 318  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 400  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 423  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 45   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 132  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 209  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 250  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 300  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 378  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 390  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 113  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 204  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 287  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 351  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 435  | freeyx.cloudflare88.eu.org            | 141.101.120.244 | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 438  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 15   | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 44   | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 117  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 178  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 355  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 389  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 411  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 179  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 219  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 401  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 408  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 409  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 206  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 229  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 282  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 356  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 373  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 377  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 419  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 293  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 290  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 253  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 295  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 414  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 405  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 36   | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 116  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 167  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 246  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 380  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 14   | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 228  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 352  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 3    | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 49   | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 51   | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 56   | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 99   | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 251  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 127  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 223  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 271  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 338  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 74   | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 82   | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 141  | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 428  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 23   | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 24   | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 66   | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 152  | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 183  | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 47   | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 135  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 202  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 210  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 218  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 312  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 335  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 336  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 19   | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 29   | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 80   | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 195  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 236  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 262  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 376  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 84   | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 190  | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 203  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 205  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 59 条记录
- **快 (50-100ms)**: 41 条记录
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
