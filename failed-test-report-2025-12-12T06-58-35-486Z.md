# HTTP/3 连接测试失败报告

## 报告概要

- **生成时间**: 2025/12/12 14:58:35
- **数据来源**: connectivity_results-20251212-145425.json
- **总测试数**: 469
- **失败测试数**: 7
- **成功测试数**: 462
- **失败率**: 1.49%
- **平均延迟**: 774.06ms
- **最小延迟**: 136ms
- **最大延迟**: 5854ms

## 🌐 当前测试环境信息

- **获取时间**: 2025/12/12 14:58:35
- **IP地址**: 240e:b8e:35cd:a601:111d:b508:1568:acd5
- **国家/地区**: China (CN)
- **ASN**: AS4134
- **网络组织**: Chinanet
- **网络域名**: chinatelecom.com.cn
- **大洲**: Asia (AS)
- **地理坐标**: 34.7732, 113.722
- **时区**: Asia/Shanghai
- **数据源**: combined

---

---

## 失败测试详情

### 📊 错误类型统计

- **连接超时: I/O超时**: 7 次 (100.0%)

### 🔍 按错误类型分类的失败测试详情

#### 连接超时: I/O超时 (7 次测试)

| 序号 | 主机/域名                | 目标IP          | IP版本 | 协议 | 状态码 | 延迟(ms) | 服务器 | 错误信息                                  |
| ---- | ------------------------ | --------------- | ------ | ---- | ------ | -------- | ------ | ----------------------------------------- |
| 53   | trevor.ns.cloudflare.com | 108.162.195.154 | IPv4   | none | N/A    | 0        | N/A    | dial tcp 108.162.195.154:443: i/o timeout |
| 89   | 172.67.49.134            | 172.67.49.134   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.67.49.134:443: i/o timeout   |
| 239  | cfip.xxxxxxxx.tk         | 104.20.255.53   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 104.20.255.53:443: i/o timeout   |
| 247  | cfip.xxxxxxxx.tk         | 198.41.212.130  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 198.41.212.130:443: i/o timeout  |
| 285  | 141.147.185.63           | 141.147.185.63  | IPv4   | none | N/A    | 0        | N/A    | dial tcp 141.147.185.63:443: i/o timeout  |
| 468  | 172.64.201.25            | 172.64.201.25   | IPv4   | none | N/A    | 0        | N/A    | dial tcp 172.64.201.25:443: i/o timeout   |
| 469  | 3.0.50.69                | 3.0.50.69       | IPv4   | none | N/A    | 0        | N/A    | dial tcp 3.0.50.69:443: i/o timeout       |

### 📈 错误分析总结

#### 主错误类型分布

- **连接超时**: 7 次 (100.0%)

#### 错误模式分析

**超时集中度分析**: 共有 7 次超时，主要集中在IP段 108.162（1
次），可能存在网络路由问题或目标服务器负载过高

**协议协商分析**: 有 7 次失败是因为协议协商失败（protocol:
none），说明无法与目标建立HTTP/3或其他现代协议连接

**IP版本分析**:
所有失败的测试都使用IPv4，IPv6连接可能更稳定或目标服务器的IPv6配置更好

---

## 🚀 延迟最低的 100 条记录

以下显示了延迟最低的测试记录，包括成功和失败的测试：

| 序号 | 主机/域名                   | 目标IP                                | IP版本 | 协议 | 状态    | 延迟(ms) | 服务器     |
| ---- | --------------------------- | ------------------------------------- | ------ | ---- | ------- | -------- | ---------- |
| 394  | lewis.ns.cloudflare.com     | 162.159.44.159                        | IPv4   | h3   | ✅ 成功 | 136      | cloudflare |
| 181  | ct.877774.xyz               | 172.64.229.174                        | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 429  | damien.ns.cloudflare.com    | 162.159.44.168                        | IPv4   | h3   | ✅ 成功 | 137      | cloudflare |
| 257  | benedict.ns.cloudflare.com  | 162.159.44.205                        | IPv4   | h3   | ✅ 成功 | 138      | cloudflare |
| 6    | cris.ns.cloudflare.com      | 162.159.44.202                        | IPv4   | h3   | ✅ 成功 | 147      | cloudflare |
| 91   | pranab.ns.cloudflare.com    | 162.159.44.199                        | IPv4   | h3   | ✅ 成功 | 148      | cloudflare |
| 177  | ct.877774.xyz               | 172.64.229.236                        | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 180  | ct.877774.xyz               | 172.64.229.173                        | IPv4   | h3   | ✅ 成功 | 149      | cloudflare |
| 178  | ct.877774.xyz               | 172.64.229.44                         | IPv4   | h3   | ✅ 成功 | 151      | cloudflare |
| 182  | ct.877774.xyz               | 172.64.229.185                        | IPv4   | h3   | ✅ 成功 | 154      | cloudflare |
| 184  | ct.877774.xyz               | 172.64.229.217                        | IPv4   | h3   | ✅ 成功 | 156      | cloudflare |
| 105  | decker.ns.cloudflare.com    | 162.159.44.155                        | IPv4   | h3   | ✅ 成功 | 158      | cloudflare |
| 25   | huxley.ns.cloudflare.com    | 162.159.44.188                        | IPv4   | h3   | ✅ 成功 | 172      | cloudflare |
| 119  | dylan.ns.cloudflare.com     | 162.159.44.187                        | IPv4   | h3   | ✅ 成功 | 177      | cloudflare |
| 54   | trevor.ns.cloudflare.com    | 162.159.44.154                        | IPv4   | h3   | ✅ 成功 | 184      | cloudflare |
| 311  | uriah.ns.cloudflare.com     | 162.159.44.194                        | IPv4   | h3   | ✅ 成功 | 185      | cloudflare |
| 379  | otto.ns.cloudflare.com      | 162.159.44.135                        | IPv4   | h3   | ✅ 成功 | 186      | cloudflare |
| 169  | bowen.ns.cloudflare.com     | 162.159.44.83                         | IPv4   | h3   | ✅ 成功 | 188      | cloudflare |
| 186  | moura.ns.cloudflare.com     | 162.159.44.217                        | IPv4   | h3   | ✅ 成功 | 189      | cloudflare |
| 151  | braden.ns.cloudflare.com    | 162.159.44.169                        | IPv4   | h3   | ✅ 成功 | 191      | cloudflare |
| 183  | ct.877774.xyz               | 172.64.229.195                        | IPv4   | h3   | ✅ 成功 | 196      | cloudflare |
| 198  | rustam.ns.cloudflare.com    | 162.159.44.148                        | IPv4   | h3   | ✅ 成功 | 197      | cloudflare |
| 439  | 108.162.198.54              | 108.162.198.54                        | IPv4   | h3   | ✅ 成功 | 203      | cloudflare |
| 326  | julio.ns.cloudflare.com     | 162.159.44.209                        | IPv4   | h3   | ✅ 成功 | 208      | cloudflare |
| 296  | ashton.ns.cloudflare.com    | 2803:f800:50::6ca2:c3ad               | IPv6   | h3   | ✅ 成功 | 210      | cloudflare |
| 398  | lewis.ns.cloudflare.com     | 2a06:98c1:50::ac40:239f               | IPv6   | h3   | ✅ 成功 | 210      | cloudflare |
| 106  | decker.ns.cloudflare.com    | 172.64.35.155                         | IPv4   | h3   | ✅ 成功 | 212      | cloudflare |
| 108  | decker.ns.cloudflare.com    | 2803:f800:50::6ca2:c39b               | IPv6   | h3   | ✅ 成功 | 212      | cloudflare |
| 118  | dylan.ns.cloudflare.com     | 108.162.195.187                       | IPv4   | h3   | ✅ 成功 | 212      | cloudflare |
| 146  | kyree.ns.cloudflare.com     | 172.64.35.207                         | IPv4   | h3   | ✅ 成功 | 216      | cloudflare |
| 28   | huxley.ns.cloudflare.com    | 2803:f800:50::6ca2:c3bc               | IPv6   | h3   | ✅ 成功 | 218      | cloudflare |
| 152  | braden.ns.cloudflare.com    | 172.64.35.169                         | IPv4   | h3   | ✅ 成功 | 218      | cloudflare |
| 260  | benedict.ns.cloudflare.com  | 2803:f800:50::6ca2:c3cd               | IPv6   | h3   | ✅ 成功 | 218      | cloudflare |
| 166  | 172.64.151.55               | 172.64.151.55                         | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 380  | otto.ns.cloudflare.com      | 172.64.35.135                         | IPv4   | h3   | ✅ 成功 | 219      | cloudflare |
| 15   | wilson.ns.cloudflare.com    | 2803:f800:50::6ca2:c36e               | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 16   | wilson.ns.cloudflare.com    | 2a06:98c1:50::ac40:236e               | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 329  | julio.ns.cloudflare.com     | 2803:f800:50::6ca2:c3d1               | IPv6   | h3   | ✅ 成功 | 220      | cloudflare |
| 312  | uriah.ns.cloudflare.com     | 172.64.35.194                         | IPv4   | h3   | ✅ 成功 | 221      | cloudflare |
| 444  | www.wto.org                 | 2606:4700:4406::ac40:9242             | IPv6   | h3   | ✅ 成功 | 221      | cloudflare |
| 52   | sullivan.ns.cloudflare.com  | 2a06:98c1:50::ac40:23a1               | IPv6   | h3   | ✅ 成功 | 222      | cloudflare |
| 7    | cris.ns.cloudflare.com      | 172.64.35.202                         | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 293  | ashton.ns.cloudflare.com    | 162.159.44.173                        | IPv4   | h3   | ✅ 成功 | 224      | cloudflare |
| 295  | ashton.ns.cloudflare.com    | 2606:4700:58::a29f:2cad               | IPv6   | h3   | ✅ 成功 | 224      | cloudflare |
| 396  | lewis.ns.cloudflare.com     | 2606:4700:58::a29f:2c9f               | IPv6   | h3   | ✅ 成功 | 224      | cloudflare |
| 90   | pranab.ns.cloudflare.com    | 108.162.195.199                       | IPv4   | h3   | ✅ 成功 | 225      | cloudflare |
| 158  | bestcf.030101.xyz           | 2606:4700:58:7e7b:edf2:ee1f:c9fe:41e3 | IPv6   | h3   | ✅ 成功 | 225      | cloudflare |
| 338  | [2606:4700:440b::3e6e:5f06] | 2606:4700:440b::3e6e:5f06             | IPv6   | h3   | ✅ 成功 | 225      | cloudflare |
| 292  | ashton.ns.cloudflare.com    | 108.162.195.173                       | IPv4   | h3   | ✅ 成功 | 227      | cloudflare |
| 142  | cf.090227.xyz               | 2a06:98c1:3108::6812:2a62             | IPv6   | h3   | ✅ 成功 | 228      | cloudflare |
| 441  | www.wto.org                 | 104.18.41.190                         | IPv4   | h3   | ✅ 成功 | 228      | cloudflare |
| 58   | trevor.ns.cloudflare.com    | 2a06:98c1:50::ac40:239a               | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 100  | cf.zhetengsha.eu.org        | 2606:4700:4407::ac40:9052             | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 433  | damien.ns.cloudflare.com    | 2a06:98c1:50::ac40:23a8               | IPv6   | h3   | ✅ 成功 | 229      | cloudflare |
| 14   | wilson.ns.cloudflare.com    | 2606:4700:58::a29f:2c6e               | IPv6   | h3   | ✅ 成功 | 230      | cloudflare |
| 145  | kyree.ns.cloudflare.com     | 162.159.44.207                        | IPv4   | h3   | ✅ 成功 | 230      | cloudflare |
| 26   | huxley.ns.cloudflare.com    | 172.64.35.188                         | IPv4   | h3   | ✅ 成功 | 231      | cloudflare |
| 56   | trevor.ns.cloudflare.com    | 2606:4700:58::a29f:2c9a               | IPv6   | h3   | ✅ 成功 | 231      | cloudflare |
| 109  | decker.ns.cloudflare.com    | 2a06:98c1:50::ac40:239b               | IPv6   | h3   | ✅ 成功 | 231      | cloudflare |
| 12   | wilson.ns.cloudflare.com    | 162.159.44.110                        | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 189  | moura.ns.cloudflare.com     | 2803:f800:50::6ca2:c3d9               | IPv6   | h3   | ✅ 成功 | 232      | cloudflare |
| 395  | lewis.ns.cloudflare.com     | 172.64.35.159                         | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 442  | www.wto.org                 | 172.64.146.66                         | IPv4   | h3   | ✅ 成功 | 232      | cloudflare |
| 68   | craig.ns.cloudflare.com     | 2803:f800:50::6ca2:c3c0               | IPv6   | h3   | ✅ 成功 | 233      | cloudflare |
| 99   | cf.zhetengsha.eu.org        | 104.18.42.98                          | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 316  | 104.18.42.26                | 104.18.42.26                          | IPv4   | h3   | ✅ 成功 | 233      | cloudflare |
| 50   | sullivan.ns.cloudflare.com  | 2606:4700:58::a29f:2ca1               | IPv6   | h3   | ✅ 成功 | 234      | cloudflare |
| 55   | trevor.ns.cloudflare.com    | 172.64.35.154                         | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 160  | saas.sin.fan                | 162.159.36.20                         | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 168  | bowen.ns.cloudflare.com     | 108.162.195.83                        | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 258  | benedict.ns.cloudflare.com  | 172.64.35.205                         | IPv4   | h3   | ✅ 成功 | 234      | cloudflare |
| 144  | kyree.ns.cloudflare.com     | 108.162.195.207                       | IPv4   | h3   | ✅ 成功 | 235      | cloudflare |
| 121  | dylan.ns.cloudflare.com     | 2606:4700:58::a29f:2cbb               | IPv6   | h3   | ✅ 成功 | 236      | cloudflare |
| 126  | na.877774.xyz               | 104.18.38.235                         | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 150  | braden.ns.cloudflare.com    | 108.162.195.169                       | IPv4   | h3   | ✅ 成功 | 236      | cloudflare |
| 148  | kyree.ns.cloudflare.com     | 2803:f800:50::6ca2:c3cf               | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 200  | rustam.ns.cloudflare.com    | 2606:4700:58::a29f:2c94               | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 397  | lewis.ns.cloudflare.com     | 2803:f800:50::6ca2:c39f               | IPv6   | h3   | ✅ 成功 | 237      | cloudflare |
| 139  | [2606:4700:4409::5b5b:7758] | 2606:4700:4409::5b5b:7758             | IPv6   | h3   | ✅ 成功 | 239      | cloudflare |
| 161  | saas.sin.fan                | 162.159.36.5                          | IPv4   | h3   | ✅ 成功 | 239      | cloudflare |
| 92   | pranab.ns.cloudflare.com    | 172.64.35.199                         | IPv4   | h3   | ✅ 成功 | 240      | cloudflare |
| 313  | uriah.ns.cloudflare.com     | 2606:4700:58::a29f:2cc2               | IPv6   | h3   | ✅ 成功 | 240      | cloudflare |
| 341  | cf.877774.xyz               | 104.18.41.190                         | IPv4   | h3   | ✅ 成功 | 240      | cloudflare |
| 64   | craig.ns.cloudflare.com     | 108.162.195.192                       | IPv4   | h3   | ✅ 成功 | 241      | cloudflare |
| 342  | cf.877774.xyz               | 2a06:98c1:3102::6812:29be             | IPv6   | h3   | ✅ 成功 | 241      | cloudflare |
| 154  | braden.ns.cloudflare.com    | 2803:f800:50::6ca2:c3a9               | IPv6   | h3   | ✅ 成功 | 242      | cloudflare |
| 190  | moura.ns.cloudflare.com     | 2a06:98c1:50::ac40:23d9               | IPv6   | h3   | ✅ 成功 | 242      | cloudflare |
| 310  | uriah.ns.cloudflare.com     | 108.162.195.194                       | IPv4   | h3   | ✅ 成功 | 242      | cloudflare |
| 343  | cf.877774.xyz               | 2606:4700:4406::ac40:9242             | IPv6   | h3   | ✅ 成功 | 242      | cloudflare |
| 27   | huxley.ns.cloudflare.com    | 2606:4700:58::a29f:2cbc               | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 98   | cf.zhetengsha.eu.org        | 172.64.145.158                        | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 340  | cf.877774.xyz               | 172.64.146.66                         | IPv4   | h3   | ✅ 成功 | 243      | cloudflare |
| 381  | otto.ns.cloudflare.com      | 2606:4700:58::a29f:2c87               | IPv6   | h3   | ✅ 成功 | 243      | cloudflare |
| 185  | moura.ns.cloudflare.com     | 108.162.195.217                       | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 378  | otto.ns.cloudflare.com      | 108.162.195.135                       | IPv4   | h3   | ✅ 成功 | 244      | cloudflare |
| 149  | kyree.ns.cloudflare.com     | 2a06:98c1:50::ac40:23cf               | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 256  | benedict.ns.cloudflare.com  | 108.162.195.205                       | IPv4   | h3   | ✅ 成功 | 245      | cloudflare |
| 357  | [2606:4700:4408::18c5:3304] | 2606:4700:4408::18c5:3304             | IPv6   | h3   | ✅ 成功 | 245      | cloudflare |
| 170  | bowen.ns.cloudflare.com     | 172.64.35.83                          | IPv4   | h3   | ✅ 成功 | 246      | cloudflare |
| 187  | moura.ns.cloudflare.com     | 172.64.35.217                         | IPv4   | h3   | ✅ 成功 | 246      | cloudflare |

### 延迟分布分析

- **超快 (<50ms)**: 0 条记录
- **快 (50-100ms)**: 0 条记录
- **正常 (100-200ms)**: 22 条记录
- **慢 (200-500ms)**: 78 条记录
- **很慢 (>500ms)**: 0 条记录

---

## 详细分析

### 按IP版本统计

- **IPv4 失败**: 7 次
- **IPv6 失败**: 0 次

### 按协议统计

- **none**: 7 次失败

---

## 建议和后续操作

1. **检查网络连接**: 确认网络连接稳定
2. **验证DNS解析**: 检查DNS服务器是否正常工作
3. **检查防火墙设置**: 确认防火墙没有阻止相关端口
4. **联系服务提供商**: 如果失败率较高，可能需要联系网络服务提供商
5. **重新运行测试**: 在网络条件改善后重新运行测试进行验证

---

_此报告由 HTTP/3 连接测试报告生成器自动生成_
