# nat64.xyz 免费 NAT64/DNS64 服务列表

如果你想让自己的服务被**添加**、更新或移除，请直接在项目的
[GitHub 仓库](https://github.com/level66network/nat64.xyz) 中提交修改。\
非常感谢所有提供免费服务的运营方！同时感谢
[@treysis](https://twitter.com/treysis) 整理了最初的服务列表。

如果你对各家服务的可用性和在线率感兴趣，可以访问由
[@unixfox](https://twitter.com/unixf0x) 搭建的
[状态监控页面](https://stats.uptimerobot.com/GQ5RyTJLKZ)。

---

## 公共 NAT64 / DNS64 服务列表

> 下表列出了目前在 nat64.xyz 上公开的 NAT64/DNS64
> 服务提供者及其对应的地址信息，方便你在 IPv6-only 环境中访问 IPv4 资源。

### Kasper Dupont

#### Germany / Nürnberg

- 提供方： [Kasper Dupont](https://nat64.net/public-providers)
- DNS64 服务器：
  - `2a00:1098:2b::1`
  - `2a00:1098:2c::1`
  - `2a01:4f8:c2c:123f::1`
- NAT64 前缀：
  - `2a00:1098:2b::/96`
  - `2a00:1098:2c:1::/96`
  - `2a01:4f8:c2c:123f:64::/96`
  - `2a01:4f9:c010:3f02:64::/96`
- DoT（DNS over TLS）：
  - `dot.nat64.dk`

---

#### United Kingdom / London

- 提供方： [Kasper Dupont](https://nat64.net/public-providers)
- DNS64 服务器：
  - `2a00:1098:2b::1`
  - `2a00:1098:2c::1`
  - `2a01:4f8:c2c:123f::1`
- NAT64 前缀：
  - `2a00:1098:2b::/96`
  - `2a00:1098:2c:1::/96`
  - `2a01:4f8:c2c:123f:64::/96`
  - `2a01:4f9:c010:3f02:64::/96`
- DoT：
  - `dot.nat64.dk`

---

#### Finland / Helsinki

- 提供方： [Kasper Dupont](https://nat64.net/public-providers)
- DNS64 服务器：
  - `2a00:1098:2b::1`
  - `2a00:1098:2c::1`
  - `2a01:4f8:c2c:123f::1`
- NAT64 前缀：
  - `2a00:1098:2b::/96`
  - `2a00:1098:2c:1::/96`
  - `2a01:4f8:c2c:123f:64::/96`
  - `2a01:4f9:c010:3f02:64::/96`
- DoT：
  - `dot.nat64.dk`

---

### level66.services

#### Germany / Anycast

- 提供方：
  [level66.services – NAT64 Gateway](https://level66.services/services/nat64/)
- DNS64 服务器：
  - `2001:67c:2960::64`
  - `2001:67c:2960::6464`
- NAT64 前缀：
  - `2001:67c:2960:6464::/96`

---

### Trex

#### Finland / Tampere

- 提供方：Trex
- DNS64 服务器：
  - `2001:67c:2b0::4`
  - `2001:67c:2b0::6`
- NAT64 前缀：
  - `2001:67c:2b0:db32:0:1::/96`

---

### ZTVI

#### U.S.A. / Fremont

- 提供方： [ZTVI](https://www.ztvi.org/)
- DNS64 服务器：
  - `dns64.fm2.ztvi.org`
  - `2602:fc59:b0:9e::64`
- NAT64 前缀：
  - `2602:fc59:b0:64::/96`

---

#### U.S.A. / Chicago

- 提供方： [ZTVI](https://www.ztvi.org/)
- DNS64 服务器：
  - `dns64.cmi.ztvi.org`
  - `2602:fc59:11:1::64`
- NAT64 前缀：
  - `2602:fc59:11:64::/96`

---

## 关于 nat64.xyz 仓库

- 项目地址：<https://github.com/level66network/nat64.xyz>
- 描述：用于生成和托管 [nat64.xyz](https://nat64.xyz/) 网站内容的 Git 仓库。
- 技术栈：
  - 使用 Hugo 生成静态网站
  - 配置文件：`hugo.toml`
  - 内容目录：`content/`
  - 主题目录：`themes/nat64.xyz/`

如果你运营公共 NAT64 / DNS64 服务，欢迎通过 Pull Request
将你的服务信息添加到列表中，造福更多仅有 IPv6 连接的用户与实验环境。
