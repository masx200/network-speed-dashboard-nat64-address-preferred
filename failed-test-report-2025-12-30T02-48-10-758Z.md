# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 02:48:10
- **数据来源**: connectivity_results-20251230-024808.json
- **总测试数**: 622
- **失败测试数**: 201
- **成功测试数**: 421
- **失败率**: 32.32%
- **平均延迟**: 113.01ms
- **最小延迟**: 65ms
- **最大延迟**: 484ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 02:48:10
- **IP地址**: 52.161.69.161
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1437, -104.8117
- **时区**: America/Denver
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 198 次 (98.5%)
- **连接超时: I/O超时**: 3 次 (1.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (198 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | 2a06:98c1:3106::c5:5d39:736d            | 2a06:98c1:3106::c5:5d39:736d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable            |
| 37   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 38   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 39   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 42   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 43   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 50   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 51   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 52   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 59   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 60   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 61   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 62   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 63   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 66   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 67   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 70   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 71   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 74   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 75   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 82   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 83   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 90   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 91   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 92   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 107  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 108  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 109  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 110  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 111  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 118  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 119  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 123  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 124  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 153  | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 157  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 158  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 159  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 160  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 166  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 167  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 168  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 178  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 179  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 180  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 184  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 185  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 186  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 187  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 188  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 189  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 193  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 194  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 197  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 198  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 202  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 203  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 204  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 205  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 206  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 207  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 216  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 217  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 218  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 222  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 223  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 224  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 225  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 235  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 236  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 237  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 243  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 244  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 245  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 249  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 250  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 251  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 259  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 260  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 261  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 264  | bestcf.030101.xyz                       | 2606:4700:0:72:e1e6:5546:888c:67d1      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:72:e1e6:5546:888c:67d1]:443: connect: network is unreachable      |
| 265  | bestcf.030101.xyz                       | 2606:4700:19:e3ed:311e:da65:ac7a:3d52   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:19:e3ed:311e:da65:ac7a:3d52]:443: connect: network is unreachable   |
| 266  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 269  | zread.ai                                | 2a06:98c1:3121::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                       |
| 270  | zread.ai                                | 2a06:98c1:3120::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                       |
| 273  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 274  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 278  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:e070:5d82:552a:2069:936a | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:e070:5d82:552a:2069:936a]:443: connect: network is unreachable |
| 281  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 282  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 290  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 291  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 292  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 294  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 299  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 300  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 304  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 305  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 306  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 309  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 310  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 314  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 315  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 316  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 320  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 321  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 322  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 326  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 327  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 328  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 331  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 332  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 339  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 340  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 341  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 348  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 349  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 350  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 354  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 355  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 363  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 364  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 365  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 372  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 373  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 376  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 377  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 381  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 382  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 383  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 387  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 388  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 389  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 393  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 394  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 395  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 399  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 400  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 401  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 403  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 406  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 428  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 429  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 432  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 437  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 438  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 439  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 443  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 444  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 445  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 446  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 451  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 452  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 456  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 462  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 463  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 464  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 472  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 473  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 474  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 478  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 481  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 482  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 486  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 487  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 488  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 492  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 493  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 494  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 497  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 503  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 504  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 519  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 520  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 526  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 527  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 529  | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 530  | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable   |
| 531  | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable   |
| 532  | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable   |
| 533  | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable    |
| 534  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable  |
| 535  | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable   |
| 536  | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 537  | 2803:f800:51:6a7b:7c95:5585:9678:1549   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable   |
| 538  | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable    |
| 569  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 570  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 571  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 602  | 2a06:98c1:3105:0:db:557f:8a53:2469      | 2a06:98c1:3105:0:db:557f:8a53:2469      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable      |
| 605  | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable   |
| 606  | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable   |
| 607  | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable   |
| 608  | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 609  | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable      |
| 610  | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable       |
| 611  | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable   |
| 612  | 2a06:98c1:3100:0:a3:1339:d974:e2c       | 2a06:98c1:3100:0:a3:1339:d974:e2c       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable       |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 417  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 468  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 622  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 198 次 (98.5%)
- **连接超时**: 3 次 (1.5%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 201 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 198 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
trevor.ns.cloudflare.com (3次), wilson.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 421 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 450  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 172  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 506  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 129  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 170  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 470  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 509  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 138  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 549  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 553  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 559  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 573  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 577  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 593  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 599  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 200  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 418  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 297  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 402  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 426  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 500  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 587  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 598  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 86   | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 469  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 101  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 151  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 361  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 476  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 484  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 257  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 424  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 518  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 121  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 528  | www.7749tv.com                        | 104.16.115.36   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 398  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 431  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 600  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 565  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 567  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 152  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 427  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 585  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 457  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 522  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 296  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 385  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 467  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 550  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 578  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 329  | cf.zhetengsha.eu.org                  | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 589  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 72   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 191  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 293  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 507  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 12   | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 25   | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 455  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 554  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 44   | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 131  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 525  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 551  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 13   | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 55   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 435  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 458  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 561  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 580  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 17   | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 40   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 112  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 415  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 555  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 556  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 581  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 10   | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 31   | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 97   | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 139  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 213  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 411  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 414  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 449  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 465  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 114  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 141  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 177  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 272  | xn--b6gac.eu.org                      | 188.114.96.3    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 359  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 475  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 122  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 165  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 258  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 275  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 295  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 324  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 335  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 371  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 378  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 574  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 23   | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 24   | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 69   | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 136  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 155  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 246  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 254  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 347  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 370  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 404  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 405  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 441  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 508  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 516  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 582  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 41   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 113  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 115  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 125  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 132  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 210  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 351  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 392  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 409  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 410  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 448  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 9    | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 32   | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 154  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 164  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 289  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 302  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 358  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 420  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 442  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 539  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 33   | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 45   | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 57   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 105  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 163  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 173  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 208  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 238  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 280  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 334  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 433  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 447  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 552  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 16   | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 73   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 78   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 94   | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 95   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 96   | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 241  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 369  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 460  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 502  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 562  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 563  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 579  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 584  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 617  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 27   | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 58   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 64   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 65   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 192  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 252  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 357  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 396  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 416  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 477  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 505  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 523  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 545  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 586  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 15   | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 99   | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 134  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 591  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 601  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 11   | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 68   | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 130  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 169  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 190  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 317  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 323  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 353  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 56   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 181  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 390  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 517  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 566  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 21   | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 271  | xn--b6gac.eu.org                      | 188.114.97.3    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 298  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 325  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 367  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 510  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 560  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 594  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 29   | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 30   | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 87   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 248  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 277  | freeyx.cloudflare88.eu.org            | 141.101.120.19  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 356  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 379  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 511  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 513  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 540  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 20   | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 116  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 195  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 219  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 267  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 498  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 102  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 161  | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 231  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 336  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 397  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 496  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 262  | bestcf.030101.xyz                     | 104.17.60.95    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 360  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 576  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 18   | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 19   | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 47   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 126  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 176  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 284  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 286  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 333  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 453  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 454  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 485  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 215  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 255  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 343  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 425  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 434  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 436  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 524  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 34   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 283  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 338  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 572  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 620  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 48   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 77   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 120  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 226  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 285  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 319  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 412  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 479  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 117  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 137  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 162  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 196  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 313  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 413  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 430  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 495  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 543  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 544  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 619  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 14   | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 330  | cf.zhetengsha.eu.org                  | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 344  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 386  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 419  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 489  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 85   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 100  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 346  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 514  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 583  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 53   | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 84   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 175  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 209  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 423  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 480  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 557  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 590  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 595  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 2    | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 26   | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 103  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 140  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 308  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 375  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 466  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 616  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 127  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 128  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 211  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 232  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 276  | freeyx.cloudflare88.eu.org            | 141.101.120.247 | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 345  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 384  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 490  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 501  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 3    | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 4    | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 352  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 421  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 541  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 546  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 89   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 229  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 240  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 483  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 76   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 233  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 263  | bestcf.030101.xyz                     | 104.17.55.198   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 268  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 374  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 391  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 408  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 54   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 81   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 279  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 380  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 459  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 547  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 564  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 568  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 575  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 604  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 98   | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 135  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 221  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 234  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 440  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 588  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 46   | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 104  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 230  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 239  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 256  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 368  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 615  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 618  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 366  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 512  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 596  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 614  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 5    | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 88   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 228  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 288  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 301  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 597  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 131      | cloudflare |
| 22   | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 106  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 312  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 171  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 28   | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 521  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 174  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 311  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 318  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 342  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 93   | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 407  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 613  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 49   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 542  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 183  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 220  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 79   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 35   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 471  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 6    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 247  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 515  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 242  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 307  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 156  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 201  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 143      | cloudflare |
| 7    | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 214  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 287  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 303  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 182  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 362  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 8    | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 227  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 36   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 199  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 491  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 592  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 157      | cloudflare |
| 548  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 461  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 499  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 168      | cloudflare |
| 558  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 80   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 133  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 174      | cloudflare |
| 603  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 180      | cloudflare |
| 337  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 193      | cloudflare |
| 422  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 313      | cloudflare |
| 212  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 476      | cloudflare |
| 253  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 479      | cloudflare |
| 621  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 484      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 120 条记录
- **正常 (100-200ms)**: 297 条记录
- **慢 (200-500ms)**: 4 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 198 次

### 按协议统计

- **none**: 201 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
