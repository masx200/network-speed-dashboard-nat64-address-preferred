# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 04:04:21
- **数据来源**: connectivity_results-20251212-040421.json
- **总测试数**: 470
- **失败测试数**: 2
- **成功测试数**: 468
- **失败率**: 0.43%
- **平均延迟**: 59.70ms
- **最小延迟**: 40ms
- **最大延迟**: 681ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 2 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 107  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 404  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

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

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                             | 目标IP                    | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | ------------------------------------- | ------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 74   | iplocation.io                         | 2606:4700:20::681a:bde    | IPv6   | h2   | ✅ 成功 | 40       | cloudflare |
| 127  | sullivan.ns.cloudflare.com            | 2803:f800:50::6ca2:c3a1   | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 322  | ae8a9c24-83de.masx200.ddns-ip.net     | 2606:4700:3030::6815:e29  | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 334  | whatismyipaddress.com                 | 104.19.223.79             | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 347  | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 43       | cloudflare |
| 415  | www.digitalocean.com                  | 2606:4700::6813:ae44      | IPv6   | h2   | ✅ 成功 | 43       | cloudflare |
| 53   | 172.67.75.172                         | 172.67.75.172             | IPv4   | h2   | ✅ 成功 | 44       | cloudflare |
| 176  | decker.ns.cloudflare.com              | 2606:4700:58::a29f:2c9b   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 332  | singapore.com                         | 2606:4700:20::ac43:4bc2   | IPv6   | h2   | ✅ 成功 | 44       | cloudflare |
| 61   | www.gov.ua                            | 2606:4700:3031::6815:1748 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 115  | www.hugedomains.com                   | 2606:4700:20::681a:725    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 262  | www.ipchicken.com                     | 104.26.7.112              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 278  | icook.tw                              | 104.20.28.74              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 287  | moura.ns.cloudflare.com               | 2a06:98c1:50::ac40:23d9   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 291  | time.is                               | 2606:4700:20::ac43:449d   | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 311  | palera.in                             | 2606:4700:3035::6815:3a48 | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 330  | singapore.com                         | 2606:4700:20::681a:d8c    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 331  | singapore.com                         | 2606:4700:20::681a:c8c    | IPv6   | h2   | ✅ 成功 | 45       | cloudflare |
| 352  | 104.18.14.76                          | 104.18.14.76              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 411  | 104.26.13.31                          | 104.26.13.31              | IPv4   | h2   | ✅ 成功 | 45       | cloudflare |
| 78   | www.4chan.org                         | 104.16.229.229            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 89   | huxley.ns.cloudflare.com              | 2a06:98c1:50::ac40:23bc   | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 95   | cf.877771.xyz                         | 2606:4700:3033::6815:50b4 | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 109  | cfip.xxxxxxxx.tk                      | 104.18.228.35             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 145  | cu.877774.xyz                         | 104.26.4.114              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 148  | cu.877774.xyz                         | 104.26.4.117              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 170  | toy-people.com                        | 2606:4700:20::681a:224    | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 203  | www.whatismyip.com                    | 172.67.69.129             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 224  | cmcc.877774.xyz                       | 104.16.149.1              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 261  | www.ipchicken.com                     | 172.67.68.101             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 282  | moura.ns.cloudflare.com               | 108.162.195.217           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 289  | time.is                               | 104.26.13.54              | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 307  | benedict.ns.cloudflare.com            | 2a06:98c1:50::ac40:23cd   | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 320  | ae8a9c24-83de.masx200.ddns-ip.net     | 172.67.157.182            | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 329  | singapore.com                         | 104.26.13.140             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 358  | ashton.ns.cloudflare.com              | 2a06:98c1:50::ac40:23ad   | IPv6   | h2   | ✅ 成功 | 46       | cloudflare |
| 419  | eur.877774.xyz                        | 104.21.29.164             | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 437  | damien.ns.cloudflare.com              | 108.162.195.168           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 465  | abdullah.ns.cloudflare.com            | 108.162.195.203           | IPv4   | h2   | ✅ 成功 | 46       | cloudflare |
| 5    | www.ipget.net                         | 2606:4700:3031::ac43:cf1a | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 82   | icook.hk                              | 2606:4700:3031::6815:5ad2 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 88   | huxley.ns.cloudflare.com              | 2803:f800:50::6ca2:c3bc   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 121  | www.okcupid.com                       | 104.18.160.63             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 143  | cu.877774.xyz                         | 104.26.4.112              | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 159  | pranab.ns.cloudflare.com              | 2803:f800:50::6ca2:c3c7   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 165  | cris.ns.cloudflare.com                | 2803:f800:50::6ca2:c3ca   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 168  | toy-people.com                        | 104.26.2.36               | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 182  | zread.ai                              | 2606:4700:3033::6815:4cf0 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 195  | dylan.ns.cloudflare.com               | 108.162.195.187           | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 198  | dylan.ns.cloudflare.com               | 2606:4700:58::a29f:2cbb   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 263  | www.ipchicken.com                     | 104.26.6.112              | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 327  | singapore.com                         | 104.26.12.140             | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 343  | www.visa.com.hk                       | 104.18.21.69              | IPv4   | h2   | ✅ 成功 | 47       | cloudflare |
| 349  | local-aria2-webui.masx200.ddns-ip.net | 2606:4700:3030::6815:e29  | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 362  | yx-auto.pages.dev                     | 2606:4700:310c::ac42:2c90 | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 368  | dnschecker.org                        | 2606:4700:20::ac43:49d8   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 425  | japan.com                             | 2606:4700:20::681a:43c    | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 431  | otto.ns.cloudflare.com                | 2606:4700:58::a29f:2c87   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 433  | otto.ns.cloudflare.com                | 2a06:98c1:50::ac40:2387   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 442  | damien.ns.cloudflare.com              | 2a06:98c1:50::ac40:23a8   | IPv6   | h2   | ✅ 成功 | 47       | cloudflare |
| 9    | comicabc.com                          | 2606:4700:3030::ac43:ae15 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 84   | huxley.ns.cloudflare.com              | 108.162.195.188           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 93   | cf.877771.xyz                         | 104.21.80.180             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 94   | cf.877771.xyz                         | 172.67.152.183            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 126  | sullivan.ns.cloudflare.com            | 2606:4700:58::a29f:2ca1   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 142  | cu.877774.xyz                         | 104.26.4.111              | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 157  | pranab.ns.cloudflare.com              | 172.64.35.199             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 158  | pranab.ns.cloudflare.com              | 2606:4700:58::a29f:2cc7   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 164  | cris.ns.cloudflare.com                | 2606:4700:58::a29f:2cca   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 169  | toy-people.com                        | 104.26.3.36               | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 188  | kyree.ns.cloudflare.com               | 108.162.195.207           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 192  | kyree.ns.cloudflare.com               | 2803:f800:50::6ca2:c3cf   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 202  | www.whatismyip.com                    | 104.26.12.23              | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 209  | asia.877774.xyz                       | 104.17.142.146            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 210  | [2606:4700:4409::5b5b:7758]           | 2606:4700:4409::5b5b:7758 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 243  | saas.sin.fan                          | 162.159.36.20             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 255  | fbi.gov                               | 2606:4700::6810:94f4      | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 273  | bowen.ns.cloudflare.com               | 2606:4700:58::a29f:2c53   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 277  | www.glassdoor.com                     | 104.17.64.70              | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 292  | time.is                               | 2606:4700:20::681a:c36    | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 310  | palera.in                             | 2606:4700:3032::ac43:9d7a | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 317  | ip.sb                                 | 2606:4700:20::681a:c1f    | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 324  | ip.gs                                 | 104.21.14.176             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 370  | julio.ns.cloudflare.com               | 108.162.195.209           | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 379  | uriah.ns.cloudflare.com               | 2606:4700:58::a29f:2cc2   | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 395  | 104.17.142.12                         | 104.17.142.12             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 396  | tasteatlas.com                        | 104.17.37.105             | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 422  | japan.com                             | 172.67.70.92              | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 445  | ifconfig.co                           | 172.67.168.106            | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 446  | ifconfig.co                           | 104.21.54.91              | IPv4   | h2   | ✅ 成功 | 48       | cloudflare |
| 454  | www.wto.org                           | 2606:4700:4406::ac40:9242 | IPv6   | h2   | ✅ 成功 | 48       | cloudflare |
| 47   | ct.877774.xyz                         | 172.64.229.185            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 57   | steamdb.info                          | 2606:4700:10::6814:22d4   | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 80   | icook.hk                              | 172.67.161.104            | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 86   | huxley.ns.cloudflare.com              | 172.64.35.188             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 113  | www.hugedomains.com                   | 104.26.7.37               | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 117  | www.okcupid.com                       | 104.17.48.63              | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 135  | craig.ns.cloudflare.com               | 2803:f800:50::6ca2:c3c0   | IPv6   | h2   | ✅ 成功 | 49       | cloudflare |
| 147  | cu.877774.xyz                         | 104.26.4.116              | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |
| 154  | na.877774.xyz                         | 104.19.74.233             | IPv4   | h2   | ✅ 成功 | 49       | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 100 条记录
- **快 (50-100ms)**: 0 条记录
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
