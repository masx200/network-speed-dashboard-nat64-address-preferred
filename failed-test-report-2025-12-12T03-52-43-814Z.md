# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 03:52:43
- **数据来源**: connectivity_results-20251212-035243.json
- **总测试数**: 363
- **失败测试数**: 18
- **成功测试数**: 345
- **失败率**: 4.96%
- **平均延迟**: 108.59ms
- **最小延迟**: 59ms
- **最大延迟**: 708ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: 上下文超时**: 12 次 (66.7%)
- **DNS解析错误: 其他DNS错误**: 4 次 (22.2%)
- **连接超时: I/O超时**: 2 次 (11.1%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: 上下文超时 (12 次测试)

| 序号 | 主机/域名                | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                              |
| ---- | ------------------------ | ------- | ------- | ---- | ------ | -------- | ------ | ----------------------------------------------------------------------------------------------------- |
| 197  | cf.zhetengsha.eu.org     | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 198  | xn--b6gac.eu.org         | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 201  | braden.ns.cloudflare.com | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 202  | fbi.gov                  | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 203  | saas.sin.fan             | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 204  | bowen.ns.cloudflare.com  | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 205  | cf.090227.xyz            | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 256  | ip.gs                    | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 295  | yx-auto.pages.dev        | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 298  | tasteatlas.com           | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 349  | stock.hostmonit.com      | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |
| 351  | www.csgo.com             | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": context deadline exceeded |

#### DNS解析错误: 其他DNS错误 (4 次测试)

| 序号 | 主机/域名       | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息      |
| ---- | --------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------- |
| 6    | ct.877774.xyz   | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 7    | cmcc.877774.xyz | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 257  | www.visa.com.hk | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |
| 296  | gamer.com.tw    | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果 |

#### 连接超时: I/O超时 (2 次测试)

| 序号 | 主机/域名        | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ---------------- | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 125  | cfip.xxxxxxxx.tk | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 300  | 172.64.201.25    | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 14 次 (77.8%)
- **DNS解析错误**: 4 次 (22.2%)

#### 错误模式分析

**超时集中度分析**: 共有 2 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 18 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 248 |
ip.sb | 2606:4700:20::681a:d1f | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 173 |
cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 60
| cloudflare | | 255 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd |
IPv6 | h2 | ✅ 成功 | 60 | cloudflare | | 268 | 104.18.42.26 | 104.18.42.26 |
IPv4 | h2 | ✅ 成功 | 60 | cloudflare | | 163 | decker.ns.cloudflare.com |
2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 266 |
whatismyipaddress.com | 2606:4700::6813:de4f | IPv6 | h2 | ✅ 成功 | 61 |
cloudflare | | 272 | local-aria2-webui.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 347 |
www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 61 | cloudflare
| | 348 | www.wto.org | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 61 |
cloudflare | | 156 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅
成功 | 62 | cloudflare | | 179 | kyree.ns.cloudflare.com |
2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 62 | cloudflare | | 210 |
www.ipchicken.com | 104.26.7.112 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 242
| singapore.com | 2606:4700:20::681a:c8c | IPv6 | h2 | ✅ 成功 | 62 | cloudflare
| | 245 | ip.sb | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 291
| cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 343
| ifconfig.co | 2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare | | 345 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 62 |
cloudflare | | 262 | silkbook.com | 2606:4700:20::ac43:4bd0 | IPv6 | h2 | ✅
成功 | 63 | cloudflare | | 312 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅
成功 | 63 | cloudflare | | 236 | rustam.ns.cloudflare.com |
2a06:98c1:50::ac40:2394 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 243 |
singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare
| | 244 | ip.sb | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | | 247 |
ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 267
| whatismyipaddress.com | 2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 64 |
cloudflare | | 270 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 64 |
cloudflare | | 302 | cfip.1323123.xyz | 104.16.133.220 | IPv4 | h2 | ✅ 成功 |
64 | cloudflare | | 303 | 104.18.37.40 | 104.18.37.40 | IPv4 | h2 | ✅ 成功 | 64
| cloudflare | | 344 | ifconfig.co | 2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅
成功 | 64 | cloudflare | | 346 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅
成功 | 64 | cloudflare | | 352 | japan.com | 104.26.4.60 | IPv4 | h2 | ✅ 成功 |
64 | cloudflare | | 165 | asia.877774.xyz | 104.17.142.146 | IPv4 | h2 | ✅ 成功
| 65 | cloudflare | | 175 | 104.17.79.11 | 104.17.79.11 | IPv4 | h2 | ✅ 成功 |
65 | cloudflare | | 192 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 |
h2 | ✅ 成功 | 65 | cloudflare | | 209 | www.ipchicken.com | 104.26.6.112 | IPv4
| h2 | ✅ 成功 | 65 | cloudflare | | 212 | moura.ns.cloudflare.com |
2606:4700:58::a29f:2cd9 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare | | 235 |
rustam.ns.cloudflare.com | 2803:f800:50::6ca2:c394 | IPv6 | h2 | ✅ 成功 | 65 |
cloudflare | | 283 | ashton.ns.cloudflare.com | 2803:f800:50::6ca2:c3ad | IPv6 |
h2 | ✅ 成功 | 65 | cloudflare | | 308 | www.digitalocean.com |
2606:4700::6813:ad44 | IPv6 | h2 | ✅ 成功 | 65 | cloudflare | | 309 |
www.udemy.com | 2606:4700::6810:8eed | IPv6 | h2 | ✅ 成功 | 65 | cloudflare | |
355 | japan.com | 2606:4700:20::681a:43c | IPv6 | h2 | ✅ 成功 | 65 | cloudflare
| | 211 | www.ipchicken.com | 172.67.68.101 | IPv4 | h2 | ✅ 成功 | 66 |
cloudflare | | 215 | palera.in | 172.67.157.122 | IPv4 | h2 | ✅ 成功 | 66 |
cloudflare | | 234 | rustam.ns.cloudflare.com | 2606:4700:58::a29f:2c94 | IPv6 |
h2 | ✅ 成功 | 66 | cloudflare | | 249 | ip.sb | 2606:4700:20::681a:c1f | IPv6 |
h2 | ✅ 成功 | 66 | cloudflare | | 254 | benedict.ns.cloudflare.com |
2803:f800:50::6ca2:c3cd | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 277 |
dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare
| | 286 | julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅
成功 | 66 | cloudflare | | 289 | [2606:4700:440b::3e6e:5f06] |
2606:4700:440b::3e6e:5f06 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 320 |
172.67.181.209 | 172.67.181.209 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare | | 162
| decker.ns.cloudflare.com | 2803:f800:50::6ca2:c39b | IPv6 | h2 | ✅ 成功 | 67
| cloudflare | | 164 | asia.877774.xyz | 104.17.139.62 | IPv4 | h2 | ✅ 成功 |
67 | cloudflare | | 169 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅
成功 | 67 | cloudflare | | 217 | palera.in | 2606:4700:3032::ac43:9d7a | IPv6 |
h2 | ✅ 成功 | 67 | cloudflare | | 219 | 104.16.61.163 | 104.16.61.163 | IPv4 |
h2 | ✅ 成功 | 67 | cloudflare | | 241 | singapore.com | 2606:4700:20::681a:d8c
| IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 246 | ip.sb | 104.26.12.31 | IPv4 |
h2 | ✅ 成功 | 67 | cloudflare | | 253 | benedict.ns.cloudflare.com |
2606:4700:58::a29f:2ccd | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 273 |
104.18.14.76 | 104.18.14.76 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 293 |
cf.877774.xyz | 2606:4700:4406::ac40:9242 | IPv6 | h2 | ✅ 成功 | 67 |
cloudflare | | 305 | icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 |
67 | cloudflare | | 191 | www.whatismyip.com | 172.67.69.129 | IPv4 | h2 | ✅
成功 | 68 | cloudflare | | 214 | moura.ns.cloudflare.com |
2a06:98c1:50::ac40:23d9 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare | | 222 |
ae8a9c24-83de.masx200.ddns-ip.net | 2606:4700:3030::6815:e29 | IPv6 | h2 | ✅
成功 | 68 | cloudflare | | 224 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 |
68 | cloudflare | | 226 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 68 |
cloudflare | | 261 | silkbook.com | 2606:4700:20::681a:8a0 | IPv6 | h2 | ✅ 成功
| 68 | cloudflare | | 292 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 |
h2 | ✅ 成功 | 68 | cloudflare | | 301 | 172.67.79.211 | 172.67.79.211 | IPv4 |
h2 | ✅ 成功 | 68 | cloudflare | | 318 | eur.877774.xyz | 104.21.47.209 | IPv4 |
h2 | ✅ 成功 | 68 | cloudflare | | 361 | lewis.ns.cloudflare.com |
2606:4700:58::a29f:2c9f | IPv6 | h2 | ✅ 成功 | 68 | cloudflare | | 155 |
toy-people.com | 2606:4700:20::681a:324 | IPv6 | h2 | ✅ 成功 | 69 | cloudflare
| | 218 | palera.in | 2606:4700:3035::6815:3a48 | IPv6 | h2 | ✅ 成功 | 69 |
cloudflare | | 229 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 |
69 | cloudflare | | 237 | 198.62.62.4 | 198.62.62.4 | IPv4 | h2 | ✅ 成功 | 69 |
cloudflare | | 265 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 | ✅ 成功
| 69 | cloudflare | | 274 | yx-auto.pages.dev | 2606:4700:310c::ac42:2f70 | IPv6
| h2 | ✅ 成功 | 69 | cloudflare | | 288 | 162.159.133.85 | 162.159.133.85 |
IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 299 | 104.17.142.12 | 104.17.142.12 |
IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 310 | www.udemy.com |
2606:4700::6810:8fed | IPv6 | h2 | ✅ 成功 | 69 | cloudflare | | 316 |
eur.877774.xyz | 104.21.26.150 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 188 |
[2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758 | IPv6 | h2 | ✅ 成功 |
70 | cloudflare | | 196 | [2606:4700:440f::53aa:4126] |
2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare | | 280 |
uriah.ns.cloudflare.com | 2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 70 |
cloudflare | | 281 | uriah.ns.cloudflare.com | 2a06:98c1:50::ac40:23c2 | IPv6 |
h2 | ✅ 成功 | 70 | cloudflare | | 297 | [2606:4700:4408::18c5:3304] |
2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅ 成功 | 70 | cloudflare | | 307 |
www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 70 |
cloudflare | | 341 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 70 |
cloudflare | | 168 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 | 71 |
cloudflare | | 187 | dylan.ns.cloudflare.com | 2a06:98c1:50::ac40:23bb | IPv6 |
h2 | ✅ 成功 | 71 | cloudflare | | 200 | bestcf.030101.xyz | 104.16.149.55 |
IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 220 |
ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 71 |
cloudflare | | 227 | time.is | 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 71
| cloudflare | | 276 | dnschecker.org | 2606:4700:20::ac43:49d8 | IPv6 | h2 | ✅
成功 | 71 | cloudflare | | 350 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅
成功 | 71 | cloudflare | | 356 | japan.com | 2606:4700:20::ac43:465c | IPv6 | h2
| ✅ 成功 | 71 | cloudflare | | 174 | cloudflare-ip.mofashi.ltd |
2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 239 |
singapore.com | 104.26.13.140 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 258 |
silkbook.com | 172.67.75.208 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 260 |
silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 321 |
104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 2 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 18 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
