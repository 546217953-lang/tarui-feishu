# 飞书项目管理客户端 - 打包指南

## 📋 重要说明

**Tauri 打包需要在目标操作系统上进行编译**，即：
- Windows 安装包 → 需要在 Windows 上编译
- macOS 安装包 → 需要在 macOS 上编译

这是因为 Tauri 使用系统原生的 WebView 和打包工具。

---

## 🪟 Windows 打包指南

### 1. 环境准备

```powershell
# 1. 安装 Node.js (v18+)
# 下载：https://nodejs.org/

# 2. 安装 Rust
# 下载：https://win.rustup.rs/x86_64
# 或使用 PowerShell:
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe -y

# 3. 安装 Visual Studio Build Tools
# 下载：https://visualstudio.microsoft.com/visual-cpp-build-tools/
# 选择 "C++ build tools" + "Windows 10 SDK"

# 4. 安装 WebView2 (Windows 10 1803+ 已自带)
# 如需要：https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

### 2. 克隆项目

```powershell
cd C:\Projects
git clone <项目地址> feishu-project-client
cd feishu-project-client
```

### 3. 安装依赖

```powershell
npm install
```

### 4. 构建应用

```powershell
# 构建 Windows 安装包
npm run build
```

### 5. 获取安装包

构建完成后，安装包位于：

```
src-tauri\target\release\bundle\
├── msi\
│   └── 飞书项目管理_1.0.0_x64_en-US.msi
└── nsis\
    └── 飞书项目管理_1.0.0_x64-setup.exe
```

**推荐使用 `.exe` 安装包**，兼容性更好。

---

## 🍎 macOS 打包指南

### 1. 环境准备

```bash
# 1. 安装 Xcode Command Line Tools
xcode-select --install

# 2. 安装 Node.js (v18+)
brew install node

# 3. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 4. 验证安装
node --version
rustc --version
cargo --version
```

### 2. 克隆项目

```bash
cd ~/Projects
git clone <项目地址> feishu-project-client
cd feishu-project-client
```

### 3. 安装依赖

```bash
npm install
```

### 4. 构建应用

```bash
# Intel Mac
npm run build:macos

# Apple Silicon (M1/M2/M3)
npm run build:macos-arm

# 或构建当前架构
npm run build
```

### 5. 获取安装包

构建完成后，安装包位于：

```
src-tauri/target/release/bundle/
├── dmg/
│   └── 飞书项目管理_1.0.0_x64.dmg (或 aarch64.dmg)
└── app/
    └── 飞书项目管理.app
```

**推荐使用 `.dmg` 安装包**，方便分发。

### 6. 代码签名（可选，用于分发）

```bash
# 查看可用的签名身份
security find-identity -v -p codesigning

# 签名应用
codesign --force --deep --sign "Your Developer ID" src-tauri/target/release/bundle/app/飞书项目管理.app

# 创建 DMG
# Tauri 会自动生成，如需自定义可手动创建
```

---

## 🐧 Linux 打包指南（可选）

### 1. 环境准备

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev

# Arch Linux
sudo pacman -S webkit2gtk base-devel curl wget file openssl appindicator-gtk3 librsvg libvips

# Fedora
sudo dnf install webkit2gtk4.1-devel openssl-devel curl wget file libxdo-devel \
  libappindicator-gtk3-devel librsvg2-devel
```

### 2. 构建

```bash
npm install
npm run build
```

### 3. 输出

```
src-tauri/target/release/bundle/
├── deb/
│   └── feishu-project-client_1.0.0_amd64.deb
└── appimage/
    └── 飞书项目管理_1.0.0_amd64.AppImage
```

---

## 📦 安装包大小参考

| 平台 | 格式 | 大小 |
|-----|------|------|
| Windows | .exe | ~5-8 MB |
| Windows | .msi | ~5-8 MB |
| macOS | .dmg | ~8-12 MB |
| macOS | .app | ~15-20 MB |
| Linux | .deb | ~10-15 MB |
| Linux | .AppImage | ~15-20 MB |

---

## ✅ 测试清单

打包完成后，请测试：

- [ ] 应用能正常启动
- [ ] 飞书项目管理页面能正常加载
- [ ] 登录功能正常
- [ ] 窗口可以正常缩放
- [ ] 全屏功能正常（双击）
- [ ] 关闭应用后重新打开能记住登录状态

---

## 🔧 常见问题

### Q: 构建时提示找不到 Rust
A: 确保 Rust 已正确安装并添加到 PATH，重启终端后重试。

### Q: Windows 构建失败
A: 确保安装了 Visual Studio Build Tools 和 Windows SDK。

### Q: macOS 构建后无法打开
A: 右键点击应用 → 打开 → 仍要打开（首次需要绕过 Gatekeeper）。

### Q: 飞书页面加载失败
A: 检查网络连接，确保 CSP 配置包含飞书域名。

---

**前端 W** 💻 | 2026-03-10
