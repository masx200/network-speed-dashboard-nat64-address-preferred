# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:46:31
- **数据来源**: connectivity_results-20251214-144630.json
- **总测试数**: 443
- **失败测试数**: 167
- **成功测试数**: 276
- **失败率**: 37.70%
- **平均延迟**: 107.38ms
- **最小延迟**: 40ms
- **最大延迟**: 638ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:46:31
- **IP地址**: 20.161.70.177
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
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
| 5    | trevor.ns.cloudflare.com              | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 6    | trevor.ns.cloudflare.com              | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 7    | trevor.ns.cloudflare.com              | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 10   | www.pcmag.com                         | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 11   | www.pcmag.com                         | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 15   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 16   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 17   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 21   | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 22   | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 23   | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 36   | craig.ns.cloudflare.com               | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 37   | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 38   | craig.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 41   | steamdb.info                          | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 42   | steamdb.info                          | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 45   | ipinfo.in                             | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 46   | ipinfo.in                             | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 52   | www.gov.ua                            | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 53   | www.gov.ua                            | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 62   | iplocation.io                         | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 63   | iplocation.io                         | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 64   | iplocation.io                         | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 65   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 70   | cf.0sm.com                            | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 71   | cf.0sm.com                            | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 78   | icook.hk                              | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 79   | icook.hk                              | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 83   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 84   | huxley.ns.cloudflare.com              | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 85   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 88   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 92   | www.hugedomains.com                   | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 93   | www.hugedomains.com                   | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 94   | www.hugedomains.com                   | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 124  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 125  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 126  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 134  | comicabc.com                          | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 135  | comicabc.com                          | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 138  | www.ipget.net                         | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 139  | www.ipget.net                         | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 151  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 152  | cf.877771.xyz                         | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 156  | benedict.ns.cloudflare.com            | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 157  | benedict.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 158  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 164  | time.is                               | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 165  | time.is                               | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 166  | time.is                               | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 174  | rustam.ns.cloudflare.com              | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 175  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 176  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 182  | palera.in                             | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 183  | palera.in                             | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 190  | singapore.com                         | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 191  | singapore.com                         | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 192  | singapore.com                         | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 196  | silkbook.com                          | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 197  | silkbook.com                          | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 198  | silkbook.com                          | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 201  | ip.gs                                 | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 202  | ip.gs                                 | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 204  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 208  | ip.sb                                 | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 209  | ip.sb                                 | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 210  | ip.sb                                 | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 214  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 215  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 216  | ashton.ns.cloudflare.com              | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 220  | dnschecker.org                        | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 221  | dnschecker.org                        | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 222  | dnschecker.org                        | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 223  | [2606:4700:964f::6e2c:588e]           | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 230  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 231  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 232  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 236  | uriah.ns.cloudflare.com               | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 237  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 238  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 239  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 244  | whatismyipaddress.com                 | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 245  | whatismyipaddress.com                 | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 250  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 251  | cf.877774.xyz                         | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 254  | tasteatlas.com                        | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 255  | tasteatlas.com                        | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 259  | www.udemy.com                         | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 260  | www.udemy.com                         | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 267  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 268  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 269  | pranab.ns.cloudflare.com              | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 272  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 273  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 276  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 277  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 281  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 282  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 283  | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 287  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 291  | toy-people.com                        | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 292  | toy-people.com                        | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 293  | toy-people.com                        | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 297  | www.whatismyip.com                    | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 298  | www.whatismyip.com                    | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 299  | www.whatismyip.com                    | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 303  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 304  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 305  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 311  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 312  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 313  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 316  | zread.ai                              | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 317  | zread.ai                              | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 321  | decker.ns.cloudflare.com              | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 322  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 323  | decker.ns.cloudflare.com              | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 327  | lewis.ns.cloudflare.com               | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 328  | lewis.ns.cloudflare.com               | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 329  | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 332  | bestcf.030101.xyz                     | 2606:4700:23:3674:16f8:2b7a:34cb:d251   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:23:3674:16f8:2b7a:34cb:d251]:443: connect: network is unreachable   |
| 333  | bestcf.030101.xyz                     | 2606:4700:23:e182:db7d:103e:afc5:c316   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:23:e182:db7d:103e:afc5:c316]:443: connect: network is unreachable   |
| 334  | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 337  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 338  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 341  | icook.tw                              | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 342  | icook.tw                              | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 343  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 347  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 348  | braden.ns.cloudflare.com              | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 349  | braden.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 360  | cf.zhetengsha.eu.org                  | 2a06:98c1:3101::ac40:919e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101::ac40:919e]:443: connect: network is unreachable               |
| 361  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108::6812:2a62]:443: connect: network is unreachable               |
| 366  | www.digitalocean.com                  | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 367  | www.digitalocean.com                  | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 374  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 375  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 376  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 382  | www.wto.org                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 383  | www.wto.org                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 384  | [2606:4700:83be::11:74f]              | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 388  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 392  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 393  | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 394  | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 398  | japan.com                             | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 399  | japan.com                             | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 400  | japan.com                             | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 405  | cf.090227.xyz                         | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 406  | cf.090227.xyz                         | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 409  | ifconfig.co                           | 2606:4700:3030::ac43:a86a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:a86a]:443: connect: network is unreachable               |
| 410  | ifconfig.co                           | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 411  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 415  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 416  | stock.hostmonit.com                   | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 424  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 425  | abdullah.ns.cloudflare.com            | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 426  | abdullah.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 431  | fbi.gov                               | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 432  | fbi.gov                               | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 436  | bowen.ns.cloudflare.com               | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 437  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 438  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 440  | freeyx.cloudflare88.eu.org            | 2606:4700:3010:e070:5d82:552a:2069:936a | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:e070:5d82:552a:2069:936a]:443: connect: network is unreachable |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 352  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 441  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 443  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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

**问题主机分析**: 以下主机出现多次失败：trevor.ns.cloudflare.com (3次),
wilson.ns.cloudflare.com (3次), moura.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 402  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 314  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 265  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 354  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 420  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 418  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 365  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 253  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 378  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 408  | ifconfig.co                           | 104.21.54.91    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 136  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 131  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 318  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 25   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 115  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 219  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 377  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 401  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 213  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 380  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 295  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 207  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 20   | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 122  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 140  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 346  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 413  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 128  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 40   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 144  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 181  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 270  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 331  | bestcf.030101.xyz                     | 104.17.211.46   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 358  | cf.zhetengsha.eu.org                  | 104.18.42.98    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 404  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 86   | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 110  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 142  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 212  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 373  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 412  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 428  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 106  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 150  | cf.877771.xyz                         | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 353  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 359  | cf.zhetengsha.eu.org                  | 172.64.145.158  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 362  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 58   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 100  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 102  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 229  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 350  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 381  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 87   | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 99   | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 114  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 145  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 148  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 307  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 355  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 108  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 169  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 185  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 188  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 339  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 356  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 154  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 290  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 370  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 387  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 43   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 57   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 95   | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 149  | cf.877771.xyz                         | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 257  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 275  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 116  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 129  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 199  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 306  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 386  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 403  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 72   | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 101  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 180  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 206  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 266  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 285  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 286  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 117  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 235  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 315  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 369  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 407  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 26   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 111  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 130  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 187  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 262  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 335  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 50   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 76   | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 391  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 28   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 160  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 289  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 396  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 81   | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 107  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 178  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 240  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 309  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 357  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 397  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 54   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 226  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 243  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 247  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 119  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 308  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 31   | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 105  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 242  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 29   | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 55   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 61   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 179  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 252  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 326  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 417  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 8    | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 59   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 73   | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 97   | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 147  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 227  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 421  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 98   | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 143  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 153  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 27   | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 120  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 385  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 419  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 80   | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 121  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 132  | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 170  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 177  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 296  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 371  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 75   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 162  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 168  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 261  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 330  | bestcf.030101.xyz                     | 104.17.212.134  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 364  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 56   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 96   | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 279  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 302  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 340  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 390  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 2    | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 44   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 146  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 193  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 280  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 325  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 434  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 3    | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 90   | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 103  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 414  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 30   | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 113  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 163  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 225  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 241  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 284  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 294  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 379  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 68   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 233  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 422  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 429  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 133  | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 159  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 203  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 205  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 274  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 336  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 395  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 24   | www.7749tv.com                        | 104.19.93.88    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 49   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 218  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 363  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 368  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 435  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 439  | freeyx.cloudflare88.eu.org            | 141.101.120.45  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 156 条记录
- **正常 (100-200ms)**: 43 条记录
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
