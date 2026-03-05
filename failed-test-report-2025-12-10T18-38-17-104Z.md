# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:38:17
- **数据来源**: connectivity_results-20251210-183817.json
- **总测试数**: 485
- **失败测试数**: 181
- **成功测试数**: 304
- **失败率**: 37.32%
- **平均延迟**: 147.19ms
- **最小延迟**: 60ms
- **最大延迟**: 677ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 172 次 (95.0%)
- **连接超时: I/O超时**: 9 次 (5.0%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (172 次测试)

| 序号 | 主机/域名                             | 目标IP                                | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                              |
| ---- | ------------------------------------- | ------------------------------------- | ------ | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------- |
| 1    | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable             |
| 2    | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable             |
| 3    | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable             |
| 6    | www.pcmag.com                         | 2606:4700::6810:1476                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                  |
| 7    | www.pcmag.com                         | 2606:4700::6810:1576                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                  |
| 10   | www.wto.org                           | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 11   | www.wto.org                           | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 19   | silkbook.com                          | 2606:4700:20::681a:8a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                |
| 20   | silkbook.com                          | 2606:4700:20::ac43:4bd0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable               |
| 21   | silkbook.com                          | 2606:4700:20::681a:9a0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                |
| 26   | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable               |
| 27   | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable               |
| 28   | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable               |
| 32   | dnschecker.org                        | 2606:4700:20::681a:659                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                |
| 33   | dnschecker.org                        | 2606:4700:20::681a:759                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                |
| 34   | dnschecker.org                        | 2606:4700:20::ac43:49d8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable               |
| 36   | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable             |
| 42   | ip.gs                                 | 2606:4700:3035::ac43:a01c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable             |
| 43   | ip.gs                                 | 2606:4700:3036::6815:eb0              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable              |
| 51   | tasteatlas.com                        | 2606:4700::6811:2469                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                  |
| 52   | tasteatlas.com                        | 2606:4700::6811:2569                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                  |
| 55   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable             |
| 56   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable             |
| 62   | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable              |
| 65   | www.udemy.com                         | 2606:4700::6810:8eed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                  |
| 66   | www.udemy.com                         | 2606:4700::6810:8fed                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                  |
| 70   | zread.ai                              | 2606:4700:3032::ac43:ca4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable             |
| 71   | zread.ai                              | 2606:4700:3033::6815:4cf0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable             |
| 75   | freeyx.cloudflare88.eu.org            | 2606:4700:3009:0:72:9d2c:ac0d:3727    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:0:72:9d2c:ac0d:3727]:443: connect: network is unreachable    |
| 76   | freeyx.cloudflare88.eu.org            | 2606:4700:3010:bf:5dba:fa1d:5993:9cf8 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:bf:5dba:fa1d:5993:9cf8]:443: connect: network is unreachable |
| 79   | cf.090227.xyz                         | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 80   | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |
| 83   | ipinfo.in                             | 2606:4700:3031::6815:1581             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable             |
| 84   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable             |
| 87   | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable             |
| 88   | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable              |
| 91   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable             |
| 92   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable             |
| 105  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable             |
| 106  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable             |
| 109  | steamdb.info                          | 2606:4700:10::6814:22d4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable               |
| 110  | steamdb.info                          | 2606:4700:10::ac42:affa               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable               |
| 117  | iplocation.io                         | 2606:4700:20::681a:ade                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                |
| 118  | iplocation.io                         | 2606:4700:20::681a:bde                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                |
| 119  | iplocation.io                         | 2606:4700:20::ac43:4664               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable               |
| 122  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 123  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 125  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable              |
| 133  | cf.0sm.com                            | 2606:4700:3032::6815:785              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable              |
| 134  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable             |
| 138  | ifconfig.co                           | 2606:4700:3037::6815:365b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable             |
| 139  | ifconfig.co                           | 2606:4700:3030::ac43:a86a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable             |
| 144  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable               |
| 145  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable               |
| 146  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable               |
| 152  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable               |
| 153  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable               |
| 154  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable               |
| 155  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable             |
| 159  | www.hugedomains.com                   | 2606:4700:20::681a:725                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                |
| 160  | www.hugedomains.com                   | 2606:4700:20::681a:625                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                |
| 161  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable               |
| 164  | yx-auto.pages.dev                     | 2606:4700:3030::6815:63c              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable              |
| 165  | yx-auto.pages.dev                     | 2606:4700:3031::ac43:868b             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable             |
| 171  | www.whatismyip.com                    | 2606:4700:20::681a:d17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                |
| 172  | www.whatismyip.com                    | 2606:4700:20::681a:c17                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                |
| 173  | www.whatismyip.com                    | 2606:4700:20::ac43:4581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable               |
| 177  | comicabc.com                          | 2606:4700:3030::ac43:ae15             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable             |
| 178  | comicabc.com                          | 2606:4700:3036::6815:400a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable             |
| 181  | www.digitalocean.com                  | 2606:4700::6813:ae44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                  |
| 182  | www.digitalocean.com                  | 2606:4700::6813:ad44                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                  |
| 187  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable             |
| 188  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable             |
| 192  | japan.com                             | 2606:4700:20::ac43:465c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable               |
| 193  | japan.com                             | 2606:4700:20::681a:53c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                |
| 194  | japan.com                             | 2606:4700:20::681a:43c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                |
| 199  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable               |
| 200  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable               |
| 201  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable               |
| 202  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                |
| 207  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3030::6815:e29              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable              |
| 208  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable             |
| 211  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable             |
| 212  | www.ipget.net                         | 2606:4700:3036::6815:fd4              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable              |
| 215  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable             |
| 216  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable             |
| 249  | ip.sb                                 | 2606:4700:20::ac43:4bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable               |
| 250  | ip.sb                                 | 2606:4700:20::681a:d1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                |
| 251  | ip.sb                                 | 2606:4700:20::681a:c1f                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                |
| 264  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable               |
| 265  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable               |
| 266  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable               |
| 271  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable               |
| 272  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable               |
| 273  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable               |
| 276  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable             |
| 277  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable             |
| 281  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable               |
| 282  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable               |
| 283  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable               |
| 291  | time.is                               | 2606:4700:20::ac43:449d               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable               |
| 292  | time.is                               | 2606:4700:20::681a:c36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                |
| 293  | time.is                               | 2606:4700:20::681a:d36                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                |
| 295  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable             |
| 298  | fbi.gov                               | 2606:4700::6810:94f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                  |
| 299  | fbi.gov                               | 2606:4700::6810:95f4                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                  |
| 307  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable               |
| 308  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable               |
| 309  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable               |
| 312  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable             |
| 316  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable               |
| 317  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable               |
| 318  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable               |
| 322  | www.gov.ua                            | 2606:4700:3031::6815:1748             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable             |
| 323  | www.gov.ua                            | 2606:4700:3033::ac43:d17f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable             |
| 330  | singapore.com                         | 2606:4700:20::681a:c8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                |
| 331  | singapore.com                         | 2606:4700:20::ac43:4bc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable               |
| 332  | singapore.com                         | 2606:4700:20::681a:d8c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                |
| 339  | toy-people.com                        | 2606:4700:20::681a:224                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                |
| 340  | toy-people.com                        | 2606:4700:20::681a:324                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                |
| 341  | toy-people.com                        | 2606:4700:20::ac43:4812               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable               |
| 345  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable               |
| 346  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable               |
| 347  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable               |
| 352  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable               |
| 353  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable               |
| 354  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable               |
| 364  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable               |
| 365  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable               |
| 366  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable               |
| 372  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable               |
| 373  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable               |
| 374  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable               |
| 381  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable               |
| 382  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable               |
| 383  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable               |
| 387  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable               |
| 388  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable               |
| 389  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable               |
| 396  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable               |
| 397  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable               |
| 398  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable               |
| 401  | icook.hk                              | 2606:4700:3031::6815:5ad2             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable             |
| 402  | icook.hk                              | 2606:4700:3037::ac43:a168             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable             |
| 406  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable               |
| 407  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable               |
| 408  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable               |
| 414  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable               |
| 415  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable               |
| 416  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable               |
| 421  | palera.in                             | 2606:4700:3035::6815:3a48             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable             |
| 422  | palera.in                             | 2606:4700:3032::ac43:9d7a             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable             |
| 426  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable               |
| 427  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable               |
| 428  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable               |
| 437  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable               |
| 438  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable               |
| 439  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable               |
| 443  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable               |
| 444  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable               |
| 445  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable               |
| 448  | yx-auto.pages.dev                     | 2606:4700:3033::ac43:a162             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable             |
| 449  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable              |
| 452  | whatismyipaddress.com                 | 2606:4700::6813:de4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                  |
| 453  | whatismyipaddress.com                 | 2606:4700::6813:df4f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                  |
| 456  | icook.tw                              | 2606:4700:10::ac42:9e73               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable               |
| 457  | icook.tw                              | 2606:4700:10::6814:1c4a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable               |
| 463  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable               |
| 464  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable               |
| 465  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable               |
| 471  | bestcf.030101.xyz                     | 2606:4700:0:c55a:62a8:85ff:cd2c:c647  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c55a:62a8:85ff:cd2c:c647]:443: connect: network is unreachable  |
| 472  | bestcf.030101.xyz                     | 2606:4700:0:cd:9b2d:c8ba:6717:5f84    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:cd:9b2d:c8ba:6717:5f84]:443: connect: network is unreachable    |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 377  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 409  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 417  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 477  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 481  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 482  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 483  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 484  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 485  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 172 次 (95.0%)
- **连接超时**: 9 次 (5.0%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 115.22（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 181 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 9 次，IPv6失败 172 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：silkbook.com (3次),
ashton.ns.cloudflare.com (3次), dnschecker.org
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 176 |
comicabc.com | 104.21.64.10 | IPv4 | h3 | ✅ 成功 | 60 | cloudflare | | 170 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 64 | cloudflare | |
147 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 65 | cloudflare | | 35
| 172.67.181.209 | 172.67.181.209 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | |
296 | fbi.gov | 104.16.148.244 | IPv4 | h3 | ✅ 成功 | 67 | cloudflare | | 297 |
fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 69 | cloudflare | | 344 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 70 | cloudflare
| | 319 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 73 | cloudflare
| | 425 | cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 76 |
cloudflare | | 454 | icook.tw | 104.20.28.74 | IPv4 | h3 | ✅ 成功 | 78 |
cloudflare | | 460 | damien.ns.cloudflare.com | 108.162.195.168 | IPv4 | h3 | ✅
成功 | 78 | cloudflare | | 474 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅
成功 | 82 | cloudflare | | 478 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 |
✅ 成功 | 82 | cloudflare | | 156 | www.hugedomains.com | 104.26.7.37 | IPv4 |
h3 | ✅ 成功 | 83 | cloudflare | | 446 | yx-auto.pages.dev | 104.21.9.230 | IPv4
| h3 | ✅ 成功 | 83 | cloudflare | | 137 | ifconfig.co | 104.21.54.91 | IPv4 |
h3 | ✅ 成功 | 86 | cloudflare | | 442 | trevor.ns.cloudflare.com |
172.64.35.154 | IPv4 | h3 | ✅ 成功 | 86 | cloudflare | | 334 | 172.67.75.172 |
172.67.75.172 | IPv4 | h3 | ✅ 成功 | 87 | cloudflare | | 244 | www.csgo.com |
195.85.59.161 | IPv4 | h3 | ✅ 成功 | 88 | cloudflare | | 77 | cf.090227.xyz |
104.18.43.174 | IPv4 | h3 | ✅ 成功 | 94 | cloudflare | | 361 |
uriah.ns.cloudflare.com | 108.162.195.194 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 22 | 104.26.6.112 | 104.26.6.112 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 359 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 | ✅ 成功 | 97 |
cloudflare | | 116 | iplocation.io | 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 100 |
cloudflare | | 278 | abdullah.ns.cloudflare.com | 108.162.195.203 | IPv4 | h3 |
✅ 成功 | 101 | cloudflare | | 310 | 104.16.223.179 | 104.16.223.179 | IPv4 | h3
| ✅ 成功 | 101 | cloudflare | | 367 | www.glassdoor.com | 104.16.25.46 | IPv4 |
h3 | ✅ 成功 | 101 | cloudflare | | 135 | 162.159.36.104 | 162.159.36.104 | IPv4
| h3 | ✅ 成功 | 102 | cloudflare | | 248 | ip.sb | 172.67.75.172 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 39 | 172.64.148.15 | 172.64.148.15 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 256 | cu.877774.xyz | 104.26.4.115 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 14 | www.ipchicken.com | 104.26.7.112 | IPv4 | h3
| ✅ 成功 | 104 | cloudflare | | 226 | cmcc.877774.xyz | 104.16.148.8 | IPv4 |
h3 | ✅ 成功 | 104 | cloudflare | | 94 | ct.877774.xyz | 172.64.229.44 | IPv4 |
h3 | ✅ 成功 | 105 | cloudflare | | 213 | cf.877771.xyz | 172.67.152.183 | IPv4
| h3 | ✅ 成功 | 105 | cloudflare | | 433 | www.okcupid.com | 104.16.144.63 |
IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 85 | stock.hostmonit.com |
104.21.7.193 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 289 | time.is |
104.26.13.54 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 114 | iplocation.io |
104.26.10.222 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 203 | 104.16.45.84 |
104.16.45.84 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 294 | 172.64.49.165 |
172.64.49.165 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 306 |
sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3 | ✅ 成功 | 107 |
cloudflare | | 351 | decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 107 | cloudflare | | 356 | 172.64.159.6 | 172.64.159.6 | IPv4 | h3 | ✅
成功 | 107 | cloudflare | | 5 | www.pcmag.com | 104.16.20.118 | IPv4 | h3 | ✅
成功 | 108 | cloudflare | | 61 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 |
✅ 成功 | 108 | cloudflare | | 224 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h3
| ✅ 成功 | 109 | cloudflare | | 399 | icook.hk | 172.67.161.104 | IPv4 | h3 |
✅ 成功 | 109 | cloudflare | | 38 | 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 169 | www.whatismyip.com | 104.26.12.23 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 229 | cmcc.877774.xyz | 104.16.148.11 | IPv4 |
h3 | ✅ 成功 | 110 | cloudflare | | 300 | 172.64.153.172 | 172.64.153.172 | IPv4
| h3 | ✅ 成功 | 110 | cloudflare | | 285 | 108.162.198.54 | 108.162.198.54 |
IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 394 | rustam.ns.cloudflare.com |
162.159.44.148 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 37 |
cfip.1323123.xyz | 104.16.133.220 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | |
214 | cf.877771.xyz | 104.21.80.180 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | |
204 | 104.16.61.163 | 104.16.61.163 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | |
326 | 172.67.79.211 | 172.67.79.211 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | |
58 | www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 195 | 172.64.156.195 | 172.64.156.195 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare
| | 287 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare |
| 419 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 114 | cloudflare | | 47
| 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 268 |
dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 115 |
cloudflare | | 315 | benedict.ns.cloudflare.com | 172.64.35.205 | IPv4 | h3 | ✅
成功 | 115 | cloudflare | | 349 | decker.ns.cloudflare.com | 108.162.195.155 |
IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 441 | trevor.ns.cloudflare.com |
162.159.44.154 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | | 476 |
cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 115 | cloudflare | |
162 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 | 116 |
cloudflare | | 86 | stock.hostmonit.com | 172.67.187.251 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 241 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h3 | ✅ 成功
| 117 | cloudflare | | 245 | www.csgo.com | 195.85.59.95 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 321 | www.gov.ua | 172.67.209.127 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 328 | singapore.com | 104.26.12.140 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 391 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3 | ✅ 成功 |
117 | cloudflare | | 412 | wilson.ns.cloudflare.com | 162.159.44.110 | IPv4 | h3
| ✅ 成功 | 117 | cloudflare | | 12 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3
| ✅ 成功 | 118 | cloudflare | | 15 | www.ipchicken.com | 172.67.68.101 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 59 | asia.877774.xyz | 104.17.142.146 | IPv4
| h3 | ✅ 成功 | 118 | cloudflare | | 102 | 198.62.62.4 | 198.62.62.4 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 121 | ae8a9c24-83de.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 247 | ip.sb |
104.26.13.31 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 258 | cu.877774.xyz |
104.26.4.117 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 355 | 172.64.33.67 |
172.64.33.67 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 369 |
moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h3 | ✅ 成功 | 118 |
cloudflare | | 473 | www.visa.cn | 162.159.153.2 | IPv4 | h3 | ✅ 成功 | 118 |
cloudflare | | 130 | na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 119 |
cloudflare | | 185 | yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 |
119 | cloudflare | | 196 | bowen.ns.cloudflare.com | 108.162.195.83 | IPv4 | h3
| ✅ 成功 | 119 | cloudflare | | 304 | sullivan.ns.cloudflare.com |
108.162.195.161 | IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 413 |
wilson.ns.cloudflare.com | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 119 |
cloudflare | | 40 | ip.gs | 104.21.14.176 | IPv4 | h3 | ✅ 成功 | 120 |
cloudflare | | 239 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 120
| cloudflare | | 242 | cmcc.877774.xyz | 104.16.149.11 | IPv4 | h3 | ✅ 成功 |
120 | cloudflare | | 48 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 128 | na.877774.xyz | 104.18.187.25 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 254 | cu.877774.xyz | 104.26.4.113 | IPv4 | h3 | ✅ 成功 |
121 | cloudflare | | 18 | silkbook.com | 172.67.75.208 | IPv4 | h3 | ✅ 成功 |
122 | cloudflare | | 63 | www.udemy.com | 104.16.143.237 | IPv4 | h3 | ✅ 成功 |
122 | cloudflare | | 104 | cf.zhetengsha.eu.org | 172.64.144.82 | IPv4 | h3 | ✅
成功 | 122 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 23 条记录
- **正常 (100-200ms)**: 77 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 9 次
- **IPv6 失败**: 172 次

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
