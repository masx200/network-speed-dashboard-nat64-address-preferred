# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 13:22:11
- **数据来源**: connectivity_results.json
- **总测试数**: 482
- **失败测试数**: 20
- **成功测试数**: 462
- **失败率**: 4.15%
- **平均延迟**: 585.94ms
- **最小延迟**: 127ms
- **最大延迟**: 5699ms

---

## 失败测试详情

### 错误类型统计

- **连接错误**: 6 次
- **连接超时**: 13 次
- **DNS解析错误**: 1 次

### 失败测试列表

| 序号 | 主机/域名                             | 目标IP          | IP版本  | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                           |
| ---- | ------------------------------------- | --------------- | ------- | ---- | ------ | -------- | ------ | -------------------------------------------------- |
| 178  | 61.83.202.17                          | 61.83.202.17    | IPv4    | none | N/A    | 0        | N/A    | dial tcp 61.83.202.17:443: connectex: No connec... |
| 197  | 175.212.207.13                        | 175.212.207.13  | IPv4    | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout           |
| 206  | 211.229.77.184                        | 211.229.77.184  | IPv4    | none | N/A    | 0        | N/A    | dial tcp 211.229.77.184:443: connectex: A conne... |
| 207  | cfip.xxxxxxxx.tk                      | 198.41.212.130  | IPv4    | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout           |
| 209  | 52.76.110.129                         | 52.76.110.129   | IPv4    | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout            |
| 210  | 119.194.220.146                       | 119.194.220.146 | IPv4    | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout          |
| 246  | 172.64.201.25                         | 172.64.201.25   | IPv4    | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 270  | 222.105.131.225                       | 222.105.131.225 | IPv4    | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: i/o timeout          |
| 295  | 115.22.115.218                        | 115.22.115.218  | IPv4    | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout           |
| 330  | local-aria2-webui.masx200.ddns-ip.net | Unknown         | Unknown | none | N/A    | 0        | N/A    | DNS解析无结果                                      |
| 361  | 121.188.182.190                       | 121.188.182.190 | IPv4    | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout          |
| 401  | 43.153.179.6                          | 43.153.179.6    | IPv4    | none | N/A    | 0        | N/A    | dial tcp 43.153.179.6:443: connectex: No connec... |
| 422  | 175.215.175.175                       | 175.215.175.175 | IPv4    | none | N/A    | 0        | N/A    | dial tcp 175.215.175.175:443: connectex: A conn... |
| 431  | 172.67.49.134                         | 172.67.49.134   | IPv4    | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 434  | 59.31.68.195                          | 59.31.68.195    | IPv4    | none | N/A    | 0        | N/A    | dial tcp 59.31.68.195:443: connectex: No connec... |
| 441  | 111.171.108.67                        | 111.171.108.67  | IPv4    | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout           |
| 442  | 61.85.1.77                            | 61.85.1.77      | IPv4    | none | N/A    | 0        | N/A    | dial tcp 61.85.1.77:443: connectex: No connecti... |
| 443  | 138.2.18.82                           | 138.2.18.82     | IPv4    | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: i/o timeout              |
| 450  | 3.0.50.69                             | 3.0.50.69       | IPv4    | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                |
| 477  | trevor.ns.cloudfl...                  | 108.162.195.154 | IPv4    | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 | | ----
| --------- | ------ | ------ | ---- | ---- | -------- | ------ || 99 |
cris.ns.cloudflar... | 162.159.44.202 | IPv4 | h3 | ✅ 成功 | 127 | cloudflare |
| 394 | uriah.ns.cloudfla... | 162.159.44.194 | IPv4 | h3 | ✅ 成功 | 128 |
cloudflare | | 376 | moura.ns.cloudfla... | 162.159.44.217 | IPv4 | h3 | ✅ 成功
| 129 | cloudflare | | 18 | damien.ns.cloudfl... | 162.159.44.168 | IPv4 | h3 |
✅ 成功 | 130 | cloudflare | | 413 | ct.877774.xyz | 172.64.229.173 | IPv4 | h3
| ✅ 成功 | 133 | cloudflare | | 426 | sullivan.ns.cloud... | 162.159.44.161 |
IPv4 | h3 | ✅ 成功 | 141 | cloudflare | | 12 | abdullah.ns.cloud... |
162.159.44.203 | IPv4 | h3 | ✅ 成功 | 146 | cloudflare | | 6 | 162.159.39.118 |
162.159.39.118 | IPv4 | h3 | ✅ 成功 | 150 | cloudflare | | 388 |
pranab.ns.cloudfl... | 162.159.44.199 | IPv4 | h3 | ✅ 成功 | 151 | cloudflare |
| 129 | braden.ns.cloudfl... | 162.159.44.169 | IPv4 | h3 | ✅ 成功 | 156 |
cloudflare | | 363 | otto.ns.cloudflar... | 162.159.44.135 | IPv4 | h3 | ✅ 成功
| 163 | cloudflare | | 410 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅
成功 | 164 | cloudflare | | 49 | craig.ns.cloudfla... | 162.159.44.192 | IPv4 |
h3 | ✅ 成功 | 165 | cloudflare | | 82 | wilson.ns.cloudfl... | 162.159.44.110 |
IPv4 | h3 | ✅ 成功 | 170 | cloudflare | | 316 | kyree.ns.cloudfla... |
162.159.44.207 | IPv4 | h3 | ✅ 成功 | 170 | cloudflare | | 478 |
trevor.ns.cloudfl... | 162.159.44.154 | IPv4 | h3 | ✅ 成功 | 174 | cloudflare |
| 382 | benedict.ns.cloud... | 162.159.44.205 | IPv4 | h3 | ✅ 成功 | 176 |
cloudflare | | 411 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 | ✅ 成功 | 176 |
cloudflare | | 412 | ct.877774.xyz | 172.64.229.161 | IPv4 | h3 | ✅ 成功 | 176
| cloudflare | | 408 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 |
179 | cloudflare | | 182 | huxley.ns.cloudfl... | 162.159.44.188 | IPv4 | h3 |
✅ 成功 | 183 | cloudflare | | 409 | ct.877774.xyz | 172.64.229.217 | IPv4 | h3
| ✅ 成功 | 184 | cloudflare | | 398 | uriah.ns.cloudfla... |
2a06:98c1:50::ac40:23c2 | IPv6 | h3 | ✅ 成功 | 187 | cloudflare | | 1 |
162.159.44.208 | 162.159.44.208 | IPv4 | h3 | ✅ 成功 | 189 | cloudflare | | 14
| abdullah.ns.cloud... | 2606:4700:58::a29f:2ccb | IPv6 | h3 | ✅ 成功 | 189 |
cloudflare | | 74 | 162.159.38.89 | 162.159.38.89 | IPv4 | h3 | ✅ 成功 | 189 |
cloudflare | | 324 | bowen.ns.cloudfla... | 162.159.44.83 | IPv4 | h3 | ✅ 成功
| 191 | cloudflare | | 406 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 | ✅
成功 | 191 | cloudflare | | 267 | dylan.ns.cloudfla... | 2803:f800:50::6ca2:c3bb
| IPv6 | h3 | ✅ 成功 | 193 | cloudflare | | 247 | decker.ns.cloudfl... |
108.162.195.155 | IPv4 | h3 | ✅ 成功 | 195 | cloudflare | | 421 | 172.64.40.9 |
172.64.40.9 | IPv4 | h3 | ✅ 成功 | 195 | cloudflare | | 185 |
huxley.ns.cloudfl... | 2803:f800:50::6ca2:c3bc | IPv6 | h3 | ✅ 成功 | 196 |
cloudflare | | 264 | dylan.ns.cloudfla... | 162.159.44.187 | IPv4 | h3 | ✅ 成功
| 197 | cloudflare | | 366 | otto.ns.cloudflar... | 2803:f800:50::6ca2:c387 |
IPv6 | h3 | ✅ 成功 | 197 | cloudflare | | 397 | uriah.ns.cloudfla... |
2803:f800:50::6ca2:c3c2 | IPv6 | h3 | ✅ 成功 | 201 | cloudflare | | 53 |
craig.ns.cloudfla... | 2a06:98c1:50::ac40:23c0 | IPv6 | h3 | ✅ 成功 | 203 |
cloudflare | | 480 | trevor.ns.cloudfl... | 2606:4700:58::a29f:2c9a | IPv6 | h3
| ✅ 成功 | 204 | cloudflare | | 308 | 108.162.198.54 | 108.162.198.54 | IPv4 |
h3 | ✅ 成功 | 205 | cloudflare | | 315 | kyree.ns.cloudfla... | 108.162.195.207
| IPv4 | h3 | ✅ 成功 | 205 | cloudflare | | 52 | craig.ns.cloudfla... |
2803:f800:50::6ca2:c3c0 | IPv6 | h3 | ✅ 成功 | 207 | cloudflare | | 101 |
cris.ns.cloudflar... | 2606:4700:58::a29f:2cca | IPv6 | h3 | ✅ 成功 | 207 |
cloudflare | | 407 | ct.877774.xyz | 172.64.229.185 | IPv4 | h3 | ✅ 成功 | 207
| cloudflare | | 24 | cf.877774.xyz | 104.18.41.190 | IPv4 | h3 | ✅ 成功 | 208
| cloudflare | | 57 | www.wto.org | 2a06:98c1:3102::6812:29be | IPv6 | h3 | ✅
成功 | 209 | cloudflare | | 391 | pranab.ns.cloudfl... | 2803:f800:50::6ca2:c3c7
| IPv6 | h3 | ✅ 成功 | 209 | cloudflare | | 249 | decker.ns.cloudfl... |
172.64.35.155 | IPv4 | h3 | ✅ 成功 | 210 | cloudflare | | 35 |
julio.ns.cloudfla... | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 211 | cloudflare |
| 392 | pranab.ns.cloudfl... | 2a06:98c1:50::ac40:23c7 | IPv6 | h3 | ✅ 成功 |
211 | cloudflare | | 364 | otto.ns.cloudflar... | 172.64.35.135 | IPv4 | h3 | ✅
成功 | 212 | cloudflare | | 367 | otto.ns.cloudflar... | 2a06:98c1:50::ac40:2387
| IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 385 | benedict.ns.cloud... |
2803:f800:50::6ca2:c3cd | IPv6 | h3 | ✅ 成功 | 212 | cloudflare | | 263 |
dylan.ns.cloudfla... | 108.162.195.187 | IPv4 | h3 | ✅ 成功 | 213 | cloudflare
| | 325 | bowen.ns.cloudfla... | 172.64.35.83 | IPv4 | h3 | ✅ 成功 | 213 |
cloudflare | | 378 | moura.ns.cloudfla... | 2606:4700:58::a29f:2cd9 | IPv6 | h3
| ✅ 成功 | 213 | cloudflare | | 415 | lewis.ns.cloudfla... | 162.159.44.159 |
IPv4 | h3 | ✅ 成功 | 213 | cloudflare | | 7 | 172.64.156.195 | 172.64.156.195 |
IPv4 | h3 | ✅ 成功 | 214 | cloudflare | | 319 | kyree.ns.cloudfla... |
2803:f800:50::6ca2:c3cf | IPv6 | h3 | ✅ 成功 | 214 | cloudflare | | 123 |
172.64.33.67 | 172.64.33.67 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare | | 11 |
abdullah.ns.cloud... | 108.162.195.203 | IPv4 | h3 | ✅ 成功 | 218 | cloudflare
| | 17 | damien.ns.cloudfl... | 108.162.195.168 | IPv4 | h3 | ✅ 成功 | 218 |
cloudflare | | 34 | julio.ns.cloudfla... | 108.162.195.209 | IPv4 | h3 | ✅ 成功
| 218 | cloudflare | | 51 | craig.ns.cloudfla... | 2606:4700:58::a29f:2cc0 |
IPv6 | h3 | ✅ 成功 | 218 | cloudflare | | 83 | wilson.ns.cloudfl... |
172.64.35.110 | IPv4 | h3 | ✅ 成功 | 219 | cloudflare | | 318 |
kyree.ns.cloudfla... | 2606:4700:58::a29f:2ccf | IPv6 | h3 | ✅ 成功 | 219 |
cloudflare | | 91 | ashton.ns.cloudfl... | 2803:f800:50::6ca2:c3ad | IPv6 | h3 |
✅ 成功 | 220 | cloudflare | | 362 | otto.ns.cloudflar... | 108.162.195.135 |
IPv4 | h3 | ✅ 成功 | 220 | cloudflare | | 416 | lewis.ns.cloudfla... |
172.64.35.159 | IPv4 | h3 | ✅ 成功 | 220 | cloudflare | | 181 |
huxley.ns.cloudfl... | 108.162.195.188 | IPv4 | h3 | ✅ 成功 | 221 | cloudflare
| | 365 | otto.ns.cloudflar... | 2606:4700:58::a29f:2c87 | IPv6 | h3 | ✅ 成功 |
221 | cloudflare | | 379 | moura.ns.cloudfla... | 2803:f800:50::6ca2:c3d9 | IPv6
| h3 | ✅ 成功 | 221 | cloudflare | | 479 | trevor.ns.cloudfl... | 172.64.35.154
| IPv4 | h3 | ✅ 成功 | 221 | cloudflare | | 44 | bestcf.030101.xyz |
104.18.33.227 | IPv4 | h3 | ✅ 成功 | 222 | cloudflare | | 265 |
dylan.ns.cloudfla... | 172.64.35.187 | IPv4 | h3 | ✅ 成功 | 222 | cloudflare |
| 419 | lewis.ns.cloudfla... | 2a06:98c1:50::ac40:239f | IPv6 | h3 | ✅ 成功 |
222 | cloudflare | | 90 | ashton.ns.cloudfl... | 2606:4700:58::a29f:2cad | IPv6
| h3 | ✅ 成功 | 223 | cloudflare | | 50 | craig.ns.cloudfla... | 172.64.35.192
| IPv4 | h3 | ✅ 成功 | 224 | cloudflare | | 320 | kyree.ns.cloudfla... |
2a06:98c1:50::ac40:23cf | IPv6 | h3 | ✅ 成功 | 224 | cloudflare | | 380 |
moura.ns.cloudfla... | 2a06:98c1:50::ac40:23d9 | IPv6 | h3 | ✅ 成功 | 224 |
cloudflare | | 418 | lewis.ns.cloudfla... | 2803:f800:50::6ca2:c39f | IPv6 | h3
| ✅ 成功 | 224 | cloudflare | | 430 | sullivan.ns.cloud... |
2a06:98c1:50::ac40:23a1 | IPv6 | h3 | ✅ 成功 | 224 | cloudflare | | 326 |
bowen.ns.cloudfla... | 2606:4700:58::a29f:2c53 | IPv6 | h3 | ✅ 成功 | 225 |
cloudflare | | 327 | bowen.ns.cloudfla... | 2803:f800:50::6ca2:c353 | IPv6 | h3
| ✅ 成功 | 225 | cloudflare | | 89 | ashton.ns.cloudfl... | 172.64.35.173 |
IPv4 | h3 | ✅ 成功 | 226 | cloudflare | | 98 | cris.ns.cloudflar... |
108.162.195.202 | IPv4 | h3 | ✅ 成功 | 226 | cloudflare | | 131 |
braden.ns.cloudfl... | 2606:4700:58::a29f:2ca9 | IPv6 | h3 | ✅ 成功 | 226 |
cloudflare | | 150 | 104.18.37.40 | 104.18.37.40 | IPv4 | h3 | ✅ 成功 | 226 |
cloudflare | | 133 | braden.ns.cloudfl... | 2a06:98c1:50::ac40:23a9 | IPv6 | h3
| ✅ 成功 | 227 | cloudflare | | 427 | sullivan.ns.cloud... | 172.64.35.161 |
IPv4 | h3 | ✅ 成功 | 227 | cloudflare | | 122 | 172.64.147.73 | 172.64.147.73 |
IPv4 | h3 | ✅ 成功 | 228 | cloudflare | | 159 | 172.64.146.16 | 172.64.146.16 |
IPv4 | h3 | ✅ 成功 | 228 | cloudflare | | 251 | decker.ns.cloudfl... |
2803:f800:50::6ca2:c39b | IPv6 | h3 | ✅ 成功 | 228 | cloudflare | | 355 |
[2606:4700:440b::... | 2606:4700:440b::3e6e:5f06 | IPv6 | h3 | ✅ 成功 | 228 |
cloudflare | | 317 | kyree.ns.cloudfla... | 172.64.35.207 | IPv4 | h3 | ✅ 成功
| 229 | cloudflare | | 38 | julio.ns.cloudfla... | 2803:f800:50::6ca2:c3d1 |
IPv6 | h3 | ✅ 成功 | 230 | cloudflare | | 104 | [2606:4700:4403::... |
2606:4700:4403::7357:544f | IPv6 | h3 | ✅ 成功 | 230 | cloudflare | | 113 |
rustam.ns.cloudfl... | 172.64.35.148 | IPv4 | h3 | ✅ 成功 | 230 | cloudflare |
| 341 | 104.18.37.13 | 104.18.37.13 | IPv4 | h3 | ✅ 成功 | 230 | cloudflare | |
117 | [2606:4700:440f::... | 2606:4700:440f::53aa:4126 | IPv6 | h3 | ✅ 成功 |
231 | cloudflare | | 177 | 172.64.49.165 | 172.64.49.165 | IPv4 | h3 | ✅ 成功 |
231 | cloudflare | | 86 | wilson.ns.cloudfl... | 2a06:98c1:50::ac40:236e | IPv6
| h3 | ✅ 成功 | 232 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 34 条记录
- **慢 (200-500ms)**: 66 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 19 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 20 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
