# 自动更新使用指南

## 概述

Sea Lantern 现在支持自动更新功能，使用 Tauri 官方的 Updater 插件实现。

## 工作原理

1. 应用启动时或用户点击"检查更新"按钮
2. 从配置的 URL 获取 `updater.json` 文件
3. 比较版本号，如果有新版本则提示用户
4. 用户点击"下载并安装"后自动下载安装包
5. 下载完成后提示用户重启应用
6. 重启后自动安装新版本

## 发布新版本的步骤

### 1. 更新版本号

修改 `src/utils/version.ts` 中的 `APP_VERSION`


### 2. 构建安装包

```bash
npm run tauri build
```

构建完成后，安装包位于：
- `src-tauri/target/release/bundle/nsis/Sea-Lantern_版本号_x64-setup.exe`

### 3. 创建 GitHub/Gitee Release

1. 在 Gitee 创建新的 Release
2. 标签名格式：`v0.2.0`（必须以 v 开头）
3. 上传构建好的安装包

### 4. 创建更新清单文件

复制 `updater.json.template` 为 `updater.json`，修改内容：

```json
{
  "version": "0.2.0",
  "notes": "更新内容：\n- 新功能1\n- 修复bug2",
  "pub_date": "2026-02-12T12:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "",
      "url": "https://gitee.com/fps_z/SeaLantern/releases/download/v0.2.0/Sea-Lantern_0.2.0_x64-setup.exe"
    }
  }
}
```

**重要字段说明：**
- `version`: 新版本号（不带 v 前缀）
- `notes`: 更新说明（支持换行符 \n）
- `pub_date`: 发布日期（ISO 8601 格式）
- `url`: 安装包的直接下载链接
- `signature`: 签名（不签名时留空字符串）

### 5. 托管更新清单

将 `updater.json` 文件上传到 Gitee 仓库的 `main` 分支根目录。

**访问地址：**
```
https://gitee.com/fps_z/SeaLantern/raw/main/updater.json
```

这个地址已经配置在 `tauri.conf.json` 的 `plugins.updater.endpoints` 中。

## 关于代码签名

当前配置为**不签名**模式（`pubkey` 为空字符串）。

**优点：**
- 免费，无需购买证书
- 配置简单

**缺点：**
- Windows 会显示"未知发布者"警告
- 用户需要点击"仍要运行"才能安装

**如果需要签名：**
1. 购买代码签名证书（约 $100-300/年）
2. 使用 Tauri CLI 生成密钥对
3. 在构建时签名安装包
4. 将公钥填入 `tauri.conf.json` 的 `pubkey` 字段
5. 将签名填入 `updater.json` 的 `signature` 字段

## 测试更新功能

1. 确保 `updater.json` 已上传到 Gitee
2. 在应用中点击"检查更新"
3. 如果版本号正确，应该能检测到更新
4. 点击"下载并安装"测试下载流程

## 常见问题

### Q: 检查更新失败？
A: 检查网络连接，确认 `updater.json` URL 可以访问

### Q: 下载失败？
A: 确认安装包 URL 正确，文件可以直接下载

### Q: 更新后版本号没变？
A: 确认所有配置文件的版本号都已更新

### Q: Windows 提示"未知发布者"？
A: 这是正常的，因为没有代码签名。用户可以选择"仍要运行"继续安装。
