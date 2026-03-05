# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/14 14:19:26
- **数据来源**: connectivity_results-20251214-141925.json
- **总测试数**: 442
- **失败测试数**: 4
- **成功测试数**: 438
- **失败率**: 0.90%
- **平均延迟**: 82.65ms
- **最小延迟**: 54ms
- **最大延迟**: 739ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/14 14:19:26
- **IP地址**: 2a09:bac5:c853:2832::401:2f
- **国家/地区**: United States (US)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: North America (NA)
- **地理坐标**: 38.6877, -77.8369
- **时区**: America/New_York
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 4 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (4 次测试)

| 序号 | 主机/域名                | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ------------------------ | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 19   | cfip.xxxxxxxx.tk         | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 169  | pranab.ns.cloudflare.com | 162.159.44.199 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.199:443: i/o timeout |
| 373  | 172.64.201.25            | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 421  | otto.ns.cloudflare.com   | 162.159.44.135 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.135:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 4 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 4 次超时，主要集中在IP段 162.159（2
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 4 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                               | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------------------ | ------ | ---- | ------- | -------- | ---------- |
| 30   | wilson.ns.cloudflare.com              | 2606:4700:58::a29f:2c6e              | IPv6   | h2   | ✅ 成功 | 54       | cloudflare |
| 199  | saas.sin.fan                          | 162.159.36.20                        | IPv4   | h2   | ✅ 成功 | 56       | cloudflare |
| 17   | www.ipget.net                         | 2606:4700:3031::ac43:cf1a            | IPv6   | h2   | ✅ 成功 | 57       | cloudflare |
| 22   | toy-people.com                        | 172.67.72.18                         | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 84   | iplocation.io                         | 104.26.11.222                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 231  | cf.zhetengsha.eu.org                  | 104.18.43.174                        | IPv4   | h2   | ✅ 成功 | 59       | cloudflare |
| 16   | www.ipget.net                         | 2606:4700:3036::6815:fd4             | IPv6   | h2   | ✅ 成功 | 60       | cloudflare |
| 7    | freeyx.cloudflare88.eu.org            | 141.101.120.163                      | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 9    | freeyx.cloudflare88.eu.org            | 2606:4700:3010:0:fb:e00f:f254:e667   | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 44   | cf.0sm.com                            | 104.21.7.133                         | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 131  | www.visa.com.hk                       | 104.18.20.69                         | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 175  | 104.17.79.11                          | 104.17.79.11                         | IPv4   | h2   | ✅ 成功 | 61       | cloudflare |
| 296  | ip.sb                                 | 2606:4700:20::681a:d1f               | IPv6   | h2   | ✅ 成功 | 61       | cloudflare |
| 147  | cu.877774.xyz                         | 104.26.4.113                         | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 152  | zread.ai                              | 172.67.202.78                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 240  | cf.090227.xyz                         | 172.64.144.82                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 259  | www.glassdoor.com                     | 104.16.25.46                         | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 309  | singapore.com                         | 2606:4700:20::ac43:4bc2              | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 343  | dnschecker.org                        | 2606:4700:20::681a:759               | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 386  | icook.tw                              | 2606:4700:10::6814:1c4a              | IPv6   | h2   | ✅ 成功 | 62       | cloudflare |
| 412  | www.wto.org                           | 172.64.146.66                        | IPv4   | h2   | ✅ 成功 | 62       | cloudflare |
| 25   | toy-people.com                        | 2606:4700:20::681a:224               | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 59   | 172.67.75.172                         | 172.67.75.172                        | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 62   | ipv4.ip.sb                            | 104.26.12.31                         | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 103  | www.visa.cn                           | 162.159.153.2                        | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 116  | www.hugedomains.com                   | 104.26.6.37                          | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 234  | cf.zhetengsha.eu.org                  | 2606:4700:4407::ac40:9052            | IPv6   | h2   | ✅ 成功 | 63       | cloudflare |
| 405  | ifconfig.co                           | 172.67.168.106                       | IPv4   | h2   | ✅ 成功 | 63       | cloudflare |
| 13   | comicabc.com                          | 2606:4700:3030::ac43:ae15            | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 109  | www.okcupid.com                       | 104.16.144.63                        | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 276  | palera.in                             | 172.67.157.122                       | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 297  | ip.sb                                 | 2606:4700:20::681a:c1f               | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 300  | ip.gs                                 | 2606:4700:3036::6815:eb0             | IPv6   | h2   | ✅ 成功 | 64       | cloudflare |
| 433  | japan.com                             | 172.67.70.92                         | IPv4   | h2   | ✅ 成功 | 64       | cloudflare |
| 6    | na.877774.xyz                         | 104.19.74.233                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 11   | comicabc.com                          | 104.21.64.10                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 53   | steamdb.info                          | 172.66.175.250                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 55   | steamdb.info                          | 2606:4700:10::6814:22d4              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 64   | ct.877774.xyz                         | 172.64.229.185                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 66   | ct.877774.xyz                         | 172.64.229.217                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 89   | www.4chan.org                         | 104.16.228.229                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 90   | www.4chan.org                         | 104.16.229.229                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 132  | www.visa.com.sg                       | 104.18.13.229                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 140  | cu.877774.xyz                         | 104.26.4.115                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 142  | cu.877774.xyz                         | 104.26.4.117                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 144  | cu.877774.xyz                         | 104.26.4.119                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 183  | dylan.ns.cloudflare.com               | 2a06:98c1:50::ac40:23bb              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 204  | cmcc.877774.xyz                       | 104.16.148.6                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 210  | cmcc.877774.xyz                       | 104.16.148.12                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 219  | cmcc.877774.xyz                       | 104.16.149.8                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 239  | cf.090227.xyz                         | 104.18.43.174                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 257  | www.ipchicken.com                     | 104.26.7.112                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 260  | www.glassdoor.com                     | 104.17.64.70                         | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 289  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 295  | ip.sb                                 | 2606:4700:20::ac43:4bac              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 307  | singapore.com                         | 172.67.75.194                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 308  | singapore.com                         | 2606:4700:20::681a:d8c               | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 340  | dnschecker.org                        | 104.26.7.89                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 353  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2              | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 390  | www.digitalocean.com                  | 2606:4700::6813:ad44                 | IPv6   | h2   | ✅ 成功 | 65       | cloudflare |
| 392  | 172.67.181.209                        | 172.67.181.209                       | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 395  | eur.877774.xyz                        | 104.21.26.150                        | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 431  | japan.com                             | 104.26.4.60                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 432  | japan.com                             | 104.26.5.60                          | IPv4   | h2   | ✅ 成功 | 65       | cloudflare |
| 21   | toy-people.com                        | 104.26.3.36                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 23   | toy-people.com                        | 104.26.2.36                          | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 34   | cris.ns.cloudflare.com                | 162.159.44.202                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 42   | www.pcmag.com                         | 2606:4700::6810:1576                 | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 51   | ipinfo.in                             | 2606:4700:3031::6815:1581            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 61   | ipv4.ip.sb                            | 172.67.75.172                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 63   | 104.18.37.13                          | 104.18.37.13                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 77   | www.gov.ua                            | 2606:4700:3031::6815:1748            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 85   | iplocation.io                         | 172.67.70.100                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 87   | iplocation.io                         | 2606:4700:20::681a:bde               | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 88   | iplocation.io                         | 2606:4700:20::ac43:4664              | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 91   | [2606:4700:8de6::5fa2:799e]           | 2606:4700:8de6::5fa2:799e            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 108  | www.okcupid.com                       | 104.16.239.254                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 111  | cf.877771.xyz                         | 104.21.80.180                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 179  | cloudflare-ip.mofashi.ltd             | 2606:4700:3037::ac43:9bac            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 216  | cmcc.877774.xyz                       | 104.16.149.5                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 220  | cmcc.877774.xyz                       | 104.16.149.9                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 229  | xn--b6gac.eu.org                      | 2a06:98c1:3121::3                    | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 267  | moura.ns.cloudflare.com               | 2803:f800:50::6ca2:c3d9              | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 284  | rustam.ns.cloudflare.com              | 2803:f800:50::6ca2:c394              | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 311  | whatismyipaddress.com                 | 104.19.223.79                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 324  | 104.18.14.76                          | 104.18.14.76                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 325  | 162.159.133.85                        | 162.159.133.85                       | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 339  | dnschecker.org                        | 172.67.73.216                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 348  | cf.877774.xyz                         | 2a06:98c1:3102::6812:29be            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 406  | ifconfig.co                           | 104.21.54.91                         | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 411  | www.wto.org                           | 104.18.41.190                        | IPv4   | h2   | ✅ 成功 | 66       | cloudflare |
| 413  | www.wto.org                           | 2a06:98c1:3102::6812:29be            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 414  | www.wto.org                           | 2606:4700:4406::ac40:9242            | IPv6   | h2   | ✅ 成功 | 66       | cloudflare |
| 3    | www.7749tv.com                        | 104.19.93.88                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 14   | www.ipget.net                         | 104.21.15.212                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 45   | cf.0sm.com                            | 172.67.187.145                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 54   | steamdb.info                          | 2606:4700:10::ac42:affa              | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 57   | 104.16.45.84                          | 104.16.45.84                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 65   | ct.877774.xyz                         | 172.64.229.195                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 74   | [2606:4700:9add::880:52fc]            | 2606:4700:9add::880:52fc             | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 94   | icook.hk                              | 2606:4700:3031::6815:5ad2            | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 106  | www.okcupid.com                       | 104.17.48.63                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 128  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90            | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 141  | cu.877774.xyz                         | 104.26.4.116                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 153  | zread.ai                              | 104.21.76.240                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 174  | 104.16.223.179                        | 104.16.223.179                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 187  | bestcf.030101.xyz                     | 104.17.212.134                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 197  | 172.64.151.55                         | 172.64.151.55                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 201  | cmcc.877774.xyz                       | 104.16.148.3                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 223  | cmcc.877774.xyz                       | 104.16.149.12                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 228  | xn--b6gac.eu.org                      | 172.67.153.253                       | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 230  | xn--b6gac.eu.org                      | 2a06:98c1:3120::3                    | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 232  | cf.zhetengsha.eu.org                  | 172.64.144.82                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 256  | www.ipchicken.com                     | 104.26.6.112                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 266  | moura.ns.cloudflare.com               | 2606:4700:58::a29f:2cd9              | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 273  | time.is                               | 2606:4700:20::ac43:449d              | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 275  | palera.in                             | 104.21.58.72                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 277  | palera.in                             | 2606:4700:3035::6815:3a48            | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 294  | ip.sb                                 | 172.67.75.172                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 303  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3031::ac43:9db6            | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 314  | whatismyipaddress.com                 | 2606:4700::6813:df4f                 | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 315  | silkbook.com                          | 104.26.9.160                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 317  | silkbook.com                          | 104.26.8.160                         | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 319  | silkbook.com                          | 2606:4700:20::681a:8a0               | IPv6   | h2   | ✅ 成功 | 67       | cloudflare |
| 323  | 172.67.106.26                         | 172.67.106.26                        | IPv4   | h2   | ✅ 成功 | 67       | cloudflare |
| 5    | na.877774.xyz                         | 104.18.187.25                        | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 18   | cfip.xxxxxxxx.tk                      | 104.16.232.223                       | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 24   | toy-people.com                        | 2606:4700:20::ac43:4812              | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 26   | toy-people.com                        | 2606:4700:20::681a:324               | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 47   | cf.0sm.com                            | 2606:4700:3032::6815:785             | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 48   | ipinfo.in                             | 104.21.21.129                        | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 71   | ct.877774.xyz                         | 172.64.229.174                       | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 75   | www.gov.ua                            | 104.21.23.72                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 82   | 172.67.49.134                         | 172.67.49.134                        | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 130  | www.visa.com.hk                       | 104.18.21.69                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 148  | cu.877774.xyz                         | 104.26.4.114                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 186  | bestcf.030101.xyz                     | 104.17.211.46                        | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 189  | bestcf.030101.xyz                     | 2606:4700:0:725f:f562:b399:6ca0:6490 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 193  | www.whatismyip.com                    | 2606:4700:20::681a:c17               | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 202  | cmcc.877774.xyz                       | 104.16.148.4                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 217  | cmcc.877774.xyz                       | 104.16.149.6                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 278  | palera.in                             | 2606:4700:3032::ac43:9d7a            | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 293  | ip.sb                                 | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 301  | ip.gs                                 | 2606:4700:3035::ac43:a01c            | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 320  | silkbook.com                          | 2606:4700:20::ac43:4bd0              | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 357  | gamer.com.tw                          | 104.18.2.197                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 367  | tasteatlas.com                        | 2606:4700::6811:2569                 | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 368  | www.udemy.com                         | 104.16.143.237                       | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 384  | icook.tw                              | 172.66.158.115                       | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 391  | [2606:4700:83bd::7d8:2b47]            | 2606:4700:83bd::7d8:2b47             | IPv6   | h2   | ✅ 成功 | 68       | cloudflare |
| 393  | eur.877774.xyz                        | 104.21.29.164                        | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 415  | stock.hostmonit.com                   | 104.21.7.193                         | IPv4   | h2   | ✅ 成功 | 68       | cloudflare |
| 8    | freeyx.cloudflare88.eu.org            | 141.101.120.247                      | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 31   | wilson.ns.cloudflare.com              | 2803:f800:50::6ca2:c36e              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 43   | www.pcmag.com                         | 2606:4700::6810:1476                 | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 69   | ct.877774.xyz                         | 172.64.229.161                       | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 73   | 172.67.110.232                        | 172.67.110.232                       | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 92   | icook.hk                              | 104.21.90.210                        | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 95   | icook.hk                              | 2606:4700:3037::ac43:a168            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 125  | sullivan.ns.cloudflare.com            | 2a06:98c1:50::ac40:23a1              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 127  | yx-auto.pages.dev                     | 172.66.47.112                        | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 129  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2f70            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 154  | zread.ai                              | 2606:4700:3033::6815:4cf0            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 155  | zread.ai                              | 2606:4700:3032::ac43:ca4e            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 177  | cloudflare-ip.mofashi.ltd             | 172.67.155.172                       | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 190  | www.whatismyip.com                    | 172.67.69.129                        | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 192  | www.whatismyip.com                    | 104.26.12.23                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 198  | [2606:4700:440f::53aa:4126]           | 2606:4700:440f::53aa:4126            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 205  | cmcc.877774.xyz                       | 104.16.148.7                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 212  | cmcc.877774.xyz                       | 104.16.149.1                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 213  | cmcc.877774.xyz                       | 104.16.149.2                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 233  | cf.zhetengsha.eu.org                  | 2a06:98c1:310d::6812:2bae            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 249  | braden.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a9              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 270  | time.is                               | 172.67.68.157                        | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 279  | 104.17.68.85                          | 104.17.68.85                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 344  | dnschecker.org                        | 2606:4700:20::ac43:49d8              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 366  | tasteatlas.com                        | 2606:4700::6811:2469                 | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 372  | [2606:4700:4408::18c5:3304]           | 2606:4700:4408::18c5:3304            | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 381  | 104.26.13.31                          | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 69       | cloudflare |
| 418  | stock.hostmonit.com                   | 2606:4700:3037::6815:7c1             | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 423  | otto.ns.cloudflare.com                | 2803:f800:50::6ca2:c387              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 424  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87              | IPv6   | h2   | ✅ 成功 | 69       | cloudflare |
| 15   | www.ipget.net                         | 172.67.207.26                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 37   | cris.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ca              | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 40   | www.pcmag.com                         | 104.16.21.118                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 52   | steamdb.info                          | 104.20.34.212                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 60   | ipv4.ip.sb                            | 104.26.13.31                         | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 101  | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc              | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 118  | www.hugedomains.com                   | 2606:4700:20::681a:625               | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 166  | kyree.ns.cloudflare.com               | 2a06:98c1:50::ac40:23cf              | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 221  | cmcc.877774.xyz                       | 104.16.149.10                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 222  | cmcc.877774.xyz                       | 104.16.149.11                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 227  | xn--b6gac.eu.org                      | 104.21.90.78                         | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 236  | fbi.gov                               | 104.16.149.244                       | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 238  | fbi.gov                               | 2606:4700::6810:94f4                 | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 254  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53              | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 271  | time.is                               | 2606:4700:20::681a:d36               | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |
| 305  | singapore.com                         | 104.26.13.140                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 306  | singapore.com                         | 104.26.12.140                        | IPv4   | h2   | ✅ 成功 | 70       | cloudflare |
| 370  | www.udemy.com                         | 2606:4700::6810:8eed                 | IPv6   | h2   | ✅ 成功 | 70       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 200 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 4 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 4 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
