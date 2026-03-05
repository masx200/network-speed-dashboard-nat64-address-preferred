# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/1/19 02:30:57
- **数据来源**: connectivity_results-20260119-023056.json
- **总测试数**: 729
- **失败测试数**: 222
- **成功测试数**: 507
- **失败率**: 30.45%
- **平均延迟**: 114.10ms
- **最小延迟**: 50ms
- **最大延迟**: 1751ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/1/19 02:30:57
- **IP地址**: 74.249.78.35
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

- **网络不可达: 网络不可达**: 219 次 (98.6%)
- **连接超时: I/O超时**: 3 次 (1.4%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (219 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 2    | 2a06:98c1:50:f771:e9b:84bd:5505:3935    | 2a06:98c1:50:f771:e9b:84bd:5505:3935    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable    |
| 13   | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 14   | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 15   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 16   | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 17   | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 18   | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 19   | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 20   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 21   | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 22   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 29   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 30   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 34   | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 35   | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 36   | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 64   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 65   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 66   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 78   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 79   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 80   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 82   | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 83   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable   |
| 84   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable   |
| 85   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable   |
| 86   | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable    |
| 87   | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable  |
| 88   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable   |
| 89   | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 90   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable   |
| 91   | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable    |
| 98   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 113  | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 114  | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 115  | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 116  | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 117  | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 122  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 123  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 127  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 128  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 129  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 139  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 140  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 141  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 175  | cf.877771.xyz                           | 2a06:98c1:3121::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                       |
| 176  | cf.877771.xyz                           | 2a06:98c1:3120::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                       |
| 190  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 191  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 192  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 195  | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 196  | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 200  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 201  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 202  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 208  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 209  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 210  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 214  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 215  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 216  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 220  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 221  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 222  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 226  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 227  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 228  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 239  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 240  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 241  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 245  | bestcf.030101.xyz                       | 2606:4700:0:8b4f:d68d:41ec:64bb:f6bf    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:8b4f:d68d:41ec:64bb:f6bf]:443: connect: network is unreachable    |
| 246  | bestcf.030101.xyz                       | 2606:4700::f982:f9ca:1eca               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::f982:f9ca:1eca]:443: connect: network is unreachable               |
| 249  | xn--b6gac.eu.org                        | 2a06:98c1:3121::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121::3]:443: connect: network is unreachable                       |
| 250  | xn--b6gac.eu.org                        | 2a06:98c1:3120::3                       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::3]:443: connect: network is unreachable                       |
| 254  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 255  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 256  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 259  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 261  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:1931:ffb6:9dbb:5ba4:26eb | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:1931:ffb6:9dbb:5ba4:26eb]:443: connect: network is unreachable |
| 262  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:e070:5d82:552a:87b5:e42c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3010:e070:5d82:552a:87b5:e42c]:443: connect: network is unreachable |
| 267  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 268  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 271  | cf.zhetengsha.eu.org                    | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 272  | cf.zhetengsha.eu.org                    | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 283  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 284  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 288  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 289  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 290  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 293  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 294  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 301  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 302  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 303  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 310  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 311  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 312  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 316  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 317  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 318  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 322  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 323  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 324  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 329  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 330  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 335  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 336  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 337  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 341  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 342  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 343  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 360  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 365  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 366  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 367  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 371  | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable   |
| 372  | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable   |
| 373  | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable   |
| 374  | 2a06:98c1:3105:0:db:557f:8a53:2469      | 2a06:98c1:3105:0:db:557f:8a53:2469      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable      |
| 376  | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable      |
| 379  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 380  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 381  | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable       |
| 384  | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 385  | 2a06:98c1:3100:0:a3:1339:d974:e2c       | 2a06:98c1:3100:0:a3:1339:d974:e2c       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable       |
| 386  | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable   |
| 388  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 389  | 2a06:98c1:3106::c5:5d39:736d            | 2a06:98c1:3106::c5:5d39:736d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable            |
| 394  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 395  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 402  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 403  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 404  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 410  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 411  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 412  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 416  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 417  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 418  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 420  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 434  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 435  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 436  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 439  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 445  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 446  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 457  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 462  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 463  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 466  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 467  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 480  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 484  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 485  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 487  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 490  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 491  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 495  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 496  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 497  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 503  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 509  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 510  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 511  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 516  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 517  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 518  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 527  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 528  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 550  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 551  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 552  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 557  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 558  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 559  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 571  | 2a06:98c1:51::c0bc:f0fe:59ba            | 2a06:98c1:51::c0bc:f0fe:59ba            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable            |
| 572  | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793    | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable    |
| 576  | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 578  | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85  | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable  |
| 579  | 2a06:98c1:50:8be4:5078:7eea:e43d:164    | 2a06:98c1:50:8be4:5078:7eea:e43d:164    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable    |
| 580  | 2a06:98c1:310a:0:15:735e:c4e:e2f7       | 2a06:98c1:310a:0:15:735e:c4e:e2f7       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable       |
| 581  | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 582  | 2a06:98c1:310c:6a:19f2:494:88cc:d5f     | 2a06:98c1:310c:6a:19f2:494:88cc:d5f     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable     |
| 583  | 2a06:98c1:310b:0:e474:ff3f:ec26:c616    | 2a06:98c1:310b:0:e474:ff3f:ec26:c616    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable    |
| 584  | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4   | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable   |
| 594  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 595  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |
| 628  | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de    | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable    |
| 629  | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 630  | 2a06:98c1:3105:0:2359:4222:d558:10fb    | 2a06:98c1:3105:0:2359:4222:d558:10fb    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable    |
| 631  | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c  | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable  |
| 632  | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59   | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable   |
| 633  | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 634  | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8    | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable    |
| 635  | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87  | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable  |
| 636  | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae    | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable    |
| 654  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 655  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 656  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 659  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 660  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 663  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 664  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 667  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 668  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 680  | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 681  | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 686  | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 687  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 689  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 695  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 696  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 697  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 708  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 709  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 719  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 720  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 721  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |

#### 连接超时: I/O超时 (3 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 449  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 727  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 729  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 219 次 (98.6%)
- **连接超时**: 3 次 (1.4%)

#### 错误模式分析

**超时集中度分析**: 共有 3 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 222 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 3 次，IPv6失败 219 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
sullivan.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 507 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 238  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 295  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 441  | 172.64.146.67                         | 172.64.146.67   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 568  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 610  | 172.64.49.146                         | 172.64.49.146   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 657  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 612  | 162.159.1.145                         | 162.159.1.145   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 710  | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 264  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 304  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 486  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 622  | 104.26.4.4                            | 104.26.4.4      | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 623  | 162.159.36.52                         | 162.159.36.52   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 593  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 605  | 172.64.32.77                          | 172.64.32.77    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 589  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 596  | 172.64.154.113                        | 172.64.154.113  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 694  | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 451  | 104.26.5.101                          | 104.26.5.101    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 206  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 315  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 473  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 617  | 172.67.78.67                          | 172.67.78.67    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 103  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 437  | 162.159.1.111                         | 162.159.1.111   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 670  | cu.877774.xyz                         | 104.26.4.111    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 683  | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 718  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 520  | 172.64.52.67                          | 172.64.52.67    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 492  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 348  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 406  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 597  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 363  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 586  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 130  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 194  | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 609  | 104.26.11.160                         | 104.26.11.160   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 478  | 104.19.34.231                         | 104.19.34.231   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 532  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 619  | 172.67.79.150                         | 172.67.79.150   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 690  | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 419  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 296  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 291  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 9    | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 465  | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 717  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 6    | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 11   | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 189  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 243  | bestcf.030101.xyz                     | 104.17.99.183   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 494  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 525  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 161  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 282  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 599  | 172.64.41.216                         | 172.64.41.216   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 678  | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 703  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 263  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 461  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 471  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 542  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 269  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 309  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 469  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 10   | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 54   | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 539  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 164  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 166  | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 178  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 182  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 277  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 448  | 104.18.40.216                         | 104.18.40.216   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 508  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 604  | 104.18.41.101                         | 104.18.41.101   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 700  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 728  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 5    | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 332  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 340  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 425  | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 499  | 104.17.215.66                         | 104.17.215.66   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 626  | 104.18.40.202                         | 104.18.40.202   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 714  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 171  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 502  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 611  | 104.18.32.174                         | 104.18.32.174   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 643  | 104.18.44.25                          | 104.18.44.25    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 138  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 396  | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 443  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 477  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 555  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 39   | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 55   | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 186  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 299  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 319  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 333  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 354  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 431  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 601  | 172.64.152.215                        | 172.64.152.215  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 67   | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 109  | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 156  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 327  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 361  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 362  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 428  | 172.64.145.119                        | 172.64.145.119  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 450  | 104.26.2.2                            | 104.26.2.2      | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 535  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 3    | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 32   | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 95   | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 180  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 328  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 524  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 530  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 565  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 665  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 702  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 106  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 135  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 167  | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 248  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 257  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 326  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 353  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 498  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 575  | 162.159.61.106                        | 162.159.61.106  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 615  | 172.64.40.196                         | 172.64.40.196   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 646  | 104.16.251.254                        | 104.16.251.254  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 673  | cu.877774.xyz                         | 104.26.4.114    | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 716  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 197  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 229  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 398  | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 637  | 104.17.153.151                        | 104.17.153.151  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 658  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 357  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 364  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 442  | 172.64.153.141                        | 172.64.153.141  | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 505  | 104.17.25.241                         | 104.17.25.241   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 569  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 675  | cu.877774.xyz                         | 104.26.4.116    | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 50   | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 112  | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 133  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 136  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 217  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 334  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 472  | 104.17.170.137                        | 104.17.170.137  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 512  | 104.17.189.30                         | 104.17.189.30   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 513  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 536  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 537  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 577  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 121  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 126  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 144  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 501  | 104.17.209.79                         | 104.17.209.79   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 662  | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 45   | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 142  | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 154  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 481  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 548  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 616  | 104.18.36.1                           | 104.18.36.1     | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 704  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 28   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 41   | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 53   | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 149  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 169  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 253  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 331  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 564  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 711  | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 47   | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 179  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 359  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 500  | 104.19.153.47                         | 104.19.153.47   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 624  | 104.20.17.51                          | 104.20.17.51    | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 33   | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 48   | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 193  | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 213  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 281  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 358  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 400  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 401  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 570  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 606  | 104.18.37.177                         | 104.18.37.177   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 148  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 199  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 203  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 223  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 237  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 538  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 7    | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 545  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 621  | 104.20.25.161                         | 104.20.25.161   | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 639  | 104.19.144.159                        | 104.19.144.159  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 715  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 107  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 110  | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 131  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 159  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 273  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 405  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 598  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 669  | cu.877774.xyz                         | 104.26.4.119    | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 68   | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 70   | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 100  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 108  | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 285  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 325  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 415  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 452  | 172.67.73.120                         | 172.67.73.120   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 454  | 104.26.1.181                          | 104.26.1.181    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 523  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 676  | cu.877774.xyz                         | 104.26.4.117    | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 69   | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 204  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 219  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 280  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 349  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 409  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 447  | 104.18.42.61                          | 104.18.42.61    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 514  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 534  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 588  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 666  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 671  | cu.877774.xyz                         | 104.26.4.112    | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 26   | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 172  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 286  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 347  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 423  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 475  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 693  | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 59   | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 168  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 414  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 488  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 642  | 172.64.154.226                        | 172.64.154.226  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 650  | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 44   | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 81   | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 346  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 378  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 458  | 104.20.25.82                          | 104.20.25.82    | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 547  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 705  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 102      | cloudflare |
| 40   | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 63   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 72   | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 94   | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 101  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 124  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 274  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 339  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 350  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 521  | 162.159.42.146                        | 162.159.42.146  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 691  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 321  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 351  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 653  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 688  | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 706  | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 42   | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 49   | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 60   | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 270  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 421  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 620  | 104.18.47.46                          | 104.18.47.46    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 644  | 104.17.25.87                          | 104.17.25.87    | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 685  | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 105      | cloudflare |
| 61   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 187  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 224  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 356  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 392  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 526  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 592  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 27   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 57   | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 146  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 184  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 234  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 276  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 305  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 352  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 397  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 460  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 591  | 162.159.6.44                          | 162.159.6.44    | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 698  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 119  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 143  | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 260  | freeyx.cloudflare88.eu.org            | 141.101.121.14  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 278  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 369  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 433  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 507  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 533  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 712  | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 218  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 258  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 266  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 345  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 464  | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 563  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 590  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 692  | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 724  | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 110      | cloudflare |
| 99   | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 120  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 181  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 183  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 111      | cloudflare |
| 150  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 158  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 468  | 104.26.10.239                         | 104.26.10.239   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 541  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 544  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 638  | 104.18.42.106                         | 104.18.42.106   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 672  | cu.877774.xyz                         | 104.26.4.113    | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 707  | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 147  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 247  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 422  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 470  | 104.17.211.218                        | 104.17.211.218  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 476  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 479  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 242  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 297  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 300  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 307  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 427  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 627  | 104.26.7.7                            | 104.26.7.7      | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 652  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 682  | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 114      | cloudflare |
| 56   | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 603  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 115      | cloudflare |
| 37   | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 185  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 225  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 230  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 314  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 474  | 104.17.115.224                        | 104.17.115.224  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 549  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 602  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 8    | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 77   | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 162  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 211  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 244  | bestcf.030101.xyz                     | 104.17.27.231   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 483  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 600  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 614  | 104.26.1.194                          | 104.26.1.194    | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 701  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 117      | cloudflare |
| 151  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 155  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 212  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 382  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 640  | 104.17.53.129                         | 104.17.53.129   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 679  | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 722  | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 118      | cloudflare |
| 24   | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 132  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 134  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 275  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 540  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 645  | 104.16.155.230                        | 104.16.155.230  | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 677  | cu.877774.xyz                         | 104.26.4.118    | IPv4   | h3   | ✅ 成功 | 119      | cloudflare |
| 58   | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 145  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 292  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 393  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 459  | 172.67.72.36                          | 172.67.72.36    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 618  | 104.26.6.171                          | 104.26.6.171    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 649  | www.7749tv.com                        | 104.19.133.4    | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 684  | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 120      | cloudflare |
| 74   | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 233  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 699  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 308  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 651  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 25   | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 97   | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 375  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 390  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 429  | 162.159.3.89                          | 162.159.3.89    | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 648  | 104.17.187.186                        | 104.17.187.186  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 641  | 104.17.110.226                        | 104.17.110.226  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 661  | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 152  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 207  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 444  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 647  | 104.17.214.136                        | 104.17.214.136  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 674  | cu.877774.xyz                         | 104.26.4.115    | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 252  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 265  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 370  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 556  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 625  | 172.67.67.152                         | 172.67.67.152   | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 43   | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 574  | 108.162.198.148                       | 108.162.198.148 | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 613  | 162.159.16.136                        | 162.159.16.136  | IPv4   | h3   | ✅ 成功 | 127      | cloudflare |
| 205  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 232  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 306  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 368  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 456  | 172.67.75.11                          | 172.67.75.11    | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 170  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 231  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 529  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 573  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 129      | cloudflare |
| 23   | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 174  | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 531  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 608  | 172.64.145.108                        | 172.64.145.108  | IPv4   | h3   | ✅ 成功 | 130      | cloudflare |
| 1    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 424  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 546  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 132      | cloudflare |
| 515  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 383  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 489  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 543  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 553  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 607  | 162.159.36.223                        | 162.159.36.223  | IPv4   | h3   | ✅ 成功 | 134      | cloudflare |
| 46   | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 173  | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 38   | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 438  | 162.159.22.29                         | 162.159.22.29   | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 102  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 235  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 236  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 522  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 377  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 139      | cloudflare |
| 157  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 140      | cloudflare |
| 338  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 399  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 430  | 162.159.18.240                        | 162.159.18.240  | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 453  | 104.20.21.147                         | 104.20.21.147   | IPv4   | h3   | ✅ 成功 | 142      | cloudflare |
| 493  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 144      | cloudflare |
| 105  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 145      | cloudflare |
| 344  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 504  | 104.19.35.242                         | 104.19.35.242   | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 92   | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 153  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 163  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 408  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 12   | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 298  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 152      | cloudflare |
| 279  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 51   | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 104  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 155      | cloudflare |
| 455  | 104.26.3.120                          | 104.26.3.120    | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 118  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 407  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 562  | 162.159.44.133                        | 162.159.44.133  | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 432  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 482  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 162      | cloudflare |
| 566  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 163      | cloudflare |
| 506  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 164      | cloudflare |
| 160  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 165  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 320  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 165      | cloudflare |
| 93   | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 166      | cloudflare |
| 125  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 167      | cloudflare |
| 111  | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 387  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 560  | 162.159.21.222                        | 162.159.21.222  | IPv4   | h3   | ✅ 成功 | 171      | cloudflare |
| 75   | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 723  | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 137  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 174      | cloudflare |
| 440  | 172.64.42.158                         | 172.64.42.158   | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 561  | 162.159.39.136                        | 162.159.39.136  | IPv4   | h3   | ✅ 成功 | 179      | cloudflare |
| 76   | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 181      | cloudflare |
| 585  | 162.159.38.67                         | 162.159.38.67   | IPv4   | h3   | ✅ 成功 | 183      | cloudflare |
| 251  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 177  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 62   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 355  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 426  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 194      | cloudflare |
| 587  | 162.159.45.165                        | 162.159.45.165  | IPv4   | h3   | ✅ 成功 | 195      | cloudflare |
| 287  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 313  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 198      | cloudflare |
| 4    | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 96   | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 73   | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 391  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 567  | 172.64.53.40                          | 172.64.53.40    | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 413  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 188  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 214      | cloudflare |
| 71   | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 52   | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 31   | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 253      | cloudflare |
| 198  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 366      | cloudflare |
| 554  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 725  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 592      | cloudflare |
| 726  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 608      | cloudflare |
| 713  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 846      | cloudflare |
| 519  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 1751     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 237 条记录
- **正常 (100-200ms)**: 255 条记录
- **慢 (200-500ms)**: 11 条记录
- **很慢 (>500ms)**: 4 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 3 次
- **IPv6 失败**: 219 次

### 按协议统计

- **none**: 222 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
