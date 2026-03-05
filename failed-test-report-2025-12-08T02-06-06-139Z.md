# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 10:06:06
- **数据来源**: connectivity_results.json
- **总测试数**: 485
- **失败测试数**: 25
- **成功测试数**: 460
- **失败率**: 5.15%
- **平均延迟**: 384.92ms
- **最小延迟**: 118ms
- **最大延迟**: 2226ms

---

## 失败测试详情

### 错误类型统计

- **连接错误**: 7 次
- **连接超时**: 14 次
- **DNS解析错误**: 1 次
- **未知错误**: 3 次

### 失败测试列表

| 序号 | 主机/域名                             | 目标IP                  | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器     | 错误信息                                           |
| ---- | ------------------------------------- | ----------------------- | ------- | ---- | ------ | -------- | ---------- | -------------------------------------------------- |
| 164  | 61.83.202.17                          | 61.83.202.17            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 61.83.202.17:443: connectex: No connec... |
| 196  | 211.229.77.184                        | 211.229.77.184          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 211.229.77.184:443: connectex: No conn... |
| 206  | 175.212.207.13                        | 175.212.207.13          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 175.212.207.13:443: i/o timeout           |
| 213  | cfip.xxxxxxxx.tk                      | 198.41.212.130          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 198.41.212.130:443: i/o timeout           |
| 240  | 52.76.110.129                         | 52.76.110.129           | IPv4    | none | N/A    | 0        | N/A        | dial tcp 52.76.110.129:443: i/o timeout            |
| 241  | 119.194.220.146                       | 119.194.220.146         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 119.194.220.146:443: i/o timeout          |
| 280  | 172.64.201.25                         | 172.64.201.25           | IPv4    | none | N/A    | 0        | N/A        | dial tcp 172.64.201.25:443: i/o timeout            |
| 284  | 222.105.131.225                       | 222.105.131.225         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 222.105.131.225:443: i/o timeout          |
| 332  | local-aria2-webui.masx200.ddns-ip.net | Unknown                 | Unknown | none | N/A    | 0        | N/A        | DNS解析无结果                                      |
| 333  | 115.22.115.218                        | 115.22.115.218          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 115.22.115.218:443: i/o timeout           |
| 347  | ifconfig.co                           | 104.16.123.96           | IPv4    | h3   | 403    | 327      | cloudflare | No error message                                   |
| 348  | ifconfig.co                           | 104.16.124.96           | IPv4    | h3   | 403    | 328      | cloudflare | No error message                                   |
| 353  | 158.101.77.33                         | 158.101.77.33           | IPv4    | h2   | N/A    | 0        | N/A        | Get "https://local-aria2-webui.masx200.ddns-ip.... |
| 361  | 175.215.175.175                       | 175.215.175.175         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 175.215.175.175:443: connectex: No con... |
| 386  | 43.153.179.6                          | 43.153.179.6            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 43.153.179.6:443: connectex: No connec... |
| 401  | 121.188.182.190                       | 121.188.182.190         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 121.188.182.190:443: i/o timeout          |
| 430  | 172.67.49.134                         | 172.67.49.134           | IPv4    | none | N/A    | 0        | N/A        | dial tcp 172.67.49.134:443: i/o timeout            |
| 434  | 111.171.108.67                        | 111.171.108.67          | IPv4    | none | N/A    | 0        | N/A        | dial tcp 111.171.108.67:443: i/o timeout           |
| 435  | 138.2.18.82                           | 138.2.18.82             | IPv4    | none | N/A    | 0        | N/A        | dial tcp 138.2.18.82:443: i/o timeout              |
| 439  | 59.31.68.195                          | 59.31.68.195            | IPv4    | none | N/A    | 0        | N/A        | dial tcp 59.31.68.195:443: connectex: No connec... |
| 467  | trevor.ns.cloudfl...                  | 108.162.195.154         | IPv4    | none | N/A    | 0        | N/A        | dial tcp 108.162.195.154:443: i/o timeout          |
| 473  | 141.147.185.63                        | 141.147.185.63          | IPv4    | h2   | N/A    | 0        | N/A        | Get "https://local-aria2-webui.masx200.ddns-ip.... |
| 474  | 61.85.1.77                            | 61.85.1.77              | IPv4    | none | N/A    | 0        | N/A        | dial tcp 61.85.1.77:443: connectex: No connecti... |
| 475  | 3.0.50.69                             | 3.0.50.69               | IPv4    | none | N/A    | 0        | N/A        | dial tcp 3.0.50.69:443: i/o timeout                |
| 483  | cloudflare.182682...                  | 2606:4700:e7::3151:47a9 | IPv6    | none | N/A    | 0        | N/A        | dial tcp [2606:4700:e7::3151:47a9]:443: i/o tim... |

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 | | ----
| --------- | ------ | ------ | ---- | ---- | -------- | ------ || 320 |
kyree.ns.cloudfla... | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 118 | cloudflare |
| 17 | abdullah.ns.cloud... | 162.159.44.203 | IPv4 | h3 | ✅ 成功 | 133 |
cloudflare | | 414 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 142
| cloudflare | | 420 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 |
150 | cloudflare | | 7 | 162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功 |
151 | cloudflare | | 23 | damien.ns.cloudfl... | 162.159.44.168 | IPv4 | h3 | ✅
成功 | 151 | cloudflare | | 417 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 |
✅ 成功 | 152 | cloudflare | | 143 | cris.ns.cloudflar... | 162.159.44.202 |
IPv4 | h3 | ✅ 成功 | 155 | cloudflare | | 374 | moura.ns.cloudfla... |
162.159.44.217 | IPv4 | h3 | ✅ 成功 | 156 | cloudflare | | 47 |
julio.ns.cloudfla... | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 158 | cloudflare |
| 228 | dylan.ns.cloudfla... | 162.159.44.187 | IPv4 | h3 | ✅ 成功 | 162 |
cloudflare | | 405 | lewis.ns.cloudfla... | 162.159.44.159 | IPv4 | h3 | ✅ 成功
| 163 | cloudflare | | 421 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功
| 164 | cloudflare | | 180 | huxley.ns.cloudfl... | 162.159.44.188 | IPv4 | h3 |
✅ 成功 | 166 | cloudflare | | 396 | sullivan.ns.cloud... | 162.159.44.161 |
IPv4 | h3 | ✅ 成功 | 166 | cloudflare | | 415 | ct.877774.xyz | 172.64.229.173
| IPv4 | h3 | ✅ 成功 | 168 | cloudflare | | 389 | pranab.ns.cloudfl... |
162.159.44.199 | IPv4 | h3 | ✅ 成功 | 169 | cloudflare | | 275 |
decker.ns.cloudfl... | 162.159.44.155 | IPv4 | h3 | ✅ 成功 | 171 | cloudflare |
| 124 | rustam.ns.cloudfl... | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 175 |
cloudflare | | 380 | benedict.ns.cloud... | 162.159.44.205 | IPv4 | h3 | ✅ 成功
| 176 | cloudflare | | 80 | braden.ns.cloudfl... | 162.159.44.169 | IPv4 | h3 |
✅ 成功 | 177 | cloudflare | | 246 | 172.64.229.249 | 172.64.229.249 | IPv4 | h3
| ✅ 成功 | 177 | cloudflare | | 199 | 162.159.45.170 | 162.159.45.170 | IPv4 |
h3 | ✅ 成功 | 179 | cloudflare | | 309 | 108.162.198.54 | 108.162.198.54 | IPv4
| h3 | ✅ 成功 | 181 | cloudflare | | 118 | ashton.ns.cloudfl... |
162.159.44.173 | IPv4 | h3 | ✅ 成功 | 184 | cloudflare | | 61 |
craig.ns.cloudfla... | 162.159.44.192 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 103 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 | ✅ 成功 | 185 | cloudflare |
| 419 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3 | ✅ 成功 | 187 | cloudflare
| | 326 | bowen.ns.cloudfla... | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 188 |
cloudflare | | 167 | 172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 191 |
cloudflare | | 81 | braden.ns.cloudfl... | 172.64.35.169 | IPv4 | h3 | ✅ 成功 |
194 | cloudflare | | 122 | ashton.ns.cloudfl... | 2a06:98c1:50::ac40:23ad | IPv6
| h3 | ✅ 成功 | 194 | cloudflare | | 418 | ct.877774.xyz | 172.64.229.195 |
IPv4 | h3 | ✅ 成功 | 196 | cloudflare | | 424 | uriah.ns.cloudfla... |
162.159.44.194 | IPv4 | h3 | ✅ 成功 | 196 | cloudflare | | 162 | 104.18.37.40 |
104.18.37.40 | IPv4 | h3 | ✅ 成功 | 198 | cloudflare | | 19 |
abdullah.ns.cloud... | 2606:4700:58::a29f:2ccb | IPv6 | h3 | ✅ 成功 | 199 |
cloudflare | | 181 | huxley.ns.cloudfl... | 172.64.35.188 | IPv4 | h3 | ✅ 成功
| 199 | cloudflare | | 52 | [2606:4700:4403::... | 2606:4700:4403::7357:544f |
IPv6 | h3 | ✅ 成功 | 200 | cloudflare | | 46 | julio.ns.cloudfla... |
108.162.195.209 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare | | 144 |
cris.ns.cloudflar... | 172.64.35.202 | IPv4 | h3 | ✅ 成功 | 201 | cloudflare |
| 214 | [2606:4700:4409::... | 2606:4700:4409::5b5b:7758 | IPv6 | h3 | ✅ 成功 |
201 | cloudflare | | 70 | www.wto.org | 172.64.146.66 | IPv4 | h3 | ✅ 成功 |
203 | cloudflare | | 276 | decker.ns.cloudfl... | 172.64.35.155 | IPv4 | h3 | ✅
成功 | 203 | cloudflare | | 400 | sullivan.ns.cloud... | 2a06:98c1:50::ac40:23a1
| IPv6 | h3 | ✅ 成功 | 205 | cloudflare | | 25 | damien.ns.cloudfl... |
2606:4700:58::a29f:2ca8 | IPv6 | h3 | ✅ 成功 | 206 | cloudflare | | 324 |
kyree.ns.cloudfla... | 2a06:98c1:50::ac40:23cf | IPv6 | h3 | ✅ 成功 | 206 |
cloudflare | | 390 | pranab.ns.cloudfl... | 172.64.35.199 | IPv4 | h3 | ✅ 成功
| 206 | cloudflare | | 120 | ashton.ns.cloudfl... | 2606:4700:58::a29f:2cad |
IPv6 | h3 | ✅ 成功 | 207 | cloudflare | | 65 | craig.ns.cloudfla... |
2a06:98c1:50::ac40:23c0 | IPv6 | h3 | ✅ 成功 | 208 | cloudflare | | 119 |
ashton.ns.cloudfl... | 172.64.35.173 | IPv4 | h3 | ✅ 成功 | 208 | cloudflare |
| 231 | dylan.ns.cloudfla... | 2803:f800:50::6ca2:c3bb | IPv6 | h3 | ✅ 成功 |
208 | cloudflare | | 322 | kyree.ns.cloudfla... | 2606:4700:58::a29f:2ccf | IPv6
| h3 | ✅ 成功 | 208 | cloudflare | | 179 | huxley.ns.cloudfl... |
108.162.195.188 | IPv4 | h3 | ✅ 成功 | 209 | cloudflare | | 115 |
wilson.ns.cloudfl... | 2803:f800:50::6ca2:c36e | IPv6 | h3 | ✅ 成功 | 210 |
cloudflare | | 114 | wilson.ns.cloudfl... | 2606:4700:58::a29f:2c6e | IPv6 | h3
| ✅ 成功 | 211 | cloudflare | | 126 | rustam.ns.cloudfl... |
2606:4700:58::a29f:2c94 | IPv6 | h3 | ✅ 成功 | 211 | cloudflare | | 364 |
otto.ns.cloudflar... | 172.64.35.135 | IPv4 | h3 | ✅ 成功 | 211 | cloudflare |
| 399 | sullivan.ns.cloud... | 2803:f800:50::6ca2:c3a1 | IPv6 | h3 | ✅ 成功 |
211 | cloudflare | | 62 | craig.ns.cloudfla... | 172.64.35.192 | IPv4 | h3 | ✅
成功 | 212 | cloudflare | | 116 | wilson.ns.cloudfl... | 2a06:98c1:50::ac40:236e
| IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 277 | decker.ns.cloudfl... |
2606:4700:58::a29f:2c9b | IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 21 |
abdullah.ns.cloud... | 2a06:98c1:50::ac40:23cb | IPv6 | h3 | ✅ 成功 | 214 |
cloudflare | | 145 | cris.ns.cloudflar... | 2606:4700:58::a29f:2cca | IPv6 | h3
| ✅ 成功 | 214 | cloudflare | | 366 | otto.ns.cloudflar... |
2803:f800:50::6ca2:c387 | IPv6 | h3 | ✅ 成功 | 214 | cloudflare | | 222 |
na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare | | 367 |
otto.ns.cloudflar... | 2a06:98c1:50::ac40:2387 | IPv6 | h3 | ✅ 成功 | 215 |
cloudflare | | 9 | 172.64.156.195 | 172.64.156.195 | IPv4 | h3 | ✅ 成功 | 216 |
cloudflare | | 16 | abdullah.ns.cloud... | 108.162.195.203 | IPv4 | h3 | ✅ 成功
| 216 | cloudflare | | 31 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 |
h3 | ✅ 成功 | 216 | cloudflare | | 212 | 172.64.159.6 | 172.64.159.6 | IPv4 |
h3 | ✅ 成功 | 216 | cloudflare | | 71 | www.wto.org | 104.18.41.190 | IPv4 | h3
| ✅ 成功 | 217 | cloudflare | | 74 | 172.64.147.73 | 172.64.147.73 | IPv4 | h3
| ✅ 成功 | 217 | cloudflare | | 318 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3
| ✅ 成功 | 217 | cloudflare | | 362 | otto.ns.cloudflar... | 108.162.195.135 |
IPv4 | h3 | ✅ 成功 | 217 | cloudflare | | 48 | julio.ns.cloudfla... |
172.64.35.209 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare | | 184 |
huxley.ns.cloudfl... | 2a06:98c1:50::ac40:23bc | IPv6 | h3 | ✅ 成功 | 218 |
cloudflare | | 379 | benedict.ns.cloud... | 108.162.195.205 | IPv4 | h3 | ✅
成功 | 218 | cloudflare | | 128 | rustam.ns.cloudfl... | 2a06:98c1:50::ac40:2394
| IPv6 | h3 | ✅ 成功 | 219 | cloudflare | | 319 | kyree.ns.cloudfla... |
108.162.195.207 | IPv4 | h3 | ✅ 成功 | 219 | cloudflare | | 355 | 104.18.39.196
| 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 219 | cloudflare | | 363 |
otto.ns.cloudflar... | 162.159.44.135 | IPv4 | h3 | ✅ 成功 | 219 | cloudflare |
| 427 | uriah.ns.cloudfla... | 2803:f800:50::6ca2:c3c2 | IPv6 | h3 | ✅ 成功 |
219 | cloudflare | | 146 | cris.ns.cloudflar... | 2803:f800:50::6ca2:c3ca | IPv6
| h3 | ✅ 成功 | 220 | cloudflare | | 468 | trevor.ns.cloudfl... |
162.159.44.154 | IPv4 | h3 | ✅ 成功 | 220 | cloudflare | | 469 |
trevor.ns.cloudfl... | 172.64.35.154 | IPv4 | h3 | ✅ 成功 | 220 | cloudflare |
| 83 | braden.ns.cloudfl... | 2803:f800:50::6ca2:c3a9 | IPv6 | h3 | ✅ 成功 |
221 | cloudflare | | 84 | braden.ns.cloudfl... | 2a06:98c1:50::ac40:23a9 | IPv6
| h3 | ✅ 成功 | 221 | cloudflare | | 112 | wilson.ns.cloudfl... |
162.159.44.110 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare | | 381 |
benedict.ns.cloud... | 172.64.35.205 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare |
| 15 | 172.64.38.15 | 172.64.38.15 | IPv4 | h3 | ✅ 成功 | 222 | cloudflare | |
113 | wilson.ns.cloudfl... | 172.64.35.110 | IPv4 | h3 | ✅ 成功 | 222 |
cloudflare | | 330 | bowen.ns.cloudfla... | 2a06:98c1:50::ac40:2353 | IPv6 | h3
| ✅ 成功 | 222 | cloudflare | | 26 | damien.ns.cloudfl... |
2803:f800:50::6ca2:c3a8 | IPv6 | h3 | ✅ 成功 | 223 | cloudflare | | 398 |
sullivan.ns.cloud... | 2606:4700:58::a29f:2ca1 | IPv6 | h3 | ✅ 成功 | 223 |
cloudflare | | 416 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅ 成功 | 223
| cloudflare | | 437 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 | 224 |
cloudflare | | 18 | abdullah.ns.cloud... | 172.64.35.203 | IPv4 | h3 | ✅ 成功 |
225 | cloudflare | | 50 | julio.ns.cloudfla... | 2803:f800:50::6ca2:c3d1 | IPv6
| h3 | ✅ 成功 | 225 | cloudflare | | 82 | braden.ns.cloudfl... |
2606:4700:58::a29f:2ca9 | IPv6 | h3 | ✅ 成功 | 225 | cloudflare | | 352 |
[2606:4700:4408::... | 2606:4700:4408::18c5:3304 | IPv6 | h3 | ✅ 成功 | 225 |
cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 37 条记录
- **慢 (200-500ms)**: 63 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 23 次
- **IPv6 失败**: 1 次

### 按协议统计

- **none**: 21 次失败
- **h3**: 2 次失败
- **h2**: 2 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
