# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 04:50:58
- **数据来源**: connectivity_results-20251211-045058.json
- **总测试数**: 484
- **失败测试数**: 175
- **成功测试数**: 309
- **失败率**: 36.16%
- **平均延迟**: 155.32ms
- **最小延迟**: 53ms
- **最大延迟**: 1399ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (97.7%)
- **连接超时: I/O超时**: 4 次 (2.3%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名                             | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | ------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 6    | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 7    | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 10   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 11   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 12   | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 13   | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 16   | bestcf.030101.xyz                     | 2606:4700::66c7:2a0d:9d15:2de9          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::66c7:2a0d:9d15:2de9]:443: connect: network is unreachable          |
| 17   | bestcf.030101.xyz                     | 2606:4700:0:874a:3dd1:a845:1f36:6609    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:874a:3dd1:a845:1f36:6609]:443: connect: network is unreachable    |
| 20   | www.pcmag.com                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 21   | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 30   | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 31   | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 32   | dnschecker.org                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 36   | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 37   | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 38   | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 39   | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 43   | silkbook.com                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 44   | silkbook.com                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 45   | silkbook.com                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 57   | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 58   | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 61   | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 62   | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 63   | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 68   | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 69   | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 72   | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 73   | www.udemy.com                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 77   | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 78   | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 82   | cf.090227.xyz                         | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 83   | cf.090227.xyz                         | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 86   | ipinfo.in                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 87   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 90   | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 91   | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 99   | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 100  | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 114  | freeyx.cloudflare88.eu.org            | 2606:4700:3009:aa59:4b67:cd47:f6f7:aaa7 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cd47:f6f7:aaa7]:443: connect: network is unreachable |
| 117  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 118  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 122  | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 123  | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 124  | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 125  | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 128  | cf.zhetengsha.eu.org                  | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 129  | cf.zhetengsha.eu.org                  | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 133  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 134  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 141  | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 142  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 145  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 146  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 150  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 151  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 152  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 154  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 163  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 164  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 165  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 168  | yx-auto.pages.dev                     | 2606:4700:3030::6815:63c                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable                |
| 169  | yx-auto.pages.dev                     | 2606:4700:3031::ac43:868b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable               |
| 173  | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 174  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 175  | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 179  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 180  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 186  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 187  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 188  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 192  | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 193  | comicabc.com                          | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 194  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 198  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 199  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 200  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 207  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 208  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 209  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 213  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 214  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3030::6815:e29                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable                |
| 220  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 221  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 222  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 226  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 227  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 228  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 234  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 235  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 236  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 249  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 250  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 253  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 254  | www.ipget.net                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 258  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 259  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 260  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 262  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 266  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 267  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 273  | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 274  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 275  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 277  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 280  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 281  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 315  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 316  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 317  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 325  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 326  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 327  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 332  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 333  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 334  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 338  | toy-people.com                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 339  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 340  | toy-people.com                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 343  | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 344  | www.gov.ua                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 350  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 351  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 352  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 356  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 357  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 358  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 366  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 367  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 368  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 372  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 373  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 374  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 379  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 380  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 381  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 393  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 394  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 395  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 399  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 400  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 401  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 405  | icook.hk                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 406  | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 410  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 411  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 412  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 417  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 418  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 419  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 423  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 424  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 425  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 428  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 429  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 438  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 439  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 443  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 444  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 445  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 448  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable                |
| 449  | yx-auto.pages.dev                     | 2606:4700:3033::ac43:a162               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable               |
| 453  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 454  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 455  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 459  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 460  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 464  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 465  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 466  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 470  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 471  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 480  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 481  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (97.7%)
- **连接超时**: 4 次 (2.3%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 175 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：dnschecker.org (3次),
ashton.ns.cloudflare.com (3次), silkbook.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 477 |
cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 | 53 | cloudflare | |
433 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 61 | cloudflare |
| 126 | cf.zhetengsha.eu.org | 104.18.35.15 | IPv4 | h3 | ✅ 成功 | 62 |
cloudflare | | 308 | cmcc.877774.xyz | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 63
| cloudflare | | 212 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 |
IPv4 | h3 | ✅ 成功 | 68 | cloudflare | | 389 | www.visa.com.hk | 104.18.21.69 |
IPv4 | h3 | ✅ 成功 | 73 | cloudflare | | 282 | 172.64.146.16 | 172.64.146.16 |
IPv4 | h3 | ✅ 成功 | 74 | cloudflare | | 446 | yx-auto.pages.dev |
172.67.161.98 | IPv4 | h3 | ✅ 成功 | 80 | cloudflare | | 106 | ct.877774.xyz |
172.64.229.174 | IPv4 | h3 | ✅ 成功 | 81 | cloudflare | | 184 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 84 | cloudflare | |
242 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
345 | 104.17.79.11 | 104.17.79.11 | IPv4 | h3 | ✅ 成功 | 85 | cloudflare | |
457 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 | ✅ 成功 | 87 |
cloudflare | | 265 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3 | ✅ 成功 |
88 | cloudflare | | 279 | fbi.gov | 104.16.149.244 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 467 | www.visa.cn | 162.159.152.2 | IPv4 | h3 | ✅ 成功 | 88 |
cloudflare | | 51 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅ 成功 | 89 |
cloudflare | | 120 | iplocation.io | 104.26.11.222 | IPv4 | h3 | ✅ 成功 | 90 |
cloudflare | | 166 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3 | ✅ 成功 |
92 | cloudflare | | 314 | sullivan.ns.cloudflare.com | 172.64.35.161 | IPv4 | h3
| ✅ 成功 | 92 | cloudflare | | 89 | stock.hostmonit.com | 172.67.187.251 | IPv4
| h3 | ✅ 成功 | 93 | cloudflare | | 172 | www.hugedomains.com | 104.26.7.37 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 283 | cmcc.877774.xyz | 104.16.148.1 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 286 | cmcc.877774.xyz | 104.16.148.4 |
IPv4 | h3 | ✅ 成功 | 93 | cloudflare | | 56 | ip.gs | 172.67.160.28 | IPv4 | h3
| ✅ 成功 | 95 | cloudflare | | 261 | 172.67.120.0 | 172.67.120.0 | IPv4 | h3 |
✅ 成功 | 95 | cloudflare | | 300 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 |
✅ 成功 | 95 | cloudflare | | 309 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h3 | ✅
成功 | 95 | cloudflare | | 347 | craig.ns.cloudflare.com | 108.162.195.192 |
IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 363 | moura.ns.cloudflare.com |
108.162.195.217 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare | | 364 |
moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h3 | ✅ 成功 | 95 | cloudflare
| | 376 | julio.ns.cloudflare.com | 108.162.195.209 | IPv4 | h3 | ✅ 成功 | 95 |
cloudflare | | 162 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 320 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅
成功 | 96 | cloudflare | | 462 | damien.ns.cloudflare.com | 162.159.44.168 |
IPv4 | h3 | ✅ 成功 | 96 | cloudflare | | 84 | ipinfo.in | 104.21.21.129 | IPv4
| h3 | ✅ 成功 | 97 | cloudflare | | 257 | abdullah.ns.cloudflare.com |
172.64.35.203 | IPv4 | h3 | ✅ 成功 | 97 | cloudflare | | 25 | www.ipchicken.com
| 172.67.68.101 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 48 | 172.64.229.249
| 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 121 | iplocation.io
| 172.67.70.100 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 155 | 104.18.14.76 |
104.18.14.76 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 196 | japan.com |
104.26.4.60 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 318 | 172.67.79.211 |
172.67.79.211 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 456 | 104.18.78.214 |
104.18.78.214 | IPv4 | h3 | ✅ 成功 | 98 | cloudflare | | 248 | cf.877771.xyz |
104.21.80.180 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare | | 303 | cmcc.877774.xyz
| 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare | | 349 |
craig.ns.cloudflare.com | 172.64.35.192 | IPv4 | h3 | ✅ 成功 | 99 | cloudflare
| | 76 | zread.ai | 104.21.76.240 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare | |
105 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 307 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare
| | 346 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 100 | cloudflare |
| 96 | 198.62.62.4 | 198.62.62.4 | IPv4 | h3 | ✅ 成功 | 101 | cloudflare | |
147 | lewis.ns.cloudflare.com | 108.162.195.159 | IPv4 | h3 | ✅ 成功 | 101 |
cloudflare | | 295 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 | ✅ 成功 |
101 | cloudflare | | 431 | www.okcupid.com | 104.16.223.254 | IPv4 | h3 | ✅
成功 | 101 | cloudflare | | 93 | asia.877774.xyz | 104.17.142.146 | IPv4 | h3 |
✅ 成功 | 102 | cloudflare | | 115 | cloudflare-ip.mofashi.ltd | 172.67.155.172
| IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 137 | www.4chan.org |
104.16.228.229 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 294 |
cmcc.877774.xyz | 104.16.148.12 | IPv4 | h3 | ✅ 成功 | 102 | cloudflare | | 355
| decker.ns.cloudflare.com | 172.64.35.155 | IPv4 | h3 | ✅ 成功 | 102 |
cloudflare | | 408 | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4 | h3 | ✅
成功 | 102 | cloudflare | | 107 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 103 | cloudflare | | 161 | huxley.ns.cloudflare.com | 162.159.44.188 |
IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 241 | cu.877774.xyz | 104.26.4.111 |
IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 247 | cf.877771.xyz | 172.67.152.183
| IPv4 | h3 | ✅ 成功 | 103 | cloudflare | | 268 | 172.64.49.165 | 172.64.49.165
| IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 71 | www.udemy.com | 104.16.142.237
| IPv4 | h3 | ✅ 成功 | 105 | cloudflare | | 321 | 172.67.110.232 |
172.67.110.232 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 95 | asia.877774.xyz
| 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 284 |
cmcc.877774.xyz | 104.16.148.2 | IPv4 | h3 | ✅ 成功 | 107 | cloudflare | | 8 |
yx-auto.pages.dev | 172.66.47.112 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | |
176 | www.7749tv.com | 172.67.101.77 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 336 | toy-people.com | 172.67.72.18 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare |
| 426 | palera.in | 104.21.58.72 | IPv4 | h3 | ✅ 成功 | 108 | cloudflare | |
441 | cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 108 |
cloudflare | | 127 | cf.zhetengsha.eu.org | 172.64.152.241 | IPv4 | h3 | ✅ 成功
| 109 | cloudflare | | 197 | japan.com | 104.26.5.60 | IPv4 | h3 | ✅ 成功 | 109
| cloudflare | | 298 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 304 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h3 | ✅ 成功
| 109 | cloudflare | | 310 | ipv4.ip.sb | 104.26.13.31 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 335 | toy-people.com | 104.26.2.36 | IPv4 | h3 | ✅ 成功 |
109 | cloudflare | | 292 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功
| 110 | cloudflare | | 319 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功
| 110 | cloudflare | | 392 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4 | h3
| ✅ 成功 | 110 | cloudflare | | 397 | rustam.ns.cloudflare.com | 162.159.44.148
| IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 483 | cfip.xxxxxxxx.tk |
198.41.214.141 | IPv4 | h3 | ✅ 成功 | 110 | cloudflare | | 263 | 104.17.142.12
| 104.17.142.12 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 302 |
cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 403
| icook.hk | 104.21.90.210 | IPv4 | h3 | ✅ 成功 | 111 | cloudflare | | 65 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 103
| ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 111
| saas.sin.fan | 162.159.36.5 | IPv4 | h3 | ✅ 成功 | 112 | cloudflare | | 113 |
freeyx.cloudflare88.eu.org | 141.101.121.127 | IPv4 | h3 | ✅ 成功 | 112 |
cloudflare | | 171 | www.hugedomains.com | 172.67.70.191 | IPv4 | h3 | ✅ 成功 |
112 | cloudflare | | 362 | www.glassdoor.com | 104.17.64.70 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 98 | steamdb.info | 104.20.34.212 | IPv4 | h3 | ✅
成功 | 113 | cloudflare | | 104 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 |
✅ 成功 | 113 | cloudflare | | 132 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 156 | 162.159.44.208
| 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare | | 311 | ipv4.ip.sb |
172.67.75.172 | IPv4 | h3 | ✅ 成功 | 113 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 47 条记录
- **正常 (100-200ms)**: 53 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 175 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
