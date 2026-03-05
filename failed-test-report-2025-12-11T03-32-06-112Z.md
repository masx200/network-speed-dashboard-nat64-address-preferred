# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/11 11:32:06
- **数据来源**: connectivity_results.json
- **总测试数**: 459
- **失败测试数**: 15
- **成功测试数**: 444
- **失败率**: 3.27%
- **平均延迟**: 106.73ms
- **最小延迟**: 60ms
- **最大延迟**: 1407ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 10 次 (66.7%)
- **DNS解析错误: 其他DNS错误**: 5 次 (33.3%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (10 次测试)

| 序号 | 主机/域名                | 目标IP         | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                 |
| ---- | ------------------------ | -------------- | ------ | ---- | ------ | -------- | ------ | ---------------------------------------- |
| 11   | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 12   | 172.64.201.25            | 172.64.201.25  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout  |
| 168  | cfip.xxxxxxxx.tk         | 198.41.212.130 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout |
| 170  | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 182  | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 250  | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 274  | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 315  | 104.26.6.112             | 104.26.6.112   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout   |
| 352  | bowen.ns.cloudflare.com  | 108.162.195.83 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.83:443: i/o timeout |
| 429  | trevor.ns.cloudflare.com | 162.159.44.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 162.159.44.154:443: i/o timeout |

#### DNS解析错误: 其他DNS错误 (5 次测试)

| 序号 | 主机/域名           | 目标IP  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                                                                                                                                           |
| ---- | ------------------- | ------- | ------- | ---- | ------ | -------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 6    | stock.hostmonit.com | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": dial tcp: lookup xget.a1u06h9fe9y5bozbmgz3.qzz.io on 127.0.0.53:53: server misbehaving |
| 7    | palera.in           | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": dial tcp: lookup xget.a1u06h9fe9y5bozbmgz3.qzz.io on 127.0.0.53:53: server misbehaving |
| 8    | www.4chan.org       | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": dial tcp: lookup xget.a1u06h9fe9y5bozbmgz3.qzz.io on 127.0.0.53:53: server misbehaving |
| 9    | fbi.gov             | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析失败: Post "https://deno-dns-over-https-server.g18uibxgnb.de5.net/": dial tcp: lookup xget.a1u06h9fe9y5bozbmgz3.qzz.io on 127.0.0.53:53: server misbehaving |
| 10   | www.ipchicken.com   | Unknown | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果                                                                                                                                                      |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 10 次 (66.7%)
- **DNS解析错误**: 5 次 (33.3%)

#### 错误模式分析

**超时集中度分析**: 共有 10 次超时，主要集中在IP段 104.26（6
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 15 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

**问题主机分析**: 以下主机出现多次失败：104.26.6.112
(6次)，建议重点检查这些主机的网络状态和服务可用性

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 261 |
icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 60 | cloudflare | |
219 | 104.19.175.123 | 104.19.175.123 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare |
| 279 | www.wto.org | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | |
414 | braden.ns.cloudflare.com | 2803:f800:50::6ca2:c3a9 | IPv6 | h2 | ✅ 成功 |
62 | cloudflare | | 111 | www.pcmag.com | 2606:4700::6810:1576 | IPv6 | h2 | ✅
成功 | 63 | cloudflare | | 136 | www.digitalocean.com | 2606:4700::6813:ae44 |
IPv6 | h2 | ✅ 成功 | 63 | cloudflare | | 38 | ipinfo.in |
2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 56 |
ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | | 152 |
toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | | 158 |
104.18.78.214 | 104.18.78.214 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | | 187 |
www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | |
267 | [2606:4700:8de6::5fa2:799e] | 2606:4700:8de6::5fa2:799e | IPv6 | h2 | ✅
成功 | 64 | cloudflare | | 269 | 172.64.159.6 | 172.64.159.6 | IPv4 | h2 | ✅
成功 | 64 | cloudflare | | 277 | eur.877774.xyz | 104.21.29.164 | IPv4 | h2 | ✅
成功 | 64 | cloudflare | | 419 | craig.ns.cloudflare.com |
2606:4700:58::a29f:2cc0 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 174 |
cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 65 |
cloudflare | | 300 | ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 65 |
cloudflare | | 17 | 162.159.133.85 | 162.159.133.85 | IPv4 | h2 | ✅ 成功 | 66 |
cloudflare | | 144 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅
成功 | 66 | cloudflare | | 159 | [2606:4700:9add::880:52fc] |
2606:4700:9add::880:52fc | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 162 |
cfip.xxxxxxxx.tk | 104.27.21.118 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare | | 179
| www.udemy.com | 104.16.143.237 | IPv4 | h2 | ✅ 成功 | 66 | cloudflare | | 206
| cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功 | 66 | cloudflare
| | 233 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 | 66 |
cloudflare | | 43 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2 | ✅
成功 | 67 | cloudflare | | 149 | 172.64.146.16 | 172.64.146.16 | IPv4 | h2 | ✅
成功 | 67 | cloudflare | | 157 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6
| h2 | ✅ 成功 | 67 | cloudflare | | 332 | 172.64.41.88 | 172.64.41.88 | IPv4 |
h2 | ✅ 成功 | 67 | cloudflare | | 362 | kyree.ns.cloudflare.com |
2803:f800:50::6ca2:c3cf | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 369 |
ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 409 |
icook.tw | 2606:4700:10::ac42:9e73 | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | |
118 | steamdb.info | 2606:4700:10::ac42:affa | IPv6 | h2 | ✅ 成功 | 68 |
cloudflare | | 142 | cf.877774.xyz | 104.18.41.190 | IPv4 | h2 | ✅ 成功 | 68 |
cloudflare | | 150 | www.visa.com.hk | 104.18.20.69 | IPv4 | h2 | ✅ 成功 | 68 |
cloudflare | | 185 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 |
68 | cloudflare | | 218 | saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 |
68 | cloudflare | | 222 | local-aria2-webui.masx200.ddns-ip.net | 172.67.157.182
| IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 296 | japan.com |
2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 68 | cloudflare | | 373 |
ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 400 |
ashton.ns.cloudflare.com | 172.64.35.173 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare
| | 454 | cmcc.877774.xyz | 104.16.149.9 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare
| | 16 | 104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
101 | 104.18.37.13 | 104.18.37.13 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
115 | steamdb.info | 172.66.175.250 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
186 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare
| | 192 | www.okcupid.com | 104.16.144.63 | IPv4 | h2 | ✅ 成功 | 69 |
cloudflare | | 280 | www.wto.org | 172.64.146.66 | IPv4 | h2 | ✅ 成功 | 69 |
cloudflare | | 314 | www.ipget.net | 2606:4700:3036::6815:fd4 | IPv6 | h2 | ✅
成功 | 69 | cloudflare | | 325 | dylan.ns.cloudflare.com |
2a06:98c1:50::ac40:23bb | IPv6 | h2 | ✅ 成功 | 69 | cloudflare | | 347 |
julio.ns.cloudflare.com | 162.159.44.209 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare
| | 364 | www.visa.cn | 162.159.152.2 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 453 | cmcc.877774.xyz | 104.16.149.8 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare |
| 97 | www.visa.com.sg | 104.18.12.229 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare |
| 114 | 104.16.223.179 | 104.16.223.179 | IPv4 | h2 | ✅ 成功 | 70 | cloudflare
| | 226 | cloudflare-ip.mofashi.ltd | 104.21.72.233 | IPv4 | h2 | ✅ 成功 | 70 |
cloudflare | | 230 | time.is | 172.67.68.157 | IPv4 | h2 | ✅ 成功 | 70 |
cloudflare | | 231 | time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 70 |
cloudflare | | 253 | silkbook.com | 104.26.9.160 | IPv4 | h2 | ✅ 成功 | 70 |
cloudflare | | 272 | ip.gs | 2606:4700:3035::ac43:a01c | IPv6 | h2 | ✅ 成功 |
70 | cloudflare | | 273 | ip.gs | 2606:4700:3036::6815:eb0 | IPv6 | h2 | ✅ 成功
| 70 | cloudflare | | 129 | cf.zhetengsha.eu.org | 2606:4700:310c::ac42:2fb3 |
IPv6 | h2 | ✅ 成功 | 71 | cloudflare | | 161 | www.csgo.com | 195.85.59.95 |
IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 211 | cu.877774.xyz | 104.26.4.119 |
IPv4 | h2 | ✅ 成功 | 71 | cloudflare | | 295 | japan.com | 104.26.5.60 | IPv4 |
h2 | ✅ 成功 | 71 | cloudflare | | 308 | na.877774.xyz | 104.19.74.233 | IPv4 |
h2 | ✅ 成功 | 71 | cloudflare | | 345 | pranab.ns.cloudflare.com |
2a06:98c1:50::ac40:23c7 | IPv6 | h2 | ✅ 成功 | 71 | cloudflare | | 393 |
bestcf.030101.xyz | 104.19.47.191 | IPv4 | h2 | ✅ 成功 | 71 | cloudflare | |
145 | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304 | IPv6 | h2 | ✅
成功 | 72 | cloudflare | | 153 | toy-people.com | 104.26.3.36 | IPv4 | h2 | ✅
成功 | 72 | cloudflare | | 156 | toy-people.com | 2606:4700:20::681a:224 | IPv6
| h2 | ✅ 成功 | 72 | cloudflare | | 259 | icook.hk | 104.21.90.210 | IPv4 | h2
| ✅ 成功 | 72 | cloudflare | | 289 | freeyx.cloudflare88.eu.org |
141.101.121.80 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 380 |
benedict.ns.cloudflare.com | 108.162.195.205 | IPv4 | h2 | ✅ 成功 | 72 |
cloudflare | | 385 | benedict.ns.cloudflare.com | 2a06:98c1:50::ac40:23cd | IPv6
| h2 | ✅ 成功 | 72 | cloudflare | | 401 | ashton.ns.cloudflare.com |
2606:4700:58::a29f:2cad | IPv6 | h2 | ✅ 成功 | 72 | cloudflare | | 456 |
cmcc.877774.xyz | 104.16.149.11 | IPv4 | h2 | ✅ 成功 | 72 | cloudflare | | 85 |
ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 | ✅ 成功 | 73 | cloudflare | | 188
| www.hugedomains.com | 2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 73 |
cloudflare | | 200 | zread.ai | 172.67.202.78 | IPv4 | h2 | ✅ 成功 | 73 |
cloudflare | | 243 | huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 |
h2 | ✅ 成功 | 73 | cloudflare | | 288 | freeyx.cloudflare88.eu.org |
141.101.120.109 | IPv4 | h2 | ✅ 成功 | 73 | cloudflare | | 302 | ifconfig.co |
2606:4700:3030::ac43:a86a | IPv6 | h2 | ✅ 成功 | 73 | cloudflare | | 337 |
dnschecker.org | 2606:4700:20::681a:659 | IPv6 | h2 | ✅ 成功 | 73 | cloudflare
| | 351 | julio.ns.cloudflare.com | 2a06:98c1:50::ac40:23d1 | IPv6 | h2 | ✅
成功 | 73 | cloudflare | | 365 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅
成功 | 73 | cloudflare | | 40 | www.whatismyip.com | 104.26.13.23 | IPv4 | h2 |
✅ 成功 | 74 | cloudflare | | 62 | damien.ns.cloudflare.com |
2606:4700:58::a29f:2ca8 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare | | 106 |
singapore.com | 2606:4700:20::ac43:4bc2 | IPv6 | h2 | ✅ 成功 | 74 | cloudflare
| | 112 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare
| | 139 | 172.64.148.15 | 172.64.148.15 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare
| | 140 | 104.26.13.31 | 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |
| 191 | www.okcupid.com | 104.16.223.254 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare
| | 201 | zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 74 |
cloudflare | | 202 | zread.ai | 2606:4700:3032::ac43:ca4e | IPv6 | h2 | ✅ 成功
| 74 | cloudflare | | 205 | cf.0sm.com | 2606:4700:3032::6815:785 | IPv6 | h2 |
✅ 成功 | 74 | cloudflare | | 212 | cu.877774.xyz | 104.26.4.111 | IPv4 | h2 |
✅ 成功 | 74 | cloudflare | | 234 | time.is | 2606:4700:20::681a:d36 | IPv6 | h2
| ✅ 成功 | 74 | cloudflare | | 281 | www.wto.org | 2a06:98c1:3102::6812:29be |
IPv6 | h2 | ✅ 成功 | 74 | cloudflare | | 284 | yx-auto.pages.dev |
104.21.73.250 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare | | 437 | cmcc.877774.xyz
| 104.16.148.5 | IPv4 | h2 | ✅ 成功 | 74 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 10 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 15 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
