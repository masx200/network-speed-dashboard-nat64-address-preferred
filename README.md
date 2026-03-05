# 网络测速结果仪表板-cloudflare优选ip地址

好的，这是一个为你生成的 `README.md`
文件，详细介绍了项目的功能、技术栈以及如何运行和使用。

---

![Vue](https://img.shields.io/badge/Vue-3-green.svg)
![Vite](https://img.shields.io/badge/Vite-4.0+-blue.svg)
![Vuetify](https://img.shields.io/badge/Vuetify-3.0+-blueviolet.svg)

这是一个基于 Vue 3 和 Vuetify
的前端项目，用于可视化和分析网络连接测速结果。它可以动态加载多个 JSON
报告文件，并提供强大的排序和筛选功能，帮助用户快速分析网络性能。

## 功能特性

- **动态数据加载**: 使用 Vite 的 `import.meta.glob` 功能，自动读取并加载
  `public` 目录下所有符合 `failed-test-report-*.json` 命名格式的报告文件。
- **综合数据展示**: 在一个统一的表格中展示所有成功和失败的测试记录。
- **多维度排序**:
  支持按任意列进行排序，默认按网络延迟（`latency_ms`）升序排列，方便快速找到最优节点。
- **高级筛选**: 提供丰富的筛选选项，可以根据以下条件过滤结果：
  - 测试环境国家
  - 自治系统号 (ASN)
  - IP 版本
  - 网络协议
  - 测试成功/失败状态
- **统计概览**: 在页面顶部直观展示总测试数、成功数、失败数和平均延迟等关键指标。
- **响应式设计**: 基于 Vuetify 的 Material Design
  组件，提供在不同设备上都有良好体验的响应式布局。
- **视觉化延迟**:
  使用不同颜色的标签来区分延迟等级（绿色=快，红色=慢），使结果一目了然。

## 技术栈

- **前端框架**: [Vue 3](https://vuejs.org/) (Composition API)
- **构建工具**: [Vite](https://vitejs.dev/)
- **UI 组件库**: [Vuetify 3](https://vuetifyjs.com/)
- **数据加载**: Vite `import.meta.glob`

## 快速开始

### 前置要求

- [Node.js](https://nodejs.org/) (推荐 v16+)
- [npm](https://www.npmjs.com/) 或 [yarn](https://yarnpkg.com/)

### 安装和运行

1. **克隆项目**

   ```bash
   git clone <your-repository-url>
   cd network-speed-dashboard-nat64-address-preferred
   ```

2. **安装依赖**

   ```bash
   npm install
   ```

3. **启动开发服务器**

   ```bash
   npm run dev
   ```

4. **访问应用** 打开浏览器访问 `http://localhost:5173` (Vite
   默认端口)，应用会自动打开。

## 项目结构

```
network-speed-dashboard-nat64-address-preferred/
├── public/                  # 静态资源
│   └── failed-test-report-*.json  # 测速报告文件存放于此
├── src/
│   ├── components/
│   │   └── SpeedDashboard.vue  # 主要的仪表板组件
│   ├── plugins/
│   │   └── vuetify.js      # Vuetify 配置和插件设置
│   ├── App.vue             # 根组件
│   └── main.js             # 应用入口文件
├── index.html              # HTML 模板
├── package.json            # 项目依赖和脚本
└── vite.config.js          # Vite 配置文件
```

## 数据格式说明

为了让应用能够正确读取和解析数据，请将你的测速结果 JSON 文件放置在 `public`
目录下，并遵循 `failed-test-report-*.json` 的命名格式（例如
`failed-test-report-20231212-154327.json`）。

每个 JSON 文件应包含以下关键结构：

```json
{
  "report_info": { ... },
  "latency_statistics": { ... },
  "error_analysis": { ... },
  "statistics": { ... },
  "failed_tests": [
    // 失败的测试记录数组
    {
      "index": 141,
      "host": "cfip.xxxxxxxx.tk",
      "target_ip": "198.41.212.130",
      "ip_version": "IPv4",
      "protocol": "none",
      "status_code": null,
      "latency_ms": 0,
      "server_header": "N/A",
      "error_msg": "dial tcp ...",
      "timestamp": "2025-12-12T07:43:34.062Z",
      "success": false
    }
  ],
  "top_latency_records": [
    // 成功的（或延迟最低的）测试记录数组
    {
      "index": 4,
      "host": "ct.877774.xyz",
      "target_ip": "172.64.229.174",
      "ip_version": "IPv4",
      "protocol": "h3",
      "status_code": 200,
      "latency_ms": 146,
      "server_header": "cloudflare",
      "error_msg": "No error message",
      "timestamp": "2025-12-12T07:43:34.062Z",
      "success": true
    }
  ],
  "test_environment": {
    "timestamp": "2025-12-12T07:43:34.075Z",
    "ip_info": {
      "ip": "2409:891f:8223:a9e6:b8fa:1abc:1a83:143b",
      "asn": "AS24400",
      "as_name": "Shanghai Mobile Communications Co.,Ltd.",
      "country": "China"
      // ... 其他 IP 信息
    }
  }
}
```

应用主要读取 `failed_tests`、`top_latency_records` 和 `test_environment`
这三个部分来生成表格和筛选器。

## 如何使用

1. **筛选数据**:
   - 在页面顶部的筛选卡片中，使用下拉菜单选择国家、ASN、IP版本或协议。
   - 使用复选框控制是否显示成功或失败的测试。
   - 点击 "清除筛选" 按钮可以重置所有筛选条件。

2. **查看统计**:
   - 筛选卡片下方的四个统计卡片会实时显示当前筛选结果下的总测试数、成功数、失败数和平均延迟。

3. **排序和搜索**:
   - 在结果表格中，点击任意列的标题可以按该列进行升序或降序排序。
   - 使用表格右上角的搜索框可以按主机名、IP 地址等关键词进行全文搜索。

## 构建生产版本

当你准备好将应用部署到生产环境时，可以运行以下命令：

```bash
npm run build
```

这会在项目根目录下生成一个 `dist`
文件夹，里面包含了所有优化和压缩后的静态文件，你可以将它们部署到任何静态文件服务器上。

## 许可证

[MIT](LICENSE)

# Vuetify (Default)

This is the official scaffolding tool for Vuetify, designed to give you a head
start in building your new Vuetify application. It sets up a base template with
all the necessary configurations and standard directory structure, enabling you
to begin development without the hassle of setting up the project from scratch.

## ❗️ Important Links

- 📄 [Docs](https://vuetifyjs.com/)
- 🚨 [Issues](https://issues.vuetifyjs.com/)
- 🏬 [Store](https://store.vuetifyjs.com/)
- 🎮 [Playground](https://play.vuetifyjs.com/)
- 💬 [Discord](https://community.vuetifyjs.com)

## 💿 Install

Set up your project using your preferred package manager. Use the corresponding
command to install the dependencies:

| Package Manager                                           | Command        |
| --------------------------------------------------------- | -------------- |
| [yarn](https://yarnpkg.com/getting-started)               | `yarn install` |
| [npm](https://docs.npmjs.com/cli/v7/commands/npm-install) | `npm install`  |
| [pnpm](https://pnpm.io/installation)                      | `pnpm install` |
| [bun](https://bun.sh/#getting-started)                    | `bun install`  |

After completing the installation, your environment is ready for Vuetify
development.

## ✨ Features

- 🖼️ **Optimized Front-End Stack**: Leverage the latest Vue 3 and Vuetify 3 for
  a modern, reactive UI development experience. [Vue 3](https://v3.vuejs.org/) |
  [Vuetify 3](https://vuetifyjs.com/en/)
- 🗃️ **State Management**: Integrated with [Pinia](https://pinia.vuejs.org/),
  the intuitive, modular state management solution for Vue.
- 🚦 **Routing and Layouts**: Utilizes Vue Router for SPA navigation and
  vite-plugin-vue-layouts-next for organizing Vue file layouts.
  [Vue Router](https://router.vuejs.org/) |
  [vite-plugin-vue-layouts-next](https://github.com/loicduong/vite-plugin-vue-layouts-next)
- 💻 **Enhanced Development Experience**: Benefit from TypeScript's static type
  checking and the ESLint plugin suite for Vue, ensuring code quality and
  consistency. [TypeScript](https://www.typescriptlang.org/) |
  [ESLint Plugin Vue](https://eslint.vuejs.org/)
- ⚡ **Next-Gen Tooling**: Powered by Vite, experience fast cold starts and
  instant HMR (Hot Module Replacement). [Vite](https://vitejs.dev/)
- 🧩 **Automated Component Importing**: Streamline your workflow with
  unplugin-vue-components, automatically importing components as you use them.
  [unplugin-vue-components](https://github.com/antfu/unplugin-vue-components)
- 🛠️ **Strongly-Typed Vue**: Use vue-tsc for type-checking your Vue components,
  and enjoy a robust development experience.
  [vue-tsc](https://github.com/johnsoncodehk/volar/tree/master/packages/vue-tsc)

These features are curated to provide a seamless development experience from
setup to deployment, ensuring that your Vuetify application is both powerful and
maintainable.

## 💡 Usage

This section covers how to start the development server and build your project
for production.

### Starting the Development Server

To start the development server with hot-reload, run the following command. The
server will be accessible at [http://localhost:3000](http://localhost:3000):

```bash
yarn dev
```

(Repeat for npm, pnpm, and bun with respective commands.)

> Add NODE_OPTIONS='--no-warnings' to suppress the JSON import warnings that
> happen as part of the Vuetify import mapping. If you are on Node
> [v21.3.0](https://nodejs.org/en/blog/release/v21.3.0) or higher, you can
> change this to NODE_OPTIONS='--disable-warning=5401'. If you don't mind the
> warning, you can remove this from your package.json dev script.

### Building for Production

To build your project for production, use:

```bash
yarn build
```

(Repeat for npm, pnpm, and bun with respective commands.)

Once the build process is completed, your application will be ready for
deployment in a production environment.

## 💪 Support Vuetify Development

This project is built with [Vuetify](https://vuetifyjs.com/en/), a UI Library
with a comprehensive collection of Vue components. Vuetify is an MIT licensed
Open Source project that has been made possible due to the generous
contributions by our
[sponsors and backers](https://vuetifyjs.com/introduction/sponsors-and-backers/).
If you are interested in supporting this project, please consider:

- [Requesting Enterprise Support](https://support.vuetifyjs.com/)
- [Sponsoring John on Github](https://github.com/users/johnleider/sponsorship)
- [Sponsoring Kael on Github](https://github.com/users/kaelwd/sponsorship)
- [Supporting the team on Open Collective](https://opencollective.com/vuetify)
- [Becoming a sponsor on Patreon](https://www.patreon.com/vuetify)
- [Becoming a subscriber on Tidelift](https://tidelift.com/subscription/npm/vuetify)
- [Making a one-time donation with Paypal](https://paypal.me/vuetify)

## 📑 License

[MIT](http://opensource.org/licenses/MIT)

Copyright (c) 2016-present Vuetify, LLC
