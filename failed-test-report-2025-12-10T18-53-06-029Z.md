# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/10 18:53:06
- **数据来源**: connectivity_results-20251210-185305.json
- **总测试数**: 481
- **失败测试数**: 6
- **成功测试数**: 475
- **失败率**: 1.25%
- **平均延迟**: 114.17ms
- **最小延迟**: 57ms
- **最大延迟**: 2011ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 6 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (6 次测试)

| 序号 | 主机/域名        | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ---------------- | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 8    | 111.171.108.67   | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |
| 81   | 121.188.182.190  | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 190  | cfip.xxxxxxxx.tk | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 239  | 175.212.207.13   | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 448  | 172.64.201.25    | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 450  | 119.194.220.146  | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 6 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 6 次超时，主要集中在IP段 111.171（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 6 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 343 |
www.glassdoor.com | 104.16.25.46 | IPv4 | h2 | ✅ 成功 | 57 | cloudflare | | 436
| pranab.ns.cloudflare.com | 108.162.195.199 | IPv4 | h2 | ✅ 成功 | 57 |
cloudflare | | 120 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 58 | cloudflare | | 173 |
huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 58 |
cloudflare | | 335 | toy-people.com | 2606:4700:20::ac43:4812 | IPv6 | h2 | ✅
成功 | 58 | cloudflare | | 376 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 |
✅ 成功 | 58 | cloudflare | | 228 | cf.877771.xyz | 2606:4700:3033::6815:50b4 |
IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 359 | cmcc.877774.xyz | 104.16.149.9 |
IPv4 | h2 | ✅ 成功 | 59 | cloudflare | | 393 | uriah.ns.cloudflare.com |
2803:f800:50::6ca2:c3c2 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 412 |
kyree.ns.cloudflare.com | 2606:4700:58::a29f:2ccf | IPv6 | h2 | ✅ 成功 | 59 |
cloudflare | | 85 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅ 成功
| 60 | cloudflare | | 177 | www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 |
h2 | ✅ 成功 | 60 | cloudflare | | 344 | www.glassdoor.com | 104.17.64.70 | IPv4
| h2 | ✅ 成功 | 60 | cloudflare | | 139 | ct.877774.xyz | 172.64.229.44 | IPv4
| h2 | ✅ 成功 | 61 | cloudflare | | 145 | iplocation.io | 104.26.11.222 | IPv4
| h2 | ✅ 成功 | 61 | cloudflare | | 183 | www.hugedomains.com |
2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 234 |
www.ipget.net | 172.67.207.26 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 275 |
time.is | 104.26.12.54 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 279 | time.is
| 2606:4700:20::681a:c36 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 301 |
172.67.75.172 | 172.67.75.172 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 243 |
xn--b6gac.eu.org | 2606:4700:3037::ac43:99fd | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare | | 257 | 172.64.49.165 | 172.64.49.165 | IPv4 | h2 | ✅ 成功 | 62 |
cloudflare | | 294 | fbi.gov | 2606:4700::6810:95f4 | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare | | 297 | ipv4.ip.sb | 104.26.12.31 | IPv4 | h2 | ✅ 成功 | 62 |
cloudflare | | 298 | 104.18.39.196 | 104.18.39.196 | IPv4 | h2 | ✅ 成功 | 62 |
cloudflare | | 336 | toy-people.com | 2606:4700:20::681a:224 | IPv6 | h2 | ✅
成功 | 62 | cloudflare | | 348 | decker.ns.cloudflare.com |
2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 62 | cloudflare | | 407 |
julio.ns.cloudflare.com | 2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare | | 235 | www.ipget.net | 2606:4700:3031::ac43:cf1a | IPv6 | h2 | ✅
成功 | 63 | cloudflare | | 330 | 162.159.38.89 | 162.159.38.89 | IPv4 | h2 | ✅
成功 | 63 | cloudflare | | 368 | cmcc.877774.xyz | 104.16.148.5 | IPv4 | h2 | ✅
成功 | 63 | cloudflare | | 369 | cmcc.877774.xyz | 104.16.148.6 | IPv4 | h2 | ✅
成功 | 63 | cloudflare | | 415 | icook.hk | 104.21.90.210 | IPv4 | h2 | ✅ 成功
| 63 | cloudflare | | 456 | whatismyipaddress.com | 104.19.223.79 | IPv4 | h2 |
✅ 成功 | 63 | cloudflare | | 9 | 104.26.6.112 | 104.26.6.112 | IPv4 | h2 | ✅
成功 | 64 | cloudflare | | 95 | zread.ai | 104.21.76.240 | IPv4 | h2 | ✅ 成功 |
64 | cloudflare | | 115 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 |
✅ 成功 | 64 | cloudflare | | 181 | www.hugedomains.com | 104.26.6.37 | IPv4 |
h2 | ✅ 成功 | 64 | cloudflare | | 221 | bowen.ns.cloudflare.com |
2a06:98c1:50::ac40:2353 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 400 |
moura.ns.cloudflare.com | 2803:f800:50::6ca2:c3d9 | IPv6 | h2 | ✅ 成功 | 64 |
cloudflare | | 425 | www.visa.cn | 162.159.153.2 | IPv4 | h2 | ✅ 成功 | 64 |
cloudflare | | 479 | damien.ns.cloudflare.com | 2606:4700:58::a29f:2ca8 | IPv6 |
h2 | ✅ 成功 | 64 | cloudflare | | 43 | ashton.ns.cloudflare.com |
2a06:98c1:50::ac40:23ad | IPv6 | h2 | ✅ 成功 | 65 | cloudflare | | 67 |
www.visa.com.sg | 104.18.13.229 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare | | 77 |
cloudflare-ip.mofashi.ltd | 2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 65
| cloudflare | | 111 | ipinfo.in | 2606:4700:3037::ac43:c6cb | IPv6 | h2 | ✅
成功 | 65 | cloudflare | | 248 | ip.sb | 2606:4700:20::681a:c1f | IPv6 | h2 | ✅
成功 | 65 | cloudflare | | 306 | www.gov.ua | 2606:4700:3031::6815:1748 | IPv6 |
h2 | ✅ 成功 | 65 | cloudflare | | 319 | benedict.ns.cloudflare.com |
162.159.44.205 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare | | 329 | singapore.com |
2606:4700:20::681a:d8c | IPv6 | h2 | ✅ 成功 | 65 | cloudflare | | 386 |
cris.ns.cloudflare.com | 2606:4700:58::a29f:2cca | IPv6 | h2 | ✅ 成功 | 65 |
cloudflare | | 397 | moura.ns.cloudflare.com | 162.159.44.217 | IPv4 | h2 | ✅
成功 | 65 | cloudflare | | 438 | pranab.ns.cloudflare.com | 172.64.35.199 | IPv4
| h2 | ✅ 成功 | 65 | cloudflare | | 83 | tasteatlas.com | 104.17.36.105 | IPv4
| h2 | ✅ 成功 | 66 | cloudflare | | 182 | www.hugedomains.com |
2606:4700:20::ac43:46bf | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 212 |
yx-auto.pages.dev | 2606:4700:310c::ac42:2c90 | IPv6 | h2 | ✅ 成功 | 66 |
cloudflare | | 286 | cu.877774.xyz | 104.26.4.118 | IPv4 | h2 | ✅ 成功 | 66 |
cloudflare | | 293 | fbi.gov | 2606:4700::6810:94f4 | IPv6 | h2 | ✅ 成功 | 66 |
cloudflare | | 309 | sullivan.ns.cloudflare.com | 162.159.44.161 | IPv4 | h2 |
✅ 成功 | 66 | cloudflare | | 334 | toy-people.com | 2606:4700:20::681a:324 |
IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 350 | decker.ns.cloudflare.com |
2a06:98c1:50::ac40:239b | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 396 |
moura.ns.cloudflare.com | 108.162.195.217 | IPv4 | h2 | ✅ 成功 | 66 |
cloudflare | | 410 | kyree.ns.cloudflare.com | 162.159.44.207 | IPv4 | h2 | ✅
成功 | 66 | cloudflare | | 411 | kyree.ns.cloudflare.com | 172.64.35.207 | IPv4
| h2 | ✅ 成功 | 66 | cloudflare | | 418 | icook.hk | 2606:4700:3037::ac43:a168
| IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 458 | whatismyipaddress.com |
2606:4700::6813:df4f | IPv6 | h2 | ✅ 成功 | 66 | cloudflare | | 60 |
freeyx.cloudflare88.eu.org | 141.101.121.48 | IPv4 | h2 | ✅ 成功 | 67 |
cloudflare | | 79 | [2606:4700:83bd::7d8:2b47] | 2606:4700:83bd::7d8:2b47 | IPv6
| h2 | ✅ 成功 | 67 | cloudflare | | 101 | cf.zhetengsha.eu.org |
2a06:98c1:3101::ac40:919e | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 123 |
na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 282 |
cu.877774.xyz | 104.26.4.114 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 292 |
fbi.gov | 104.16.149.244 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 299 |
172.67.79.211 | 172.67.79.211 | IPv4 | h2 | ✅ 成功 | 67 | cloudflare | | 307 |
www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 | ✅ 成功 | 67 | cloudflare |
| 345 | decker.ns.cloudflare.com | 108.162.195.155 | IPv4 | h2 | ✅ 成功 | 67 |
cloudflare | | 362 | cmcc.877774.xyz | 104.16.149.12 | IPv4 | h2 | ✅ 成功 | 67
| cloudflare | | 364 | cmcc.877774.xyz | 104.16.148.1 | IPv4 | h2 | ✅ 成功 | 67
| cloudflare | | 457 | whatismyipaddress.com | 104.19.222.79 | IPv4 | h2 | ✅
成功 | 67 | cloudflare | | 469 | icook.tw | 2606:4700:10::6814:1c4a | IPv6 | h2
| ✅ 成功 | 67 | cloudflare | | 475 | trevor.ns.cloudflare.com |
2a06:98c1:50::ac40:239a | IPv6 | h2 | ✅ 成功 | 67 | cloudflare | | 25 |
www.pcmag.com | 104.16.21.118 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 58 |
104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 100 |
cf.zhetengsha.eu.org | 172.64.145.158 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 103 | 172.64.154.18 | 172.64.154.18 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare |
| 219 | bowen.ns.cloudflare.com | 2606:4700:58::a29f:2c53 | IPv6 | h2 | ✅ 成功
| 68 | cloudflare | | 222 | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41
| IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 246 | ip.sb | 104.26.13.31 | IPv4 |
h2 | ✅ 成功 | 68 | cloudflare | | 312 | sullivan.ns.cloudflare.com |
2803:f800:50::6ca2:c3a1 | IPv6 | h2 | ✅ 成功 | 68 | cloudflare | | 373 |
cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 68 | cloudflare | | 384
| cris.ns.cloudflare.com | 162.159.44.202 | IPv4 | h2 | ✅ 成功 | 68 |
cloudflare | | 402 | 172.67.106.26 | 172.67.106.26 | IPv4 | h2 | ✅ 成功 | 68 |
cloudflare | | 417 | icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功
| 68 | cloudflare | | 445 | wilson.ns.cloudflare.com | 2606:4700:58::a29f:2c6e |
IPv6 | h2 | ✅ 成功 | 68 | cloudflare | | 59 | 172.64.41.88 | 172.64.41.88 |
IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 63 | ip.gs | 172.67.160.28 | IPv4 | h2
| ✅ 成功 | 69 | cloudflare | | 118 | ae8a9c24-83de.masx200.ddns-ip.net |
172.67.157.182 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | | 175 |
www.digitalocean.com | 104.19.174.68 | IPv4 | h2 | ✅ 成功 | 69 | cloudflare | |
244 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 | 69 |
cloudflare | | 258 | [2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 |
IPv6 | h2 | ✅ 成功 | 69 | cloudflare | | 291 | fbi.gov | 104.16.148.244 | IPv4
| h2 | ✅ 成功 | 69 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 100 条记录
- **正常 (100-200ms)**: 0 条记录
- **慢 (200-500ms)**: 0 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 6 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 6 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
