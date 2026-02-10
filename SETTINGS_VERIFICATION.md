# 设置功能验证文档

## 所有设置项及其使用情况

### ✅ 1. close_servers_on_exit (关闭软件时停止所有服务器)
**位置**: `src-tauri/src/lib.rs:70`
**功能**: 窗口关闭事件中检查此设置，如果为 true 则调用 `stop_all_servers()`
**测试方法**: 
- 启动一个服务器
- 开启此设置
- 关闭应用
- 应该看到服务器自动停止

### ✅ 2. auto_accept_eula (自动同意 EULA)
**位置**: `src-tauri/src/services/server_manager.rs:94-97`
**功能**: 服务器启动前自动创建 eula.txt 并写入 `eula=true`
**测试方法**:
- 开启此设置
- 启动一个新服务器
- 检查服务器目录下的 eula.txt 文件

### ✅ 3. default_max_memory (默认最大内存)
**位置**: `src/views/CreateServerView.vue:10`
**功能**: 创建服务器页面自动填充此值
**测试方法**:
- 在设置中修改为 4096
- 进入创建服务器页面
- 应该看到最大内存默认显示 4096MB

### ✅ 4. default_min_memory (默认最小内存)
**位置**: `src/views/CreateServerView.vue:11`
**功能**: 创建服务器页面自动填充此值
**测试方法**:
- 在设置中修改为 1024
- 进入创建服务器页面
- 应该看到最小内存默认显示 1024MB

### ✅ 5. default_port (默认端口)
**位置**: `src/views/CreateServerView.vue:12`
**功能**: 创建服务器页面自动填充此值
**测试方法**:
- 在设置中修改为 25566
- 进入创建服务器页面
- 应该看到端口默认显示 25566

### ✅ 6. default_java_path (默认 Java 路径)
**位置**: `src/views/CreateServerView.vue:19`
**功能**: 创建服务器时优先使用此 Java 路径
**测试方法**:
- 在设置中指定一个 Java 路径
- 进入创建服务器页面
- 应该看到自动选中该 Java

### ✅ 7. default_jvm_args (默认 JVM 参数)
**位置**: `src-tauri/src/services/server_manager.rs:102-103`
**功能**: 启动服务器时附加这些 JVM 参数
**测试方法**:
- 在设置中添加 `-XX:+UseG1GC`
- 启动服务器
- 通过进程管理器查看 java 进程的启动参数中应该包含此参数

### ✅ 8. console_font_size (控制台字体大小)
**位置**: `src/views/ConsoleView.vue:23,91,244,262`
**功能**: 动态设置控制台输出和输入框的字体大小
**测试方法**:
- 在设置中修改字体大小为 16
- 进入控制台页面
- 应该看到日志和输入框的字体变大

### ✅ 9. max_log_lines (最大日志行数)
**位置**: `src-tauri/src/services/server_manager.rs:124,132`
**功能**: 限制每个服务器保留的最大日志行数，超出后自动删除旧日志
**测试方法**:
- 在设置中修改为 100
- 启动服务器并产生大量日志
- 日志应该只保留最新的 100 行

## 修复的问题

### 问题 1: 数据目录不一致
**原因**: 使用 `current_exe()` 导致开发模式和生产模式数据目录不同
**修复**: 改用 `~/.sea-lantern/` 固定目录

### 问题 2: 多个单例实例冲突
**原因**: 不同模块各自创建 SettingsManager 实例
**修复**: 创建统一的 `services/global.rs` 管理所有单例

### 问题 3: 创建服务器页面不使用默认设置
**原因**: 硬编码默认值，未从设置加载
**修复**: onMounted 时从设置 API 加载所有默认值

### 问题 4: 控制台字体大小设置无效
**原因**: CSS 中硬编码字体大小
**修复**: 使用响应式变量动态绑定字体大小

## 数据文件位置

所有数据现在统一保存在：
- **Windows**: `C:\Users\用户名\.sea-lantern\`
  - `sea_lantern_settings.json` - 应用设置
  - `sea_lantern_servers.json` - 服务器列表

这个位置在开发和生产环境下保持一致。
