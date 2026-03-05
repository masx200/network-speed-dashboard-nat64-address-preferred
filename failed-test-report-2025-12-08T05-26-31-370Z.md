# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/8 13:26:31
- **数据来源**: connectivity_results.json
- **总测试数**: 483
- **失败测试数**: 20
- **成功测试数**: 463
- **失败率**: 4.14%
- **平均延迟**: 533.95ms
- **最小延迟**: 120ms
- **最大延迟**: 5773ms

---

## 失败测试详情

### 错误类型统计

- **连接错误**: 5 次
- **未知错误**: 1 次
- **连接超时**: 14 次

### 失败测试列表

| 序号 | 主机/域名            | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                           |
| ---- | -------------------- | --------------- | ------ | ---- | ------ | -------- | ------ | -------------------------------------------------- |
| 177  | 61.83.202.17         | 61.83.202.17    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 61.83.202.17:443: connectex: No connec... |
| 184  | 211.229.77.184       | 211.229.77.184  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 211.229.77.184:443: connectex: No conn... |
| 185  | 168.138.184.172      | 168.138.184.172 | IPv4   | h2   | N/A    | 0        | N/A    | Get "https://local-aria2-webui.masx200.ddns-ip.... |
| 208  | cfip.xxxxxxxx.tk     | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout           |
| 212  | 175.212.207.13       | 175.212.207.13  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.212.207.13:443: i/o timeout           |
| 221  | 52.76.110.129        | 52.76.110.129   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 52.76.110.129:443: i/o timeout            |
| 222  | 119.194.220.146      | 119.194.220.146 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 119.194.220.146:443: i/o timeout          |
| 261  | 172.64.201.25        | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout            |
| 262  | 115.22.115.218       | 115.22.115.218  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 115.22.115.218:443: i/o timeout           |
| 284  | 222.105.131.225      | 222.105.131.225 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 222.105.131.225:443: i/o timeout          |
| 356  | 175.215.175.175      | 175.215.175.175 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 175.215.175.175:443: connectex: No con... |
| 364  | 43.153.179.6         | 43.153.179.6    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 43.153.179.6:443: connectex: No connec... |
| 365  | 121.188.182.190      | 121.188.182.190 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 121.188.182.190:443: i/o timeout          |
| 408  | 172.67.49.134        | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout            |
| 437  | 111.171.108.67       | 111.171.108.67  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 111.171.108.67:443: i/o timeout           |
| 444  | 138.2.18.82          | 138.2.18.82     | IPv4   | none | N/A    | 0        | N/A    | dial tcp 138.2.18.82:443: i/o timeout              |
| 449  | 59.31.68.195         | 59.31.68.195    | IPv4   | none | N/A    | 0        | N/A    | dial tcp 59.31.68.195:443: connectex: No connec... |
| 476  | trevor.ns.cloudfl... | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout          |
| 482  | 3.0.50.69            | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout                |
| 483  | 61.85.1.77           | 61.85.1.77      | IPv4   | none | N/A    | 0        | N/A    | dial tcp 61.85.1.77:443: i/o timeout               |

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名 | 目标IP | IP版本 | 协议 | 状态 | 延迟(ms) | 服务器 |
|------|-----------|--------|--------|------|------|----------|--------|| 105 |
rustam.ns.cloudfl... | 162.159.44.148 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare |
| 426 | ct.877774.xyz | 172.64.229.236 | IPv4 | h3 | ✅ 成功 | 120 | cloudflare
| | 308 | 108.162.198.54 | 108.162.198.54 | IPv4 | h3 | ✅ 成功 | 128 |
cloudflare | | 410 | uriah.ns.cloudfla... | 162.159.44.194 | IPv4 | h3 | ✅ 成功
| 134 | cloudflare | | 111 | cris.ns.cloudflar... | 162.159.44.202 | IPv4 | h3 |
✅ 成功 | 140 | cloudflare | | 228 | dylan.ns.cloudfla... | 162.159.44.187 |
IPv4 | h3 | ✅ 成功 | 144 | cloudflare | | 144 | braden.ns.cloudfl... |
162.159.44.169 | IPv4 | h3 | ✅ 成功 | 161 | cloudflare | | 47 |
julio.ns.cloudfla... | 162.159.44.209 | IPv4 | h3 | ✅ 成功 | 163 | cloudflare |
| 179 | huxley.ns.cloudfl... | 162.159.44.188 | IPv4 | h3 | ✅ 成功 | 163 |
cloudflare | | 4 | 162.159.39.118 | 162.159.39.118 | IPv4 | h3 | ✅ 成功 | 166 |
cloudflare | | 424 | ct.877774.xyz | 172.64.229.195 | IPv4 | h3 | ✅ 成功 | 166
| cloudflare | | 18 | abdullah.ns.cloud... | 162.159.44.203 | IPv4 | h3 | ✅
成功 | 167 | cloudflare | | 72 | craig.ns.cloudfla... | 162.159.44.192 | IPv4 |
h3 | ✅ 成功 | 167 | cloudflare | | 199 | 162.159.45.170 | 162.159.45.170 | IPv4
| h3 | ✅ 成功 | 168 | cloudflare | | 429 | ct.877774.xyz | 172.64.229.173 |
IPv4 | h3 | ✅ 成功 | 168 | cloudflare | | 425 | ct.877774.xyz | 172.64.229.217
| IPv4 | h3 | ✅ 成功 | 173 | cloudflare | | 428 | ct.877774.xyz |
172.64.229.161 | IPv4 | h3 | ✅ 成功 | 173 | cloudflare | | 98 |
ashton.ns.cloudfl... | 162.159.44.173 | IPv4 | h3 | ✅ 成功 | 174 | cloudflare |
| 85 | wilson.ns.cloudfl... | 162.159.44.110 | IPv4 | h3 | ✅ 成功 | 179 |
cloudflare | | 244 | 172.64.229.249 | 172.64.229.249 | IPv4 | h3 | ✅ 成功 | 180
| cloudflare | | 416 | lewis.ns.cloudfla... | 162.159.44.159 | IPv4 | h3 | ✅
成功 | 182 | cloudflare | | 430 | ct.877774.xyz | 172.64.229.174 | IPv4 | h3 |
✅ 成功 | 189 | cloudflare | | 477 | trevor.ns.cloudfl... | 162.159.44.154 |
IPv4 | h3 | ✅ 成功 | 189 | cloudflare | | 7 | 162.159.44.208 | 162.159.44.208 |
IPv4 | h3 | ✅ 成功 | 192 | cloudflare | | 70 | 162.159.38.89 | 162.159.38.89 |
IPv4 | h3 | ✅ 成功 | 197 | cloudflare | | 478 | trevor.ns.cloudfl... |
172.64.35.154 | IPv4 | h3 | ✅ 成功 | 197 | cloudflare | | 64 | www.wto.org |
172.64.146.66 | IPv4 | h3 | ✅ 成功 | 199 | cloudflare | | 101 |
ashton.ns.cloudfl... | 2803:f800:50::6ca2:c3ad | IPv6 | h3 | ✅ 成功 | 200 |
cloudflare | | 256 | decker.ns.cloudfl... | 162.159.44.155 | IPv4 | h3 | ✅ 成功
| 200 | cloudflare | | 21 | abdullah.ns.cloud... | 2803:f800:50::6ca2:c3cb |
IPv6 | h3 | ✅ 成功 | 202 | cloudflare | | 66 | www.wto.org |
2a06:98c1:3102::6812:29be | IPv6 | h3 | ✅ 成功 | 204 | cloudflare | | 394 |
pranab.ns.cloudfl... | 2803:f800:50::6ca2:c3c7 | IPv6 | h3 | ✅ 成功 | 205 |
cloudflare | | 52 | 172.64.41.88 | 172.64.41.88 | IPv4 | h3 | ✅ 成功 | 207 |
cloudflare | | 84 | wilson.ns.cloudfl... | 108.162.195.110 | IPv4 | h3 | ✅ 成功
| 207 | cloudflare | | 11 | damien.ns.cloudfl... | 108.162.195.168 | IPv4 | h3 |
✅ 成功 | 208 | cloudflare | | 128 | 172.64.33.67 | 172.64.33.67 | IPv4 | h3 |
✅ 成功 | 209 | cloudflare | | 427 | ct.877774.xyz | 172.64.229.44 | IPv4 | h3 |
✅ 成功 | 209 | cloudflare | | 25 | cf.877774.xyz | 2606:4700:4406::ac40:9242 |
IPv6 | h3 | ✅ 成功 | 210 | cloudflare | | 88 | wilson.ns.cloudfl... |
2803:f800:50::6ca2:c36e | IPv6 | h3 | ✅ 成功 | 210 | cloudflare | | 182 |
huxley.ns.cloudfl... | 2803:f800:50::6ca2:c3bc | IPv6 | h3 | ✅ 成功 | 210 |
cloudflare | | 73 | craig.ns.cloudfla... | 172.64.35.192 | IPv4 | h3 | ✅ 成功 |
211 | cloudflare | | 316 | 172.64.154.18 | 172.64.154.18 | IPv4 | h3 | ✅ 成功 |
213 | cloudflare | | 439 | 104.18.42.26 | 104.18.42.26 | IPv4 | h3 | ✅ 成功 |
213 | cloudflare | | 323 | kyree.ns.cloudfla... | 108.162.195.207 | IPv4 | h3 |
✅ 成功 | 214 | cloudflare | | 159 | 172.64.146.16 | 172.64.146.16 | IPv4 | h3 |
✅ 成功 | 215 | cloudflare | | 229 | dylan.ns.cloudfla... | 172.64.35.187 | IPv4
| h3 | ✅ 成功 | 215 | cloudflare | | 403 | sullivan.ns.cloud... |
162.159.44.161 | IPv4 | h3 | ✅ 成功 | 215 | cloudflare | | 24 | cf.877774.xyz |
104.18.41.190 | IPv4 | h3 | ✅ 成功 | 216 | cloudflare | | 181 |
huxley.ns.cloudfl... | 2606:4700:58::a29f:2cbc | IPv6 | h3 | ✅ 成功 | 217 |
cloudflare | | 404 | sullivan.ns.cloud... | 172.64.35.161 | IPv4 | h3 | ✅ 成功
| 217 | cloudflare | | 412 | uriah.ns.cloudfla... | 2606:4700:58::a29f:2cc2 |
IPv6 | h3 | ✅ 成功 | 217 | cloudflare | | 423 | ct.877774.xyz | 172.64.229.185
| IPv4 | h3 | ✅ 成功 | 217 | cloudflare | | 65 | www.wto.org | 104.18.41.190 |
IPv4 | h3 | ✅ 成功 | 218 | cloudflare | | 12 | damien.ns.cloudfl... |
162.159.44.168 | IPv4 | h3 | ✅ 成功 | 219 | cloudflare | | 147 |
braden.ns.cloudfl... | 2803:f800:50::6ca2:c3a9 | IPv6 | h3 | ✅ 成功 | 219 |
cloudflare | | 50 | julio.ns.cloudfla... | 2803:f800:50::6ca2:c3d1 | IPv6 | h3 |
✅ 成功 | 221 | cloudflare | | 406 | sullivan.ns.cloud... |
2803:f800:50::6ca2:c3a1 | IPv6 | h3 | ✅ 成功 | 221 | cloudflare | | 109 |
rustam.ns.cloudfl... | 2a06:98c1:50::ac40:2394 | IPv6 | h3 | ✅ 成功 | 222 |
cloudflare | | 87 | wilson.ns.cloudfl... | 2606:4700:58::a29f:2c6e | IPv6 | h3 |
✅ 成功 | 223 | cloudflare | | 114 | cris.ns.cloudflar... |
2803:f800:50::6ca2:c3ca | IPv6 | h3 | ✅ 成功 | 223 | cloudflare | | 200 |
172.64.148.15 | 172.64.148.15 | IPv4 | h3 | ✅ 成功 | 223 | cloudflare | | 258 |
decker.ns.cloudfl... | 2606:4700:58::a29f:2c9b | IPv6 | h3 | ✅ 成功 | 223 |
cloudflare | | 17 | abdullah.ns.cloud... | 108.162.195.203 | IPv4 | h3 | ✅ 成功
| 224 | cloudflare | | 23 | cf.877774.xyz | 172.64.146.66 | IPv4 | h3 | ✅ 成功
| 224 | cloudflare | | 49 | julio.ns.cloudfla... | 2606:4700:58::a29f:2cd1 |
IPv6 | h3 | ✅ 成功 | 224 | cloudflare | | 230 | dylan.ns.cloudfla... |
2606:4700:58::a29f:2cbb | IPv6 | h3 | ✅ 成功 | 224 | cloudflare | | 324 |
kyree.ns.cloudfla... | 162.159.44.207 | IPv4 | h3 | ✅ 成功 | 224 | cloudflare |
| 110 | cris.ns.cloudflar... | 108.162.195.202 | IPv4 | h3 | ✅ 成功 | 225 |
cloudflare | | 178 | huxley.ns.cloudfl... | 108.162.195.188 | IPv4 | h3 | ✅
成功 | 225 | cloudflare | | 325 | kyree.ns.cloudfla... | 172.64.35.207 | IPv4 |
h3 | ✅ 成功 | 225 | cloudflare | | 333 | bowen.ns.cloudfla... |
2803:f800:50::6ca2:c353 | IPv6 | h3 | ✅ 成功 | 225 | cloudflare | | 411 |
uriah.ns.cloudfla... | 172.64.35.194 | IPv4 | h3 | ✅ 成功 | 225 | cloudflare |
| 330 | bowen.ns.cloudfla... | 162.159.44.83 | IPv4 | h3 | ✅ 成功 | 226 |
cloudflare | | 48 | julio.ns.cloudfla... | 172.64.35.209 | IPv4 | h3 | ✅ 成功 |
227 | cloudflare | | 53 | 172.64.35.24 | 172.64.35.24 | IPv4 | h3 | ✅ 成功 |
227 | cloudflare | | 107 | rustam.ns.cloudfl... | 2606:4700:58::a29f:2c94 | IPv6
| h3 | ✅ 成功 | 227 | cloudflare | | 255 | decker.ns.cloudfl... |
108.162.195.155 | IPv4 | h3 | ✅ 成功 | 228 | cloudflare | | 259 |
decker.ns.cloudfl... | 2803:f800:50::6ca2:c39b | IPv6 | h3 | ✅ 成功 | 228 |
cloudflare | | 355 | 104.18.39.196 | 104.18.39.196 | IPv4 | h3 | ✅ 成功 | 228 |
cloudflare | | 71 | craig.ns.cloudfla... | 108.162.195.192 | IPv4 | h3 | ✅ 成功
| 229 | cloudflare | | 102 | ashton.ns.cloudfl... | 2a06:98c1:50::ac40:23ad |
IPv6 | h3 | ✅ 成功 | 229 | cloudflare | | 148 | braden.ns.cloudfl... |
2a06:98c1:50::ac40:23a9 | IPv6 | h3 | ✅ 成功 | 229 | cloudflare | | 219 |
na.877774.xyz | 104.18.38.235 | IPv4 | h3 | ✅ 成功 | 229 | cloudflare | | 479 |
trevor.ns.cloudfl... | 2606:4700:58::a29f:2c9a | IPv6 | h3 | ✅ 成功 | 229 |
cloudflare | | 13 | damien.ns.cloudfl... | 172.64.35.168 | IPv4 | h3 | ✅ 成功 |
230 | cloudflare | | 26 | cf.877774.xyz | 2a06:98c1:3102::6812:29be | IPv6 | h3
| ✅ 成功 | 230 | cloudflare | | 213 | [2606:4700:4409::... |
2606:4700:4409::5b5b:7758 | IPv6 | h3 | ✅ 成功 | 230 | cloudflare | | 354 |
[2606:4700:440b::... | 2606:4700:440b::3e6e:5f06 | IPv6 | h3 | ✅ 成功 | 230 |
cloudflare | | 97 | ashton.ns.cloudfl... | 108.162.195.173 | IPv4 | h3 | ✅ 成功
| 231 | cloudflare | | 46 | julio.ns.cloudfla... | 108.162.195.209 | IPv4 | h3 |
✅ 成功 | 232 | cloudflare | | 104 | rustam.ns.cloudfl... | 108.162.195.148 |
IPv4 | h3 | ✅ 成功 | 232 | cloudflare | | 113 | cris.ns.cloudflar... |
2606:4700:58::a29f:2cca | IPv6 | h3 | ✅ 成功 | 232 | cloudflare | | 166 |
172.64.151.55 | 172.64.151.55 | IPv4 | h3 | ✅ 成功 | 232 | cloudflare | | 329 |
bowen.ns.cloudfla... | 108.162.195.83 | IPv4 | h3 | ✅ 成功 | 232 | cloudflare |
| 418 | lewis.ns.cloudfla... | 2606:4700:58::a29f:2c9f | IPv6 | h3 | ✅ 成功 |
232 | cloudflare | | 19 | abdullah.ns.cloud... | 172.64.35.203 | IPv4 | h3 | ✅
成功 | 233 | cloudflare | | 115 | cris.ns.cloudflar... | 2a06:98c1:50::ac40:23ca
| IPv6 | h3 | ✅ 成功 | 233 | cloudflare | | 74 | craig.ns.cloudfla... |
2606:4700:58::a29f:2cc0 | IPv6 | h3 | ✅ 成功 | 234 | cloudflare | | 75 |
craig.ns.cloudfla... | 2803:f800:50::6ca2:c3c0 | IPv6 | h3 | ✅ 成功 | 234 |
cloudflare | | 405 | sullivan.ns.cloud... | 2606:4700:58::a29f:2ca1 | IPv6 | h3
| ✅ 成功 | 234 | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 27 条记录
- **慢 (200-500ms)**: 73 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 20 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 19 次失败
- **h2**: 1 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
