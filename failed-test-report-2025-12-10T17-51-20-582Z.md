# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 17:51:20
- **数据来源**: connectivity_results-20251210-175120.json
- **总测试数**: 489
- **失败测试数**: 182
- **成功测试数**: 307
- **失败率**: 37.22%
- **平均延迟**: 197.90ms
- **最小延迟**: 84ms
- **最大延迟**: 990ms

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 171 次 (94.0%)
- **连接超时: I/O超时**: 9 次 (4.9%)
- **DNS解析错误: 其他DNS错误**: 1 次 (0.5%)
- **连接超时: 上下文超时**: 1 次 (0.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (171 次测试)

| 序号 | 主机/域名                             | 目标IP                             | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                           |
| ---- | ------------------------------------- | ---------------------------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------------------------------------------------- |
| 1    | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable          |
| 10   | comicabc.com                          | 2606:4700:3030::ac43:ae15          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable          |
| 11   | comicabc.com                          | 2606:4700:3036::6815:400a          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable          |
| 14   | icook.tw                              | 2606:4700:10::ac42:9e73            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable            |
| 15   | icook.tw                              | 2606:4700:10::6814:1c4a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable            |
| 18   | fbi.gov                               | 2606:4700::6810:94f4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable               |
| 19   | fbi.gov                               | 2606:4700::6810:95f4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable               |
| 22   | palera.in                             | 2606:4700:3035::6815:3a48          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable          |
| 23   | palera.in                             | 2606:4700:3032::ac43:9d7a          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable          |
| 28   | singapore.com                         | 2606:4700:20::ac43:4bc2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable            |
| 29   | singapore.com                         | 2606:4700:20::681a:d8c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable             |
| 30   | singapore.com                         | 2606:4700:20::681a:c8c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable             |
| 36   | toy-people.com                        | 2606:4700:20::ac43:4812            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable            |
| 37   | toy-people.com                        | 2606:4700:20::681a:324             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable             |
| 38   | toy-people.com                        | 2606:4700:20::681a:224             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable             |
| 39   | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable          |
| 41   | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable          |
| 42   | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable          |
| 46   | japan.com                             | 2606:4700:20::ac43:465c            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable            |
| 47   | japan.com                             | 2606:4700:20::681a:53c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable             |
| 48   | japan.com                             | 2606:4700:20::681a:43c             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable             |
| 51   | ifconfig.co                           | 2606:4700:3037::6815:365b          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable          |
| 52   | ifconfig.co                           | 2606:4700:3030::ac43:a86a          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable          |
| 55   | www.udemy.com                         | 2606:4700::6810:8fed               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable               |
| 56   | www.udemy.com                         | 2606:4700::6810:8eed               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable               |
| 59   | steamdb.info                          | 2606:4700:10::ac42:affa            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable            |
| 60   | steamdb.info                          | 2606:4700:10::6814:22d4            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable            |
| 67   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable          |
| 68   | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable          |
| 71   | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable           |
| 75   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable           |
| 80   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable          |
| 81   | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3030::6815:e29           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable           |
| 85   | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable            |
| 86   | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable            |
| 87   | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable            |
| 88   | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable             |
| 92   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable          |
| 93   | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable          |
| 106  | iplocation.io                         | 2606:4700:20::681a:ade             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable             |
| 107  | iplocation.io                         | 2606:4700:20::681a:bde             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable             |
| 108  | iplocation.io                         | 2606:4700:20::ac43:4664            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable            |
| 111  | whatismyipaddress.com                 | 2606:4700::6813:df4f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable               |
| 112  | whatismyipaddress.com                 | 2606:4700::6813:de4f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable               |
| 116  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable            |
| 117  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable            |
| 118  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable            |
| 127  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable            |
| 128  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable            |
| 129  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable            |
| 130  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable          |
| 134  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable            |
| 135  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable            |
| 136  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable            |
| 139  | www.wto.org                           | 2a06:98c1:3102::6812:29be          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable          |
| 140  | www.wto.org                           | 2606:4700:4406::ac40:9242          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable          |
| 145  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable            |
| 146  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable            |
| 147  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable            |
| 150  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable          |
| 151  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable          |
| 155  | www.hugedomains.com                   | 2606:4700:20::681a:725             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable             |
| 156  | www.hugedomains.com                   | 2606:4700:20::681a:625             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable             |
| 157  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable            |
| 158  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable          |
| 163  | cf.zhetengsha.eu.org                  | 2606:4700:310c::ac42:2c4d          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable          |
| 164  | cf.zhetengsha.eu.org                  | 2606:4700:310c::ac42:2fb3          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable          |
| 169  | icook.hk                              | 2606:4700:3037::ac43:a168          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable          |
| 170  | icook.hk                              | 2606:4700:3031::6815:5ad2          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable          |
| 174  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable            |
| 175  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable            |
| 176  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable            |
| 179  | ip.gs                                 | 2606:4700:3035::ac43:a01c          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable          |
| 180  | ip.gs                                 | 2606:4700:3036::6815:eb0           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable           |
| 186  | tasteatlas.com                        | 2606:4700::6811:2569               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable               |
| 187  | tasteatlas.com                        | 2606:4700::6811:2469               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable               |
| 191  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable            |
| 192  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable            |
| 193  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable            |
| 198  | dnschecker.org                        | 2606:4700:20::681a:659             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable             |
| 199  | dnschecker.org                        | 2606:4700:20::ac43:49d8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable            |
| 200  | dnschecker.org                        | 2606:4700:20::681a:759             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable             |
| 205  | bestcf.030101.xyz                     | 2606:4700:0:c4:5fdb:4ee0:e0b:b5a7  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:c4:5fdb:4ee0:e0b:b5a7]:443: connect: network is unreachable  |
| 206  | bestcf.030101.xyz                     | 2606:4700::a39f:3268:cfbf:c34d     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::a39f:3268:cfbf:c34d]:443: connect: network is unreachable     |
| 210  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable            |
| 211  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable            |
| 212  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable            |
| 216  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable            |
| 217  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable            |
| 218  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable            |
| 219  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable          |
| 228  | www.whatismyip.com                    | 2606:4700:20::681a:d17             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable             |
| 229  | www.whatismyip.com                    | 2606:4700:20::ac43:4581            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable            |
| 230  | www.whatismyip.com                    | 2606:4700:20::681a:c17             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable             |
| 234  | zread.ai                              | 2606:4700:3033::6815:4cf0          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable          |
| 235  | zread.ai                              | 2606:4700:3032::ac43:ca4e          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable          |
| 257  | ip.sb                                 | 2606:4700:20::681a:c1f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable             |
| 258  | ip.sb                                 | 2606:4700:20::ac43:4bac            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable            |
| 259  | ip.sb                                 | 2606:4700:20::681a:d1f             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable             |
| 264  | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable            |
| 265  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable            |
| 266  | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable            |
| 272  | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable            |
| 273  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable            |
| 274  | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable            |
| 278  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable            |
| 279  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable            |
| 280  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable            |
| 286  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable            |
| 287  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable            |
| 288  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable            |
| 293  | yx-auto.pages.dev                     | 2606:4700:3033::ac43:a162          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:a162]:443: connect: network is unreachable          |
| 294  | yx-auto.pages.dev                     | 2606:4700:3034::6815:9e6           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3034::6815:9e6]:443: connect: network is unreachable           |
| 298  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable            |
| 299  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable            |
| 300  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable            |
| 304  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable          |
| 305  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable          |
| 310  | time.is                               | 2606:4700:20::ac43:449d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable            |
| 311  | time.is                               | 2606:4700:20::681a:d36             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable             |
| 312  | time.is                               | 2606:4700:20::681a:c36             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable             |
| 325  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable            |
| 326  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable            |
| 327  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable            |
| 331  | www.pcmag.com                         | 2606:4700::6810:1476               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable               |
| 332  | www.pcmag.com                         | 2606:4700::6810:1576               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable               |
| 336  | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable            |
| 337  | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable            |
| 338  | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable            |
| 352  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable          |
| 353  | www.ipget.net                         | 2606:4700:3036::6815:fd4           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable           |
| 357  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable            |
| 358  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable            |
| 359  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable            |
| 368  | cf.090227.xyz                         | 2606:4700:310c::ac42:2c4d          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c4d]:443: connect: network is unreachable          |
| 369  | cf.090227.xyz                         | 2606:4700:310c::ac42:2fb3          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2fb3]:443: connect: network is unreachable          |
| 372  | yx-auto.pages.dev                     | 2606:4700:3031::ac43:868b          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:868b]:443: connect: network is unreachable          |
| 373  | yx-auto.pages.dev                     | 2606:4700:3030::6815:63c           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:63c]:443: connect: network is unreachable           |
| 377  | cf.0sm.com                            | 2606:4700:3032::6815:785           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable           |
| 378  | cf.0sm.com                            | 2606:4700:3037::ac43:bb91          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable          |
| 384  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3031::ac43:9db6          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable          |
| 385  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::6815:e29]:443: connect: network is unreachable           |
| 388  | www.digitalocean.com                  | 2606:4700::6813:ad44               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable               |
| 389  | www.digitalocean.com                  | 2606:4700::6813:ae44               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable               |
| 393  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable            |
| 394  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable            |
| 395  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable            |
| 399  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable            |
| 400  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable            |
| 401  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable            |
| 406  | ipinfo.in                             | 2606:4700:3031::6815:1581          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable          |
| 407  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable          |
| 415  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:fb:e00f:f254:e667 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:0:fb:e00f:f254:e667]:443: connect: network is unreachable |
| 418  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable          |
| 419  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1           | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable           |
| 423  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable            |
| 424  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable            |
| 425  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable            |
| 429  | www.gov.ua                            | 2606:4700:3031::6815:1748          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable          |
| 430  | www.gov.ua                            | 2606:4700:3033::ac43:d17f          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable          |
| 435  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable          |
| 436  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable          |
| 440  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable            |
| 441  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable            |
| 442  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable            |
| 450  | silkbook.com                          | 2606:4700:20::681a:8a0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable             |
| 451  | silkbook.com                          | 2606:4700:20::681a:9a0             | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable             |
| 452  | silkbook.com                          | 2606:4700:20::ac43:4bd0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable            |
| 456  | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable            |
| 457  | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable            |
| 458  | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable            |

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 220  | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 314  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 319  | 115.22.115.218   | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 346  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 348  | 52.76.110.129    | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 402  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 408  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 431  | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 489  | 172.67.49.134    | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |

#### DNS解析错误: 其他DNS错误 (1 次测试)

| 序号 | 主机/域名       | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                       |
| ---- | --------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------------------------------------------------------------- |
| 5    | 168.138.184.172 | 168.138.184.172 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": remote error: tls: unrecognized name |

#### 连接超时: 上下文超时 (1 次测试)

| 序号 | 主机/域名      | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                            |
| ---- | -------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------- |
| 349  | 20.247.137.183 | 20.247.137.183 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 171 次 (94.0%)
- **连接超时**: 10 次 (5.5%)
- **DNS解析错误**: 1 次 (0.5%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 121.188（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 180 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 11 次，IPv6失败 171 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：singapore.com (3次), toy-people.com
(3次), japan.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 411 |
freeyx.cloudflare88.eu.org | 141.101.121.255 | IPv4 | h3 | ✅ 成功 | 84 |
cloudflare | | 246 | www.okcupid.com | 104.16.239.254 | IPv4 | h3 | ✅ 成功 | 86
| cloudflare | | 253 | www.glassdoor.com | 104.16.25.46 | IPv4 | h3 | ✅ 成功 |
94 | cloudflare | | 485 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h3 | ✅ 成功
| 94 | cloudflare | | 328 | 172.67.75.172 | 172.67.75.172 | IPv4 | h3 | ✅ 成功
| 96 | cloudflare | | 283 | otto.ns.cloudflare.com | 108.162.195.135 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 469 | cmcc.877774.xyz | 104.16.149.7 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 483 | cmcc.877774.xyz | 104.16.148.8 | IPv4 | h3
| ✅ 成功 | 97 | cloudflare | | 486 | cmcc.877774.xyz | 104.16.148.11 | IPv4 |
h3 | ✅ 成功 | 97 | cloudflare | | 417 | stock.hostmonit.com | 172.67.187.251 |
IPv4 | h3 | ✅ 成功 | 100 | cloudflare | | 165 | www.csgo.com | 195.85.59.161 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 166 | www.csgo.com | 195.85.59.95 |
IPv4 | h3 | ✅ 成功 | 101 | cloudflare | | 396 | braden.ns.cloudflare.com |
108.162.195.169 | IPv4 | h3 | ✅ 成功 | 104 | cloudflare | | 247 |
www.okcupid.com | 104.18.160.63 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | | 317
| www.4chan.org | 104.16.229.229 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare | |
365 | asia.877774.xyz | 104.17.139.62 | IPv4 | h3 | ✅ 成功 | 106 | cloudflare |
| 420 | benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h3 | ✅ 成功 | 109
| cloudflare | | 61 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 110 |
cloudflare | | 188 | kyree.ns.cloudflare.com | 108.162.195.207 | IPv4 | h3 | ✅
成功 | 110 | cloudflare | | 462 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h3 |
✅ 成功 | 110 | cloudflare | | 58 | steamdb.info | 172.66.175.250 | IPv4 | h3 |
✅ 成功 | 111 | cloudflare | | 70 | 104.26.13.31 | 104.26.13.31 | IPv4 | h3 | ✅
成功 | 112 | cloudflare | | 177 | ip.gs | 172.67.160.28 | IPv4 | h3 | ✅ 成功 |
113 | cloudflare | | 241 | cu.877774.xyz | 104.26.4.114 | IPv4 | h3 | ✅ 成功 |
114 | cloudflare | | 427 | www.gov.ua | 104.21.23.72 | IPv4 | h3 | ✅ 成功 | 114
| cloudflare | | 333 | wilson.ns.cloudflare.com | 108.162.195.110 | IPv4 | h3 |
✅ 成功 | 116 | cloudflare | | 40 | 162.159.39.118 | 162.159.39.118 | IPv4 | h3
| ✅ 成功 | 117 | cloudflare | | 361 | eur.877774.xyz | 104.21.47.209 | IPv4 |
h3 | ✅ 成功 | 118 | cloudflare | | 387 | www.digitalocean.com | 104.19.173.68 |
IPv4 | h3 | ✅ 成功 | 118 | cloudflare | | 367 | cf.090227.xyz | 172.66.47.179 |
IPv4 | h3 | ✅ 成功 | 119 | cloudflare | | 16 | fbi.gov | 104.16.148.244 | IPv4
| h3 | ✅ 成功 | 120 | cloudflare | | 249 | www.okcupid.com | 104.16.223.254 |
IPv4 | h3 | ✅ 成功 | 120 | cloudflare | | 356 | pranab.ns.cloudflare.com |
172.64.35.199 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 379 | shopify.com |
23.227.38.33 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 432 | 172.64.154.18 |
172.64.154.18 | IPv4 | h3 | ✅ 成功 | 121 | cloudflare | | 84 |
cris.ns.cloudflare.com | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 122 | cloudflare
| | 144 | julio.ns.cloudflare.com | 172.64.35.209 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 239 | cu.877774.xyz | 104.26.4.112 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 289 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 122 |
cloudflare | | 79 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182 |
IPv4 | h3 | ✅ 成功 | 123 | cloudflare | | 428 | www.gov.ua | 172.67.209.127 |
IPv4 | h3 | ✅ 成功 | 123 | cloudflare | | 171 | uriah.ns.cloudflare.com |
108.162.195.194 | IPv4 | h3 | ✅ 成功 | 124 | cloudflare | | 270 |
craig.ns.cloudflare.com | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 124 |
cloudflare | | 376 | cf.0sm.com | 172.67.187.145 | IPv4 | h3 | ✅ 成功 | 124 |
cloudflare | | 467 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h3 | ✅ 成功 | 124
| cloudflare | | 90 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h3 | ✅
成功 | 125 | cloudflare | | 371 | yx-auto.pages.dev | 172.67.134.139 | IPv4 | h3
| ✅ 成功 | 125 | cloudflare | | 91 | cloudflare-ip.mofashi.ltd | 172.67.155.172
| IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 102 | ct.877774.xyz |
172.64.229.161 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 403 | 172.64.146.16
| 172.64.146.16 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare | | 12 | icook.tw |
172.66.158.115 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare | | 225 |
www.whatismyip.com | 104.26.13.23 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare | |
231 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 128 | cloudflare |
| 263 | huxley.ns.cloudflare.com | 172.64.35.188 | IPv4 | h3 | ✅ 成功 | 128 |
cloudflare | | 262 | huxley.ns.cloudflare.com | 162.159.44.188 | IPv4 | h3 | ✅
成功 | 130 | cloudflare | | 447 | silkbook.com | 104.26.8.160 | IPv4 | h3 | ✅
成功 | 130 | cloudflare | | 21 | palera.in | 172.67.157.122 | IPv4 | h3 | ✅
成功 | 131 | cloudflare | | 24 | 104.18.14.76 | 104.18.14.76 | IPv4 | h3 | ✅
成功 | 131 | cloudflare | | 77 | 104.16.45.84 | 104.16.45.84 | IPv4 | h3 | ✅
成功 | 131 | cloudflare | | 195 | dnschecker.org | 104.26.6.89 | IPv4 | h3 | ✅
成功 | 131 | cloudflare | | 443 | gamer.com.tw | 104.18.2.197 | IPv4 | h3 | ✅
成功 | 131 | cloudflare | | 433 | xn--b6gac.eu.org | 172.67.153.253 | IPv4 | h3
| ✅ 成功 | 132 | cloudflare | | 69 | cfip.1323123.xyz | 104.16.133.220 | IPv4 |
h3 | ✅ 成功 | 133 | cloudflare | | 256 | ip.sb | 104.26.13.31 | IPv4 | h3 | ✅
成功 | 133 | cloudflare | | 437 | bowen.ns.cloudflare.com | 108.162.195.83 |
IPv4 | h3 | ✅ 成功 | 133 | cloudflare | | 453 | trevor.ns.cloudflare.com |
108.162.195.154 | IPv4 | h3 | ✅ 成功 | 133 | cloudflare | | 25 | singapore.com
| 172.67.75.194 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare | | 202 | www.visa.cn |
162.159.153.2 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare | | 366 | cf.090227.xyz |
172.66.44.77 | IPv4 | h3 | ✅ 成功 | 134 | cloudflare | | 391 |
sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h3 | ✅ 成功 | 134 |
cloudflare | | 95 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3 | ✅ 成功 | 135 |
cloudflare | | 101 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 135 |
cloudflare | | 351 | www.ipget.net | 104.21.15.212 | IPv4 | h3 | ✅ 成功 | 135 |
cloudflare | | 347 | cfip.xxxxxxxx.tk | 104.16.232.223 | IPv4 | h3 | ✅ 成功 |
136 | cloudflare | | 465 | cmcc.877774.xyz | 104.16.149.3 | IPv4 | h3 | ✅ 成功
| 136 | cloudflare | | 152 | www.hugedomains.com | 104.26.6.37 | IPv4 | h3 | ✅
成功 | 137 | cloudflare | | 360 | eur.877774.xyz | 104.21.29.164 | IPv4 | h3 |
✅ 成功 | 137 | cloudflare | | 421 | benedict.ns.cloudflare.com | 162.159.44.205
| IPv4 | h3 | ✅ 成功 | 137 | cloudflare | | 455 | trevor.ns.cloudflare.com |
172.64.35.154 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare | | 470 | cmcc.877774.xyz
| 104.16.149.8 | IPv4 | h3 | ✅ 成功 | 137 | cloudflare | | 162 |
cf.zhetengsha.eu.org | 172.66.47.179 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare |
| 250 | www.okcupid.com | 104.16.144.63 | IPv4 | h3 | ✅ 成功 | 138 | cloudflare
| | 362 | eur.877774.xyz | 104.21.26.150 | IPv4 | h3 | ✅ 成功 | 138 |
cloudflare | | 382 | ae8a9c24-83de.masx200.ddns-ip.net | 104.21.14.41 | IPv4 |
h3 | ✅ 成功 | 138 | cloudflare | | 53 | www.udemy.com | 104.16.143.237 | IPv4 |
h3 | ✅ 成功 | 139 | cloudflare | | 78 | local-aria2-webui.masx200.ddns-ip.net |
104.21.14.41 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare | | 138 | www.wto.org |
104.18.41.190 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare | | 233 | zread.ai |
104.21.76.240 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare | | 364 | asia.877774.xyz
| 104.16.211.153 | IPv4 | h3 | ✅ 成功 | 139 | cloudflare | | 124 |
dylan.ns.cloudflare.com | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 140 |
cloudflare | | 237 | cu.877774.xyz | 104.26.4.119 | IPv4 | h3 | ✅ 成功 | 140 |
cloudflare | | 482 | cmcc.877774.xyz | 104.16.148.7 | IPv4 | h3 | ✅ 成功 | 140
| cloudflare | | 34 | toy-people.com | 104.26.3.36 | IPv4 | h3 | ✅ 成功 | 141 |
cloudflare | | 49 | ifconfig.co | 172.67.168.106 | IPv4 | h3 | ✅ 成功 | 141 |
cloudflare | | 224 | ipv4.ip.sb | 172.67.75.172 | IPv4 | h3 | ✅ 成功 | 141 |
cloudflare | | 343 | cfip.xxxxxxxx.tk | 190.93.244.201 | IPv4 | h3 | ✅ 成功 |
141 | cloudflare | | 110 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h3 |
✅ 成功 | 142 | cloudflare | | 143 | julio.ns.cloudflare.com | 162.159.44.209 |
IPv4 | h3 | ✅ 成功 | 142 | cloudflare | | 160 | 162.159.36.104 | 162.159.36.104
| IPv4 | h3 | ✅ 成功 | 144 | cloudflare | | 340 | 172.64.148.15 | 172.64.148.15
| IPv4 | h3 | ✅ 成功 | 144 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 9 条记录
- **正常 (100-200ms)**: 91 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 11 次
- **IPv6 失败**: 171 次

### 按协议统计

- **none**: 180 次失败
- **h2**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
