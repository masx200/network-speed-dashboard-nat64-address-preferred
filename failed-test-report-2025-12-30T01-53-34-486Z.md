# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 09:53:34
- **数据来源**: connectivity_results-20251230-095313.json
- **总测试数**: 505
- **失败测试数**: 9
- **成功测试数**: 496
- **失败率**: 1.78%
- **平均延迟**: 504.46ms
- **最小延迟**: 126ms
- **最大延迟**: 3876ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 09:53:34
- **IP地址**: 240e:b8e:35cd:a601:595c:e1c1:66d0:9f36
- **国家/地区**: China (CN)
- **ASN**: AS4134
- **网络组织**: Chinanet
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 9 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (9 次测试)

| 序号 | 主机/域名                | 目标IP                  | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                            |
| ---- | ------------------------ | ----------------------- | ------ | ---- | ------ | -------- | ------ | --------------------------------------------------- |
| 76   | 172.67.49.134            | 172.67.49.134           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout             |
| 177  | cloudflare.182682.xyz    | 2606:4700:e7::3151:47a9 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:e7::3151:47a9]:443: i/o timeout |
| 227  | 141.147.185.63           | 141.147.185.63          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout            |
| 292  | trevor.ns.cloudflare.com | 108.162.195.154         | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout           |
| 365  | 172.64.201.25            | 172.64.201.25           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout             |
| 482  | 3.0.50.69                | 3.0.50.69               | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                 |
| 487  | ct.877774.xyz            | 172.64.229.236          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.229.236:443: i/o timeout            |
| 501  | cfip.xxxxxxxx.tk         | 104.20.255.53           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout             |
| 504  | cfip.xxxxxxxx.tk         | 198.41.212.130          | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout            |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 9 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 9 次超时，主要集中在IP段 172.64（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 9 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 8 次，IPv6失败 1 次，两种协议都存在问题

---

## 🚀 延迟最低的 496 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 294  | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h3   | ✅ 成功 | 126      | cloudflare |
| 56   | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h3   | ✅ 成功 | 133      | cloudflare |
| 488  | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 133      | cloudflare |
| 485  | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 251  | julio.ns.cloudflare.com                 | 162.159.44.209                          | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 484  | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 143      | cloudflare |
| 141  | bowen.ns.cloudflare.com                 | 162.159.44.83                           | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 61   | kyree.ns.cloudflare.com                 | 162.159.44.207                          | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 243  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h3   | ✅ 成功 | 150      | cloudflare |
| 433  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h3   | ✅ 成功 | 153      | cloudflare |
| 489  | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 155      | cloudflare |
| 425  | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 374  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h3   | ✅ 成功 | 159      | cloudflare |
| 483  | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h3   | ✅ 成功 | 160      | cloudflare |
| 363  | 172.64.52.127                           | 172.64.52.127                           | IPv4   | h3   | ✅ 成功 | 170      | cloudflare |
| 302  | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h3   | ✅ 成功 | 175      | cloudflare |
| 490  | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h3   | ✅ 成功 | 176      | cloudflare |
| 411  | abdullah.ns.cloudflare.com              | 162.159.44.203                          | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 391  | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h3   | ✅ 成功 | 178      | cloudflare |
| 486  | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 183      | cloudflare |
| 133  | rustam.ns.cloudflare.com                | 162.159.44.148                          | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 427  | huxley.ns.cloudflare.com                | 108.162.195.188                         | IPv4   | h3   | ✅ 成功 | 199      | cloudflare |
| 307  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h3   | ✅ 成功 | 202      | cloudflare |
| 69   | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 310  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h3   | ✅ 成功 | 203      | cloudflare |
| 304  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h3   | ✅ 成功 | 204      | cloudflare |
| 309  | damien.ns.cloudflare.com                | 172.64.35.168                           | IPv4   | h3   | ✅ 成功 | 204      | cloudflare |
| 126  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h3   | ✅ 成功 | 207      | cloudflare |
| 4    | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 378  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h3   | ✅ 成功 | 210      | cloudflare |
| 210  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h3   | ✅ 成功 | 215      | cloudflare |
| 111  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 375  | lewis.ns.cloudflare.com                 | 172.64.35.159                           | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 412  | abdullah.ns.cloudflare.com              | 172.64.35.203                           | IPv4   | h3   | ✅ 成功 | 217      | cloudflare |
| 77   | decker.ns.cloudflare.com                | 162.159.44.155                          | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 386  | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 473  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 160  | cf.090227.xyz                           | 172.64.145.158                          | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 296  | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 78   | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 132  | rustam.ns.cloudflare.com                | 172.64.35.148                           | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 385  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 399  | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 408  | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 416  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h3   | ✅ 成功 | 221      | cloudflare |
| 94   | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h3   | ✅ 成功 | 222      | cloudflare |
| 293  | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h3   | ✅ 成功 | 223      | cloudflare |
| 225  | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h3   | ✅ 成功 | 224      | cloudflare |
| 303  | otto.ns.cloudflare.com                  | 172.64.35.135                           | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 131  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 401  | www.wto.org                             | 172.64.146.66                           | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 63   | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h3   | ✅ 成功 | 226      | cloudflare |
| 66   | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | h3   | ✅ 成功 | 226      | cloudflare |
| 252  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | h3   | ✅ 成功 | 227      | cloudflare |
| 107  | braden.ns.cloudflare.com                | 172.64.35.169                           | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 115  | cf.zhetengsha.eu.org                    | 2a06:98c1:3108::6812:2a62               | IPv6   | h3   | ✅ 成功 | 228      | cloudflare |
| 130  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h3   | ✅ 成功 | 228      | cloudflare |
| 249  | julio.ns.cloudflare.com                 | 108.162.195.209                         | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 267  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 51   | sullivan.ns.cloudflare.com              | 162.159.44.161                          | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 62   | kyree.ns.cloudflare.com                 | 172.64.35.207                           | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 89   | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 106  | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 116  | cf.zhetengsha.eu.org                    | 2a06:98c1:3101::ac40:919e               | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 159  | 172.64.154.18                           | 172.64.154.18                           | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 359  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h3   | ✅ 成功 | 229      | cloudflare |
| 404  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 222  | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h3   | ✅ 成功 | 230      | cloudflare |
| 8    | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h3   | ✅ 成功 | 232      | cloudflare |
| 43   | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 79   | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 144  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h3   | ✅ 成功 | 232      | cloudflare |
| 377  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h3   | ✅ 成功 | 232      | cloudflare |
| 5    | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 34   | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 246  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | h3   | ✅ 成功 | 233      | cloudflare |
| 414  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h3   | ✅ 成功 | 233      | cloudflare |
| 430  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h3   | ✅ 成功 | 233      | cloudflare |
| 70   | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 212  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 253  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 402  | www.wto.org                             | 104.18.41.190                           | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 55   | cris.ns.cloudflare.com                  | 108.162.195.202                         | IPv4   | h3   | ✅ 成功 | 235      | cloudflare |
| 157  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h3   | ✅ 成功 | 235      | cloudflare |
| 54   | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 105  | braden.ns.cloudflare.com                | 108.162.195.169                         | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 135  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 145  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 156  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 297  | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 388  | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 37   | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 125  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 181  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h3   | ✅ 成功 | 237      | cloudflare |
| 277  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 161  | cf.090227.xyz                           | 104.18.42.98                            | IPv4   | h3   | ✅ 成功 | 238      | cloudflare |
| 68   | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 142  | bowen.ns.cloudflare.com                 | 172.64.35.83                            | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 143  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 146  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h3   | ✅ 成功 | 239      | cloudflare |
| 300  | 172.64.35.24                            | 172.64.35.24                            | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 52   | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 72   | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 245  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 129  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 247  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 254  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 308  | damien.ns.cloudflare.com                | 108.162.195.168                         | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 415  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 136  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | h3   | ✅ 成功 | 242      | cloudflare |
| 266  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 299  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 155  | benedict.ns.cloudflare.com              | 108.162.195.205                         | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 163  | cf.090227.xyz                           | 2a06:98c1:3101::ac40:919e               | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 49   | sullivan.ns.cloudflare.com              | 172.64.35.161                           | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 110  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h3   | ✅ 成功 | 244      | cloudflare |
| 57   | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 38   | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 81   | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 214  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h3   | ✅ 成功 | 246      | cloudflare |
| 413  | abdullah.ns.cloudflare.com              | 108.162.195.203                         | IPv4   | h3   | ✅ 成功 | 246      | cloudflare |
| 153  | benedict.ns.cloudflare.com              | 172.64.35.205                           | IPv4   | h3   | ✅ 成功 | 247      | cloudflare |
| 244  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h3   | ✅ 成功 | 247      | cloudflare |
| 7    | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h3   | ✅ 成功 | 248      | cloudflare |
| 211  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h3   | ✅ 成功 | 248      | cloudflare |
| 213  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h3   | ✅ 成功 | 248      | cloudflare |
| 295  | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h3   | ✅ 成功 | 249      | cloudflare |
| 301  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h3   | ✅ 成功 | 249      | cloudflare |
| 59   | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h3   | ✅ 成功 | 250      | cloudflare |
| 268  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h3   | ✅ 成功 | 250      | cloudflare |
| 1    | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h3   | ✅ 成功 | 251      | cloudflare |
| 36   | pranab.ns.cloudflare.com                | 172.64.35.199                           | IPv4   | h3   | ✅ 成功 | 251      | cloudflare |
| 221  | wilson.ns.cloudflare.com                | 108.162.195.110                         | IPv4   | h3   | ✅ 成功 | 251      | cloudflare |
| 64   | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | h3   | ✅ 成功 | 252      | cloudflare |
| 108  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h3   | ✅ 成功 | 254      | cloudflare |
| 152  | 34.143.159.175                          | 34.143.159.175                          | IPv4   | h2   | ✅ 成功 | 254      | cloudflare |
| 226  | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h3   | ✅ 成功 | 254      | cloudflare |
| 373  | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h3   | ✅ 成功 | 254      | cloudflare |
| 127  | moura.ns.cloudflare.com                 | 172.64.35.217                           | IPv4   | h3   | ✅ 成功 | 256      | cloudflare |
| 162  | cf.090227.xyz                           | 2a06:98c1:3108::6812:2a62               | IPv6   | h3   | ✅ 成功 | 256      | cloudflare |
| 269  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h3   | ✅ 成功 | 256      | cloudflare |
| 114  | cf.zhetengsha.eu.org                    | 172.64.149.159                          | IPv4   | h3   | ✅ 成功 | 257      | cloudflare |
| 426  | huxley.ns.cloudflare.com                | 172.64.35.188                           | IPv4   | h3   | ✅ 成功 | 257      | cloudflare |
| 112  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h3   | ✅ 成功 | 259      | cloudflare |
| 428  | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h3   | ✅ 成功 | 260      | cloudflare |
| 60   | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h3   | ✅ 成功 | 262      | cloudflare |
| 35   | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 113  | cf.zhetengsha.eu.org                    | 104.18.38.97                            | IPv4   | h3   | ✅ 成功 | 266      | cloudflare |
| 376  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h3   | ✅ 成功 | 306      | cloudflare |
| 449  | www.4chan.org                           | 104.16.228.229                          | IPv4   | h3   | ✅ 成功 | 308      | cloudflare |
| 65   | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | h3   | ✅ 成功 | 318      | cloudflare |
| 270  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h3   | ✅ 成功 | 322      | cloudflare |
| 205  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h3   | ✅ 成功 | 323      | cloudflare |
| 128  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h3   | ✅ 成功 | 325      | cloudflare |
| 25   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h3   | ✅ 成功 | 327      | cloudflare |
| 500  | cfip.xxxxxxxx.tk                        | 190.93.247.169                          | IPv4   | h3   | ✅ 成功 | 332      | cloudflare |
| 74   | freeyx.cloudflare88.eu.org              | 141.101.120.197                         | IPv4   | h3   | ✅ 成功 | 334      | cloudflare |
| 357  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h3   | ✅ 成功 | 334      | cloudflare |
| 453  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h3   | ✅ 成功 | 334      | cloudflare |
| 502  | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h3   | ✅ 成功 | 334      | cloudflare |
| 228  | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h3   | ✅ 成功 | 337      | cloudflare |
| 332  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h3   | ✅ 成功 | 339      | cloudflare |
| 451  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h3   | ✅ 成功 | 339      | cloudflare |
| 22   | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h3   | ✅ 成功 | 341      | cloudflare |
| 320  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h3   | ✅ 成功 | 341      | cloudflare |
| 499  | cfip.xxxxxxxx.tk                        | 104.17.127.110                          | IPv4   | h3   | ✅ 成功 | 341      | cloudflare |
| 461  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h3   | ✅ 成功 | 351      | cloudflare |
| 364  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h3   | ✅ 成功 | 354      | cloudflare |
| 454  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h3   | ✅ 成功 | 354      | cloudflare |
| 319  | cmcc.877774.xyz                         | 104.16.149.7                            | IPv4   | h3   | ✅ 成功 | 356      | cloudflare |
| 340  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h3   | ✅ 成功 | 356      | cloudflare |
| 459  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h3   | ✅ 成功 | 357      | cloudflare |
| 407  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h3   | ✅ 成功 | 359      | cloudflare |
| 431  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h3   | ✅ 成功 | 360      | cloudflare |
| 180  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h3   | ✅ 成功 | 361      | cloudflare |
| 175  | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h3   | ✅ 成功 | 363      | cloudflare |
| 424  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h3   | ✅ 成功 | 363      | cloudflare |
| 403  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h3   | ✅ 成功 | 365      | cloudflare |
| 479  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h3   | ✅ 成功 | 366      | cloudflare |
| 91   | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h3   | ✅ 成功 | 368      | cloudflare |
| 218  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | h3   | ✅ 成功 | 368      | cloudflare |
| 342  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h3   | ✅ 成功 | 368      | cloudflare |
| 462  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h3   | ✅ 成功 | 368      | cloudflare |
| 23   | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h3   | ✅ 成功 | 369      | cloudflare |
| 151  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h3   | ✅ 成功 | 369      | cloudflare |
| 497  | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h3   | ✅ 成功 | 369      | cloudflare |
| 19   | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 478  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h3   | ✅ 成功 | 370      | cloudflare |
| 174  | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h3   | ✅ 成功 | 371      | cloudflare |
| 13   | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h3   | ✅ 成功 | 372      | cloudflare |
| 288  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h3   | ✅ 成功 | 372      | cloudflare |
| 384  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h3   | ✅ 成功 | 372      | cloudflare |
| 92   | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h3   | ✅ 成功 | 373      | cloudflare |
| 171  | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h3   | ✅ 成功 | 373      | cloudflare |
| 248  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h3   | ✅ 成功 | 373      | cloudflare |
| 229  | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h3   | ✅ 成功 | 374      | cloudflare |
| 264  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h3   | ✅ 成功 | 374      | cloudflare |
| 481  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h3   | ✅ 成功 | 374      | cloudflare |
| 464  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h3   | ✅ 成功 | 375      | cloudflare |
| 287  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h3   | ✅ 成功 | 376      | cloudflare |
| 343  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4   | h3   | ✅ 成功 | 378      | cloudflare |
| 220  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h3   | ✅ 成功 | 379      | cloudflare |
| 20   | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 32   | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h3   | ✅ 成功 | 380      | cloudflare |
| 44   | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 289  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h3   | ✅ 成功 | 380      | cloudflare |
| 224  | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h3   | ✅ 成功 | 381      | cloudflare |
| 237  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h3   | ✅ 成功 | 382      | cloudflare |
| 396  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h3   | ✅ 成功 | 382      | cloudflare |
| 398  | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h3   | ✅ 成功 | 382      | cloudflare |
| 208  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h3   | ✅ 成功 | 383      | cloudflare |
| 336  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h3   | ✅ 成功 | 383      | cloudflare |
| 348  | ipinfo.in                               | 104.21.21.129                           | IPv4   | h3   | ✅ 成功 | 383      | cloudflare |
| 432  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h3   | ✅ 成功 | 383      | cloudflare |
| 446  | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h3   | ✅ 成功 | 383      | cloudflare |
| 271  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 369  | www.gov.ua                              | 104.21.23.72                            | IPv4   | h3   | ✅ 成功 | 384      | cloudflare |
| 41   | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 138  | cloudflare-ip.mofashi.ltd               | 188.114.96.3                            | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 298  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h3   | ✅ 成功 | 385      | cloudflare |
| 350  | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h3   | ✅ 成功 | 385      | cloudflare |
| 104  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h3   | ✅ 成功 | 386      | cloudflare |
| 206  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h3   | ✅ 成功 | 386      | cloudflare |
| 203  | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h3   | ✅ 成功 | 387      | cloudflare |
| 30   | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 223  | wilson.ns.cloudflare.com                | 162.159.44.110                          | IPv4   | h3   | ✅ 成功 | 388      | cloudflare |
| 468  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h3   | ✅ 成功 | 388      | cloudflare |
| 324  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h3   | ✅ 成功 | 389      | cloudflare |
| 154  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 176  | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h3   | ✅ 成功 | 390      | cloudflare |
| 263  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h3   | ✅ 成功 | 390      | cloudflare |
| 334  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 458  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h3   | ✅ 成功 | 390      | cloudflare |
| 261  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h3   | ✅ 成功 | 391      | cloudflare |
| 317  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h3   | ✅ 成功 | 391      | cloudflare |
| 280  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h3   | ✅ 成功 | 392      | cloudflare |
| 419  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h3   | ✅ 成功 | 392      | cloudflare |
| 437  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h3   | ✅ 成功 | 393      | cloudflare |
| 15   | www.visa.cn                             | 162.159.152.2                           | IPv4   | h3   | ✅ 成功 | 394      | cloudflare |
| 18   | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h3   | ✅ 成功 | 394      | cloudflare |
| 47   | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:ab56:90d6:b7ab    | IPv6   | h3   | ✅ 成功 | 394      | cloudflare |
| 241  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h3   | ✅ 成功 | 394      | cloudflare |
| 257  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h3   | ✅ 成功 | 394      | cloudflare |
| 178  | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h3   | ✅ 成功 | 395      | cloudflare |
| 262  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h3   | ✅ 成功 | 395      | cloudflare |
| 272  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h3   | ✅ 成功 | 395      | cloudflare |
| 265  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h3   | ✅ 成功 | 396      | cloudflare |
| 371  | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h3   | ✅ 成功 | 396      | cloudflare |
| 380  | iplocation.io                           | 172.67.70.100                           | IPv4   | h3   | ✅ 成功 | 396      | cloudflare |
| 463  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h3   | ✅ 成功 | 396      | cloudflare |
| 467  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h3   | ✅ 成功 | 396      | cloudflare |
| 315  | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h3   | ✅ 成功 | 397      | cloudflare |
| 346  | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h3   | ✅ 成功 | 397      | cloudflare |
| 16   | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h3   | ✅ 成功 | 398      | cloudflare |
| 200  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h3   | ✅ 成功 | 398      | cloudflare |
| 273  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h3   | ✅ 成功 | 398      | cloudflare |
| 279  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h3   | ✅ 成功 | 398      | cloudflare |
| 316  | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h3   | ✅ 成功 | 398      | cloudflare |
| 393  | japan.com                               | 104.26.4.60                             | IPv4   | h3   | ✅ 成功 | 398      | cloudflare |
| 12   | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h3   | ✅ 成功 | 399      | cloudflare |
| 14   | www.visa.cn                             | 162.159.153.2                           | IPv4   | h3   | ✅ 成功 | 399      | cloudflare |
| 331  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h3   | ✅ 成功 | 400      | cloudflare |
| 97   | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h3   | ✅ 成功 | 401      | cloudflare |
| 172  | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h3   | ✅ 成功 | 401      | cloudflare |
| 260  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h3   | ✅ 成功 | 401      | cloudflare |
| 11   | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h3   | ✅ 成功 | 402      | cloudflare |
| 443  | icook.hk                                | 188.114.97.3                            | IPv4   | h3   | ✅ 成功 | 402      | cloudflare |
| 45   | bestcf.030101.xyz                       | 104.19.156.139                          | IPv4   | h3   | ✅ 成功 | 403      | cloudflare |
| 469  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h3   | ✅ 成功 | 403      | cloudflare |
| 195  | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h3   | ✅ 成功 | 404      | cloudflare |
| 445  | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h3   | ✅ 成功 | 404      | cloudflare |
| 46   | bestcf.030101.xyz                       | 104.19.157.251                          | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 333  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 387  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 434  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h3   | ✅ 成功 | 405      | cloudflare |
| 234  | singapore.com                           | 172.67.75.194                           | IPv4   | h3   | ✅ 成功 | 407      | cloudflare |
| 405  | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h3   | ✅ 成功 | 407      | cloudflare |
| 95   | zread.ai                                | 172.67.202.78                           | IPv4   | h3   | ✅ 成功 | 408      | cloudflare |
| 216  | silkbook.com                            | 172.67.75.208                           | IPv4   | h3   | ✅ 成功 | 408      | cloudflare |
| 274  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h3   | ✅ 成功 | 408      | cloudflare |
| 480  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h3   | ✅ 成功 | 408      | cloudflare |
| 494  | cfip.xxxxxxxx.tk                        | 190.93.246.67                           | IPv4   | h3   | ✅ 成功 | 408      | cloudflare |
| 58   | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h3   | ✅ 成功 | 410      | cloudflare |
| 170  | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h3   | ✅ 成功 | 410      | cloudflare |
| 258  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h3   | ✅ 成功 | 410      | cloudflare |
| 360  | 198.41.194.162                          | 198.41.194.162                          | IPv4   | h3   | ✅ 成功 | 410      | cloudflare |
| 165  | time.is                                 | 104.26.12.54                            | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 355  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h3   | ✅ 成功 | 411      | cloudflare |
| 457  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h3   | ✅ 成功 | 411      | cloudflare |
| 184  | palera.in                               | 104.21.58.72                            | IPv4   | h3   | ✅ 成功 | 412      | cloudflare |
| 3    | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h3   | ✅ 成功 | 413      | cloudflare |
| 219  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h3   | ✅ 成功 | 413      | cloudflare |
| 198  | ip.gs                                   | 172.67.160.28                           | IPv4   | h3   | ✅ 成功 | 414      | cloudflare |
| 118  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h3   | ✅ 成功 | 415      | cloudflare |
| 207  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h3   | ✅ 成功 | 415      | cloudflare |
| 147  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h3   | ✅ 成功 | 416      | cloudflare |
| 444  | icook.hk                                | 188.114.96.3                            | IPv4   | h3   | ✅ 成功 | 416      | cloudflare |
| 28   | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 409  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h3   | ✅ 成功 | 417      | cloudflare |
| 465  | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | h3   | ✅ 成功 | 417      | cloudflare |
| 40   | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h3   | ✅ 成功 | 418      | cloudflare |
| 88   | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h3   | ✅ 成功 | 419      | cloudflare |
| 366  | eur.877774.xyz                          | 104.21.47.209                           | IPv4   | h3   | ✅ 成功 | 419      | cloudflare |
| 291  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h3   | ✅ 成功 | 420      | cloudflare |
| 347  | ipinfo.in                               | 172.67.198.203                          | IPv4   | h3   | ✅ 成功 | 420      | cloudflare |
| 2    | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h3   | ✅ 成功 | 422      | cloudflare |
| 17   | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h3   | ✅ 成功 | 422      | cloudflare |
| 318  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h3   | ✅ 成功 | 422      | cloudflare |
| 390  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h3   | ✅ 成功 | 422      | cloudflare |
| 281  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h3   | ✅ 成功 | 424      | cloudflare |
| 328  | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h3   | ✅ 成功 | 424      | cloudflare |
| 10   | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h3   | ✅ 成功 | 425      | cloudflare |
| 341  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h3   | ✅ 成功 | 426      | cloudflare |
| 438  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h3   | ✅ 成功 | 427      | cloudflare |
| 149  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 150  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 450  | www.4chan.org                           | 104.16.229.229                          | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 495  | cfip.xxxxxxxx.tk                        | 188.114.97.144                          | IPv4   | h3   | ✅ 成功 | 428      | cloudflare |
| 98   | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h3   | ✅ 成功 | 429      | cloudflare |
| 329  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h3   | ✅ 成功 | 431      | cloudflare |
| 169  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h3   | ✅ 成功 | 432      | cloudflare |
| 439  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h3   | ✅ 成功 | 432      | cloudflare |
| 188  | ip.sb                                   | 104.26.13.31                            | IPv4   | h3   | ✅ 成功 | 433      | cloudflare |
| 199  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h3   | ✅ 成功 | 433      | cloudflare |
| 202  | comicabc.com                            | 188.114.96.7                            | IPv4   | h3   | ✅ 成功 | 433      | cloudflare |
| 476  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h3   | ✅ 成功 | 434      | cloudflare |
| 134  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h3   | ✅ 成功 | 437      | cloudflare |
| 285  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h3   | ✅ 成功 | 437      | cloudflare |
| 240  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 394  | japan.com                               | 172.67.70.92                            | IPv4   | h3   | ✅ 成功 | 438      | cloudflare |
| 382  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h3   | ✅ 成功 | 440      | cloudflare |
| 421  | shopify.com                             | 23.227.38.33                            | IPv4   | h3   | ✅ 成功 | 440      | cloudflare |
| 471  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h3   | ✅ 成功 | 440      | cloudflare |
| 283  | icook.tw                                | 104.20.28.74                            | IPv4   | h3   | ✅ 成功 | 444      | cloudflare |
| 311  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h3   | ✅ 成功 | 444      | cloudflare |
| 392  | japan.com                               | 104.26.5.60                             | IPv4   | h3   | ✅ 成功 | 444      | cloudflare |
| 121  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h3   | ✅ 成功 | 446      | cloudflare |
| 358  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h3   | ✅ 成功 | 446      | cloudflare |
| 42   | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 379  | iplocation.io                           | 104.26.10.222                           | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 389  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h3   | ✅ 成功 | 447      | cloudflare |
| 448  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h3   | ✅ 成功 | 448      | cloudflare |
| 137  | cloudflare-ip.mofashi.ltd               | 188.114.97.3                            | IPv4   | h3   | ✅ 成功 | 449      | cloudflare |
| 67   | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h3   | ✅ 成功 | 451      | cloudflare |
| 238  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h3   | ✅ 成功 | 451      | cloudflare |
| 381  | iplocation.io                           | 104.26.11.222                           | IPv4   | h3   | ✅ 成功 | 451      | cloudflare |
| 256  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h3   | ✅ 成功 | 452      | cloudflare |
| 417  | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h3   | ✅ 成功 | 452      | cloudflare |
| 168  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h3   | ✅ 成功 | 454      | cloudflare |
| 196  | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h3   | ✅ 成功 | 454      | cloudflare |
| 351  | steamdb.info                            | 104.20.34.212                           | IPv4   | h3   | ✅ 成功 | 455      | cloudflare |
| 442  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h3   | ✅ 成功 | 455      | cloudflare |
| 493  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h3   | ✅ 成功 | 455      | cloudflare |
| 103  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h3   | ✅ 成功 | 457      | cloudflare |
| 232  | singapore.com                           | 104.26.12.140                           | IPv4   | h3   | ✅ 成功 | 457      | cloudflare |
| 235  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h3   | ✅ 成功 | 457      | cloudflare |
| 75   | freeyx.cloudflare88.eu.org              | 2606:4700:3010:0:f5e8:7af2:1225:455c    | IPv6   | h3   | ✅ 成功 | 460      | cloudflare |
| 447  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h3   | ✅ 成功 | 460      | cloudflare |
| 397  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h3   | ✅ 成功 | 462      | cloudflare |
| 436  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h3   | ✅ 成功 | 462      | cloudflare |
| 183  | palera.in                               | 172.67.157.122                          | IPv4   | h3   | ✅ 成功 | 463      | cloudflare |
| 209  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h3   | ✅ 成功 | 463      | cloudflare |
| 395  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h3   | ✅ 成功 | 464      | cloudflare |
| 314  | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h3   | ✅ 成功 | 467      | cloudflare |
| 284  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h3   | ✅ 成功 | 468      | cloudflare |
| 306  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h3   | ✅ 成功 | 468      | cloudflare |
| 83   | toy-people.com                          | 104.26.2.36                             | IPv4   | h3   | ✅ 成功 | 469      | cloudflare |
| 362  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h3   | ✅ 成功 | 469      | cloudflare |
| 282  | icook.tw                                | 172.66.158.115                          | IPv4   | h3   | ✅ 成功 | 470      | cloudflare |
| 422  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h3   | ✅ 成功 | 470      | cloudflare |
| 85   | toy-people.com                          | 104.26.3.36                             | IPv4   | h3   | ✅ 成功 | 472      | cloudflare |
| 99   | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h3   | ✅ 成功 | 473      | cloudflare |
| 53   | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h3   | ✅ 成功 | 478      | cloudflare |
| 90   | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h3   | ✅ 成功 | 478      | cloudflare |
| 9    | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h3   | ✅ 成功 | 479      | cloudflare |
| 356  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h3   | ✅ 成功 | 479      | cloudflare |
| 242  | uriah.ns.cloudflare.com                 | 108.162.195.194                         | IPv4   | h3   | ✅ 成功 | 482      | cloudflare |
| 312  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h3   | ✅ 成功 | 482      | cloudflare |
| 82   | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h3   | ✅ 成功 | 483      | cloudflare |
| 344  | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h3   | ✅ 成功 | 485      | cloudflare |
| 101  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h3   | ✅ 成功 | 489      | cloudflare |
| 278  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h3   | ✅ 成功 | 490      | cloudflare |
| 474  | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h3   | ✅ 成功 | 496      | cloudflare |
| 71   | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h3   | ✅ 成功 | 497      | cloudflare |
| 102  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h3   | ✅ 成功 | 498      | cloudflare |
| 325  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h3   | ✅ 成功 | 498      | cloudflare |
| 6    | craig.ns.cloudflare.com                 | 108.162.195.192                         | IPv4   | h3   | ✅ 成功 | 499      | cloudflare |
| 33   | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h3   | ✅ 成功 | 506      | cloudflare |
| 87   | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h3   | ✅ 成功 | 509      | cloudflare |
| 39   | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h3   | ✅ 成功 | 513      | cloudflare |
| 148  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h3   | ✅ 成功 | 516      | cloudflare |
| 236  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 519      | cloudflare |
| 491  | cfip.xxxxxxxx.tk                        | 104.25.105.1                            | IPv4   | h3   | ✅ 成功 | 529      | cloudflare |
| 352  | steamdb.info                            | 172.66.175.250                          | IPv4   | h3   | ✅ 成功 | 534      | cloudflare |
| 290  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h3   | ✅ 成功 | 537      | cloudflare |
| 435  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h3   | ✅ 成功 | 539      | cloudflare |
| 109  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h3   | ✅ 成功 | 542      | cloudflare |
| 93   | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h3   | ✅ 成功 | 544      | cloudflare |
| 338  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h3   | ✅ 成功 | 560      | cloudflare |
| 345  | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h3   | ✅ 成功 | 563      | cloudflare |
| 255  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h3   | ✅ 成功 | 578      | cloudflare |
| 185  | palera.in                               | 2a06:98c1:3121::3                       | IPv6   | h3   | ✅ 成功 | 588      | cloudflare |
| 339  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h3   | ✅ 成功 | 597      | cloudflare |
| 122  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h3   | ✅ 成功 | 621      | cloudflare |
| 420  | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h3   | ✅ 成功 | 625      | cloudflare |
| 321  | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h3   | ✅ 成功 | 626      | cloudflare |
| 26   | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h3   | ✅ 成功 | 642      | cloudflare |
| 24   | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h3   | ✅ 成功 | 652      | cloudflare |
| 305  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h3   | ✅ 成功 | 655      | cloudflare |
| 187  | ip.sb                                   | 172.67.75.172                           | IPv4   | h3   | ✅ 成功 | 677      | cloudflare |
| 231  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h3   | ✅ 成功 | 677      | cloudflare |
| 275  | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h3   | ✅ 成功 | 685      | cloudflare |
| 492  | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h3   | ✅ 成功 | 691      | cloudflare |
| 123  | xn--b6gac.eu.org                        | 2a06:98c1:3120::3                       | IPv6   | h3   | ✅ 成功 | 692      | cloudflare |
| 429  | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | h3   | ✅ 成功 | 730      | cloudflare |
| 120  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h3   | ✅ 成功 | 734      | cloudflare |
| 410  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h3   | ✅ 成功 | 738      | cloudflare |
| 182  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h3   | ✅ 成功 | 744      | cloudflare |
| 361  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h3   | ✅ 成功 | 745      | cloudflare |
| 230  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h3   | ✅ 成功 | 754      | cloudflare |
| 335  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h3   | ✅ 成功 | 757      | cloudflare |
| 158  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h3   | ✅ 成功 | 769      | cloudflare |
| 197  | ip.gs                                   | 104.21.14.176                           | IPv4   | h3   | ✅ 成功 | 784      | cloudflare |
| 173  | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h3   | ✅ 成功 | 786      | cloudflare |
| 326  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h3   | ✅ 成功 | 786      | cloudflare |
| 370  | www.gov.ua                              | 172.67.209.127                          | IPv4   | h3   | ✅ 成功 | 823      | cloudflare |
| 337  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h3   | ✅ 成功 | 826      | cloudflare |
| 477  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h3   | ✅ 成功 | 830      | cloudflare |
| 117  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h3   | ✅ 成功 | 838      | cloudflare |
| 186  | palera.in                               | 2a06:98c1:3120::3                       | IPv6   | h3   | ✅ 成功 | 838      | cloudflare |
| 423  | www.7749tv.com                          | 104.16.115.36                           | IPv4   | h3   | ✅ 成功 | 840      | cloudflare |
| 327  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h3   | ✅ 成功 | 849      | cloudflare |
| 400  | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h3   | ✅ 成功 | 859      | cloudflare |
| 276  | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h3   | ✅ 成功 | 867      | cloudflare |
| 166  | time.is                                 | 104.26.13.54                            | IPv4   | h3   | ✅ 成功 | 872      | cloudflare |
| 204  | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h3   | ✅ 成功 | 875      | cloudflare |
| 456  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h3   | ✅ 成功 | 880      | cloudflare |
| 192  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h3   | ✅ 成功 | 881      | cloudflare |
| 452  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h3   | ✅ 成功 | 881      | cloudflare |
| 505  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h3   | ✅ 成功 | 888      | cloudflare |
| 239  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h3   | ✅ 成功 | 891      | cloudflare |
| 460  | 168.138.165.174                         | 168.138.165.174                         | IPv4   | h2   | ✅ 成功 | 899      | cloudflare |
| 86   | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h3   | ✅ 成功 | 900      | cloudflare |
| 372  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h3   | ✅ 成功 | 916      | cloudflare |
| 418  | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h3   | ✅ 成功 | 923      | cloudflare |
| 383  | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h3   | ✅ 成功 | 925      | cloudflare |
| 354  | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h3   | ✅ 成功 | 927      | cloudflare |
| 124  | xn--b6gac.eu.org                        | 2a06:98c1:3121::3                       | IPv6   | h3   | ✅ 成功 | 929      | cloudflare |
| 313  | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h3   | ✅ 成功 | 932      | cloudflare |
| 496  | cfip.xxxxxxxx.tk                        | 104.21.91.19                            | IPv4   | h3   | ✅ 成功 | 940      | cloudflare |
| 194  | www.ipget.net                           | 172.67.207.26                           | IPv4   | h3   | ✅ 成功 | 951      | cloudflare |
| 233  | singapore.com                           | 104.26.13.140                           | IPv4   | h3   | ✅ 成功 | 951      | cloudflare |
| 21   | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h3   | ✅ 成功 | 953      | cloudflare |
| 193  | www.ipget.net                           | 104.21.15.212                           | IPv4   | h3   | ✅ 成功 | 967      | cloudflare |
| 441  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h3   | ✅ 成功 | 988      | cloudflare |
| 190  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h3   | ✅ 成功 | 989      | cloudflare |
| 367  | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h3   | ✅ 成功 | 993      | cloudflare |
| 353  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h3   | ✅ 成功 | 1000     | cloudflare |
| 48   | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:bd32:7275:fc25    | IPv6   | h3   | ✅ 成功 | 1011     | cloudflare |
| 455  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h3   | ✅ 成功 | 1016     | cloudflare |
| 29   | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h3   | ✅ 成功 | 1017     | cloudflare |
| 73   | freeyx.cloudflare88.eu.org              | 141.101.120.187                         | IPv4   | h3   | ✅ 成功 | 1020     | cloudflare |
| 50   | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h3   | ✅ 成功 | 1032     | cloudflare |
| 191  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h3   | ✅ 成功 | 1063     | cloudflare |
| 217  | silkbook.com                            | 104.26.9.160                            | IPv4   | h3   | ✅ 成功 | 1068     | cloudflare |
| 179  | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h3   | ✅ 成功 | 1086     | cloudflare |
| 189  | ip.sb                                   | 104.26.12.31                            | IPv4   | h3   | ✅ 成功 | 1138     | cloudflare |
| 475  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h3   | ✅ 成功 | 1164     | cloudflare |
| 140  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h3   | ✅ 成功 | 1170     | cloudflare |
| 84   | toy-people.com                          | 172.67.72.18                            | IPv4   | h3   | ✅ 成功 | 1174     | cloudflare |
| 322  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h3   | ✅ 成功 | 1226     | cloudflare |
| 31   | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h3   | ✅ 成功 | 1235     | cloudflare |
| 167  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h3   | ✅ 成功 | 1274     | cloudflare |
| 286  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h3   | ✅ 成功 | 1310     | cloudflare |
| 368  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h3   | ✅ 成功 | 1314     | cloudflare |
| 119  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h3   | ✅ 成功 | 1371     | cloudflare |
| 470  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h3   | ✅ 成功 | 1387     | cloudflare |
| 139  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h3   | ✅ 成功 | 1402     | cloudflare |
| 250  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h3   | ✅ 成功 | 1470     | cloudflare |
| 96   | zread.ai                                | 104.21.76.240                           | IPv4   | h3   | ✅ 成功 | 1498     | cloudflare |
| 330  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h3   | ✅ 成功 | 1501     | cloudflare |
| 503  | cfip.xxxxxxxx.tk                        | 104.18.228.35                           | IPv4   | h3   | ✅ 成功 | 1503     | cloudflare |
| 201  | comicabc.com                            | 188.114.97.7                            | IPv4   | h3   | ✅ 成功 | 1566     | cloudflare |
| 100  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h3   | ✅ 成功 | 1578     | cloudflare |
| 80   | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h3   | ✅ 成功 | 1589     | cloudflare |
| 349  | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h3   | ✅ 成功 | 1619     | cloudflare |
| 27   | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h3   | ✅ 成功 | 1709     | cloudflare |
| 215  | silkbook.com                            | 104.26.8.160                            | IPv4   | h3   | ✅ 成功 | 2225     | cloudflare |
| 259  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h3   | ✅ 成功 | 2812     | cloudflare |
| 440  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h3   | ✅ 成功 | 2974     | cloudflare |
| 498  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h3   | ✅ 成功 | 3074     | cloudflare |
| 466  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h3   | ✅ 成功 | 3117     | cloudflare |
| 323  | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h3   | ✅ 成功 | 3271     | cloudflare |
| 164  | time.is                                 | 172.67.68.157                           | IPv4   | h3   | ✅ 成功 | 3320     | cloudflare |
| 472  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h3   | ✅ 成功 | 3470     | cloudflare |
| 406  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h3   | ✅ 成功 | 3876     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 22 条记录
- **慢 (200-500ms)**: 365 条记录
- **很慢 (>500ms)**: 109 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 8 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 9 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
