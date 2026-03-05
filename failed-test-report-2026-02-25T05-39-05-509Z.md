# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2026/2/25 05:39:05
- **数据来源**: connectivity_results-20260225-053904.json
- **总测试数**: 952
- **失败测试数**: 262
- **成功测试数**: 690
- **失败率**: 27.52%
- **平均延迟**: 65.36ms
- **最小延迟**: 29ms
- **最大延迟**: 771ms

## 🌐 当前测试环境信息

- **获取时间**: 2026/2/25 05:39:05
- **IP地址**: 40.76.191.133
- **国家/地区**: 美国 (US)
- **ASN**: 8075
- **网络组织**: Microsoft Corporation
- **网络域名**: microsoft.com
- **大洲**: North America (NA)
- **地理坐标**: 38.7095, -78.1539
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **网络不可达: 网络不可达**: 258 次 (98.5%)
- **连接超时: I/O超时**: 4 次 (1.5%)

### 🔍 按错误类型分类的失败测试详情

#### 网络不可达: 网络不可达 (258 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------------------------------------------- |
| 1    | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: connect: network is unreachable               |
| 2    | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:5d:1caa:56dd:a908:af7b]:443: connect: network is unreachable   |
| 9    | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c36e]:443: connect: network is unreachable                 |
| 10   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: connect: network is unreachable                 |
| 11   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c6e]:443: connect: network is unreachable                 |
| 14   | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:400a]:443: connect: network is unreachable               |
| 15   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3030::ac43:ae15]:443: connect: network is unreachable               |
| 18   | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8eed]:443: connect: network is unreachable                    |
| 19   | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:8fed]:443: connect: network is unreachable                    |
| 23   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39a]:443: connect: network is unreachable                 |
| 24   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9a]:443: connect: network is unreachable                 |
| 25   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239a]:443: connect: network is unreachable                 |
| 28   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::6815:785]:443: connect: network is unreachable                |
| 29   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:bb91]:443: connect: network is unreachable               |
| 32   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:cf1a]:443: connect: network is unreachable               |
| 33   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:fd4]:443: connect: network is unreachable                |
| 39   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120::5692:61a4]:443: connect: network is unreachable               |
| 40   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: connect: network is unreachable                 |
| 41   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::818:669e]:443: connect: network is unreachable                |
| 42   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::1a4f:5642]:443: connect: network is unreachable               |
| 43   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8ca0::3dc4:21a2]:443: connect: network is unreachable               |
| 46   | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:98b7]:443: connect: network is unreachable               |
| 47   | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:50b4]:443: connect: network is unreachable               |
| 50   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1581]:443: connect: network is unreachable               |
| 51   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:c6cb]:443: connect: network is unreachable               |
| 52   | 2400:cb00:2049:5d:a92a:97f:6fa3:f803    | 2400:cb00:2049:5d:a92a:97f:6fa3:f803    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:2049:5d:a92a:97f:6fa3:f803]:443: connect: network is unreachable    |
| 65   | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:22d4]:443: connect: network is unreachable                 |
| 66   | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:affa]:443: connect: network is unreachable                 |
| 72   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:d17f]:443: connect: network is unreachable               |
| 73   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:1748]:443: connect: network is unreachable               |
| 77   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:bde]:443: connect: network is unreachable                  |
| 78   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4664]:443: connect: network is unreachable                 |
| 79   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:ade]:443: connect: network is unreachable                  |
| 93   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:9add::880:52fc]:443: connect: network is unreachable                |
| 125  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbc]:443: connect: network is unreachable                 |
| 126  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bc]:443: connect: network is unreachable                 |
| 127  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bc]:443: connect: network is unreachable                 |
| 133  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:a168]:443: connect: network is unreachable               |
| 134  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: connect: network is unreachable               |
| 136  | 2a06:98c1:3108:0:edda:98f0:da65:4271    | 2a06:98c1:3108:0:edda:98f0:da65:4271    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3108:0:edda:98f0:da65:4271]:443: connect: network is unreachable    |
| 137  | 2a06:98c1:310d:85:ac4c:8137:506:5188    | 2a06:98c1:310d:85:ac4c:8137:506:5188    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:85:ac4c:8137:506:5188]:443: connect: network is unreachable    |
| 138  | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | 2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:ee7a:af11:b020:b50d:d4e2]:443: connect: network is unreachable |
| 139  | 2803:f800:51:0:fc87:e2d6:88c3:378b      | 2803:f800:51:0:fc87:e2d6:88c3:378b      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:0:fc87:e2d6:88c3:378b]:443: connect: network is unreachable      |
| 140  | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | 2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:73ae:49fb:f5c4:1394:7e53]:443: connect: network is unreachable |
| 141  | 2a06:98c1:51:6e:e874:db4f:a1d5:2163     | 2a06:98c1:51:6e:e874:db4f:a1d5:2163     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:6e:e874:db4f:a1d5:2163]:443: connect: network is unreachable     |
| 142  | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | 2a06:98c1:3105:900d:4f38:5221:f77f:fe11 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:900d:4f38:5221:f77f:fe11]:443: connect: network is unreachable |
| 143  | 2a06:98c1:51::ee:b8fb:877f              | 2a06:98c1:51::ee:b8fb:877f              | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51::ee:b8fb:877f]:443: connect: network is unreachable              |
| 144  | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | 2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:900d:3fc7:e3c6:68cd:ece3]:443: connect: network is unreachable |
| 145  | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7   | 2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:f0:fa21:b1c1:bf1b:efd7]:443: connect: network is unreachable   |
| 157  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:725]:443: connect: network is unreachable                  |
| 158  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:46bf]:443: connect: network is unreachable                 |
| 159  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:625]:443: connect: network is unreachable                  |
| 166  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1476]:443: connect: network is unreachable                    |
| 167  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:1576]:443: connect: network is unreachable                    |
| 180  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2569]:443: connect: network is unreachable                    |
| 181  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6811:2469]:443: connect: network is unreachable                    |
| 192  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:1f96:d4bf:a723]:443: connect: network is unreachable |
| 210  | 2a06:98c1:51:a594:2926:2b16:6e8d:843e   | 2a06:98c1:51:a594:2926:2b16:6e8d:843e   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:a594:2926:2b16:6e8d:843e]:443: connect: network is unreachable   |
| 211  | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9     | 2a06:98c1:51:aa:3e22:dd48:6279:eeb9     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:aa:3e22:dd48:6279:eeb9]:443: connect: network is unreachable     |
| 212  | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | 2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310c:5874:e72e:d139:ebe3:e5ea]:443: connect: network is unreachable |
| 213  | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b   | 2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:e8:d68a:c7f6:afaa:c80b]:443: connect: network is unreachable   |
| 214  | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | 2a06:98c1:3104:da84:1c63:f149:4d21:b339 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104:da84:1c63:f149:4d21:b339]:443: connect: network is unreachable |
| 215  | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533   | 2803:f800:50:9516:e4a1:4ba9:1c5e:7533   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9516:e4a1:4ba9:1c5e:7533]:443: connect: network is unreachable   |
| 216  | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7  | 2a06:98c1:3100:f702:ebbf:618b:76c:9ba7  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:f702:ebbf:618b:76c:9ba7]:443: connect: network is unreachable  |
| 217  | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b   | 2a06:98c1:3107:54:2c60:eafc:f14d:ca4b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:54:2c60:eafc:f14d:ca4b]:443: connect: network is unreachable   |
| 218  | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7   | 2a06:98c1:3106:6a:7ba4:346b:e06c:71c7   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:6a:7ba4:346b:e06c:71c7]:443: connect: network is unreachable   |
| 219  | 2a06:98c1:3104::f3:8fed:cac0            | 2a06:98c1:3104::f3:8fed:cac0            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104::f3:8fed:cac0]:443: connect: network is unreachable            |
| 233  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a1]:443: connect: network is unreachable                 |
| 234  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a1]:443: connect: network is unreachable                 |
| 235  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca1]:443: connect: network is unreachable                 |
| 269  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:efde:82d1:8124:3fed]:443: connect: network is unreachable    |
| 288  | 2a06:98c1:3102:94:16cd:b988:5dae:1295   | 2a06:98c1:3102:94:16cd:b988:5dae:1295   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:94:16cd:b988:5dae:1295]:443: connect: network is unreachable   |
| 289  | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | 2400:cb00:2049:ec9e:b468:412c:1558:69cb | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:2049:ec9e:b468:412c:1558:69cb]:443: connect: network is unreachable |
| 290  | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | 2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:f7b9:fbc7:ac52:15f3:609c]:443: connect: network is unreachable |
| 291  | 2a06:98c1:3102:94:1604:ebd:f1ec:37be    | 2a06:98c1:3102:94:1604:ebd:f1ec:37be    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:94:1604:ebd:f1ec:37be]:443: connect: network is unreachable    |
| 292  | 2606:4700:59:764d:d406:c823:e52f:4f3a   | 2606:4700:59:764d:d406:c823:e52f:4f3a   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:59:764d:d406:c823:e52f:4f3a]:443: connect: network is unreachable   |
| 293  | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | 2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:d5f5:74cf:317a:6c39:4c5f]:443: connect: network is unreachable |
| 294  | 2a06:98c1:51:8:7944:48b0:1301:5ced      | 2a06:98c1:51:8:7944:48b0:1301:5ced      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:8:7944:48b0:1301:5ced]:443: connect: network is unreachable      |
| 295  | 2a06:98c1:310b::fda8:fa9e:4a3e          | 2a06:98c1:310b::fda8:fa9e:4a3e          | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b::fda8:fa9e:4a3e]:443: connect: network is unreachable          |
| 296  | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535    | 2a06:98c1:310b:0:cfd2:7ebe:a043:8535    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:0:cfd2:7ebe:a043:8535]:443: connect: network is unreachable    |
| 297  | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339    | 2a06:98c1:3102:0:e263:6cdc:a8ce:a339    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:0:e263:6cdc:a8ce:a339]:443: connect: network is unreachable    |
| 330  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:0:ef18:6ab0:b648:d756]:443: connect: network is unreachable    |
| 338  | 2a06:98c1:3102:8768:b929:7455:f040:5aee | 2a06:98c1:3102:8768:b929:7455:f040:5aee | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:8768:b929:7455:f040:5aee]:443: connect: network is unreachable |
| 339  | 2a06:98c1:310a:0:de:2b25:85a5:8a26      | 2a06:98c1:310a:0:de:2b25:85a5:8a26      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:0:de:2b25:85a5:8a26]:443: connect: network is unreachable      |
| 340  | 2a06:98c1:3100:22:21ad:d760:d542:16c8   | 2a06:98c1:3100:22:21ad:d760:d542:16c8   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:22:21ad:d760:d542:16c8]:443: connect: network is unreachable   |
| 341  | 2a06:98c1:310c::dd:f399:427e            | 2a06:98c1:310c::dd:f399:427e            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310c::dd:f399:427e]:443: connect: network is unreachable            |
| 342  | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | 2a06:98c1:3100:e1e7:ae26:af07:44a6:82da | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:e1e7:ae26:af07:44a6:82da]:443: connect: network is unreachable |
| 343  | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3    | 2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:d:1bd6:bbd1:d9a0:60b3]:443: connect: network is unreachable    |
| 344  | 2a06:98c1:3100:b3:af54:9923:e84:af58    | 2a06:98c1:3100:b3:af54:9923:e84:af58    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:b3:af54:9923:e84:af58]:443: connect: network is unreachable    |
| 345  | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f   | 2a06:98c1:3100:22:21cb:7546:1cd8:a79f   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:22:21cb:7546:1cd8:a79f]:443: connect: network is unreachable   |
| 346  | 2a06:98c1:51:878:e123:da31:b2ee:2017    | 2a06:98c1:51:878:e123:da31:b2ee:2017    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:878:e123:da31:b2ee:2017]:443: connect: network is unreachable    |
| 347  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c677:c614:7606:cec1:f722]:443: connect: network is unreachable |
| 348  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2]:443: connect: network is unreachable   |
| 349  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:7522:c680:d288:d13c]:443: connect: network is unreachable |
| 350  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3120:c39b:f77:4fc1:b18b:c12]:443: connect: network is unreachable   |
| 392  | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | 2a06:98c1:310b:2522:4bfe:492f:64b3:2d36 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:2522:4bfe:492f:64b3:2d36]:443: connect: network is unreachable |
| 393  | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | 2a06:98c1:310e:68:b803:ed16:7e58:d249   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310e:68:b803:ed16:7e58:d249]:443: connect: network is unreachable   |
| 394  | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | 2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:eac5:5d97:a2b9:5c3d:de2b]:443: connect: network is unreachable   |
| 395  | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | 2803:f800:50:9a81:aaf8:2b9b:dd37:67e2   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50:9a81:aaf8:2b9b:dd37:67e2]:443: connect: network is unreachable   |
| 396  | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | 2a06:98c1:310d:4c:4b41:a84:50ee:e810    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d:4c:4b41:a84:50ee:e810]:443: connect: network is unreachable    |
| 397  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | 2a06:98c1:310b:5429:cdf:3003:ae9c:e62e  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:5429:cdf:3003:ae9c:e62e]:443: connect: network is unreachable  |
| 398  | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | 2a06:98c1:3105:af:a833:8bb4:57b3:c4fd   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:af:a833:8bb4:57b3:c4fd]:443: connect: network is unreachable   |
| 399  | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | 2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:7319:3a35:b2f6:24ef:ea97]:443: connect: network is unreachable |
| 400  | 2803:f800:51:6a7b:7c95:5585:9678:1549   | 2803:f800:51:6a7b:7c95:5585:9678:1549   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:51:6a7b:7c95:5585:9678:1549]:443: connect: network is unreachable   |
| 401  | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | 2a06:98c1:3107:0:cf9c:104d:1e03:9644    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3107:0:cf9c:104d:1e03:9644]:443: connect: network is unreachable    |
| 430  | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | 2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:eb89:13e1:e7d0:738a:d1e6]:443: connect: network is unreachable   |
| 431  | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | 2a06:98c1:310b:43:e83a:f5ed:8126:81dc   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:43:e83a:f5ed:8126:81dc]:443: connect: network is unreachable   |
| 449  | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | 2a06:98c1:3101:6cce:1edc:88:628d:fd50   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:6cce:1edc:88:628d:fd50]:443: connect: network is unreachable   |
| 450  | 2a06:98c1:3105:0:db:557f:8a53:2469      | 2a06:98c1:3105:0:db:557f:8a53:2469      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:db:557f:8a53:2469]:443: connect: network is unreachable      |
| 451  | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | 2a06:98c1:51:e7:5abb:89e:d67d:c1a4      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:e7:5abb:89e:d67d:c1a4]:443: connect: network is unreachable      |
| 452  | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | 2a06:98c1:3104:0:4:5eb4:7182:42a0       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3104:0:4:5eb4:7182:42a0]:443: connect: network is unreachable       |
| 458  | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | 2a06:98c1:3100:27a8:686d:aa56:c917:4726 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:27a8:686d:aa56:c917:4726]:443: connect: network is unreachable |
| 459  | 2a06:98c1:3100:0:a3:1339:d974:e2c       | 2a06:98c1:3100:0:a3:1339:d974:e2c       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3100:0:a3:1339:d974:e2c]:443: connect: network is unreachable       |
| 460  | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | 2a06:98c1:310b:eecc:184:7caf:f7e0:b92   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:eecc:184:7caf:f7e0:b92]:443: connect: network is unreachable   |
| 461  | 2a06:98c1:3106::c5:5d39:736d            | 2a06:98c1:3106::c5:5d39:736d            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106::c5:5d39:736d]:443: connect: network is unreachable            |
| 498  | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85  | 2a06:98c1:310a:b523:52dd:b43c:a5f:5a85  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:b523:52dd:b43c:a5f:5a85]:443: connect: network is unreachable  |
| 523  | 2a06:98c1:51::c0bc:f0fe:59ba            | 2a06:98c1:51::c0bc:f0fe:59ba            | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51::c0bc:f0fe:59ba]:443: connect: network is unreachable            |
| 524  | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793    | 2a06:98c1:310b:0:e474:ff3f:ecc6:5793    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:0:e474:ff3f:ecc6:5793]:443: connect: network is unreachable    |
| 525  | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | 2400:cb00:f00e:9635:6a0b:4525:95ff:26a3 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:f00e:9635:6a0b:4525:95ff:26a3]:443: connect: network is unreachable |
| 553  | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de    | 2a06:98c1:3101:d7:eb36:3a1:c94d:32de    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3101:d7:eb36:3a1:c94d:32de]:443: connect: network is unreachable    |
| 554  | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | 2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3109:be88:aeb7:b6d2:c9f2:4d65]:443: connect: network is unreachable |
| 555  | 2a06:98c1:3105:0:2359:4222:d558:10fb    | 2a06:98c1:3105:0:2359:4222:d558:10fb    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:0:2359:4222:d558:10fb]:443: connect: network is unreachable    |
| 556  | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c  | 2a06:98c1:3103:c550:9adb:34b4:ce11:19c  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3103:c550:9adb:34b4:ce11:19c]:443: connect: network is unreachable  |
| 557  | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59   | 2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:51:8a7e:2be1:4da9:97bb:7c59]:443: connect: network is unreachable   |
| 558  | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | 2a06:98c1:310f:5820:a733:3f39:ff68:f260 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310f:5820:a733:3f39:ff68:f260]:443: connect: network is unreachable |
| 559  | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8    | 2a06:98c1:3102:0:90e8:b850:3d09:cfc8    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102:0:90e8:b850:3d09:cfc8]:443: connect: network is unreachable    |
| 560  | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87  | 2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105:3dea:69ff:1edd:4cd:ed87]:443: connect: network is unreachable  |
| 561  | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae    | 2a06:98c1:3106:0:ef95:8505:25ee:e5ae    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3106:0:ef95:8505:25ee:e5ae]:443: connect: network is unreachable    |
| 562  | 2a06:98c1:50:f771:e9b:84bd:5505:3935    | 2a06:98c1:50:f771:e9b:84bd:5505:3935    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:f771:e9b:84bd:5505:3935]:443: connect: network is unreachable    |
| 580  | 2a06:98c1:50:8be4:5078:7eea:e43d:164    | 2a06:98c1:50:8be4:5078:7eea:e43d:164    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50:8be4:5078:7eea:e43d:164]:443: connect: network is unreachable    |
| 581  | 2a06:98c1:310a:0:15:735e:c4e:e2f7       | 2a06:98c1:310a:0:15:735e:c4e:e2f7       | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310a:0:15:735e:c4e:e2f7]:443: connect: network is unreachable       |
| 582  | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | 2400:cb00:2049:e59d:7af6:c00c:4418:a88a | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2400:cb00:2049:e59d:7af6:c00c:4418:a88a]:443: connect: network is unreachable |
| 583  | 2a06:98c1:310c:6a:19f2:494:88cc:d5f     | 2a06:98c1:310c:6a:19f2:494:88cc:d5f     | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310c:6a:19f2:494:88cc:d5f]:443: connect: network is unreachable     |
| 584  | 2a06:98c1:310b:0:e474:ff3f:ec26:c616    | 2a06:98c1:310b:0:e474:ff3f:ec26:c616    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:0:e474:ff3f:ec26:c616]:443: connect: network is unreachable    |
| 585  | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4   | 2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310b:fd:febc:dbaf:d5f9:76d4]:443: connect: network is unreachable   |
| 602  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440f::53aa:4126]:443: connect: network is unreachable               |
| 617  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc0]:443: connect: network is unreachable                 |
| 618  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c0]:443: connect: network is unreachable                 |
| 619  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c0]:443: connect: network is unreachable                 |
| 627  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:de4f]:443: connect: network is unreachable                    |
| 628  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: connect: network is unreachable                    |
| 632  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd9]:443: connect: network is unreachable                 |
| 633  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d9]:443: connect: network is unreachable                 |
| 634  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d9]:443: connect: network is unreachable                 |
| 637  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2c90]:443: connect: network is unreachable               |
| 638  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:310c::ac42:2f70]:443: connect: network is unreachable               |
| 642  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c7]:443: connect: network is unreachable                 |
| 643  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc7]:443: connect: network is unreachable                 |
| 644  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c7]:443: connect: network is unreachable                 |
| 648  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: connect: network is unreachable                  |
| 649  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4812]:443: connect: network is unreachable                 |
| 650  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:224]:443: connect: network is unreachable                  |
| 654  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c36]:443: connect: network is unreachable                  |
| 655  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d36]:443: connect: network is unreachable                  |
| 656  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:449d]:443: connect: network is unreachable                 |
| 660  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ca]:443: connect: network is unreachable                 |
| 661  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ca]:443: connect: network is unreachable                 |
| 662  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cca]:443: connect: network is unreachable                 |
| 666  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9b]:443: connect: network is unreachable                 |
| 667  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39b]:443: connect: network is unreachable                 |
| 668  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239b]:443: connect: network is unreachable                 |
| 672  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c394]:443: connect: network is unreachable                 |
| 673  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2394]:443: connect: network is unreachable                 |
| 674  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c94]:443: connect: network is unreachable                 |
| 678  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cf]:443: connect: network is unreachable                 |
| 679  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cf]:443: connect: network is unreachable                 |
| 680  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccf]:443: connect: network is unreachable                 |
| 687  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cbb]:443: connect: network is unreachable                 |
| 688  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3bb]:443: connect: network is unreachable                 |
| 689  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23bb]:443: connect: network is unreachable                 |
| 692  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:ca4e]:443: connect: network is unreachable               |
| 693  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::6815:4cf0]:443: connect: network is unreachable               |
| 697  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c17]:443: connect: network is unreachable                  |
| 698  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4581]:443: connect: network is unreachable                 |
| 699  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d17]:443: connect: network is unreachable                  |
| 703  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccd]:443: connect: network is unreachable                 |
| 704  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cd]:443: connect: network is unreachable                 |
| 705  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cd]:443: connect: network is unreachable                 |
| 707  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cdf7:37be:7114 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3009:aa59:4b67:cdf7:37be:7114]:443: connect: network is unreachable |
| 713  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bac]:443: connect: network is unreachable                 |
| 714  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d1f]:443: connect: network is unreachable                  |
| 715  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c1f]:443: connect: network is unreachable                  |
| 720  | bestcf.030101.xyz                       | 2606:4700:0:d9:2acf:b5e0:5a46:4358      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:d9:2acf:b5e0:5a46:4358]:443: connect: network is unreachable      |
| 721  | bestcf.030101.xyz                       | 2606:4700:0:dd:df95:6eb1:ffa4:6779      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:0:dd:df95:6eb1:ffa4:6779]:443: connect: network is unreachable      |
| 727  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3032::ac43:9d7a]:443: connect: network is unreachable               |
| 728  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:3a48]:443: connect: network is unreachable               |
| 732  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:d8c]:443: connect: network is unreachable                  |
| 733  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:c8c]:443: connect: network is unreachable                  |
| 734  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bc2]:443: connect: network is unreachable                 |
| 736  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4409::5b5b:7758]:443: connect: network is unreachable               |
| 741  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:9a0]:443: connect: network is unreachable                  |
| 742  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:8a0]:443: connect: network is unreachable                  |
| 743  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:4bd0]:443: connect: network is unreachable                 |
| 746  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440b::3e6e:5f06]:443: connect: network is unreachable               |
| 749  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::ac43:a01c]:443: connect: network is unreachable               |
| 750  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3036::6815:eb0]:443: connect: network is unreachable                |
| 753  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:99fd]:443: connect: network is unreachable               |
| 754  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3035::6815:5a4e]:443: connect: network is unreachable               |
| 758  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca9]:443: connect: network is unreachable                 |
| 759  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a9]:443: connect: network is unreachable                 |
| 760  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a9]:443: connect: network is unreachable                 |
| 763  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::ac43:9bac]:443: connect: network is unreachable               |
| 764  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:48e9]:443: connect: network is unreachable               |
| 765  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4403::7357:544f]:443: connect: network is unreachable               |
| 768  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3105::6812:230f]:443: connect: network is unreachable               |
| 769  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:440a::ac40:98f1]:443: connect: network is unreachable               |
| 772  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:94f4]:443: connect: network is unreachable                    |
| 773  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6810:95f4]:443: connect: network is unreachable                    |
| 779  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4408::18c5:3304]:443: connect: network is unreachable               |
| 782  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::ac42:9e73]:443: connect: network is unreachable                 |
| 783  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:10::6814:1c4a]:443: connect: network is unreachable                 |
| 786  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ad44]:443: connect: network is unreachable                    |
| 787  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:ae44]:443: connect: network is unreachable                    |
| 792  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c53]:443: connect: network is unreachable                 |
| 793  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c353]:443: connect: network is unreachable                 |
| 794  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2353]:443: connect: network is unreachable                 |
| 798  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4407::ac40:9052]:443: connect: network is unreachable               |
| 799  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:310d::6812:2bae]:443: connect: network is unreachable               |
| 804  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c87]:443: connect: network is unreachable                 |
| 805  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:2387]:443: connect: network is unreachable                 |
| 806  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c387]:443: connect: network is unreachable                 |
| 810  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23a8]:443: connect: network is unreachable                 |
| 811  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ca8]:443: connect: network is unreachable                 |
| 812  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3a8]:443: connect: network is unreachable                 |
| 815  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 816  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 819  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::ac43:9db6]:443: connect: network is unreachable               |
| 823  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cad]:443: connect: network is unreachable                 |
| 824  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23ad]:443: connect: network is unreachable                 |
| 825  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3ad]:443: connect: network is unreachable                 |
| 832  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:49d8]:443: connect: network is unreachable                 |
| 833  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:659]:443: connect: network is unreachable                  |
| 834  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:759]:443: connect: network is unreachable                  |
| 838  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83be::11:74f]:443: connect: network is unreachable                  |
| 844  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23d1]:443: connect: network is unreachable                 |
| 845  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cd1]:443: connect: network is unreachable                 |
| 846  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3d1]:443: connect: network is unreachable                 |
| 853  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2cc2]:443: connect: network is unreachable                 |
| 854  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3c2]:443: connect: network is unreachable                 |
| 855  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23c2]:443: connect: network is unreachable                 |
| 856  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:83bd::7d8:2b47]:443: connect: network is unreachable                |
| 864  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2c9f]:443: connect: network is unreachable                 |
| 865  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c39f]:443: connect: network is unreachable                 |
| 866  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:239f]:443: connect: network is unreachable                 |
| 878  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:365b]:443: connect: network is unreachable               |
| 894  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:43c]:443: connect: network is unreachable                  |
| 895  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:53c]:443: connect: network is unreachable                  |
| 896  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::ac43:465c]:443: connect: network is unreachable                 |
| 909  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3102::6812:29be]:443: connect: network is unreachable               |
| 910  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: connect: network is unreachable               |
| 916  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:58::a29f:2ccb]:443: connect: network is unreachable                 |
| 917  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2803:f800:50::6ca2:c3cb]:443: connect: network is unreachable                 |
| 918  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:23cb]:443: connect: network is unreachable                 |
| 923  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:af96:6677:59bf:faec]:443: connect: network is unreachable |
| 924  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:3121:c6d4:4130:7992:df42:f04c]:443: connect: network is unreachable |
| 925  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:964f::6e2c:588e]:443: connect: network is unreachable               |
| 934  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3033::ac43:bbfb]:443: connect: network is unreachable               |
| 935  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3037::6815:7c1]:443: connect: network is unreachable                |

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 931  | 172.67.49.134    | 172.67.49.134  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout  |
| 937  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 943  | cfip.xxxxxxxx.tk | 104.20.255.53  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout  |
| 952  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **网络不可达**: 258 次 (98.5%)
- **连接超时**: 4 次 (1.5%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 172.67（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 262 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 4 次，IPv6失败 258 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cloudflare.182682.xyz (5次),
wilson.ns.cloudflare.com (3次), trevor.ns.cloudflare.com
(3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 690 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP          | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | --------------- | ------ | ---- | ------- | -------- | ---------- |
| 906  | 104.18.189.153                        | 104.18.189.153  | IPv4   | h3   | ✅ 成功 | 29       | cloudflare |
| 193  | ct.877774.xyz                         | 172.64.229.217  | IPv4   | h3   | ✅ 成功 | 30       | cloudflare |
| 266  | 172.67.78.23                          | 172.67.78.23    | IPv4   | h3   | ✅ 成功 | 31       | cloudflare |
| 286  | 162.159.0.79                          | 162.159.0.79    | IPv4   | h3   | ✅ 成功 | 31       | cloudflare |
| 595  | 172.64.152.215                        | 172.64.152.215  | IPv4   | h3   | ✅ 成功 | 31       | cloudflare |
| 920  | 104.17.139.37                         | 104.17.139.37   | IPv4   | h3   | ✅ 成功 | 31       | cloudflare |
| 209  | 162.159.43.50                         | 162.159.43.50   | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 423  | 162.159.34.55                         | 162.159.34.55   | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 503  | 172.67.75.11                          | 172.67.75.11    | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 897  | 104.18.81.19                          | 104.18.81.19    | IPv4   | h3   | ✅ 成功 | 32       | cloudflare |
| 130  | 104.18.254.88                         | 104.18.254.88   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 206  | 172.64.52.181                         | 172.64.52.181   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 283  | 172.64.229.185                        | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 425  | 104.18.35.166                         | 104.18.35.166   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 446  | 104.17.50.237                         | 104.17.50.237   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 568  | 172.64.53.165                         | 172.64.53.165   | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 818  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182  | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 912  | 104.19.154.200                        | 104.19.154.200  | IPv4   | h3   | ✅ 成功 | 33       | cloudflare |
| 153  | cu.877774.xyz                         | 104.18.42.54    | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 405  | 104.26.8.148                          | 104.26.8.148    | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 455  | 104.16.147.114                        | 104.16.147.114  | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 496  | 104.18.42.61                          | 104.18.42.61    | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 497  | 104.18.40.216                         | 104.18.40.216   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 536  | 104.17.53.129                         | 104.17.53.129   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 550  | 162.159.1.145                         | 162.159.1.145   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 613  | www.visa.cn                           | 162.159.152.2   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 624  | na.877774.xyz                         | 104.18.187.25   | IPv4   | h3   | ✅ 成功 | 34       | cloudflare |
| 200  | ct.877774.xyz                         | 172.64.229.174  | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 377  | 104.16.255.1                          | 104.16.255.1    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 436  | 104.26.12.113                         | 104.26.12.113   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 608  | 172.67.120.0                          | 172.67.120.0    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 623  | na.877774.xyz                         | 104.18.38.235   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 663  | decker.ns.cloudflare.com              | 108.162.195.155 | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 681  | www.ipchicken.com                     | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 729  | singapore.com                         | 104.26.12.140   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 752  | xn--b6gac.eu.org                      | 172.67.153.253  | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 755  | braden.ns.cloudflare.com              | 108.162.195.169 | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 802  | otto.ns.cloudflare.com                | 172.64.35.135   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 814  | cf.877774.xyz                         | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 847  | 162.159.24.131                        | 162.159.24.131  | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 870  | 104.26.5.134                          | 104.26.5.134    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 883  | www.csgo.com                          | 195.85.59.161   | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 891  | japan.com                             | 172.67.70.92    | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 901  | 104.17.167.134                        | 104.17.167.134  | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 904  | 104.18.223.253                        | 104.18.223.253  | IPv4   | h3   | ✅ 成功 | 35       | cloudflare |
| 259  | 172.64.229.149                        | 172.64.229.149  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 321  | 104.25.240.21                         | 104.25.240.21   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 403  | 104.17.60.233                         | 104.17.60.233   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 433  | 104.18.44.148                         | 104.18.44.148   | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 487  | 172.64.145.242                        | 172.64.145.242  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 599  | 172.64.145.108                        | 172.64.145.108  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 718  | bestcf.030101.xyz                     | 104.17.206.188  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 724  | asia.877774.xyz                       | 104.17.142.146  | IPv4   | h3   | ✅ 成功 | 36       | cloudflare |
| 419  | 104.19.50.35                          | 104.19.50.35    | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 442  | 104.20.17.233                         | 104.20.17.233   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 526  | 162.159.61.106                        | 162.159.61.106  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 548  | 172.64.49.146                         | 172.64.49.146   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 874  | eur.877774.xyz                        | 104.21.26.150   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 890  | 198.41.208.15                         | 198.41.208.15   | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 948  | cfip.xxxxxxxx.tk                      | 190.93.244.201  | IPv4   | h3   | ✅ 成功 | 37       | cloudflare |
| 411  | 104.26.4.190                          | 104.26.4.190    | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 474  | 104.26.13.110                         | 104.26.13.110   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 796  | cf.090227.xyz                         | 104.18.43.174   | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 826  | 198.62.62.4                           | 198.62.62.4     | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 858  | 162.159.136.89                        | 162.159.136.89  | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 872  | 162.159.128.253                       | 162.159.128.253 | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 898  | 104.18.166.129                        | 104.18.166.129  | IPv4   | h3   | ✅ 成功 | 38       | cloudflare |
| 175  | 104.20.22.91                          | 104.20.22.91    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 279  | 104.25.241.19                         | 104.25.241.19   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 311  | 172.67.77.104                         | 172.67.77.104   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 315  | 172.67.65.44                          | 172.67.65.44    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 462  | 104.17.53.25                          | 104.17.53.25    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 543  | 104.16.155.230                        | 104.16.155.230  | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 588  | 172.67.65.150                         | 172.67.65.150   | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 766  | cf.zhetengsha.eu.org                  | 104.18.35.15    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 869  | www.visa.com.hk                       | 104.18.20.69    | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 903  | 104.16.65.1                           | 104.16.65.1     | IPv4   | h3   | ✅ 成功 | 39       | cloudflare |
| 515  | 104.17.189.30                         | 104.17.189.30   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 739  | silkbook.com                          | 104.26.9.160    | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 915  | abdullah.ns.cloudflare.com            | 172.64.35.203   | IPv4   | h3   | ✅ 成功 | 40       | cloudflare |
| 587  | 104.20.25.181                         | 104.20.25.181   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 600  | 104.26.11.160                         | 104.26.11.160   | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 645  | toy-people.com                        | 172.67.72.18    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 950  | cfip.xxxxxxxx.tk                      | 104.21.91.19    | IPv4   | h3   | ✅ 成功 | 41       | cloudflare |
| 36   | cloudflare.182682.xyz                 | 104.17.25.173   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 369  | 172.67.65.159                         | 172.67.65.159   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 414  | 104.20.19.201                         | 104.20.19.201   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 514  | 104.17.25.241                         | 104.17.25.241   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 521  | 162.159.21.222                        | 162.159.21.222  | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 695  | www.whatismyip.com                    | 104.26.12.23    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 842  | julio.ns.cloudflare.com               | 172.64.35.209   | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 919  | 104.17.162.3                          | 104.17.162.3    | IPv4   | h3   | ✅ 成功 | 42       | cloudflare |
| 272  | 172.67.68.110                         | 172.67.68.110   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 351  | 104.18.47.193                         | 104.18.47.193   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 413  | 104.26.12.227                         | 104.26.12.227   | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 670  | rustam.ns.cloudflare.com              | 108.162.195.148 | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 945  | cfip.xxxxxxxx.tk                      | 188.114.96.125  | IPv4   | h3   | ✅ 成功 | 43       | cloudflare |
| 98   | cmcc.877774.xyz                       | 104.16.148.1    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 244  | 104.25.254.89                         | 104.25.254.89   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 352  | 172.64.53.103                         | 172.64.53.103   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 566  | 172.64.154.226                        | 172.64.154.226  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 607  | 104.26.6.171                          | 104.26.6.171    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 747  | ip.gs                                 | 172.67.160.28   | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 868  | www.visa.com.hk                       | 104.18.21.69    | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 877  | ifconfig.co                           | 172.67.168.106  | IPv4   | h3   | ✅ 成功 | 44       | cloudflare |
| 74   | iplocation.io                         | 104.26.10.222   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 119  | cmcc.877774.xyz                       | 104.16.149.9    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 205  | 172.64.144.132                        | 172.64.144.132  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 280  | 104.16.247.125                        | 104.16.247.125  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 457  | 104.17.30.164                         | 104.17.30.164   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 472  | 162.159.7.12                          | 162.159.7.12    | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 610  | 172.67.243.218                        | 172.67.243.218  | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 785  | www.digitalocean.com                  | 104.19.174.68   | IPv4   | h3   | ✅ 成功 | 45       | cloudflare |
| 182  | 104.20.19.180                         | 104.20.19.180   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 373  | 104.17.169.180                        | 104.17.169.180  | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 421  | 104.16.155.76                         | 104.16.155.76   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 456  | 104.16.153.12                         | 104.16.153.12   | IPv4   | h3   | ✅ 成功 | 46       | cloudflare |
| 82   | 104.25.250.174                        | 104.25.250.174  | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 104  | cmcc.877774.xyz                       | 104.16.148.7    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 221  | www.okcupid.com                       | 104.17.48.63    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 267  | 104.26.6.238                          | 104.26.6.238    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 310  | 172.67.64.123                         | 172.67.64.123   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 367  | 104.26.15.85                          | 104.26.15.85    | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 379  | 104.17.156.81                         | 104.17.156.81   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 501  | 104.20.21.147                         | 104.20.21.147   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 730  | singapore.com                         | 104.26.13.140   | IPv4   | h3   | ✅ 成功 | 47       | cloudflare |
| 89   | 162.159.58.17                         | 162.159.58.17   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 169  | 162.159.20.46                         | 162.159.20.46   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 171  | 172.67.70.56                          | 172.67.70.56    | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 208  | 172.64.151.253                        | 172.64.151.253  | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 261  | 104.26.1.55                           | 104.26.1.55     | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 335  | 162.159.33.28                         | 162.159.33.28   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 537  | 104.26.7.7                            | 104.26.7.7      | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 626  | whatismyipaddress.com                 | 104.19.223.79   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 691  | zread.ai                              | 104.21.76.240   | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 841  | julio.ns.cloudflare.com               | 108.162.195.209 | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 893  | japan.com                             | 104.26.5.60     | IPv4   | h3   | ✅ 成功 | 48       | cloudflare |
| 58   | 104.17.111.150                        | 104.17.111.150  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 61   | ipv4.ip.sb                            | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 91   | 172.64.53.202                         | 172.64.53.202   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 102  | cmcc.877774.xyz                       | 104.16.148.5    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 146  | 172.64.53.41                          | 172.64.53.41    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 149  | 172.64.52.224                         | 172.64.52.224   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 161  | 108.162.198.198                       | 108.162.198.198 | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 224  | www.okcupid.com                       | 104.16.239.254  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 237  | 172.67.75.212                         | 172.67.75.212   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 241  | 104.26.3.117                          | 104.26.3.117    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 260  | 162.159.21.16                         | 162.159.21.16   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 287  | 172.64.53.15                          | 172.64.53.15    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 301  | 172.64.53.195                         | 172.64.53.195   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 443  | 104.26.6.159                          | 104.26.6.159    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 506  | 104.20.25.82                          | 104.20.25.82    | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 538  | 104.19.144.159                        | 104.19.144.159  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 612  | www.visa.cn                           | 162.159.153.2   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 621  | www.visa.com.sg                       | 104.18.12.229   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 640  | pranab.ns.cloudflare.com              | 108.162.195.199 | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 716  | 104.16.223.179                        | 104.16.223.179  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 745  | 162.159.133.85                        | 162.159.133.85  | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 778  | saas.sin.fan                          | 162.159.36.20   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 797  | cf.090227.xyz                         | 172.64.144.82   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 820  | ashton.ns.cloudflare.com              | 172.64.35.173   | IPv4   | h3   | ✅ 成功 | 49       | cloudflare |
| 20   | trevor.ns.cloudflare.com              | 108.162.195.154 | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 44   | cf.877771.xyz                         | 172.67.152.183  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 76   | iplocation.io                         | 172.67.70.100   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 122  | huxley.ns.cloudflare.com              | 172.64.35.188   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 204  | 172.64.153.183                        | 172.64.153.183  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 319  | 104.26.4.213                          | 104.26.4.213    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 359  | 162.159.46.238                        | 162.159.46.238  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 361  | 104.18.47.253                         | 104.18.47.253   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 371  | 104.20.30.198                         | 104.20.30.198   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 386  | 172.64.53.0                           | 172.64.53.0     | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 410  | 172.67.67.5                           | 172.67.67.5     | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 471  | 172.64.52.15                          | 172.64.52.15    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 504  | 172.67.73.120                         | 172.67.73.120   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 546  | 104.17.214.136                        | 104.17.214.136  | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 762  | cloudflare-ip.mofashi.ltd             | 104.21.72.233   | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 774  | gamer.com.tw                          | 104.18.3.197    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 809  | damien.ns.cloudflare.com              | 108.162.195.168 | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 829  | dnschecker.org                        | 104.26.6.89     | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 837  | 172.64.91.69                          | 172.64.91.69    | IPv4   | h3   | ✅ 成功 | 50       | cloudflare |
| 69   | 172.67.75.172                         | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 85   | 104.18.37.13                          | 104.18.37.13    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 105  | cmcc.877774.xyz                       | 104.16.148.8    | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 128  | 198.41.222.191                        | 198.41.222.191  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 186  | 104.17.56.208                         | 104.17.56.208   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 187  | 104.25.247.78                         | 104.25.247.78   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 191  | 104.25.249.225                        | 104.25.249.225  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 232  | sullivan.ns.cloudflare.com            | 172.64.35.161   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 424  | 104.17.100.254                        | 104.17.100.254  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 426  | 104.16.157.50                         | 104.16.157.50   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 463  | 104.16.251.143                        | 104.16.251.143  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 529  | 104.20.25.161                         | 104.20.25.161   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 534  | 172.67.79.150                         | 172.67.79.150   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 551  | 104.18.32.174                         | 104.18.32.174   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 579  | 104.20.28.239                         | 104.20.28.239   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 601  | 162.159.36.223                        | 162.159.36.223  | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 821  | ashton.ns.cloudflare.com              | 108.162.195.173 | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 860  | 162.159.58.65                         | 162.159.58.65   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 888  | 172.64.82.114                         | 172.64.82.114   | IPv4   | h3   | ✅ 成功 | 51       | cloudflare |
| 100  | cmcc.877774.xyz                       | 104.16.148.3    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 103  | cmcc.877774.xyz                       | 104.16.148.6    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 114  | cmcc.877774.xyz                       | 104.16.149.4    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 198  | ct.877774.xyz                         | 172.64.229.161  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 275  | 172.64.146.137                        | 172.64.146.137  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 284  | 104.17.171.88                         | 104.17.171.88   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 285  | 104.18.44.159                         | 104.18.44.159   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 441  | 172.67.68.252                         | 172.67.68.252   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 469  | 108.162.198.69                        | 108.162.198.69  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 470  | 162.159.19.219                        | 162.159.19.219  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 517  | 104.17.209.79                         | 104.17.209.79   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 518  | 172.64.52.67                          | 172.64.52.67    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 564  | 104.18.42.106                         | 104.18.42.106   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 592  | 172.64.154.113                        | 172.64.154.113  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 620  | www.visa.com.sg                       | 104.18.13.229   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 725  | palera.in                             | 104.21.58.72    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 800  | 172.67.79.211                         | 172.67.79.211   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 840  | 104.26.13.31                          | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 905  | 104.16.105.166                        | 104.16.105.166  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 922  | 104.19.220.22                         | 104.19.220.22   | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 926  | 104.19.212.207                        | 104.19.212.207  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 933  | stock.hostmonit.com                   | 172.67.187.251  | IPv4   | h3   | ✅ 成功 | 52       | cloudflare |
| 53   | 104.17.97.228                         | 104.17.97.228   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 99   | cmcc.877774.xyz                       | 104.16.148.2    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 110  | cmcc.877774.xyz                       | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 115  | cmcc.877774.xyz                       | 104.16.149.5    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 150  | www.4chan.org                         | 104.16.228.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 226  | 172.64.229.15                         | 172.64.229.15   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 282  | 104.25.255.103                        | 104.25.255.103  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 322  | 104.20.18.125                         | 104.20.18.125   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 353  | 104.18.40.200                         | 104.18.40.200   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 356  | 162.159.6.115                         | 162.159.6.115   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 365  | 172.64.150.30                         | 172.64.150.30   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 368  | 172.67.64.214                         | 172.67.64.214   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 382  | 104.17.168.159                        | 104.17.168.159  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 417  | 104.16.148.187                        | 104.16.148.187  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 418  | 104.18.40.39                          | 104.18.40.39    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 440  | 104.26.2.166                          | 104.26.2.166    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 482  | 162.159.41.141                        | 162.159.41.141  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 513  | 104.17.215.66                         | 104.17.215.66   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 516  | 104.19.153.47                         | 104.19.153.47   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 589  | 104.26.14.117                         | 104.26.14.117   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 591  | 104.26.0.210                          | 104.26.0.210    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 603  | 104.18.41.101                         | 104.18.41.101   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 700  | benedict.ns.cloudflare.com            | 108.162.195.205 | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 735  | 104.16.61.163                         | 104.16.61.163   | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 770  | fbi.gov                               | 104.16.148.244  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 871  | 162.159.137.204                       | 162.159.137.204 | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 913  | abdullah.ns.cloudflare.com            | 108.162.195.203 | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 938  | cfip.xxxxxxxx.tk                      | 198.41.214.141  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 939  | cfip.xxxxxxxx.tk                      | 104.16.241.229  | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 951  | cfip.xxxxxxxx.tk                      | 104.25.105.1    | IPv4   | h3   | ✅ 成功 | 53       | cloudflare |
| 75   | iplocation.io                         | 104.26.11.222   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 88   | 162.159.12.120                        | 162.159.12.120  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 129  | 162.159.1.39                          | 162.159.1.39    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 172  | 172.67.67.0                           | 172.67.67.0     | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 174  | 104.20.24.244                         | 104.20.24.244   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 270  | 172.67.72.250                         | 172.67.72.250   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 273  | 172.67.76.61                          | 172.67.76.61    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 317  | 172.67.73.129                         | 172.67.73.129   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 408  | 172.67.77.196                         | 172.67.77.196   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 486  | 172.64.151.235                        | 172.64.151.235  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 494  | 162.159.22.29                         | 162.159.22.29   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 508  | 104.17.170.137                        | 104.17.170.137  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 586  | 104.26.6.247                          | 104.26.6.247    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 686  | dylan.ns.cloudflare.com               | 172.64.35.187   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 711  | ip.sb                                 | 104.26.13.31    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 757  | braden.ns.cloudflare.com              | 172.64.35.169   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 776  | 172.64.151.55                         | 172.64.151.55   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 789  | bowen.ns.cloudflare.com               | 172.64.35.83    | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 836  | 162.159.61.183                        | 162.159.61.183  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 859  | 104.19.223.58                         | 104.19.223.58   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 907  | www.wto.org                           | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 927  | 104.18.255.167                        | 104.18.255.167  | IPv4   | h3   | ✅ 成功 | 54       | cloudflare |
| 27   | cf.0sm.com                            | 104.21.7.133    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 95   | 103.160.204.59                        | 103.160.204.59  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 101  | cmcc.877774.xyz                       | 104.16.148.4    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 106  | cmcc.877774.xyz                       | 104.16.148.9    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 118  | cmcc.877774.xyz                       | 104.16.149.8    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 165  | www.pcmag.com                         | 104.16.20.118   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 236  | 172.67.73.196                         | 172.67.73.196   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 276  | 104.17.97.146                         | 104.17.97.146   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 375  | 172.67.72.254                         | 172.67.72.254   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 429  | 104.18.37.110                         | 104.18.37.110   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 438  | 162.159.19.37                         | 162.159.19.37   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 478  | 172.64.52.90                          | 172.64.52.90    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 511  | 104.17.115.224                        | 104.17.115.224  | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 539  | 104.26.4.4                            | 104.26.4.4      | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 647  | toy-people.com                        | 104.26.2.36     | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 740  | silkbook.com                          | 172.67.75.208   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 875  | eur.877774.xyz                        | 104.21.29.164   | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 882  | www.csgo.com                          | 195.85.59.95    | IPv4   | h3   | ✅ 成功 | 55       | cloudflare |
| 13   | comicabc.com                          | 172.67.174.21   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 16   | www.udemy.com                         | 104.16.143.237  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 35   | cloudflare.182682.xyz                 | 104.16.250.22   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 86   | 104.26.6.112                          | 104.26.6.112    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 107  | cmcc.877774.xyz                       | 104.16.148.10   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 120  | cmcc.877774.xyz                       | 104.16.149.10   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 131  | icook.hk                              | 172.67.161.104  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 163  | 172.64.229.156                        | 172.64.229.156  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 190  | 104.25.240.123                        | 104.25.240.123  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 201  | 104.25.254.14                         | 104.25.254.14   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 202  | 162.159.33.191                        | 162.159.33.191  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 239  | 172.67.65.81                          | 172.67.65.81    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 243  | 104.25.244.87                         | 104.25.244.87   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 257  | 108.162.198.232                       | 108.162.198.232 | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 268  | 104.26.12.33                          | 104.26.12.33    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 318  | 172.67.76.20                          | 172.67.76.20    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 357  | 162.159.21.116                        | 162.159.21.116  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 366  | 172.67.68.211                         | 172.67.68.211   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 383  | 104.18.39.15                          | 104.18.39.15    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 402  | 172.64.52.110                         | 172.64.52.110   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 465  | 104.17.21.106                         | 104.17.21.106   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 535  | 172.67.67.152                         | 172.67.67.152   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 542  | 104.17.110.226                        | 104.17.110.226  | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 597  | 172.64.41.216                         | 172.64.41.216   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 625  | whatismyipaddress.com                 | 104.19.222.79   | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 651  | time.is                               | 104.26.12.54    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 899  | 104.18.89.52                          | 104.18.89.52    | IPv4   | h3   | ✅ 成功 | 56       | cloudflare |
| 38   | cloudflare.182682.xyz                 | 104.21.224.5    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 59   | 172.64.153.140                        | 172.64.153.140  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 152  | cu.877774.xyz                         | 172.64.145.202  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 178  | tasteatlas.com                        | 104.17.37.105   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 199  | ct.877774.xyz                         | 172.64.229.185  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 227  | 104.20.24.239                         | 104.20.24.239   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 324  | 104.25.243.36                         | 104.25.243.36   | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 334  | 108.162.198.206                       | 108.162.198.206 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 420  | 104.17.119.199                        | 104.17.119.199  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 428  | 162.159.42.140                        | 162.159.42.140  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 447  | 104.26.4.44                           | 104.26.4.44     | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 485  | 108.162.198.170                       | 108.162.198.170 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 659  | cris.ns.cloudflare.com                | 108.162.195.202 | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 683  | www.ipchicken.com                     | 104.26.7.112    | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 722  | asia.877774.xyz                       | 104.16.211.153  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 889  | 198.41.194.162                        | 198.41.194.162  | IPv4   | h3   | ✅ 成功 | 57       | cloudflare |
| 37   | cloudflare.182682.xyz                 | 104.18.185.26   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 92   | 172.67.110.232                        | 172.67.110.232  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 188  | 104.25.244.36                         | 104.25.244.36   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 281  | 104.17.129.66                         | 104.17.129.66   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 360  | 108.162.192.66                        | 108.162.192.66  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 412  | 104.20.21.161                         | 104.20.21.161   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 509  | 104.17.211.218                        | 104.17.211.218  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 522  | 172.64.53.40                          | 172.64.53.40    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 544  | 104.16.251.254                        | 104.16.251.254  | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 652  | time.is                               | 172.67.68.157   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 653  | time.is                               | 104.26.13.54    | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 706  | freeyx.cloudflare88.eu.org            | 141.101.121.247 | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 857  | 104.18.78.214                         | 104.18.78.214   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 886  | 162.159.140.116                       | 162.159.140.116 | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 949  | cfip.xxxxxxxx.tk                      | 190.93.246.67   | IPv4   | h3   | ✅ 成功 | 58       | cloudflare |
| 21   | trevor.ns.cloudflare.com              | 172.64.35.154   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 94   | shopify.com                           | 23.227.38.33    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 113  | cmcc.877774.xyz                       | 104.16.149.3    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 117  | cmcc.877774.xyz                       | 104.16.149.7    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 156  | www.hugedomains.com                   | 172.67.70.191   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 251  | 104.17.62.194                         | 104.17.62.194   | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 502  | 104.26.2.2                            | 104.26.2.2      | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 505  | 104.26.5.101                          | 104.26.5.101    | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 527  | 108.162.198.148                       | 108.162.198.148 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 771  | fbi.gov                               | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 803  | otto.ns.cloudflare.com                | 108.162.195.135 | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 879  | 172.67.181.209                        | 172.67.181.209  | IPv4   | h3   | ✅ 成功 | 59       | cloudflare |
| 111  | cmcc.877774.xyz                       | 104.16.149.1    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 170  | 104.20.31.132                         | 104.20.31.132   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 173  | 104.26.11.33                          | 104.26.11.33    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 242  | 172.67.77.185                         | 172.67.77.185   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 328  | 104.25.254.47                         | 104.25.254.47   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 389  | 172.64.50.51                          | 172.64.50.51    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 432  | 108.162.195.1                         | 108.162.195.1   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 439  | 162.159.62.6                          | 162.159.62.6    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 593  | 104.25.253.253                        | 104.25.253.253  | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 780  | icook.tw                              | 104.20.28.74    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 788  | 104.17.68.85                          | 104.17.68.85    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 830  | dnschecker.org                        | 172.67.73.216   | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 929  | 104.18.42.26                          | 104.18.42.26    | IPv4   | h3   | ✅ 成功 | 60       | cloudflare |
| 62   | ipv4.ip.sb                            | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 176  | 172.67.72.212                         | 172.67.72.212   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 245  | 104.25.246.24                         | 104.25.246.24   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 248  | 104.25.242.249                        | 104.25.242.249  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 265  | 104.26.14.88                          | 104.26.14.88    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 313  | 104.20.20.156                         | 104.20.20.156   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 325  | 104.25.245.173                        | 104.25.245.173  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 336  | 172.64.229.82                         | 172.64.229.82   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 364  | 104.18.45.95                          | 104.18.45.95    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 385  | 104.17.105.198                        | 104.17.105.198  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 520  | 162.159.42.146                        | 162.159.42.146  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 545  | 104.18.44.25                          | 104.18.44.25    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 571  | 172.64.52.189                         | 172.64.52.189   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 630  | moura.ns.cloudflare.com               | 172.64.35.217   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 867  | 172.64.229.7                          | 172.64.229.7    | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 876  | eur.877774.xyz                        | 104.21.47.209   | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 944  | cfip.xxxxxxxx.tk                      | 104.17.127.110  | IPv4   | h3   | ✅ 成功 | 61       | cloudflare |
| 48   | ipinfo.in                             | 104.21.21.129   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 222  | www.okcupid.com                       | 104.16.144.63   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 355  | 172.64.146.121                        | 172.64.146.121  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 437  | 172.64.53.144                         | 172.64.53.144   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 493  | 162.159.1.111                         | 162.159.1.111   | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 604  | 172.64.32.77                          | 172.64.32.77    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 710  | ip.sb                                 | 104.26.12.31    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 902  | 162.159.140.85                        | 162.159.140.85  | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 932  | stock.hostmonit.com                   | 104.21.7.193    | IPv4   | h3   | ✅ 成功 | 62       | cloudflare |
| 12   | comicabc.com                          | 104.21.64.10    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 68   | 104.16.45.84                          | 104.16.45.84    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 168  | 162.159.0.41                          | 162.159.0.41    | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 179  | tasteatlas.com                        | 104.17.36.105   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 300  | 108.162.198.223                       | 108.162.198.223 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 427  | 104.17.101.208                        | 104.17.101.208  | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 448  | 172.67.76.195                         | 172.67.76.195   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 507  | 104.26.10.239                         | 104.26.10.239   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 616  | craig.ns.cloudflare.com               | 108.162.195.192 | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 622  | na.877774.xyz                         | 104.19.74.233   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 690  | zread.ai                              | 172.67.202.78   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 839  | 172.64.48.226                         | 172.64.48.226   | IPv4   | h3   | ✅ 成功 | 63       | cloudflare |
| 26   | cf.0sm.com                            | 172.67.187.145  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 54   | 104.25.241.198                        | 104.25.241.198  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 71   | www.gov.ua                            | 104.21.23.72    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 97   | cmcc.877774.xyz                       | 104.16.149.244  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 240  | 104.20.22.141                         | 104.20.22.141   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 258  | 172.64.52.194                         | 172.64.52.194   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 302  | 172.64.152.85                         | 172.64.152.85   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 320  | 172.67.79.218                         | 172.67.79.218   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 331  | 104.25.246.117                        | 104.25.246.117  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 390  | 108.162.194.125                       | 108.162.194.125 | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 406  | 104.20.26.58                          | 104.20.26.58    | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 466  | 104.16.144.235                        | 104.16.144.235  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 541  | 104.17.153.151                        | 104.17.153.151  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 606  | 162.159.16.136                        | 162.159.16.136  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 696  | www.whatismyip.com                    | 172.67.69.129   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 726  | palera.in                             | 172.67.157.122  | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 744  | 172.64.52.127                         | 172.64.52.127   | IPv4   | h3   | ✅ 成功 | 64       | cloudflare |
| 60   | ipv4.ip.sb                            | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 67   | 104.18.39.196                         | 104.18.39.196   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 81   | 104.18.166.232                        | 104.18.166.232  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 83   | 162.159.3.128                         | 162.159.3.128   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 109  | cmcc.877774.xyz                       | 104.16.148.12   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 305  | 108.162.198.152                       | 108.162.198.152 | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 306  | 172.64.229.106                        | 172.64.229.106  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 380  | 172.64.229.191                        | 172.64.229.191  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 512  | 104.19.34.231                         | 104.19.34.231   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 577  | 162.159.36.205                        | 162.159.36.205  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 665  | decker.ns.cloudflare.com              | 172.64.35.155   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 751  | xn--b6gac.eu.org                      | 104.21.90.78    | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 921  | 104.18.151.172                        | 104.18.151.172  | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 946  | cfip.xxxxxxxx.tk                      | 104.27.21.118   | IPv4   | h3   | ✅ 成功 | 65       | cloudflare |
| 151  | www.4chan.org                         | 104.16.229.229  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 416  | 104.20.18.47                          | 104.20.18.47    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 488  | 104.26.1.181                          | 104.26.1.181    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 500  | 172.64.146.67                         | 172.64.146.67   | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 709  | www.glassdoor.com                     | 104.17.64.70    | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 941  | cfip.xxxxxxxx.tk                      | 190.93.247.169  | IPv4   | h3   | ✅ 成功 | 66       | cloudflare |
| 30   | www.ipget.net                         | 172.67.207.26   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 229  | 172.67.79.249                         | 172.67.79.249   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 299  | 162.159.40.8                          | 162.159.40.8    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 378  | 104.17.101.37                         | 104.17.101.37   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 381  | 104.17.16.248                         | 104.17.16.248   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 384  | 104.17.170.110                        | 104.17.170.110  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 404  | 162.159.0.115                         | 162.159.0.115   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 609  | 104.26.1.194                          | 104.26.1.194    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 631  | moura.ns.cloudflare.com               | 108.162.195.217 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 636  | yx-auto.pages.dev                     | 172.66.44.144   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 717  | 104.17.79.11                          | 104.17.79.11    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 719  | bestcf.030101.xyz                     | 104.19.156.188  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 777  | saas.sin.fan                          | 162.159.36.5    | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 795  | 104.17.142.12                         | 104.17.142.12   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 852  | uriah.ns.cloudflare.com               | 108.162.195.194 | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 863  | lewis.ns.cloudflare.com               | 172.64.35.159   | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 911  | 104.17.142.212                        | 104.17.142.212  | IPv4   | h3   | ✅ 成功 | 67       | cloudflare |
| 247  | 104.25.240.227                        | 104.25.240.227  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 263  | 162.159.6.186                         | 162.159.6.186   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 333  | 162.159.49.244                        | 162.159.49.244  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 540  | 104.20.17.51                          | 104.20.17.51    | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 563  | 172.64.40.196                         | 172.64.40.196   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 565  | 162.159.36.52                         | 162.159.36.52   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 570  | 108.162.198.85                        | 108.162.198.85  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 605  | 104.18.37.177                         | 104.18.37.177   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 615  | craig.ns.cloudflare.com               | 172.64.35.192   | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 675  | kyree.ns.cloudflare.com               | 108.162.195.207 | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 767  | cf.zhetengsha.eu.org                  | 172.64.152.241  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 900  | 198.41.208.224                        | 198.41.208.224  | IPv4   | h3   | ✅ 成功 | 68       | cloudflare |
| 55   | 104.25.244.239                        | 104.25.244.239  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 112  | cmcc.877774.xyz                       | 104.16.149.2    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 256  | 172.64.53.220                         | 172.64.53.220   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 278  | 104.25.247.129                        | 104.25.247.129  | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 415  | 104.20.20.42                          | 104.20.20.42    | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 434  | 162.159.36.26                         | 162.159.36.26   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 635  | yx-auto.pages.dev                     | 172.66.47.112   | IPv4   | h3   | ✅ 成功 | 69       | cloudflare |
| 277  | 104.25.252.135                        | 104.25.252.135  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 312  | 172.64.41.47                          | 172.64.41.47    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 326  | 104.25.250.121                        | 104.25.250.121  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 332  | 104.25.248.93                         | 104.25.248.93   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 481  | 172.64.34.153                         | 172.64.34.153   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 490  | 162.159.3.89                          | 162.159.3.89    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 575  | 172.64.229.134                        | 172.64.229.134  | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 596  | 104.20.20.192                         | 104.20.20.192   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 738  | silkbook.com                          | 104.26.8.160    | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 748  | ip.gs                                 | 104.21.14.176   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 813  | cf.877774.xyz                         | 172.64.146.66   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 851  | uriah.ns.cloudflare.com               | 172.64.35.194   | IPv4   | h3   | ✅ 成功 | 70       | cloudflare |
| 56   | 104.17.193.113                        | 104.17.193.113  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 164  | www.pcmag.com                         | 104.16.21.118   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 195  | ct.877774.xyz                         | 172.64.229.236  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 327  | 104.25.242.137                        | 104.25.242.137  | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 362  | 162.159.13.51                         | 162.159.13.51   | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 533  | 172.67.78.67                          | 172.67.78.67    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 775  | gamer.com.tw                          | 104.18.2.197    | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 831  | dnschecker.org                        | 104.26.7.89     | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 849  | 104.26.4.90                           | 104.26.4.90     | IPv4   | h3   | ✅ 成功 | 71       | cloudflare |
| 34   | cloudflare.182682.xyz                 | 104.21.227.134  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 84   | 162.159.11.128                        | 162.159.11.128  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 223  | www.okcupid.com                       | 104.16.223.254  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 252  | 104.16.245.121                        | 104.16.245.121  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 323  | 104.25.241.235                        | 104.25.241.235  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 363  | 173.245.58.237                        | 173.245.58.237  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 468  | 104.26.3.120                          | 104.26.3.120    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 510  | 172.67.72.36                          | 172.67.72.36    | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 549  | 104.17.187.186                        | 104.17.187.186  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 881  | 104.19.175.123                        | 104.19.175.123  | IPv4   | h3   | ✅ 成功 | 72       | cloudflare |
| 7    | wilson.ns.cloudflare.com              | 172.64.35.110   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 45   | cf.877771.xyz                         | 104.21.80.180   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 116  | cmcc.877774.xyz                       | 104.16.149.6    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 123  | huxley.ns.cloudflare.com              | 108.162.195.188 | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 154  | www.hugedomains.com                   | 104.26.6.37     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 183  | 104.26.5.194                          | 104.26.5.194    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 184  | 104.25.245.215                        | 104.25.245.215  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 492  | 172.64.42.158                         | 172.64.42.158   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 598  | 162.159.6.44                          | 162.159.6.44    | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 646  | toy-people.com                        | 104.26.3.36     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 712  | ip.sb                                 | 172.67.75.172   | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 790  | bowen.ns.cloudflare.com               | 108.162.195.83  | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 892  | japan.com                             | 104.26.4.60     | IPv4   | h3   | ✅ 成功 | 73       | cloudflare |
| 17   | www.udemy.com                         | 104.16.142.237  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 189  | 104.25.241.85                         | 104.25.241.85   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 337  | 162.159.27.183                        | 162.159.27.183  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 370  | 172.67.74.57                          | 172.67.74.57    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 387  | 108.162.198.48                        | 108.162.198.48  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 435  | 104.18.39.228                         | 104.18.39.228   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 499  | 172.64.153.141                        | 172.64.153.141  | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 576  | 104.18.42.16                          | 104.18.42.16    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 594  | 104.26.8.171                          | 104.26.8.171    | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 639  | pranab.ns.cloudflare.com              | 172.64.35.199   | IPv4   | h3   | ✅ 成功 | 74       | cloudflare |
| 49   | ipinfo.in                             | 172.67.198.203  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 249  | 104.17.56.177                         | 104.17.56.177   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 298  | 172.64.157.214                        | 172.64.157.214  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 316  | 104.20.29.234                         | 104.20.29.234   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 464  | 104.17.154.254                        | 104.17.154.254  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 467  | 172.64.147.235                        | 172.64.147.235  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 491  | 162.159.18.240                        | 162.159.18.240  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 547  | 104.17.25.87                          | 104.17.25.87    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 669  | rustam.ns.cloudflare.com              | 172.64.35.148   | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 685  | dylan.ns.cloudflare.com               | 108.162.195.187 | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 737  | 162.159.36.104                        | 162.159.36.104  | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 880  | 104.26.3.162                          | 104.26.3.162    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 887  | www.7749tv.com                        | 104.19.13.63    | IPv4   | h3   | ✅ 成功 | 75       | cloudflare |
| 376  | 104.26.3.176                          | 104.26.3.176    | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 454  | 104.17.24.232                         | 104.17.24.232   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 484  | 172.64.53.181                         | 172.64.53.181   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 569  | 104.18.40.202                         | 104.18.40.202   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 682  | www.ipchicken.com                     | 172.67.68.101   | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 942  | cfip.xxxxxxxx.tk                      | 104.16.232.223  | IPv4   | h3   | ✅ 成功 | 76       | cloudflare |
| 96   | cmcc.877774.xyz                       | 104.16.149.12   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 197  | ct.877774.xyz                         | 172.64.229.195  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 677  | kyree.ns.cloudflare.com               | 172.64.35.207   | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 694  | www.whatismyip.com                    | 104.26.13.23    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 827  | 104.18.37.40                          | 104.18.37.40    | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 835  | 104.19.148.121                        | 104.19.148.121  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 848  | 173.245.49.194                        | 173.245.49.194  | IPv4   | h3   | ✅ 成功 | 77       | cloudflare |
| 64   | steamdb.info                          | 104.20.34.212   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 238  | 104.26.5.53                           | 104.26.5.53     | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 873  | cfip.1323123.xyz                      | 104.16.133.220  | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 885  | 104.26.8.117                          | 104.26.8.117    | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 930  | 104.31.16.158                         | 104.31.16.158   | IPv4   | h3   | ✅ 成功 | 78       | cloudflare |
| 63   | steamdb.info                          | 172.66.175.250  | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 108  | cmcc.877774.xyz                       | 104.16.148.11   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 225  | www.okcupid.com                       | 104.18.160.63   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 590  | 104.26.1.88                           | 104.26.1.88     | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 731  | singapore.com                         | 172.67.75.194   | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 828  | 172.64.35.24                          | 172.64.35.24    | IPv4   | h3   | ✅ 成功 | 79       | cloudflare |
| 3    | 104.18.14.76                          | 104.18.14.76    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 57   | 104.18.172.20                         | 104.18.172.20   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 230  | sullivan.ns.cloudflare.com            | 108.162.195.161 | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 314  | 172.64.40.68                          | 172.64.40.68    | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 358  | 162.159.17.243                        | 162.159.17.243  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 372  | 104.20.22.185                         | 104.20.22.185   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 658  | cris.ns.cloudflare.com                | 172.64.35.202   | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 761  | cloudflare-ip.mofashi.ltd             | 172.67.155.172  | IPv4   | h3   | ✅ 成功 | 80       | cloudflare |
| 6    | wilson.ns.cloudflare.com              | 108.162.195.110 | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 31   | www.ipget.net                         | 104.21.15.212   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 196  | ct.877774.xyz                         | 172.64.229.173  | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 274  | 172.67.64.116                         | 172.67.64.116   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 374  | 104.20.24.107                         | 104.20.24.107   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 445  | 172.67.70.253                         | 172.67.70.253   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 519  | 104.19.35.242                         | 104.19.35.242   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 552  | 104.18.36.1                           | 104.18.36.1     | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 567  | 104.18.47.46                          | 104.18.47.46    | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 908  | www.wto.org                           | 104.18.41.190   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 947  | cfip.xxxxxxxx.tk                      | 104.18.228.35   | IPv4   | h3   | ✅ 成功 | 81       | cloudflare |
| 121  | cmcc.877774.xyz                       | 104.16.149.11   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 185  | 104.18.148.235                        | 104.18.148.235  | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 250  | 104.17.143.82                         | 104.17.143.82   | IPv4   | h3   | ✅ 成功 | 82       | cloudflare |
| 203  | 104.25.250.205                        | 104.25.250.205  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 271  | 104.20.29.62                          | 104.20.29.62    | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 444  | 104.20.16.244                         | 104.20.16.244   | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 884  | 108.162.198.54                        | 108.162.198.54  | IPv4   | h3   | ✅ 成功 | 83       | cloudflare |
| 70   | www.gov.ua                            | 172.67.209.127  | IPv4   | h3   | ✅ 成功 | 84       | cloudflare |
| 304  | 172.64.52.150                         | 172.64.52.150   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 422  | 104.19.44.238                         | 104.19.44.238   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 453  | 104.26.8.192                          | 104.26.8.192    | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 781  | icook.tw                              | 172.66.158.115  | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 928  | 104.17.69.244                         | 104.17.69.244   | IPv4   | h3   | ✅ 成功 | 85       | cloudflare |
| 135  | 108.162.198.70                        | 108.162.198.70  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 307  | 162.159.39.196                        | 162.159.39.196  | IPv4   | h3   | ✅ 成功 | 86       | cloudflare |
| 801  | otto.ns.cloudflare.com                | 162.159.44.135  | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 808  | damien.ns.cloudflare.com              | 172.64.35.168   | IPv4   | h3   | ✅ 成功 | 87       | cloudflare |
| 723  | asia.877774.xyz                       | 104.17.139.62   | IPv4   | h3   | ✅ 成功 | 88       | cloudflare |
| 4    | 172.64.154.18                         | 172.64.154.18   | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 756  | braden.ns.cloudflare.com              | 162.159.44.169  | IPv4   | h3   | ✅ 成功 | 89       | cloudflare |
| 80   | 104.25.245.233                        | 104.25.245.233  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 177  | 104.26.4.135                          | 104.26.4.135    | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 262  | 162.159.38.52                         | 162.159.38.52   | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 574  | 162.159.44.101                        | 162.159.44.101  | IPv4   | h3   | ✅ 成功 | 90       | cloudflare |
| 194  | ct.877774.xyz                         | 172.64.229.44   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 253  | 162.159.44.246                        | 162.159.44.246  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 388  | 162.159.44.176                        | 162.159.44.176  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 657  | cris.ns.cloudflare.com                | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 664  | decker.ns.cloudflare.com              | 162.159.44.155  | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 784  | www.digitalocean.com                  | 104.19.173.68   | IPv4   | h3   | ✅ 成功 | 91       | cloudflare |
| 207  | 162.159.39.74                         | 162.159.39.74   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 264  | 162.159.39.26                         | 162.159.39.26   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 489  | 162.159.38.35                         | 162.159.38.35   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 702  | benedict.ns.cloudflare.com            | 172.64.35.205   | IPv4   | h3   | ✅ 成功 | 92       | cloudflare |
| 5    | 172.67.106.26                         | 172.67.106.26   | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 124  | huxley.ns.cloudflare.com              | 162.159.44.188  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 843  | julio.ns.cloudflare.com               | 162.159.44.209  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 862  | lewis.ns.cloudflare.com               | 162.159.44.159  | IPv4   | h3   | ✅ 成功 | 93       | cloudflare |
| 228  | 162.159.38.83                         | 162.159.38.83   | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 530  | 162.159.45.165                        | 162.159.45.165  | IPv4   | h3   | ✅ 成功 | 94       | cloudflare |
| 572  | 162.159.45.65                         | 162.159.45.65   | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 807  | damien.ns.cloudflare.com              | 162.159.44.168  | IPv4   | h3   | ✅ 成功 | 95       | cloudflare |
| 329  | 104.25.252.192                        | 104.25.252.192  | IPv4   | h3   | ✅ 成功 | 96       | cloudflare |
| 409  | 162.159.39.62                         | 162.159.39.62   | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 614  | craig.ns.cloudflare.com               | 162.159.44.192  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 822  | ashton.ns.cloudflare.com              | 162.159.44.173  | IPv4   | h3   | ✅ 成功 | 97       | cloudflare |
| 480  | 162.159.39.177                        | 162.159.39.177  | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 791  | bowen.ns.cloudflare.com               | 162.159.44.83   | IPv4   | h3   | ✅ 成功 | 98       | cloudflare |
| 162  | 162.159.44.202                        | 162.159.44.202  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 914  | abdullah.ns.cloudflare.com            | 162.159.44.203  | IPv4   | h3   | ✅ 成功 | 99       | cloudflare |
| 391  | 162.159.38.192                        | 162.159.38.192  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 528  | 162.159.44.133                        | 162.159.44.133  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 641  | pranab.ns.cloudflare.com              | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 850  | uriah.ns.cloudflare.com               | 162.159.44.194  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 940  | cfip.xxxxxxxx.tk                      | 188.114.97.144  | IPv4   | h3   | ✅ 成功 | 100      | cloudflare |
| 87   | 162.159.38.134                        | 162.159.38.134  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 671  | rustam.ns.cloudflare.com              | 162.159.44.148  | IPv4   | h3   | ✅ 成功 | 101      | cloudflare |
| 155  | www.hugedomains.com                   | 104.26.7.37     | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 495  | 172.64.145.119                        | 172.64.145.119  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 701  | benedict.ns.cloudflare.com            | 162.159.44.205  | IPv4   | h3   | ✅ 成功 | 103      | cloudflare |
| 354  | 162.159.44.199                        | 162.159.44.199  | IPv4   | h3   | ✅ 成功 | 104      | cloudflare |
| 220  | 162.159.45.150                        | 162.159.45.150  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 255  | 162.159.45.121                        | 162.159.45.121  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 308  | 162.159.45.237                        | 162.159.45.237  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 476  | 162.159.39.99                         | 162.159.39.99   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 531  | 162.159.38.67                         | 162.159.38.67   | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 578  | 162.159.39.146                        | 162.159.39.146  | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 708  | www.glassdoor.com                     | 104.16.25.46    | IPv4   | h3   | ✅ 成功 | 106      | cloudflare |
| 160  | 162.159.45.67                         | 162.159.45.67   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 254  | 104.18.160.38                         | 104.18.160.38   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 309  | 162.159.44.60                         | 162.159.44.60   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 407  | 162.159.45.93                         | 162.159.45.93   | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 477  | 162.159.44.128                        | 162.159.44.128  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 676  | kyree.ns.cloudflare.com               | 162.159.44.207  | IPv4   | h3   | ✅ 成功 | 107      | cloudflare |
| 573  | 162.159.38.226                        | 162.159.38.226  | IPv4   | h3   | ✅ 成功 | 108      | cloudflare |
| 473  | 162.159.45.176                        | 162.159.45.176  | IPv4   | h3   | ✅ 成功 | 109      | cloudflare |
| 132  | icook.hk                              | 104.21.90.210   | IPv4   | h3   | ✅ 成功 | 112      | cloudflare |
| 483  | 162.159.45.145                        | 162.159.45.145  | IPv4   | h3   | ✅ 成功 | 113      | cloudflare |
| 861  | lewis.ns.cloudflare.com               | 108.162.195.159 | IPv4   | h3   | ✅ 成功 | 116      | cloudflare |
| 629  | moura.ns.cloudflare.com               | 162.159.44.217  | IPv4   | h3   | ✅ 成功 | 121      | cloudflare |
| 479  | 162.159.38.171                        | 162.159.38.171  | IPv4   | h3   | ✅ 成功 | 122      | cloudflare |
| 90   | 162.159.39.180                        | 162.159.39.180  | IPv4   | h3   | ✅ 成功 | 123      | cloudflare |
| 303  | 162.159.39.156                        | 162.159.39.156  | IPv4   | h3   | ✅ 成功 | 124      | cloudflare |
| 8    | wilson.ns.cloudflare.com              | 162.159.44.110  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 684  | dylan.ns.cloudflare.com               | 162.159.44.187  | IPv4   | h3   | ✅ 成功 | 125      | cloudflare |
| 148  | 162.159.39.20                         | 162.159.39.20   | IPv4   | h3   | ✅ 成功 | 128      | cloudflare |
| 532  | 162.159.39.136                        | 162.159.39.136  | IPv4   | h3   | ✅ 成功 | 135      | cloudflare |
| 147  | 162.159.38.45                         | 162.159.38.45   | IPv4   | h3   | ✅ 成功 | 141      | cloudflare |
| 22   | trevor.ns.cloudflare.com              | 162.159.44.154  | IPv4   | h3   | ✅ 成功 | 146      | cloudflare |
| 231  | sullivan.ns.cloudflare.com            | 162.159.44.161  | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 246  | 104.20.26.221                         | 104.20.26.221   | IPv4   | h3   | ✅ 成功 | 161      | cloudflare |
| 817  | 141.147.185.63                        | 141.147.185.63  | IPv4   | h2   | ✅ 成功 | 403      | cloudflare |
| 475  | 34.143.159.175                        | 34.143.159.175  | IPv4   | h2   | ✅ 成功 | 515      | cloudflare |
| 936  | 3.0.50.69                             | 3.0.50.69       | IPv4   | h2   | ✅ 成功 | 624      | cloudflare |
| 611  | 168.138.165.174                       | 168.138.165.174 | IPv4   | h2   | ✅ 成功 | 771      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 161 条记录
- **快 (50-100ms)**: 484 条记录
- **正常 (100-200ms)**: 41 条记录
- **慢 (200-500ms)**: 1 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 258 次

### 按协议统计

- **none**: 262 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
