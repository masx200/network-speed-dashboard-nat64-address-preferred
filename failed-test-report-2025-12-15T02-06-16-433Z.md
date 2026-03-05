# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/15 02:06:16
- **数据来源**: connectivity_results-20251215-020615.json
- **总测试数**: 458
- **失败测试数**: 2
- **成功测试数**: 456
- **失败率**: 0.44%
- **平均延迟**: 84.39ms
- **最小延迟**: 47ms
- **最大延迟**: 921ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/15 02:06:16
- **IP地址**: 2a09:bac5:9f24:a0::10:4f9
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 36.4766, -78.1847
- **时区**: America/New_York
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
| 7    | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 286  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 2 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 172.64（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 2 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                              | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ----------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 245  | iplocation.io                         | 2606:4700:20::681a:bde              | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 263  | 104.18.14.76                          | 104.18.14.76                        | IPv4   | h2   | ✅ 成功 | 50       | cloudflare |
| 182  | cf.877771.xyz                         | 172.67.152.183                      | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 294  | cfip.xxxxxxxx.tk                      | 104.16.241.229                      | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 445  | fbi.gov                               | 104.16.148.244                      | IPv4   | h2   | ✅ 成功 | 51       | cloudflare |
| 80   | ifconfig.co                           | 2606:4700:3037::6815:365b           | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 82   | www.csgo.com                          | 195.85.59.95                        | IPv4   | h2   | ✅ 成功 | 52       | cloudflare |
| 179  | dnschecker.org                        | 2606:4700:20::681a:659              | IPv6   | h2   | ✅ 成功 | 52       | cloudflare |
| 185  | cf.877771.xyz                         | 2606:4700:3033::ac43:98b7           | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 210  | ipv4.ip.sb                            | 104.26.12.31                        | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 301  | www.udemy.com                         | 104.16.143.237                      | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 321  | trevor.ns.cloudflare.com              | 172.64.35.154                       | IPv4   | h2   | ✅ 成功 | 53       | cloudflare |
| 421  | www.whatismyip.com                    | 2606:4700:20::ac43:4581             | IPv6   | h2   | ✅ 成功 | 53       | cloudflare |
| 27   | time.is                               | 2606:4700:20::681a:c36              | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 150  | singapore.com                         | 2606:4700:20::681a:d8c              | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 299  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1             | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 399  | pranab.ns.cloudflare.com              | 172.64.35.199                       | IPv4   | h2   | ✅ 成功 | 54       | cloudflare |
| 458  | bowen.ns.cloudflare.com               | 2a06:98c1:50::ac40:2353             | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 208  | 104.16.45.84                          | 104.16.45.84                        | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 211  | ipv4.ip.sb                            | 172.67.75.172                       | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 323  | trevor.ns.cloudflare.com              | 162.159.44.154                      | IPv4   | h2   | ✅ 成功 | 55       | cloudflare |
| 181  | dnschecker.org                        | 2606:4700:20::ac43:49d8             | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 189  | julio.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d1             | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 305  | 172.67.120.0                          | 172.67.120.0                        | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 322  | trevor.ns.cloudflare.com              | 108.162.195.154                     | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 366  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf             | IPv6   | h2   | ✅ 成功 | 56       | cloudflare |
| 370  | 104.16.223.179                        | 104.16.223.179                      | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 20   | comicabc.com                          | 2606:4700:3030::ac43:ae15           | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 197  | uriah.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c2             | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 295  | sullivan.ns.cloudflare.com            | 108.162.195.161                     | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 372  | cmcc.877774.xyz                       | 104.16.149.6                        | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 396  | cmcc.877774.xyz                       | 104.16.149.4                        | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 415  | asia.877774.xyz                       | 104.17.142.146                      | IPv4   | h2   | ✅ 成功 | 57       | cloudflare |
| 42   | cloudflare.182682.xyz                 | 2606:4700:3032::818:669e            | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 101  | japan.com                             | 104.26.4.60                         | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 113  | abdullah.ns.cloudflare.com            | 2606:4700:58::a29f:2ccb             | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 129  | ashton.ns.cloudflare.com              | 172.64.35.173                       | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 144  | ip.gs                                 | 2606:4700:3035::ac43:a01c           | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 163  | ip.sb                                 | 2606:4700:20::681a:c1f              | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 225  | www.gov.ua                            | 2606:4700:3033::ac43:d17f           | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 253  | icook.hk                              | 2606:4700:3037::ac43:a168           | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 273  | www.hugedomains.com                   | 104.26.7.37                         | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 274  | www.hugedomains.com                   | 2606:4700:20::681a:625              | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 283  | cfip.xxxxxxxx.tk                      | 198.41.214.141                      | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 374  | cmcc.877774.xyz                       | 104.16.149.8                        | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 380  | cmcc.877774.xyz                       | 104.16.148.1                        | IPv4   | h2   | ✅ 成功 | 58       | cloudflare |
| 422  | www.whatismyip.com                    | 2606:4700:20::681a:d17              | IPv6   | h2   | ✅ 成功 | 58       | cloudflare |
| 116  | www.glassdoor.com                     | 104.17.64.70                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 160  | ip.sb                                 | 104.26.13.31                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 193  | uriah.ns.cloudflare.com               | 108.162.195.194                     | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 207  | 104.18.39.196                         | 104.18.39.196                       | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 220  | ipinfo.in                             | 2606:4700:3037::ac43:c6cb           | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 266  | www.okcupid.com                       | 104.16.223.254                      | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 293  | cfip.xxxxxxxx.tk                      | 104.18.228.35                       | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 311  | zread.ai                              | 2606:4700:3033::6815:4cf0           | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 318  | cu.877774.xyz                         | 104.26.4.119                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 387  | cmcc.877774.xyz                       | 104.16.148.8                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 407  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb             | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 439  | saas.sin.fan                          | 162.159.36.20                       | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 448  | fbi.gov                               | 2606:4700::6810:95f4                | IPv6   | h2   | ✅ 成功 | 59       | cloudflare |
| 6    | [2606:4700:4403::7357:544f]           | 2606:4700:4403::7357:544f           | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 76   | damien.ns.cloudflare.com              | 2803:f800:50::6ca2:c3a8             | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 87   | www.wto.org                           | 2606:4700:4406::ac40:9242           | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 141  | 104.18.42.26                          | 104.18.42.26                        | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 167  | www.visa.com.hk                       | 104.18.21.69                        | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 190  | julio.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d1             | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 198  | www.pcmag.com                         | 104.16.21.118                       | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 240  | 104.18.254.88                         | 104.18.254.88                       | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 252  | icook.hk                              | 172.67.161.104                      | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 258  | huxley.ns.cloudflare.com              | 162.159.44.188                      | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 277  | tasteatlas.com                        | 104.17.36.105                       | IPv4   | h2   | ✅ 成功 | 60       | cloudflare |
| 280  | tasteatlas.com                        | 2606:4700::6811:2569                | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 456  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53             | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 84   | www.wto.org                           | 172.64.146.66                       | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 104  | japan.com                             | 2606:4700:20::681a:43c              | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 162  | ip.sb                                 | 2606:4700:20::ac43:4bac             | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 169  | whatismyipaddress.com                 | 104.19.222.79                       | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 172  | cf.877774.xyz                         | 172.64.146.66                       | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 173  | cf.877774.xyz                         | 104.18.41.190                       | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 201  | www.pcmag.com                         | 2606:4700::6810:1476                | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 204  | cf.0sm.com                            | 2606:4700:3032::6815:785            | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 214  | steamdb.info                          | 172.66.175.250                      | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 222  | www.gov.ua                            | 104.21.23.72                        | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 227  | ct.877774.xyz                         | 172.64.229.161                      | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 246  | iplocation.io                         | 2606:4700:20::681a:ade              | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 254  | icook.hk                              | 2606:4700:3031::6815:5ad2           | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 264  | 162.159.133.85                        | 162.159.133.85                      | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 265  | [2606:4700:440b::3e6e:5f06]           | 2606:4700:440b::3e6e:5f06           | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 276  | www.hugedomains.com                   | 2606:4700:20::ac43:46bf             | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 282  | cfip.xxxxxxxx.tk                      | 104.25.105.1                        | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 289  | cfip.xxxxxxxx.tk                      | 104.17.127.110                      | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 365  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf             | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 424  | braden.ns.cloudflare.com              | 172.64.35.169                       | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 68   | eur.877774.xyz                        | 104.21.26.150                       | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 89   | stock.hostmonit.com                   | 172.67.187.251                      | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 102  | japan.com                             | 2606:4700:20::681a:53c              | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 105  | www.ipchicken.com                     | 172.67.68.101                       | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 161  | ip.sb                                 | 2606:4700:20::681a:d1f              | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 165  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6           | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 209  | ipv4.ip.sb                            | 104.26.13.31                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 231  | ct.877774.xyz                         | 172.64.229.195                      | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 268  | www.okcupid.com                       | 104.17.48.63                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 306  | www.visa.cn                           | 162.159.153.2                       | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 312  | cu.877774.xyz                         | 104.26.4.113                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 315  | cu.877774.xyz                         | 104.26.4.116                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 368  | na.877774.xyz                         | 104.18.187.25                       | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 386  | cmcc.877774.xyz                       | 104.16.148.7                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 406  | dylan.ns.cloudflare.com               | 172.64.35.187                       | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 409  | dylan.ns.cloudflare.com               | 2803:f800:50::6ca2:c3bb             | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 413  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac           | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 437  | xn--b6gac.eu.org                      | 2606:4700:3037::ac43:99fd           | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 444  | cf.zhetengsha.eu.org                  | 2a06:98c1:3108::6812:2a62           | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 16   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a           | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 91   | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1            | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 146  | singapore.com                         | 104.26.13.140                       | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 191  | julio.ns.cloudflare.com               | 2606:4700:58::a29f:2cd1             | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 233  | ct.877774.xyz                         | 172.64.229.236                      | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 275  | www.hugedomains.com                   | 2606:4700:20::681a:725              | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 279  | tasteatlas.com                        | 2606:4700::6811:2469                | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 329  | freeyx.cloudflare88.eu.org            | 141.101.120.1                       | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 398  | pranab.ns.cloudflare.com              | 108.162.195.199                     | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 438  | xn--b6gac.eu.org                      | 2606:4700:3035::6815:5a4e           | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 31   | moura.ns.cloudflare.com               | 172.64.35.217                       | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 43   | cloudflare.182682.xyz                 | 2606:4700:3035::1a4f:5642           | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 48   | wilson.ns.cloudflare.com              | 162.159.44.110                      | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 55   | 104.26.13.31                          | 104.26.13.31                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 58   | 104.19.223.58                         | 104.19.223.58                       | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 67   | eur.877774.xyz                        | 104.21.47.209                       | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 100  | japan.com                             | 172.67.70.92                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 106  | www.ipchicken.com                     | 104.26.6.112                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 131  | ashton.ns.cloudflare.com              | 2606:4700:58::a29f:2cad             | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 140  | 104.17.68.85                          | 104.17.68.85                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 203  | cf.0sm.com                            | 172.67.187.145                      | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 239  | 103.160.204.59                        | 103.160.204.59                      | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 250  | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e           | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 287  | cfip.xxxxxxxx.tk                      | 104.20.255.53                       | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 364  | kyree.ns.cloudflare.com               | 2606:4700:58::a29f:2ccf             | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 431  | bestcf.030101.xyz                     | 104.17.54.87                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 434  | bestcf.030101.xyz                     | 2606:4700:0:6de1:d9f:4f26:52e9:5692 | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 18   | comicabc.com                          | 172.67.174.21                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 39   | cloudflare.182682.xyz                 | 2606:4700:8ca0::3dc4:21a2           | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 117  | palera.in                             | 104.21.58.72                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 127  | rustam.ns.cloudflare.com              | 2a06:98c1:50::ac40:2394             | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 142  | ip.gs                                 | 104.21.14.176                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 156  | silkbook.com                          | 2606:4700:20::681a:9a0              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 170  | whatismyipaddress.com                 | 2606:4700::6813:de4f                | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 202  | cf.0sm.com                            | 104.21.7.133                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 218  | ipinfo.in                             | 172.67.198.203                      | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 223  | www.gov.ua                            | 172.67.209.127                      | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 224  | www.gov.ua                            | 2606:4700:3031::6815:1748           | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 272  | www.hugedomains.com                   | 104.26.6.37                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 310  | zread.ai                              | 2606:4700:3032::ac43:ca4e           | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 314  | cu.877774.xyz                         | 104.26.4.115                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 336  | craig.ns.cloudflare.com               | 108.162.195.192                     | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 362  | kyree.ns.cloudflare.com               | 162.159.44.207                      | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 373  | cmcc.877774.xyz                       | 104.16.149.7                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 375  | cmcc.877774.xyz                       | 104.16.149.9                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 376  | cmcc.877774.xyz                       | 104.16.149.10                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 383  | cmcc.877774.xyz                       | 104.16.148.4                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 411  | cloudflare-ip.mofashi.ltd             | 172.67.155.172                      | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 420  | www.whatismyip.com                    | 2606:4700:20::681a:c17              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 436  | xn--b6gac.eu.org                      | 104.21.90.78                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 440  | saas.sin.fan                          | 162.159.36.5                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 59   | otto.ns.cloudflare.com                | 108.162.195.135                     | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 85   | www.wto.org                           | 104.18.41.190                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 96   | lewis.ns.cloudflare.com               | 2a06:98c1:50::ac40:239f             | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 115  | www.glassdoor.com                     | 104.16.25.46                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 120  | palera.in                             | 2606:4700:3032::ac43:9d7a           | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 126  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394             | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 171  | whatismyipaddress.com                 | 2606:4700::6813:df4f                | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 175  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be           | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 176  | dnschecker.org                        | 104.26.6.89                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 183  | cf.877771.xyz                         | 104.21.80.180                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 192  | uriah.ns.cloudflare.com               | 172.64.35.194                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 213  | steamdb.info                          | 104.20.34.212                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 234  | ct.877774.xyz                         | 172.64.229.44                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 236  | 172.67.110.232                        | 172.67.110.232                      | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 269  | www.okcupid.com                       | 104.16.144.63                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 291  | cfip.xxxxxxxx.tk                      | 190.93.247.169                      | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 292  | cfip.xxxxxxxx.tk                      | 190.93.244.201                      | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 369  | na.877774.xyz                         | 104.19.74.233                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 446  | fbi.gov                               | 104.16.149.244                      | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 49   | wilson.ns.cloudflare.com              | 2a06:98c1:50::ac40:236e             | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 74   | damien.ns.cloudflare.com              | 2606:4700:58::a29f:2ca8             | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 79   | ifconfig.co                           | 104.21.54.91                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 108  | abdullah.ns.cloudflare.com            | 172.64.35.203                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 180  | dnschecker.org                        | 2606:4700:20::681a:759              | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 221  | 172.67.75.172                         | 172.67.75.172                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 248  | www.4chan.org                         | 104.16.229.229                      | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 251  | icook.hk                              | 104.21.90.210                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 281  | cfip.xxxxxxxx.tk                      | 188.114.97.144                      | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 285  | cfip.xxxxxxxx.tk                      | 190.93.246.67                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 319  | cu.877774.xyz                         | 104.26.4.111                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 418  | www.whatismyip.com                    | 104.26.13.23                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 432  | bestcf.030101.xyz                     | 104.17.214.216                      | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 26   | time.is                               | 2606:4700:20::ac43:449d             | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 36   | www.digitalocean.com                  | 104.19.173.68                       | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 50   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e             | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 77   | 108.162.198.54                        | 108.162.198.54                      | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 109  | abdullah.ns.cloudflare.com            | 162.159.44.203                      | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 1 条记录
- **快 (50-100ms)**: 199 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

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
