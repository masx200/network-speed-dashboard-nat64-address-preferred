# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 14:54:36
- **数据来源**: connectivity_results.json
- **总测试数**: 485
- **失败测试数**: 10
- **成功测试数**: 475
- **失败率**: 2.06%
- **平均延迟**: 88.23ms
- **最小延迟**: 53ms
- **最大延迟**: 685ms

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 10 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (10 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 10   | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 76   | 175.212.207.13           | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout  |
| 190  | 119.194.220.146          | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout |
| 199  | 121.188.182.190          | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout |
| 263  | 115.22.115.218           | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout  |
| 307  | decker.ns.cloudflare.com | 108.162.195.155 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.155:443: i/o timeout |
| 351  | 104.26.6.112             | 104.26.6.112    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.26.6.112:443: i/o timeout    |
| 384  | 52.76.110.129            | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout   |
| 393  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 396  | 111.171.108.67           | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout  |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 10 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 10 次超时，主要集中在IP段 198.41（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 10 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 433 |
www.hugedomains.com | 2606:4700:20::681a:625 | IPv6 | h2 | ✅ 成功 | 53 |
cloudflare | | 409 | huxley.ns.cloudflare.com | 2606:4700:58::a29f:2cbc | IPv6 |
h2 | ✅ 成功 | 55 | cloudflare | | 445 | damien.ns.cloudflare.com |
2a06:98c1:50::ac40:23a8 | IPv6 | h2 | ✅ 成功 | 55 | cloudflare | | 467 |
cmcc.877774.xyz | 104.16.149.1 | IPv4 | h2 | ✅ 成功 | 55 | cloudflare | | 404 |
172.67.49.134 | 172.67.49.134 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare | | 478 |
ct.877774.xyz | 172.64.229.173 | IPv4 | h2 | ✅ 成功 | 56 | cloudflare | | 398 |
[2606:4700:440f::53aa:4126] | 2606:4700:440f::53aa:4126 | IPv6 | h2 | ✅ 成功 |
57 | cloudflare | | 171 | cf.877771.xyz | 2606:4700:3033::6815:50b4 | IPv6 | h2
| ✅ 成功 | 58 | cloudflare | | 420 | japan.com | 2606:4700:20::681a:53c | IPv6
| h2 | ✅ 成功 | 58 | cloudflare | | 44 | www.okcupid.com | 104.16.144.63 | IPv4
| h2 | ✅ 成功 | 59 | cloudflare | | 49 | iplocation.io | 2606:4700:20::681a:bde
| IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 50 | iplocation.io |
2606:4700:20::ac43:4664 | IPv6 | h2 | ✅ 成功 | 59 | cloudflare | | 170 |
cf.877771.xyz | 2606:4700:3033::ac43:98b7 | IPv6 | h2 | ✅ 成功 | 59 |
cloudflare | | 444 | damien.ns.cloudflare.com | 2803:f800:50::6ca2:c3a8 | IPv6 |
h2 | ✅ 成功 | 59 | cloudflare | | 125 | palera.in | 2606:4700:3035::6815:3a48 |
IPv6 | h2 | ✅ 成功 | 60 | cloudflare | | 140 | 104.26.13.31 | 104.26.13.31 |
IPv4 | h2 | ✅ 成功 | 60 | cloudflare | | 165 | cu.877774.xyz | 104.26.4.117 |
IPv4 | h2 | ✅ 成功 | 60 | cloudflare | | 413 |
ae8a9c24-83de.masx200.ddns-ip.net | 172.67.157.182 | IPv4 | h2 | ✅ 成功 | 60 |
cloudflare | | 429 | www.hugedomains.com | 104.26.7.37 | IPv4 | h2 | ✅ 成功 |
60 | cloudflare | | 471 | cmcc.877774.xyz | 104.16.149.5 | IPv4 | h2 | ✅ 成功 |
60 | cloudflare | | 481 | ct.877774.xyz | 172.64.229.195 | IPv4 | h2 | ✅ 成功 |
60 | cloudflare | | 31 | uriah.ns.cloudflare.com | 2606:4700:58::a29f:2cc2 |
IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 63 | cloudflare-ip.mofashi.ltd |
2606:4700:3037::6815:48e9 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 72 |
ifconfig.co | 104.21.54.91 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 139 |
icook.hk | 2606:4700:3031::6815:5ad2 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | |
151 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h2 | ✅ 成功 | 61 |
cloudflare | | 153 | www.ipchicken.com | 104.26.6.112 | IPv4 | h2 | ✅ 成功 | 61
| cloudflare | | 259 | tasteatlas.com | 2606:4700::6811:2469 | IPv6 | h2 | ✅
成功 | 61 | cloudflare | | 273 | www.csgo.com | 195.85.59.95 | IPv4 | h2 | ✅
成功 | 61 | cloudflare | | 293 | whatismyipaddress.com | 2606:4700::6813:de4f |
IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 310 | decker.ns.cloudflare.com |
2606:4700:58::a29f:2c9b | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 397 |
172.64.151.55 | 172.64.151.55 | IPv4 | h2 | ✅ 成功 | 61 | cloudflare | | 410 |
huxley.ns.cloudflare.com | 2803:f800:50::6ca2:c3bc | IPv6 | h2 | ✅ 成功 | 61 |
cloudflare | | 414 | ae8a9c24-83de.masx200.ddns-ip.net |
2606:4700:3030::6815:e29 | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | | 419 |
japan.com | 2606:4700:20::ac43:465c | IPv6 | h2 | ✅ 成功 | 61 | cloudflare | |
427 | craig.ns.cloudflare.com | 2a06:98c1:50::ac40:23c0 | IPv6 | h2 | ✅ 成功 |
61 | cloudflare | | 464 | cmcc.877774.xyz | 104.16.148.11 | IPv4 | h2 | ✅ 成功
| 61 | cloudflare | | 468 | cmcc.877774.xyz | 104.16.149.2 | IPv4 | h2 | ✅ 成功
| 61 | cloudflare | | 485 | ct.877774.xyz | 172.64.229.161 | IPv4 | h2 | ✅ 成功
| 61 | cloudflare | | 26 | www.whatismyip.com | 2606:4700:20::ac43:4581 | IPv6 |
h2 | ✅ 成功 | 62 | cloudflare | | 89 | ip.sb | 2606:4700:20::681a:d1f | IPv6 |
h2 | ✅ 成功 | 62 | cloudflare | | 112 | freeyx.cloudflare88.eu.org |
2606:4700:3010:e070:5d82:552a:2069:936a | IPv6 | h2 | ✅ 成功 | 62 | cloudflare
| | 134 | xn--b6gac.eu.org | 2606:4700:3035::6815:5a4e | IPv6 | h2 | ✅ 成功 |
62 | cloudflare | | 172 | 104.18.42.26 | 104.18.42.26 | IPv4 | h2 | ✅ 成功 | 62
| cloudflare | | 173 | 172.67.110.232 | 172.67.110.232 | IPv4 | h2 | ✅ 成功 |
62 | cloudflare | | 174 | www.udemy.com | 104.16.142.237 | IPv4 | h2 | ✅ 成功 |
62 | cloudflare | | 188 | www.pcmag.com | 2606:4700::6810:1476 | IPv6 | h2 | ✅
成功 | 62 | cloudflare | | 212 | comicabc.com | 104.21.64.10 | IPv4 | h2 | ✅
成功 | 62 | cloudflare | | 217 | na.877774.xyz | 104.18.187.25 | IPv4 | h2 | ✅
成功 | 62 | cloudflare | | 244 | stock.hostmonit.com | 2606:4700:3037::6815:7c1
| IPv6 | h2 | ✅ 成功 | 62 | cloudflare | | 381 |
04c6cf21-1294-4fae-8bf8-715bbc897b60.masx200.netlib.re |
2606:4700:3037::ac43:aedb | IPv6 | h2 | ✅ 成功 | 62 | cloudflare | | 383 |
104.17.68.85 | 104.17.68.85 | IPv4 | h2 | ✅ 成功 | 62 | cloudflare | | 411 |
huxley.ns.cloudflare.com | 2a06:98c1:50::ac40:23bc | IPv6 | h2 | ✅ 成功 | 62 |
cloudflare | | 463 | cmcc.877774.xyz | 104.16.148.10 | IPv4 | h2 | ✅ 成功 | 62
| cloudflare | | 466 | cmcc.877774.xyz | 104.16.148.244 | IPv4 | h2 | ✅ 成功 |
62 | cloudflare | | 470 | cmcc.877774.xyz | 104.16.149.4 | IPv4 | h2 | ✅ 成功 |
62 | cloudflare | | 27 | www.whatismyip.com | 2606:4700:20::681a:c17 | IPv6 | h2
| ✅ 成功 | 63 | cloudflare | | 46 | iplocation.io | 104.26.11.222 | IPv4 | h2 |
✅ 成功 | 63 | cloudflare | | 48 | iplocation.io | 2606:4700:20::681a:ade | IPv6
| h2 | ✅ 成功 | 63 | cloudflare | | 73 | ifconfig.co |
2606:4700:3037::6815:365b | IPv6 | h2 | ✅ 成功 | 63 | cloudflare | | 86 | ip.sb
| 104.26.13.31 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare | | 93 |
www.digitalocean.com | 2606:4700::6813:ae44 | IPv6 | h2 | ✅ 成功 | 63 |
cloudflare | | 121 | time.is | 2606:4700:20::ac43:449d | IPv6 | h2 | ✅ 成功 |
63 | cloudflare | | 156 | eur.877774.xyz | 104.21.47.209 | IPv4 | h2 | ✅ 成功 |
63 | cloudflare | | 183 | dnschecker.org | 2606:4700:20::681a:759 | IPv6 | h2 |
✅ 成功 | 63 | cloudflare | | 184 | dnschecker.org | 2606:4700:20::ac43:49d8 |
IPv6 | h2 | ✅ 成功 | 63 | cloudflare | | 209 | asia.877774.xyz | 104.17.139.62
| IPv4 | h2 | ✅ 成功 | 63 | cloudflare | | 210 | asia.877774.xyz |
104.17.142.146 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare | | 214 | comicabc.com |
2606:4700:3036::6815:400a | IPv6 | h2 | ✅ 成功 | 63 | cloudflare | | 243 |
stock.hostmonit.com | 2606:4700:3033::ac43:bbfb | IPv6 | h2 | ✅ 成功 | 63 |
cloudflare | | 279 |
72806a5a-a251-48b4-a523-dfbd1c981ec0.ce225219-fea4-47a0-bb82-70b612b27ab7.netlib.re
| 2606:4700:3034::6815:3db6 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare | | 360 |
zread.ai | 2606:4700:3033::6815:4cf0 | IPv6 | h2 | ✅ 成功 | 63 | cloudflare | |
417 | japan.com | 172.67.70.92 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare | | 428 |
www.hugedomains.com | 172.67.70.191 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare | |
430 | www.hugedomains.com | 104.26.6.37 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare
| | 446 | cmcc.877774.xyz | 104.16.149.6 | IPv4 | h2 | ✅ 成功 | 63 | cloudflare
| | 450 | cmcc.877774.xyz | 104.16.149.10 | IPv4 | h2 | ✅ 成功 | 63 |
cloudflare | | 465 | cmcc.877774.xyz | 104.16.148.12 | IPv4 | h2 | ✅ 成功 | 63
| cloudflare | | 23 | www.whatismyip.com | 104.26.12.23 | IPv4 | h2 | ✅ 成功 |
64 | cloudflare | | 25 | www.whatismyip.com | 2606:4700:20::681a:d17 | IPv6 | h2
| ✅ 成功 | 64 | cloudflare | | 36 | singapore.com | 172.67.75.194 | IPv4 | h2 |
✅ 成功 | 64 | cloudflare | | 47 | iplocation.io | 172.67.70.100 | IPv4 | h2 |
✅ 成功 | 64 | cloudflare | | 55 | julio.ns.cloudflare.com |
2803:f800:50::6ca2:c3d1 | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 58 |
saas.sin.fan | 162.159.36.20 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | | 64 |
cloudflare-ip.mofashi.ltd | 2606:4700:3037::ac43:9bac | IPv6 | h2 | ✅ 成功 | 64
| cloudflare | | 75 | 172.64.41.88 | 172.64.41.88 | IPv4 | h2 | ✅ 成功 | 64 |
cloudflare | | 80 | cf.0sm.com | 2606:4700:3037::ac43:bb91 | IPv6 | h2 | ✅ 成功
| 64 | cloudflare | | 84 | www.gov.ua | 2606:4700:3033::ac43:d17f | IPv6 | h2 |
✅ 成功 | 64 | cloudflare | | 88 | ip.sb | 2606:4700:20::ac43:4bac | IPv6 | h2 |
✅ 成功 | 64 | cloudflare | | 92 | www.digitalocean.com | 104.19.173.68 | IPv4 |
h2 | ✅ 成功 | 64 | cloudflare | | 147 | cris.ns.cloudflare.com |
2a06:98c1:50::ac40:23ca | IPv6 | h2 | ✅ 成功 | 64 | cloudflare | | 205 |
bestcf.030101.xyz | 104.19.41.154 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | |
230 | toy-people.com | 104.26.2.36 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | |
231 | toy-people.com | 172.67.72.18 | IPv4 | h2 | ✅ 成功 | 64 | cloudflare | |
313 | local-aria2-webui.masx200.ddns-ip.net | 104.21.14.41 | IPv4 | h2 | ✅ 成功
| 64 | cloudflare | | 356 | www.7749tv.com | 104.19.133.4 | IPv4 | h2 | ✅ 成功
| 64 | cloudflare | | 392 | steamdb.info | 2606:4700:10::6814:22d4 | IPv6 | h2 |
✅ 成功 | 64 | cloudflare | | 421 | japan.com | 2606:4700:20::681a:43c | IPv6 |
h2 | ✅ 成功 | 64 | cloudflare | | 469 | cmcc.877774.xyz | 104.16.149.3 | IPv4 |
h2 | ✅ 成功 | 64 | cloudflare | | 61 | cloudflare-ip.mofashi.ltd |
104.21.72.233 | IPv4 | h2 | ✅ 成功 | 65 | cloudflare |

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

- **none**: 10 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
