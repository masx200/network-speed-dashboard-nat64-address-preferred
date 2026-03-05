# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/30 01:12:25
- **数据来源**: connectivity_results-20251229-171132.json
- **总测试数**: 484
- **失败测试数**: 56
- **成功测试数**: 428
- **失败率**: 11.57%
- **平均延迟**: 1393.54ms
- **最小延迟**: 451ms
- **最大延迟**: 7580ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/30 01:12:25
- **IP地址**: 104.28.208.133
- **国家/地区**: China (CN)
- **ASN**: AS13335
- **网络组织**: CLOUDFLARENET
- **网络域名**: cloudflare.com
- **大洲**: Asia (AS)
- **地理坐标**: 31.2222, 121.4581
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: 上下文超时**: 28 次 (50.0%)
- **连接超时: I/O超时**: 28 次 (50.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: 上下文超时 (28 次测试)

| 序号 | 主机/域名                               | 目标IP                                  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                              |
| ---- | --------------------------------------- | --------------------------------------- | ------- | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------------------------- |
| 23   | comicabc.com                            | 104.21.64.10                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 49   | trevor.ns.cloudflare.com                | 162.159.44.154                          | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 101  | eur.877774.xyz                          | Unknown                                 | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 104  | otto.ns.cloudflare.com                  | 108.162.195.135                         | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 108  | icook.tw                                | 104.20.28.74                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 128  | toy-people.com                          | 104.26.2.36                             | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 135  | www.pcmag.com                           | 2606:4700::6810:1576                    | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 161  | ct.877774.xyz                           | 172.64.229.185                          | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 173  | 104.26.4.90                             | 104.26.4.90                             | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 177  | 172.64.229.7                            | 172.64.229.7                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 197  | 104.26.6.112                            | 104.26.6.112                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 198  | 168.138.165.174                         | 168.138.165.174                         | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 225  | huxley.ns.cloudflare.com                | 162.159.44.188                          | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 284  | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | 2a06:98c1:3121:c6d4:af96:6677:59bf:faec | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 285  | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | 2a06:98c1:3121:5d:1caa:56dd:a908:af7b   | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 288  | freeyx.cloudflare88.eu.org              | 2606:4700:3009:aa59:4b67:cdf7:b91:1a47  | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 290  | cris.ns.cloudflare.com                  | 172.64.35.202                           | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 298  | cmcc.877774.xyz                         | 104.16.149.6                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 321  | cmcc.877774.xyz                         | 104.16.149.3                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 359  | bestcf.030101.xyz                       | 104.19.47.227                           | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 366  | kyree.ns.cloudflare.com                 | 2606:4700:58::a29f:2ccf                 | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 367  | kyree.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3cf                 | IPv6    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 406  | bowen.ns.cloudflare.com                 | 108.162.195.83                          | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 446  | rustam.ns.cloudflare.com                | 172.64.35.148                           | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 447  | rustam.ns.cloudflare.com                | 108.162.195.148                         | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 453  | 104.18.42.26                            | 104.18.42.26                            | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 474  | ashton.ns.cloudflare.com                | 162.159.44.173                          | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |
| 479  | julio.ns.cloudflare.com                 | 108.162.195.209                         | IPv4    | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.net:443/": context deadline exceeded                   |

#### 连接超时: I/O超时 (28 次测试)

| 序号 | 主机/域名                   | 目标IP                    | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                              |
| ---- | --------------------------- | ------------------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------------------- |
| 30   | www.wto.org                 | 2606:4700:4406::ac40:9242 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:4406::ac40:9242]:443: i/o timeout |
| 50   | trevor.ns.cloudflare.com    | 172.64.35.154             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.154:443: i/o timeout               |
| 59   | 198.41.194.162              | 198.41.194.162            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.194.162:443: i/o timeout              |
| 78   | 162.159.133.85              | 162.159.133.85            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.133.85:443: i/o timeout              |
| 84   | 172.67.79.211               | 172.67.79.211             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.79.211:443: i/o timeout               |
| 89   | 172.64.201.25               | 172.64.201.25             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout               |
| 102  | otto.ns.cloudflare.com      | 172.64.35.135             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.135:443: i/o timeout               |
| 113  | damien.ns.cloudflare.com    | 108.162.195.168           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.168:443: i/o timeout             |
| 121  | cfip.xxxxxxxx.tk            | 198.41.212.130            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout              |
| 129  | toy-people.com              | 2606:4700:20::681a:324    | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:20::681a:324]:443: i/o timeout    |
| 143  | wilson.ns.cloudflare.com    | 2a06:98c1:50::ac40:236e   | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2a06:98c1:50::ac40:236e]:443: i/o timeout   |
| 184  | www.gov.ua                  | 104.21.23.72              | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.21.23.72:443: i/o timeout                |
| 200  | iplocation.io               | 104.26.11.222             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.11.222:443: i/o timeout               |
| 209  | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:8de6::5fa2:799e]:443: i/o timeout |
| 211  | cu.877774.xyz               | 172.64.145.202            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.145.202:443: i/o timeout              |
| 218  | icook.hk                    | 2606:4700:3031::6815:5ad2 | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700:3031::6815:5ad2]:443: i/o timeout |
| 238  | www.hugedomains.com         | 172.67.70.191             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.70.191:443: i/o timeout               |
| 245  | cf.877771.xyz               | 104.21.80.180             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.21.80.180:443: i/o timeout               |
| 246  | cf.877771.xyz               | 172.67.152.183            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.152.183:443: i/o timeout              |
| 269  | craig.ns.cloudflare.com     | 172.64.35.192             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.35.192:443: i/o timeout               |
| 291  | cris.ns.cloudflare.com      | 108.162.195.202           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.202:443: i/o timeout             |
| 311  | cmcc.877774.xyz             | 104.16.148.6              | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.16.148.6:443: i/o timeout                |
| 326  | 104.17.79.11                | 104.17.79.11              | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.17.79.11:443: i/o timeout                |
| 350  | www.whatismyip.com          | 172.67.69.129             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.69.129:443: i/o timeout               |
| 376  | saas.sin.fan                | 162.159.36.20             | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.36.20:443: i/o timeout               |
| 448  | rustam.ns.cloudflare.com    | 162.159.44.148            | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.148:443: i/o timeout              |
| 461  | whatismyipaddress.com       | 2606:4700::6813:df4f      | IPv6   | none | N/A    | 0        | N/A    | dial tcp [2606:4700::6813:df4f]:443: i/o timeout      |
| 473  | ashton.ns.cloudflare.com    | 108.162.195.173           | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.173:443: i/o timeout             |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 56 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 28 次超时，主要集中在IP段 172.64（5
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 29 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**: IPv4失败 43 次，IPv6失败 12 次，两种协议都存在问题

**问题主机分析**: 以下主机出现多次失败：cmcc.877774.xyz (3次),
rustam.ns.cloudflare.com (3次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 200 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                               | 目标IP                                  | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------------------- | --------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 42   | japan.com                               | 172.67.70.92                            | IPv4   | h2   | ✅ 成功 | 451      | cloudflare |
| 191  | shopify.com                             | 23.227.38.33                            | IPv4   | h2   | ✅ 成功 | 457      | cloudflare |
| 282  | pranab.ns.cloudflare.com                | 2803:f800:50::6ca2:c3c7                 | IPv6   | h2   | ✅ 成功 | 475      | cloudflare |
| 375  | saas.sin.fan                            | 162.159.36.5                            | IPv4   | h2   | ✅ 成功 | 478      | cloudflare |
| 413  | palera.in                               | 2606:4700:3035::6815:3a48               | IPv6   | h2   | ✅ 成功 | 479      | cloudflare |
| 323  | time.is                                 | 172.67.68.157                           | IPv4   | h2   | ✅ 成功 | 485      | cloudflare |
| 260  | na.877774.xyz                           | 104.19.74.233                           | IPv4   | h2   | ✅ 成功 | 491      | cloudflare |
| 309  | cmcc.877774.xyz                         | 104.16.148.4                            | IPv4   | h2   | ✅ 成功 | 492      | cloudflare |
| 210  | cu.877774.xyz                           | 104.18.42.54                            | IPv4   | h2   | ✅ 成功 | 497      | cloudflare |
| 181  | 104.18.89.52                            | 104.18.89.52                            | IPv4   | h2   | ✅ 成功 | 506      | cloudflare |
| 180  | 104.18.81.19                            | 104.18.81.19                            | IPv4   | h2   | ✅ 成功 | 516      | cloudflare |
| 262  | na.877774.xyz                           | 104.18.187.25                           | IPv4   | h2   | ✅ 成功 | 517      | cloudflare |
| 37   | lewis.ns.cloudflare.com                 | 2606:4700:58::a29f:2c9f                 | IPv6   | h2   | ✅ 成功 | 518      | cloudflare |
| 116  | damien.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a8                 | IPv6   | h2   | ✅ 成功 | 523      | cloudflare |
| 76   | cloudflare.182682.xyz                   | 2606:4700:3035::1a4f:5642               | IPv6   | h2   | ✅ 成功 | 524      | cloudflare |
| 466  | dnschecker.org                          | 2606:4700:20::681a:659                  | IPv6   | h2   | ✅ 成功 | 538      | cloudflare |
| 80   | 198.62.62.4                             | 198.62.62.4                             | IPv4   | h2   | ✅ 成功 | 542      | cloudflare |
| 85   | www.digitalocean.com                    | 104.19.174.68                           | IPv4   | h2   | ✅ 成功 | 542      | cloudflare |
| 418  | ip.sb                                   | 2606:4700:20::ac43:4bac                 | IPv6   | h2   | ✅ 成功 | 547      | cloudflare |
| 430  | ip.gs                                   | 104.21.14.176                           | IPv4   | h2   | ✅ 成功 | 556      | cloudflare |
| 44   | japan.com                               | 2606:4700:20::681a:43c                  | IPv6   | h2   | ✅ 成功 | 558      | cloudflare |
| 273  | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | 2a06:98c1:3121:0:ef18:6ab0:b648:d756    | IPv6   | h2   | ✅ 成功 | 563      | cloudflare |
| 166  | 162.159.136.89                          | 162.159.136.89                          | IPv4   | h2   | ✅ 成功 | 570      | cloudflare |
| 402  | moura.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d9                 | IPv6   | h2   | ✅ 成功 | 574      | cloudflare |
| 83   | cfip.1323123.xyz                        | 104.16.133.220                          | IPv4   | h2   | ✅ 成功 | 579      | cloudflare |
| 2    | [2606:4700:83be::11:74f]                | 2606:4700:83be::11:74f                  | IPv6   | h2   | ✅ 成功 | 580      | cloudflare |
| 52   | trevor.ns.cloudflare.com                | 2803:f800:50::6ca2:c39a                 | IPv6   | h2   | ✅ 成功 | 580      | cloudflare |
| 1    | 172.64.154.18                           | 172.64.154.18                           | IPv4   | h2   | ✅ 成功 | 581      | cloudflare |
| 240  | www.hugedomains.com                     | 2606:4700:20::ac43:46bf                 | IPv6   | h2   | ✅ 成功 | 582      | cloudflare |
| 383  | cf.090227.xyz                           | 104.18.35.15                            | IPv4   | h2   | ✅ 成功 | 582      | cloudflare |
| 3    | 162.159.61.183                          | 162.159.61.183                          | IPv4   | h2   | ✅ 成功 | 585      | cloudflare |
| 87   | www.digitalocean.com                    | 2606:4700::6813:ad44                    | IPv6   | h2   | ✅ 成功 | 586      | cloudflare |
| 250  | 104.19.220.22                           | 104.19.220.22                           | IPv4   | h2   | ✅ 成功 | 586      | cloudflare |
| 4    | [2606:4700:964f::6e2c:588e]             | 2606:4700:964f::6e2c:588e               | IPv6   | h2   | ✅ 成功 | 588      | cloudflare |
| 69   | cloudflare.182682.xyz                   | 104.16.250.22                           | IPv4   | h2   | ✅ 成功 | 591      | cloudflare |
| 214  | www.visa.cn                             | 162.159.152.2                           | IPv4   | h2   | ✅ 成功 | 591      | cloudflare |
| 251  | 104.19.154.200                          | 104.19.154.200                          | IPv4   | h2   | ✅ 成功 | 592      | cloudflare |
| 313  | cmcc.877774.xyz                         | 104.16.148.8                            | IPv4   | h2   | ✅ 成功 | 592      | cloudflare |
| 295  | cris.ns.cloudflare.com                  | 2606:4700:58::a29f:2cca                 | IPv6   | h2   | ✅ 成功 | 595      | cloudflare |
| 146  | 173.245.49.194                          | 173.245.49.194                          | IPv4   | h2   | ✅ 成功 | 611      | cloudflare |
| 465  | dnschecker.org                          | 172.67.73.216                           | IPv4   | h2   | ✅ 成功 | 641      | cloudflare |
| 90   | www.udemy.com                           | 104.16.143.237                          | IPv4   | h2   | ✅ 成功 | 644      | cloudflare |
| 13   | www.ipget.net                           | 172.67.207.26                           | IPv4   | h2   | ✅ 成功 | 656      | cloudflare |
| 436  | singapore.com                           | 104.26.12.140                           | IPv4   | h2   | ✅ 成功 | 666      | cloudflare |
| 263  | 104.18.223.253                          | 104.18.223.253                          | IPv4   | h2   | ✅ 成功 | 668      | cloudflare |
| 266  | 104.17.142.212                          | 104.17.142.212                          | IPv4   | h2   | ✅ 成功 | 671      | cloudflare |
| 189  | 104.18.151.172                          | 104.18.151.172                          | IPv4   | h2   | ✅ 成功 | 672      | cloudflare |
| 190  | 104.17.162.3                            | 104.17.162.3                            | IPv4   | h2   | ✅ 成功 | 672      | cloudflare |
| 15   | www.ipget.net                           | 2606:4700:3031::ac43:cf1a               | IPv6   | h2   | ✅ 成功 | 673      | cloudflare |
| 322  | time.is                                 | 104.26.12.54                            | IPv4   | h2   | ✅ 成功 | 673      | cloudflare |
| 233  | sullivan.ns.cloudflare.com              | 2606:4700:58::a29f:2ca1                 | IPv6   | h2   | ✅ 成功 | 680      | cloudflare |
| 334  | zread.ai                                | 188.114.97.3                            | IPv4   | h2   | ✅ 成功 | 681      | cloudflare |
| 199  | iplocation.io                           | 104.26.10.222                           | IPv4   | h2   | ✅ 成功 | 684      | cloudflare |
| 179  | 104.26.3.162                            | 104.26.3.162                            | IPv4   | h2   | ✅ 成功 | 688      | cloudflare |
| 449  | rustam.ns.cloudflare.com                | 2606:4700:58::a29f:2c94                 | IPv6   | h2   | ✅ 成功 | 688      | cloudflare |
| 54   | trevor.ns.cloudflare.com                | 2606:4700:58::a29f:2c9a                 | IPv6   | h2   | ✅ 成功 | 695      | cloudflare |
| 67   | cloudflare.182682.xyz                   | 104.21.224.5                            | IPv4   | h2   | ✅ 成功 | 695      | cloudflare |
| 175  | 162.159.140.116                         | 162.159.140.116                         | IPv4   | h2   | ✅ 成功 | 696      | cloudflare |
| 306  | cmcc.877774.xyz                         | 104.16.148.1                            | IPv4   | h2   | ✅ 成功 | 697      | cloudflare |
| 35   | lewis.ns.cloudflare.com                 | 2803:f800:50::6ca2:c39f                 | IPv6   | h2   | ✅ 成功 | 698      | cloudflare |
| 420  | ip.sb                                   | 2606:4700:20::681a:c1f                  | IPv6   | h2   | ✅ 成功 | 698      | cloudflare |
| 186  | www.gov.ua                              | 2606:4700:3031::6815:1748               | IPv6   | h2   | ✅ 成功 | 699      | cloudflare |
| 443  | silkbook.com                            | 2606:4700:20::ac43:4bd0                 | IPv6   | h2   | ✅ 成功 | 699      | cloudflare |
| 435  | singapore.com                           | 104.26.13.140                           | IPv4   | h2   | ✅ 成功 | 700      | cloudflare |
| 156  | ct.877774.xyz                           | 172.64.229.161                          | IPv4   | h2   | ✅ 成功 | 702      | cloudflare |
| 426  | benedict.ns.cloudflare.com              | 2606:4700:58::a29f:2ccd                 | IPv6   | h2   | ✅ 成功 | 702      | cloudflare |
| 439  | singapore.com                           | 2606:4700:20::681a:d8c                  | IPv6   | h2   | ✅ 成功 | 702      | cloudflare |
| 318  | cmcc.877774.xyz                         | 104.16.148.244                          | IPv4   | h2   | ✅ 成功 | 706      | cloudflare |
| 227  | huxley.ns.cloudflare.com                | 2803:f800:50::6ca2:c3bc                 | IPv6   | h2   | ✅ 成功 | 708      | cloudflare |
| 256  | 104.17.69.244                           | 104.17.69.244                           | IPv4   | h2   | ✅ 成功 | 709      | cloudflare |
| 244  | www.visa.com.sg                         | 104.18.13.229                           | IPv4   | h2   | ✅ 成功 | 710      | cloudflare |
| 325  | 104.16.223.179                          | 104.16.223.179                          | IPv4   | h2   | ✅ 成功 | 710      | cloudflare |
| 31   | www.wto.org                             | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 712      | cloudflare |
| 477  | ashton.ns.cloudflare.com                | 2606:4700:58::a29f:2cad                 | IPv6   | h2   | ✅ 成功 | 712      | cloudflare |
| 160  | ct.877774.xyz                           | 172.64.229.44                           | IPv4   | h2   | ✅ 成功 | 713      | cloudflare |
| 410  | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | 2a06:98c1:3120:c39b:7522:c680:d288:d13c | IPv6   | h2   | ✅ 成功 | 714      | cloudflare |
| 417  | ip.sb                                   | 104.26.12.31                            | IPv4   | h2   | ✅ 成功 | 715      | cloudflare |
| 297  | cmcc.877774.xyz                         | 104.16.149.5                            | IPv4   | h2   | ✅ 成功 | 718      | cloudflare |
| 82   | gamer.com.tw                            | 104.18.3.197                            | IPv4   | h2   | ✅ 成功 | 719      | cloudflare |
| 215  | icook.hk                                | 104.21.90.210                           | IPv4   | h2   | ✅ 成功 | 719      | cloudflare |
| 303  | cmcc.877774.xyz                         | 104.16.149.11                           | IPv4   | h2   | ✅ 成功 | 719      | cloudflare |
| 92   | www.udemy.com                           | 2606:4700::6810:8eed                    | IPv6   | h2   | ✅ 成功 | 721      | cloudflare |
| 384  | cf.090227.xyz                           | 2a06:98c1:310d::6812:2bae               | IPv6   | h2   | ✅ 成功 | 722      | cloudflare |
| 159  | ct.877774.xyz                           | 172.64.229.173                          | IPv4   | h2   | ✅ 成功 | 724      | cloudflare |
| 176  | 104.26.8.117                            | 104.26.8.117                            | IPv4   | h2   | ✅ 成功 | 725      | cloudflare |
| 428  | benedict.ns.cloudflare.com              | 2803:f800:50::6ca2:c3cd                 | IPv6   | h2   | ✅ 成功 | 726      | cloudflare |
| 6    | 104.18.78.214                           | 104.18.78.214                           | IPv4   | h2   | ✅ 成功 | 727      | cloudflare |
| 7    | [2606:4700:83bd::7d8:2b47]              | 2606:4700:83bd::7d8:2b47                | IPv6   | h2   | ✅ 成功 | 727      | cloudflare |
| 17   | 172.67.181.209                          | 172.67.181.209                          | IPv4   | h2   | ✅ 成功 | 727      | cloudflare |
| 242  | www.hugedomains.com                     | 2606:4700:20::681a:725                  | IPv6   | h2   | ✅ 成功 | 728      | cloudflare |
| 433  | ip.gs                                   | 2606:4700:3036::6815:eb0                | IPv6   | h2   | ✅ 成功 | 728      | cloudflare |
| 445  | silkbook.com                            | 2606:4700:20::681a:9a0                  | IPv6   | h2   | ✅ 成功 | 729      | cloudflare |
| 315  | cmcc.877774.xyz                         | 104.16.148.10                           | IPv4   | h2   | ✅ 成功 | 730      | cloudflare |
| 172  | 104.26.5.134                            | 104.26.5.134                            | IPv4   | h2   | ✅ 成功 | 731      | cloudflare |
| 248  | cf.877771.xyz                           | 2606:4700:3033::ac43:98b7               | IPv6   | h2   | ✅ 成功 | 731      | cloudflare |
| 73   | cloudflare.182682.xyz                   | 2a06:98c1:3120::5692:61a4               | IPv6   | h2   | ✅ 成功 | 732      | cloudflare |
| 119  | 172.64.91.69                            | 172.64.91.69                            | IPv4   | h2   | ✅ 成功 | 732      | cloudflare |
| 305  | cmcc.877774.xyz                         | 104.16.149.244                          | IPv4   | h2   | ✅ 成功 | 732      | cloudflare |
| 139  | cf.0sm.com                              | 2606:4700:3037::ac43:bb91               | IPv6   | h2   | ✅ 成功 | 733      | cloudflare |
| 294  | cris.ns.cloudflare.com                  | 2a06:98c1:50::ac40:23ca                 | IPv6   | h2   | ✅ 成功 | 733      | cloudflare |
| 79   | 104.17.142.12                           | 104.17.142.12                           | IPv4   | h2   | ✅ 成功 | 735      | cloudflare |
| 300  | cmcc.877774.xyz                         | 104.16.149.8                            | IPv4   | h2   | ✅ 成功 | 735      | cloudflare |
| 400  | moura.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23d9                 | IPv6   | h2   | ✅ 成功 | 735      | cloudflare |
| 333  | zread.ai                                | 188.114.96.3                            | IPv4   | h2   | ✅ 成功 | 736      | cloudflare |
| 276  | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | 2a06:98c1:3121:0:efde:82d1:8124:3fed    | IPv6   | h2   | ✅ 成功 | 737      | cloudflare |
| 369  | xn--b6gac.eu.org                        | 172.67.153.253                          | IPv4   | h2   | ✅ 成功 | 737      | cloudflare |
| 393  | www.glassdoor.com                       | 104.16.25.46                            | IPv4   | h2   | ✅ 成功 | 737      | cloudflare |
| 70   | cloudflare.182682.xyz                   | 104.17.25.173                           | IPv4   | h2   | ✅ 成功 | 738      | cloudflare |
| 293  | cris.ns.cloudflare.com                  | 2803:f800:50::6ca2:c3ca                 | IPv6   | h2   | ✅ 成功 | 738      | cloudflare |
| 328  | asia.877774.xyz                         | 104.17.139.62                           | IPv4   | h2   | ✅ 成功 | 738      | cloudflare |
| 416  | ip.sb                                   | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 738      | cloudflare |
| 8    | 104.26.13.31                            | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 739      | cloudflare |
| 165  | steamdb.info                            | 2606:4700:10::6814:22d4                 | IPv6   | h2   | ✅ 成功 | 739      | cloudflare |
| 168  | ipv4.ip.sb                              | 104.26.13.31                            | IPv4   | h2   | ✅ 成功 | 739      | cloudflare |
| 371  | xn--b6gac.eu.org                        | 2606:4700:3037::ac43:99fd               | IPv6   | h2   | ✅ 成功 | 739      | cloudflare |
| 134  | www.pcmag.com                           | 2606:4700::6810:1476                    | IPv6   | h2   | ✅ 成功 | 740      | cloudflare |
| 264  | 104.16.105.166                          | 104.16.105.166                          | IPv4   | h2   | ✅ 成功 | 741      | cloudflare |
| 91   | www.udemy.com                           | 104.16.142.237                          | IPv4   | h2   | ✅ 成功 | 742      | cloudflare |
| 277  | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | 2a06:98c1:3121:c677:c614:1f96:d4bf:a723 | IPv6   | h2   | ✅ 成功 | 742      | cloudflare |
| 271  | craig.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3c0                 | IPv6   | h2   | ✅ 成功 | 744      | cloudflare |
| 283  | pranab.ns.cloudflare.com                | 2a06:98c1:50::ac40:23c7                 | IPv6   | h2   | ✅ 成功 | 745      | cloudflare |
| 330  | decker.ns.cloudflare.com                | 2606:4700:58::a29f:2c9b                 | IPv6   | h2   | ✅ 成功 | 749      | cloudflare |
| 106  | otto.ns.cloudflare.com                  | 2803:f800:50::6ca2:c387                 | IPv6   | h2   | ✅ 成功 | 750      | cloudflare |
| 274  | 104.18.189.153                          | 104.18.189.153                          | IPv4   | h2   | ✅ 成功 | 753      | cloudflare |
| 48   | 162.159.140.85                          | 162.159.140.85                          | IPv4   | h2   | ✅ 成功 | 757      | cloudflare |
| 46   | japan.com                               | 2606:4700:20::681a:53c                  | IPv6   | h2   | ✅ 成功 | 760      | cloudflare |
| 205  | 104.18.254.88                           | 104.18.254.88                           | IPv4   | h2   | ✅ 成功 | 762      | cloudflare |
| 167  | 104.16.45.84                            | 104.16.45.84                            | IPv4   | h2   | ✅ 成功 | 764      | cloudflare |
| 348  | www.whatismyip.com                      | 104.26.13.23                            | IPv4   | h2   | ✅ 成功 | 764      | cloudflare |
| 157  | ct.877774.xyz                           | 172.64.229.174                          | IPv4   | h2   | ✅ 成功 | 768      | cloudflare |
| 38   | www.7749tv.com                          | 104.19.133.4                            | IPv4   | h2   | ✅ 成功 | 769      | cloudflare |
| 346  | dylan.ns.cloudflare.com                 | 2a06:98c1:50::ac40:23bb                 | IPv6   | h2   | ✅ 成功 | 769      | cloudflare |
| 358  | 172.64.151.55                           | 172.64.151.55                           | IPv4   | h2   | ✅ 成功 | 769      | cloudflare |
| 469  | cf.877774.xyz                           | 104.18.41.190                           | IPv4   | h2   | ✅ 成功 | 776      | cloudflare |
| 254  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2c90               | IPv6   | h2   | ✅ 成功 | 777      | cloudflare |
| 222  | www.okcupid.com                         | 104.18.160.63                           | IPv4   | h2   | ✅ 成功 | 780      | cloudflare |
| 396  | www.ipchicken.com                       | 104.26.6.112                            | IPv4   | h2   | ✅ 成功 | 780      | cloudflare |
| 24   | comicabc.com                            | 172.67.174.21                           | IPv4   | h2   | ✅ 成功 | 784      | cloudflare |
| 96   | tasteatlas.com                          | 104.17.36.105                           | IPv4   | h2   | ✅ 成功 | 788      | cloudflare |
| 220  | www.okcupid.com                         | 104.16.239.254                          | IPv4   | h2   | ✅ 成功 | 794      | cloudflare |
| 337  | cloudflare-ip.mofashi.ltd               | 104.21.72.233                           | IPv4   | h2   | ✅ 成功 | 794      | cloudflare |
| 123  | cfip.xxxxxxxx.tk                        | 104.16.241.229                          | IPv4   | h2   | ✅ 成功 | 797      | cloudflare |
| 194  | 172.67.110.232                          | 172.67.110.232                          | IPv4   | h2   | ✅ 成功 | 798      | cloudflare |
| 252  | yx-auto.pages.dev                       | 172.66.47.112                           | IPv4   | h2   | ✅ 成功 | 798      | cloudflare |
| 440  | silkbook.com                            | 104.26.8.160                            | IPv4   | h2   | ✅ 成功 | 799      | cloudflare |
| 472  | cf.877774.xyz                           | 2a06:98c1:3102::6812:29be               | IPv6   | h2   | ✅ 成功 | 800      | cloudflare |
| 158  | ct.877774.xyz                           | 172.64.229.217                          | IPv4   | h2   | ✅ 成功 | 801      | cloudflare |
| 195  | [2606:4700:9add::880:52fc]              | 2606:4700:9add::880:52fc                | IPv6   | h2   | ✅ 成功 | 801      | cloudflare |
| 193  | 104.18.37.13                            | 104.18.37.13                            | IPv4   | h2   | ✅ 成功 | 802      | cloudflare |
| 203  | iplocation.io                           | 2606:4700:20::681a:bde                  | IPv6   | h2   | ✅ 成功 | 803      | cloudflare |
| 352  | www.whatismyip.com                      | 2606:4700:20::681a:c17                  | IPv6   | h2   | ✅ 成功 | 822      | cloudflare |
| 71   | cloudflare.182682.xyz                   | 104.18.185.26                           | IPv4   | h2   | ✅ 成功 | 825      | cloudflare |
| 329  | asia.877774.xyz                         | 104.16.211.153                          | IPv4   | h2   | ✅ 成功 | 825      | cloudflare |
| 65   | abdullah.ns.cloudflare.com              | 2606:4700:58::a29f:2ccb                 | IPv6   | h2   | ✅ 成功 | 826      | cloudflare |
| 427  | benedict.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cd                 | IPv6   | h2   | ✅ 成功 | 829      | cloudflare |
| 452  | local-aria2-webui.masx200.ddns-ip.net   | 2606:4700:3031::ac43:9db6               | IPv6   | h2   | ✅ 成功 | 829      | cloudflare |
| 394  | www.ipchicken.com                       | 172.67.68.101                           | IPv4   | h2   | ✅ 成功 | 831      | cloudflare |
| 204  | iplocation.io                           | 2606:4700:20::ac43:4664                 | IPv6   | h2   | ✅ 成功 | 837      | cloudflare |
| 75   | cloudflare.182682.xyz                   | 2606:4700:3032::818:669e                | IPv6   | h2   | ✅ 成功 | 845      | cloudflare |
| 281  | pranab.ns.cloudflare.com                | 2606:4700:58::a29f:2cc7                 | IPv6   | h2   | ✅ 成功 | 880      | cloudflare |
| 380  | fbi.gov                                 | 2606:4700::6810:95f4                    | IPv6   | h2   | ✅ 成功 | 888      | cloudflare |
| 401  | moura.ns.cloudflare.com                 | 2606:4700:58::a29f:2cd9                 | IPv6   | h2   | ✅ 成功 | 888      | cloudflare |
| 442  | silkbook.com                            | 172.67.75.208                           | IPv4   | h2   | ✅ 成功 | 890      | cloudflare |
| 419  | ip.sb                                   | 2606:4700:20::681a:d1f                  | IPv6   | h2   | ✅ 成功 | 892      | cloudflare |
| 389  | braden.ns.cloudflare.com                | 2803:f800:50::6ca2:c3a9                 | IPv6   | h2   | ✅ 成功 | 893      | cloudflare |
| 118  | 104.17.139.37                           | 104.17.139.37                           | IPv4   | h2   | ✅ 成功 | 905      | cloudflare |
| 476  | ashton.ns.cloudflare.com                | 2a06:98c1:50::ac40:23ad                 | IPv6   | h2   | ✅ 成功 | 907      | cloudflare |
| 347  | dylan.ns.cloudflare.com                 | 2606:4700:58::a29f:2cbb                 | IPv6   | h2   | ✅ 成功 | 908      | cloudflare |
| 153  | 104.18.39.196                           | 104.18.39.196                           | IPv4   | h2   | ✅ 成功 | 910      | cloudflare |
| 36   | lewis.ns.cloudflare.com                 | 2a06:98c1:50::ac40:239f                 | IPv6   | h2   | ✅ 成功 | 913      | cloudflare |
| 289  | freeyx.cloudflare88.eu.org              | 2606:4700:3010:3392:350c:968b:1f41:32b0 | IPv6   | h2   | ✅ 成功 | 924      | cloudflare |
| 332  | decker.ns.cloudflare.com                | 2a06:98c1:50::ac40:239b                 | IPv6   | h2   | ✅ 成功 | 925      | cloudflare |
| 234  | sullivan.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a1                 | IPv6   | h2   | ✅ 成功 | 926      | cloudflare |
| 53   | trevor.ns.cloudflare.com                | 2a06:98c1:50::ac40:239a                 | IPv6   | h2   | ✅ 成功 | 928      | cloudflare |
| 110  | icook.tw                                | 2606:4700:10::6814:1c4a                 | IPv6   | h2   | ✅ 成功 | 932      | cloudflare |
| 56   | 172.64.82.114                           | 172.64.82.114                           | IPv4   | h2   | ✅ 成功 | 940      | cloudflare |
| 310  | cmcc.877774.xyz                         | 104.16.148.5                            | IPv4   | h2   | ✅ 成功 | 940      | cloudflare |
| 196  | 103.160.204.59                          | 103.160.204.59                          | IPv4   | h2   | ✅ 成功 | 942      | cloudflare |
| 206  | 172.67.49.134                           | 172.67.49.134                           | IPv4   | h2   | ✅ 成功 | 942      | cloudflare |
| 414  | palera.in                               | 2606:4700:3032::ac43:9d7a               | IPv6   | h2   | ✅ 成功 | 945      | cloudflare |
| 98   | tasteatlas.com                          | 2606:4700::6811:2469                    | IPv6   | h2   | ✅ 成功 | 946      | cloudflare |
| 257  | 104.31.16.158                           | 104.31.16.158                           | IPv4   | h2   | ✅ 成功 | 948      | cloudflare |
| 467  | dnschecker.org                          | 2606:4700:20::681a:759                  | IPv6   | h2   | ✅ 成功 | 951      | cloudflare |
| 391  | braden.ns.cloudflare.com                | 2a06:98c1:50::ac40:23a9                 | IPv6   | h2   | ✅ 成功 | 952      | cloudflare |
| 438  | singapore.com                           | 2606:4700:20::681a:c8c                  | IPv6   | h2   | ✅ 成功 | 954      | cloudflare |
| 483  | julio.ns.cloudflare.com                 | 2803:f800:50::6ca2:c3d1                 | IPv6   | h2   | ✅ 成功 | 955      | cloudflare |
| 255  | yx-auto.pages.dev                       | 2606:4700:310c::ac42:2f70               | IPv6   | h2   | ✅ 成功 | 958      | cloudflare |
| 60   | [2606:4700:440b::3e6e:5f06]             | 2606:4700:440b::3e6e:5f06               | IPv6   | h2   | ✅ 成功 | 962      | cloudflare |
| 66   | abdullah.ns.cloudflare.com              | 2a06:98c1:50::ac40:23cb                 | IPv6   | h2   | ✅ 成功 | 962      | cloudflare |
| 20   | stock.hostmonit.com                     | 2606:4700:3037::6815:7c1                | IPv6   | h2   | ✅ 成功 | 963      | cloudflare |
| 470  | cf.877774.xyz                           | 172.64.146.66                           | IPv4   | h2   | ✅ 成功 | 966      | cloudflare |
| 26   | comicabc.com                            | 2606:4700:3030::ac43:ae15               | IPv6   | h2   | ✅ 成功 | 970      | cloudflare |
| 192  | 172.67.75.172                           | 172.67.75.172                           | IPv4   | h2   | ✅ 成功 | 970      | cloudflare |
| 434  | singapore.com                           | 172.67.75.194                           | IPv4   | h2   | ✅ 成功 | 975      | cloudflare |
| 361  | bestcf.030101.xyz                       | 2606:4700::f61:72ba:362:704d            | IPv6   | h2   | ✅ 成功 | 977      | cloudflare |
| 163  | steamdb.info                            | 172.66.175.250                          | IPv4   | h2   | ✅ 成功 | 979      | cloudflare |
| 390  | braden.ns.cloudflare.com                | 2606:4700:58::a29f:2ca9                 | IPv6   | h2   | ✅ 成功 | 979      | cloudflare |
| 421  | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | 2a06:98c1:3121:cdc1:6b1b:cfe6:f0:eaa2   | IPv6   | h2   | ✅ 成功 | 980      | cloudflare |
| 422  | 104.17.68.85                            | 104.17.68.85                            | IPv4   | h2   | ✅ 成功 | 980      | cloudflare |
| 5    | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | 2a06:98c1:3120:c39b:f77:4fc1:b18b:c12   | IPv6   | h2   | ✅ 成功 | 981      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 9 条记录
- **很慢 (>500ms)**: 191 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 43 次
- **IPv6 失败**: 12 次

### 按协议统计

- **h2**: 27 次失败
- **none**: 29 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
