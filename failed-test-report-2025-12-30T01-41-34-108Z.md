# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:41:34
- **数据来源**: connectivity_results-20251230-014132.json
- **总测试数**: 502
- **失败测试数**: 2
- **成功测试数**: 500
- **失败率**: 0.40%
- **平均延迟**: 102.41ms
- **最小延迟**: 73ms
- **最大延迟**: 2570ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:41:34
- **IP地址**: 2a09:bac1:76a0:8628::281:64
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 41.1446, -104.8116
- **时区**: America/Denver
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 120  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 387  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 500 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 252  | fbi.gov                                 | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 73       | cloudflare |
| 35   | ipv4.ip.sb                              | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 114  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 134  | sullivan.ns.cloudflare.com              | 108.162.195.161                         | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 253  | fbi.gov                                 | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 257  | cf.090227.xyz                           | 104.18.43.174                           | IPv4   | h2   | ✅ 成功 | 76       | cloudflare |
| 290  | time.is                                 | 2606:4700:20::ac43:449d                 | IPv6   | h2   | ✅ 成功 | 76       | cloudflare |
| 346  | dnschecker.org                          | 104.26.6.89                             | IPv4   | h2   | ✅ 成功 | 77       | cloudflare |
| 36   | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 57   | www.gov.ua                              | 104.21.23.72                            | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 67   | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 106  | www.visa.com.sg                         | 104.18.12.229                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 203  | www.whatismyip.com                      | 2606:4700:20::ac43:4581                 | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 245  | cf.zhetengsha.eu.org                    | 172.66.47.179                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 293  | rustam.ns.cloudflare.com                | 172.64.35.148                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 331  | silkbook.com                            | 2606:4700:20::681a:8a0                  | IPv6   | h2   | ✅ 成功 | 78       | cloudflare |
| 409  | ifconfig.co                             | 172.67.168.106                          | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 483  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 78       | cloudflare |
| 45   | ct.877774.xyz                           | 172.64.229.195                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 47   | ct.877774.xyz                           | 172.64.229.236                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 66   | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 116  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 204  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 225  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 298  | palera.in                               | 172.67.157.122                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 309  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 310  | ip.sb                                   | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 312  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 313  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 388  | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 396  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 453  | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 79       | cloudflare |
| 493  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 79       | cloudflare |
| 13   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 15   | www.pcmag.com                           | 104.16.20.118                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 16   | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 28   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 33   | cf.0sm.com                              | 2606:4700:3032::6815:785                | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 107  | cf.877771.xyz                           | 172.67.152.183                          | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 112  | www.hugedomains.com                     | 104.26.7.37                             | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 138  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 216  | cmcc.877774.xyz                         | 104.16.149.12                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 254  | fbi.gov                                 | 2606:4700::6810:94f4                    | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 288  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 306  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 80       | cloudflare |
| 321  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 393  | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 404  | 104.19.223.58                           | 104.19.223.58                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 432  | www.wto.org                             | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 80       | cloudflare |
| 8    | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 30   | cf.0sm.com                              | 104.21.7.133                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 62   | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 65   | iplocation.io                           | 2606:4700:20::681a:ade                  | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 75   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 83   | www.4chan.org                           | 104.16.229.229                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 85   | cu.877774.xyz                           | 172.64.145.202                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 98   | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 99   | www.visa.cn                             | 162.159.153.2                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 105  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 123  | cfip.xxxxxxxx.tk                        | 190.93.247.169                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 130  | cfip.xxxxxxxx.tk                        | 190.93.246.67                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 197  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::ac43:9bac               | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 221  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 224  | cmcc.877774.xyz                         | 104.16.148.7                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 226  | cmcc.877774.xyz                         | 104.16.148.9                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 235  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 249  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 259  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 268  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 276  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 286  | time.is                                 | 104.26.13.54                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 301  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 311  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 317  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 385  | www.udemy.com                           | 2606:4700::6810:8fed                    | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 467  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 81       | cloudflare |
| 499  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 81       | cloudflare |
| 6    | comicabc.com                            | 172.67.174.21                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 22   | wilson.ns.cloudflare.com                | 2a06:98c1:50::ac40:236e                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 39   | ipinfo.in                               | 172.67.198.203                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 41   | ipinfo.in                               | 2606:4700:3037::ac43:c6cb               | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 53   | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 55   | 104.26.6.112                            | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 60   | www.gov.ua                              | 2606:4700:3033::ac43:d17f               | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 78   | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 101  | www.okcupid.com                         | 104.17.48.63                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 110  | cf.877771.xyz                           | 2606:4700:3033::6815:50b4               | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 128  | cfip.xxxxxxxx.tk                        | 190.93.244.201                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 147  | craig.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc0                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 151  | na.877774.xyz                           | 104.18.38.235                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 153  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 155  | pranab.ns.cloudflare.com                | 162.159.44.199                          | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 166  | zread.ai                                | 104.21.76.240                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 214  | cmcc.877774.xyz                         | 104.16.149.10                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 223  | cmcc.877774.xyz                         | 104.16.148.6                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 236  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 255  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 291  | time.is                                 | 2606:4700:20::681a:c36                  | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 299  | palera.in                               | 104.21.58.72                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 349  | dnschecker.org                          | 2606:4700:20::ac43:49d8                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 378  | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 397  | icook.tw                                | 2606:4700:10::ac42:9e73                 | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 403  | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 480  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 82       | cloudflare |
| 495  | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | 2a06:98c1:3121:c6d4:4130:7992:df42:f04c | IPv6   | h2   | ✅ 成功 | 82       | cloudflare |
| 11   | www.ipget.net                           | 104.21.15.212                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 44   | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 71   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 72   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 87   | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 109  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 124  | cfip.xxxxxxxx.tk                        | 188.114.96.125                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 131  | cfip.xxxxxxxx.tk                        | 104.18.228.35                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 146  | craig.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c0                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 167  | zread.ai                                | 172.67.202.78                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 168  | zread.ai                                | 2606:4700:3032::ac43:ca4e               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 173  | toy-people.com                          | 2606:4700:20::ac43:4812                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 186  | kyree.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23cf                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 229  | cmcc.877774.xyz                         | 104.16.148.12                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 242  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 251  | xn--b6gac.eu.org                        | 2606:4700:3035::6815:5a4e               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 274  | bowen.ns.cloudflare.com                 | 2803:f800:50::6ca2:c353                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 300  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 323  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 381  | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 414  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 463  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 481  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 83       | cloudflare |
| 498  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 83       | cloudflare |
| 14   | www.pcmag.com                           | 104.16.21.118                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 37   | ipv4.ip.sb                              | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 42   | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 48   | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 50   | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 64   | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 89   | icook.hk                                | 2606:4700:3037::ac43:a168               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 111  | www.hugedomains.com                     | 172.67.70.191                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 142  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 181  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 205  | www.whatismyip.com                      | 2606:4700:20::681a:d17                  | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 209  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 219  | cmcc.877774.xyz                         | 104.16.148.2                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 240  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:bd32:7275:fc25    | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 241  | [2606:4700:440f::53aa:4126]             | 2606:4700:440f::53aa:4126               | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 277  | www.glassdoor.com                       | 104.17.64.70                            | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 357  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 377  | tasteatlas.com                          | 104.17.37.105                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 406  | eur.877774.xyz                          | 104.21.26.150                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 427  | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 468  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 84       | cloudflare |
| 500  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 84       | cloudflare |
| 12   | www.ipget.net                           | 2606:4700:3036::6815:fd4                | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 31   | cf.0sm.com                              | 172.67.187.145                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 46   | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 56   | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 73   | cloudflare.182682.xyz                   | 2606:4700:e7::3151:47a9                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 79   | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 86   | [2606:4700:8de6::5fa2:799e]             | 2606:4700:8de6::5fa2:799e               | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 91   | 172.67.120.0                            | 172.67.120.0                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 104  | www.okcupid.com                         | 104.16.144.63                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 119  | cfip.xxxxxxxx.tk                        | 104.17.127.110                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 132  | cfip.xxxxxxxx.tk                        | 104.21.91.19                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 140  | yx-auto.pages.dev                       | 172.66.44.144                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 165  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 169  | zread.ai                                | 2606:4700:3033::6815:4cf0               | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 195  | cloudflare-ip.mofashi.ltd               | 172.67.155.172                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 207  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 227  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 269  | www.ipchicken.com                       | 104.26.7.112                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 273  | bowen.ns.cloudflare.com                 | 2606:4700:58::a29f:2c53                 | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 302  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 324  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 342  | 104.18.14.76                            | 104.18.14.76                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 343  | 162.159.133.85                          | 162.159.133.85                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 351  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 382  | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 384  | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 417  | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 431  | www.wto.org                             | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 466  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 472  | 198.41.208.224                          | 198.41.208.224                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 492  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 85       | cloudflare |
| 501  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 85       | cloudflare |
| 4    | steamdb.info                            | 2606:4700:10::ac42:affa                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 5    | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 34   | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 40   | ipinfo.in                               | 2606:4700:3031::6815:1581               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 52   | ct.877774.xyz                           | 172.64.229.185                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 76   | cloudflare.182682.xyz                   | 2606:4700:8ca0::3dc4:21a2               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 81   | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 82   | www.4chan.org                           | 104.16.228.229                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 100  | www.okcupid.com                         | 104.16.223.254                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 117  | 172.67.243.218                          | 172.67.243.218                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 118  | cfip.xxxxxxxx.tk                        | 188.114.97.144                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 129  | cfip.xxxxxxxx.tk                        | 104.27.21.118                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 149  | freeyx.cloudflare88.eu.org              | 141.101.120.39                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 163  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 172  | toy-people.com                          | 104.26.3.36                             | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 232  | cmcc.877774.xyz                         | 104.16.149.2                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 244  | cf.zhetengsha.eu.org                    | 172.66.44.77                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 250  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 258  | cf.090227.xyz                           | 172.64.144.82                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 265  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 278  | 104.16.61.163                           | 104.16.61.163                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 289  | time.is                                 | 2606:4700:20::681a:d36                  | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 296  | rustam.ns.cloudflare.com                | 2a06:98c1:50::ac40:2394                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 308  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 325  | singapore.com                           | 2606:4700:20::ac43:4bc2                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 332  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 340  | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 374  | uriah.ns.cloudflare.com                 | 2606:4700:58::a29f:2cc2                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 376  | [2606:4700:4408::18c5:3304]             | 2606:4700:4408::18c5:3304               | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 386  | 172.67.79.211                           | 172.67.79.211                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 400  | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 435  | stock.hostmonit.com                     | 172.67.187.251                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 442  | lewis.ns.cloudflare.com                 | 162.159.44.159                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 454  | japan.com                               | 2606:4700:20::ac43:465c                 | IPv6   | h2   | ✅ 成功 | 86       | cloudflare |
| 464  | 162.159.137.204                         | 162.159.137.204                         | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 470  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 488  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 489  | 104.17.167.134                          | 104.17.167.134                          | IPv4   | h2   | ✅ 成功 | 86       | cloudflare |
| 127  | cfip.xxxxxxxx.tk                        | 198.41.214.141                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 175  | toy-people.com                          | 2606:4700:20::681a:324                  | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 184  | kyree.ns.cloudflare.com                 | 172.64.35.207                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 201  | www.whatismyip.com                      | 172.67.69.129                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 210  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 213  | cmcc.877774.xyz                         | 104.16.149.9                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 233  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 316  | ip.gs                                   | 172.67.160.28                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 322  | singapore.com                           | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 358  | 104.26.4.90                             | 104.26.4.90                             | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 361  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 362  | cf.877774.xyz                           | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 394  | icook.tw                                | 172.66.158.115                          | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 399  | www.digitalocean.com                    | 104.19.173.68                           | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 418  | damien.ns.cloudflare.com                | 108.162.195.168                         | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 443  | lewis.ns.cloudflare.com                 | 108.162.195.159                         | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 452  | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 455  | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 462  | 172.64.229.7                            | 172.64.229.7                            | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 478  | abdullah.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cb                 | IPv6   | h2   | ✅ 成功 | 87       | cloudflare |
| 494  | 104.16.65.1                             | 104.16.65.1                             | IPv4   | h2   | ✅ 成功 | 87       | cloudflare |
| 43   | gamer.com.tw                            | 104.18.2.197                            | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 58   | www.gov.ua                              | 172.67.209.127                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 63   | iplocation.io                           | 104.26.11.222                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 113  | www.hugedomains.com                     | 104.26.6.37                             | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 115  | www.hugedomains.com                     | 2606:4700:20::681a:625                  | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 170  | toy-people.com                          | 104.26.2.36                             | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 174  | toy-people.com                          | 2606:4700:20::681a:224                  | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 182  | kyree.ns.cloudflare.com                 | 108.162.195.207                         | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 198  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 202  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 228  | cmcc.877774.xyz                         | 104.16.148.11                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 231  | cmcc.877774.xyz                         | 104.16.149.1                            | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 239  | bestcf.030101.xyz                       | 2606:4700:0:7809:f54e:ab56:90d6:b7ab    | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 267  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 315  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 366  | julio.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd1                 | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 371  | uriah.ns.cloudflare.com                 | 172.64.35.194                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 383  | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 422  | damien.ns.cloudflare.com                | 2606:4700:58::a29f:2ca8                 | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 428  | www.csgo.com                            | 195.85.59.161                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 429  | www.csgo.com                            | 195.85.59.95                            | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 486  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6   | h2   | ✅ 成功 | 88       | cloudflare |
| 487  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 490  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 88       | cloudflare |
| 10   | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 84   | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 90   | icook.hk                                | 2606:4700:3031::6815:5ad2               | IPv6   | h2   | ✅ 成功 | 89       | cloudflare |
| 136  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 89       | cloudflare |
| 187  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6   | h2   | ✅ 成功 | 89       | cloudflare |
| 212  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 217  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 248  | xn--b6gac.eu.org                        | 104.21.90.78                            | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 295  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 89       | cloudflare |
| 305  | benedict.ns.cloudflare.com              | 108.162.195.205                         | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 329  | silkbook.com                            | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 335  | whatismyipaddress.com                   | 2606:4700::6813:de4f                    | IPv6   | h2   | ✅ 成功 | 89       | cloudflare |
| 469  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 89       | cloudflare |
| 51   | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 88   | icook.hk                                | 172.67.161.104                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 125  | cfip.xxxxxxxx.tk                        | 104.20.255.53                           | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 154  | pranab.ns.cloudflare.com                | 108.162.195.199                         | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 208  | asia.877774.xyz                         | 104.17.142.146                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 275  | bowen.ns.cloudflare.com                 | 2a06:98c1:50::ac40:2353                 | IPv6   | h2   | ✅ 成功 | 90       | cloudflare |
| 341  | 172.67.106.26                           | 172.67.106.26                           | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 355  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 90       | cloudflare |
| 360  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 364  | julio.ns.cloudflare.com                 | 162.159.44.209                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 459  | 104.19.212.207                          | 104.19.212.207                          | IPv4   | h2   | ✅ 成功 | 90       | cloudflare |
| 496  | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | 2a06:98c1:3121:c677:c614:7606:cec1:f722 | IPv6   | h2   | ✅ 成功 | 90       | cloudflare |
| 9    | comicabc.com                            | 2606:4700:3036::6815:400a               | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 77   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 94   | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 126  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 188  | dylan.ns.cloudflare.com                 | 108.162.195.187                         | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 234  | cmcc.877774.xyz                         | 104.16.149.4                            | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 246  | cf.zhetengsha.eu.org                    | 2606:4700:440a::ac40:98f1               | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 333  | whatismyipaddress.com                   | 104.19.223.79                           | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 337  | www.visa.com.hk                         | 104.18.21.69                            | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 347  | dnschecker.org                          | 104.26.7.89                             | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 426  | www.7749tv.com                          | 104.16.115.36                           | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 446  | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 471  | 104.18.166.129                          | 104.18.166.129                          | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 482  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 91       | cloudflare |
| 497  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6   | h2   | ✅ 成功 | 91       | cloudflare |
| 7    | comicabc.com                            | 104.21.64.10                            | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 32   | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 92       | cloudflare |
| 38   | ipinfo.in                               | 104.21.21.129                           | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 139  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 215  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 314  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 92       | cloudflare |
| 328  | silkbook.com                            | 104.26.9.160                            | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 330  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 92       | cloudflare |
| 338  | www.visa.com.hk                         | 104.18.20.69                            | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 411  | otto.ns.cloudflare.com                  | 162.159.44.135                          | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 491  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 92       | cloudflare |
| 19   | wilson.ns.cloudflare.com                | 162.159.44.110                          | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 68   | cloudflare.182682.xyz                   | 104.21.227.134                          | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 287  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 392  | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 93       | cloudflare |
| 398  | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 407  | eur.877774.xyz                          | 104.21.29.164                           | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 445  | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 93       | cloudflare |
| 450  | japan.com                               | 104.26.4.60                             | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 485  | 104.18.255.167                          | 104.18.255.167                          | IPv4   | h2   | ✅ 成功 | 93       | cloudflare |
| 54   | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 74   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 108  | cf.877771.xyz                           | 104.21.80.180                           | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 143  | craig.ns.cloudflare.com                 | 172.64.35.192                           | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 150  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:e070:5d82:552a:2069:936a | IPv6   | h2   | ✅ 成功 | 94       | cloudflare |
| 199  | 104.17.79.11                            | 104.17.79.11                            | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 285  | 162.159.36.104                          | 162.159.36.104                          | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 390  | 104.18.37.40                            | 104.18.37.40                            | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 395  | icook.tw                                | 104.20.28.74                            | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 440  | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 458  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 465  | 162.159.128.253                         | 162.159.128.253                         | IPv4   | h2   | ✅ 成功 | 94       | cloudflare |
| 26   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 49   | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 59   | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 121  | cfip.xxxxxxxx.tk                        | 104.16.232.223                          | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 141  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 152  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 171  | toy-people.com                          | 172.67.72.18                            | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 237  | bestcf.030101.xyz                       | 104.19.157.251                          | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 326  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 334  | whatismyipaddress.com                   | 104.19.222.79                           | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 350  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 389  | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 405  | eur.877774.xyz                          | 104.21.47.209                           | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 451  | japan.com                               | 104.26.5.60                             | IPv4   | h2   | ✅ 成功 | 95       | cloudflare |
| 502  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 95       | cloudflare |
| 61   | iplocation.io                           | 172.67.70.100                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 69   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 70   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 102  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 200  | www.whatismyip.com                      | 104.26.12.23                            | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 218  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 344  | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 96       | cloudflare |
| 438  | stock.hostmonit.com                     | 2606:4700:3033::ac43:bbfb               | IPv6   | h2   | ✅ 成功 | 96       | cloudflare |
| 456  | 172.64.48.226                           | 172.64.48.226                           | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 457  | 162.159.24.131                          | 162.159.24.131                          | IPv4   | h2   | ✅ 成功 | 96       | cloudflare |
| 243  | saas.sin.fan                            | 162.159.36.20                           | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 247  | cf.zhetengsha.eu.org                    | 2a06:98c1:3105::6812:230f               | IPv6   | h2   | ✅ 成功 | 97       | cloudflare |
| 372  | uriah.ns.cloudflare.com                 | 162.159.44.194                          | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 474  | abdullah.ns.cloudflare.com              | 162.159.44.203                          | IPv4   | h2   | ✅ 成功 | 97       | cloudflare |
| 17   | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 122  | cfip.xxxxxxxx.tk                        | 104.25.105.1                            | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 211  | cmcc.877774.xyz                         | 104.16.149.7                            | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 220  | cmcc.877774.xyz                         | 104.16.148.3                            | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 339  | 104.18.42.26                            | 104.18.42.26                            | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 348  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 368  | julio.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d1                 | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 402  | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 412  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 413  | otto.ns.cloudflare.com                  | 172.64.35.135                           | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 434  | www.wto.org                             | 2606:4700:4406::ac40:9242               | IPv6   | h2   | ✅ 成功 | 98       | cloudflare |
| 479  | 104.19.148.121                          | 104.19.148.121                          | IPv4   | h2   | ✅ 成功 | 98       | cloudflare |
| 206  | [2606:4700:4409::5b5b:7758]             | 2606:4700:4409::5b5b:7758               | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 318  | ip.gs                                   | 2606:4700:3035::ac43:a01c               | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 320  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 359  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 99       | cloudflare |
| 367  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 99       | cloudflare |
| 424  | 104.19.175.123                          | 104.19.175.123                          | IPv4   | h2   | ✅ 成功 | 99       | cloudflare |
| 460  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 99       | cloudflare |
| 194  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 100      | cloudflare |
| 238  | bestcf.030101.xyz                       | 104.19.156.139                          | IPv4   | h2   | ✅ 成功 | 100      | cloudflare |
| 379  | tasteatlas.com                          | 2606:4700::6811:2569                    | IPv6   | h2   | ✅ 成功 | 100      | cloudflare |
| 444  | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 100      | cloudflare |
| 449  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 100      | cloudflare |
| 284  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h2   | ✅ 成功 | 101      | cloudflare |
| 319  | local-aria2-webui.masx200.ddns-ip.net   | 172.67.157.182                          | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 439  | 198.41.208.15                           | 198.41.208.15                           | IPv4   | h2   | ✅ 成功 | 101      | cloudflare |
| 103  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 178  | decker.ns.cloudflare.com                | 162.159.44.155                          | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 196  | cloudflare-ip.mofashi.ltd               | 2606:4700:3037::6815:48e9               | IPv6   | h2   | ✅ 成功 | 102      | cloudflare |
| 327  | silkbook.com                            | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 436  | stock.hostmonit.com                     | 104.21.7.193                            | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 448  | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 102      | cloudflare |
| 281  | moura.ns.cloudflare.com                 | 108.162.195.217                         | IPv4   | h2   | ✅ 成功 | 103      | cloudflare |
| 391  | 172.64.35.24                            | 172.64.35.24                            | IPv4   | h2   | ✅ 成功 | 103      | cloudflare |
| 419  | damien.ns.cloudflare.com                | 162.159.44.168                          | IPv4   | h2   | ✅ 成功 | 103      | cloudflare |
| 476  | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 103      | cloudflare |
| 176  | decker.ns.cloudflare.com                | 108.162.195.155                         | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 230  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 336  | whatismyipaddress.com                   | 2606:4700::6813:df4f                    | IPv6   | h2   | ✅ 成功 | 104      | cloudflare |
| 410  | ifconfig.co                             | 2606:4700:3037::6815:365b               | IPv6   | h2   | ✅ 成功 | 104      | cloudflare |
| 420  | damien.ns.cloudflare.com                | 172.64.35.168                           | IPv4   | h2   | ✅ 成功 | 104      | cloudflare |
| 190  | dylan.ns.cloudflare.com                 | 162.159.44.187                          | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 222  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 271  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 272  | bowen.ns.cloudflare.com                 | 162.159.44.83                           | IPv4   | h2   | ✅ 成功 | 105      | cloudflare |
| 433  | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 105      | cloudflare |
| 133  | sullivan.ns.cloudflare.com              | 172.64.35.161                           | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |
| 189  | dylan.ns.cloudflare.com                 | 172.64.35.187                           | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |
| 437  | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 106      | cloudflare |
| 441  | lewis.ns.cloudflare.com                 | 172.64.35.159                           | IPv4   | h2   | ✅ 成功 | 106      | cloudflare |
| 135  | sullivan.ns.cloudflare.com              | 162.159.44.161                          | IPv4   | h2   | ✅ 成功 | 107      | cloudflare |
| 261  | braden.ns.cloudflare.com                | 108.162.195.169                         | IPv4   | h2   | ✅ 成功 | 108      | cloudflare |
| 380  | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 108      | cloudflare |
| 292  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4   | h2   | ✅ 成功 | 109      | cloudflare |
| 1    | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 114      | cloudflare |
| 156  | pranab.ns.cloudflare.com                | 172.64.35.199                           | IPv4   | h2   | ✅ 成功 | 114      | cloudflare |
| 303  | benedict.ns.cloudflare.com              | 172.64.35.205                           | IPv4   | h2   | ✅ 成功 | 116      | cloudflare |
| 363  | julio.ns.cloudflare.com                 | 108.162.195.209                         | IPv4   | h2   | ✅ 成功 | 116      | cloudflare |
| 2    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | h2   | ✅ 成功 | 117      | cloudflare |
| 401  | www.digitalocean.com                    | 2606:4700::6813:ae44                    | IPv6   | h2   | ✅ 成功 | 117      | cloudflare |
| 370  | uriah.ns.cloudflare.com                 | 108.162.195.194                         | IPv4   | h2   | ✅ 成功 | 118      | cloudflare |
| 3    | 172.64.154.18                           | 172.64.154.18                           | IPv4   | h2   | ✅ 成功 | 119      | cloudflare |
| 20   | wilson.ns.cloudflare.com                | 172.64.35.110                           | IPv4   | h2   | ✅ 成功 | 119      | cloudflare |
| 353  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4   | h2   | ✅ 成功 | 119      | cloudflare |
| 473  | abdullah.ns.cloudflare.com              | 108.162.195.203                         | IPv4   | h2   | ✅ 成功 | 119      | cloudflare |
| 137  | sullivan.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a1                 | IPv6   | h2   | ✅ 成功 | 120      | cloudflare |
| 283  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 120      | cloudflare |
| 294  | rustam.ns.cloudflare.com                | 162.159.44.148                          | IPv4   | h2   | ✅ 成功 | 120      | cloudflare |
| 21   | wilson.ns.cloudflare.com                | 2803:f800:50::6ca2:c36e                 | IPv6   | h2   | ✅ 成功 | 121      | cloudflare |
| 430  | 198.41.194.162                          | 198.41.194.162                          | IPv4   | h2   | ✅ 成功 | 121      | cloudflare |
| 475  | abdullah.ns.cloudflare.com              | 172.64.35.203                           | IPv4   | h2   | ✅ 成功 | 121      | cloudflare |
| 23   | wilson.ns.cloudflare.com                | 2606:4700:58::a29f:2c6e                 | IPv6   | h2   | ✅ 成功 | 122      | cloudflare |
| 27   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h2   | ✅ 成功 | 122      | cloudflare |
| 179  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 122      | cloudflare |
| 148  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h2   | ✅ 成功 | 123      | cloudflare |
| 157  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 123      | cloudflare |
| 260  | cf.090227.xyz                           | 2606:4700:4407::ac40:9052               | IPv6   | h2   | ✅ 成功 | 123      | cloudflare |
| 425  | 108.162.198.54                          | 108.162.198.54                          | IPv4   | h2   | ✅ 成功 | 123      | cloudflare |
| 29   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 124      | cloudflare |
| 159  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h2   | ✅ 成功 | 124      | cloudflare |
| 354  | ashton.ns.cloudflare.com                | 172.64.35.173                           | IPv4   | h2   | ✅ 成功 | 124      | cloudflare |
| 369  | [2606:4700:4403::7357:544f]             | 2606:4700:4403::7357:544f               | IPv6   | h2   | ✅ 成功 | 124      | cloudflare |
| 164  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 125      | cloudflare |
| 307  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 125      | cloudflare |
| 416  | otto.ns.cloudflare.com                  | 2a06:98c1:50::ac40:2387                 | IPv6   | h2   | ✅ 成功 | 125      | cloudflare |
| 95   | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 126      | cloudflare |
| 180  | decker.ns.cloudflare.com                | 2803:f800:50::6ca2:c39b                 | IPv6   | h2   | ✅ 成功 | 126      | cloudflare |
| 185  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6   | h2   | ✅ 成功 | 126      | cloudflare |
| 352  | ashton.ns.cloudflare.com                | 108.162.195.173                         | IPv4   | h2   | ✅ 成功 | 126      | cloudflare |
| 282  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h2   | ✅ 成功 | 127      | cloudflare |
| 461  | 162.159.58.65                           | 162.159.58.65                           | IPv4   | h2   | ✅ 成功 | 127      | cloudflare |
| 96   | huxley.ns.cloudflare.com                | 2a06:98c1:50::ac40:23bc                 | IPv6   | h2   | ✅ 成功 | 128      | cloudflare |
| 191  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 128      | cloudflare |
| 280  | moura.ns.cloudflare.com                 | 172.64.35.217                           | IPv4   | h2   | ✅ 成功 | 128      | cloudflare |
| 375  | uriah.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c2                 | IPv6   | h2   | ✅ 成功 | 128      | cloudflare |
| 158  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 129      | cloudflare |
| 177  | decker.ns.cloudflare.com                | 172.64.35.155                           | IPv4   | h2   | ✅ 成功 | 129      | cloudflare |
| 373  | uriah.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23c2                 | IPv6   | h2   | ✅ 成功 | 129      | cloudflare |
| 421  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 129      | cloudflare |
| 447  | 172.64.52.127                           | 172.64.52.127                           | IPv4   | h2   | ✅ 成功 | 129      | cloudflare |
| 264  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h2   | ✅ 成功 | 130      | cloudflare |
| 266  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 130      | cloudflare |
| 297  | rustam.ns.cloudflare.com                | 2803:f800:50::6ca2:c394                 | IPv6   | h2   | ✅ 成功 | 130      | cloudflare |
| 423  | damien.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a8                 | IPv6   | h2   | ✅ 成功 | 130      | cloudflare |
| 97   | huxley.ns.cloudflare.com                | 2606:4700:58::a29f:2cbc                 | IPv6   | h2   | ✅ 成功 | 131      | cloudflare |
| 356  | ashton.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ad                 | IPv6   | h2   | ✅ 成功 | 131      | cloudflare |
| 160  | cris.ns.cloudflare.com                  | 108.162.195.202                         | IPv4   | h2   | ✅ 成功 | 132      | cloudflare |
| 270  | bowen.ns.cloudflare.com                 | 172.64.35.83                            | IPv4   | h2   | ✅ 成功 | 135      | cloudflare |
| 415  | otto.ns.cloudflare.com                  | 2606:4700:58::a29f:2c87                 | IPv6   | h2   | ✅ 成功 | 135      | cloudflare |
| 262  | braden.ns.cloudflare.com                | 162.159.44.169                          | IPv4   | h2   | ✅ 成功 | 136      | cloudflare |
| 477  | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 136      | cloudflare |
| 144  | craig.ns.cloudflare.com                 | 108.162.195.192                         | IPv4   | h2   | ✅ 成功 | 137      | cloudflare |
| 484  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 137      | cloudflare |
| 25   | trevor.ns.cloudflare.com                | 108.162.195.154                         | IPv4   | h2   | ✅ 成功 | 138      | cloudflare |
| 192  | dylan.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3bb                 | IPv6   | h2   | ✅ 成功 | 140      | cloudflare |
| 24   | trevor.ns.cloudflare.com                | 172.64.35.154                           | IPv4   | h2   | ✅ 成功 | 141      | cloudflare |
| 193  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 141      | cloudflare |
| 304  | benedict.ns.cloudflare.com              | 162.159.44.205                          | IPv4   | h2   | ✅ 成功 | 141      | cloudflare |
| 161  | cris.ns.cloudflare.com                  | 162.159.44.202                          | IPv4   | h2   | ✅ 成功 | 149      | cloudflare |
| 279  | moura.ns.cloudflare.com                 | 162.159.44.217                          | IPv4   | h2   | ✅ 成功 | 150      | cloudflare |
| 18   | wilson.ns.cloudflare.com                | 108.162.195.110                         | IPv4   | h2   | ✅ 成功 | 155      | cloudflare |
| 365  | julio.ns.cloudflare.com                 | 172.64.35.209                           | IPv4   | h2   | ✅ 成功 | 161      | cloudflare |
| 145  | craig.ns.cloudflare.com                 | 162.159.44.192                          | IPv4   | h2   | ✅ 成功 | 163      | cloudflare |
| 183  | kyree.ns.cloudflare.com                 | 162.159.44.207                          | IPv4   | h2   | ✅ 成功 | 163      | cloudflare |
| 93   | huxley.ns.cloudflare.com                | 108.162.195.188                         | IPv4   | h2   | ✅ 成功 | 170      | cloudflare |
| 92   | huxley.ns.cloudflare.com                | 172.64.35.188                           | IPv4   | h2   | ✅ 成功 | 172      | cloudflare |
| 162  | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4   | h2   | ✅ 成功 | 176      | cloudflare |
| 263  | braden.ns.cloudflare.com                | 172.64.35.169                           | IPv4   | h2   | ✅ 成功 | 210      | cloudflare |
| 256  | 141.147.185.63                          | 141.147.185.63                          | IPv4   | h2   | ✅ 成功 | 378      | cloudflare |
| 345  | 34.143.159.175                          | 34.143.159.175                          | IPv4   | h2   | ✅ 成功 | 538      | cloudflare |
| 408  | 3.0.50.69                               | 3.0.50.69                               | IPv4   | h2   | ✅ 成功 | 832      | cloudflare |
| 80   | 168.138.165.174                         | 168.138.165.174                         | IPv4   | h2   | ✅ 成功 | 2570     | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 386 条记录
- **正常 (100-200ms)**: 109 条记录
- **慢 (200-500ms)**: 2 条记录
- **很慢 (>500ms)**: 3 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
