# 飞书项目管理桌面客户端

基于 Tauri 2.0 构建的飞书项目管理桌面应用，支持 Windows 和 macOS 系统。

## 📦 项目结构

```
feishu-project-client/
├── src/                    # 前端代码
│   └── index.html         # 主页面 (加载飞书项目管理)
├── src-tauri/             # Rust 后端
│   ├── src/
│   │   └── lib.rs        # Rust 入口
│   ├── icons/            # 应用图标
│   ├── Cargo.toml        # Rust 依赖
│   └── tauri.conf.json   # Tauri 配置
└── package.json          # Node.js 配置
```

## 🚀 快速开始

### 环境要求

**必须安装：**

1. **Node.js** (v18+)
   ```bash
   node --version
   ```

2. **Rust** (最新稳定版)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **系统依赖**

   **Windows:**
   - Microsoft Visual Studio C++ Build Tools
   - WebView2 (Windows 10 1803+ 自带)

   **macOS:**
   - Xcode Command Line Tools
   ```bash
   xcode-select --install
   ```

   **Linux:**
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
     libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
   ```

### 开发模式

```bash
# 安装依赖
npm install

# 启动开发服务器
npm run dev
```

### 构建生产版本

```bash
# 构建当前平台
npm run build

# 构建 Windows 版本
npm run build:windows

# 构建 macOS Intel 版本
npm run build:macos

# 构建 macOS ARM 版本 (M1/M2)
npm run build:macos-arm
```

## 📁 输出文件

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── msi/          # Windows 安装包 (.msi)
├── nsis/         # Windows 安装包 (.exe)
├── dmg/          # macOS 安装包 (.dmg)
└── app/          # macOS 应用 (.app)
```

## ⚙️ 配置说明

### 应用信息

在 `src-tauri/tauri.conf.json` 中配置：

```json
{
  "productName": "飞书项目管理",
  "version": "1.0.0",
  "identifier": "com.feishu.project-client"
}
```

### 窗口设置

```json
{
  "windows": [{
    "title": "飞书项目管理",
    "width": 1400,
    "height": 900,
    "minWidth": 1024,
    "minHeight": 768,
    "center": true,
    "resizable": true
  }]
}
```

### CSP 配置

已配置飞书域名白名单，确保飞书项目管理正常加载。

## 🎯 功能特性

- ✅ 加载飞书项目管理 (https://project.feishu.cn/)
- ✅ 支持 Windows 和 macOS
- ✅ 原生窗口体验
- ✅ 全屏支持 (双击切换)
- ✅ 加载动画
- ✅ 安全 CSP 配置

## 📝 注意事项

1. **跨平台打包**
   - Windows 包需要在 Windows 上编译
   - macOS 包需要在 macOS 上编译
   - 无法在一个平台上编译另一个平台的包

2. **代码签名 (macOS)**
   - 分发 macOS 应用需要 Apple Developer 证书
   - 开发测试可以跳过签名

3. **飞书登录**
   - 首次使用需要登录飞书账号
   - 登录状态会保存在本地

## 🛠️ 故障排除

### 构建失败

```bash
# 清理缓存
rm -rf src-tauri/target

# 重新安装依赖
npm install

# 重新构建
npm run build
```

### Rust 编译错误

```bash
# 更新 Rust
rustup update

# 检查 Rust 安装
rustc --version
cargo --version
```

### WebView 问题

如果飞书页面无法加载，检查：
- 网络连接
- CSP 配置是否包含飞书域名
- 浏览器控制台错误

## 📄 许可证

MIT License

---

**前端 W** 💻 | 2026-03-10
