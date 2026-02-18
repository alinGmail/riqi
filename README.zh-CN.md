# Riqi（日期）


<p align="center">
  <img src="./assets/demo.gif" alt="描述文字" />
</p>

一个用 Rust 编写的终端日历应用（TUI），可显示农历日期、节假日并支持直观的键盘导航。

## 功能特性

- **月历视图**：简洁的终端日历界面
- **农历支持**：使用 `tyme4rs` 库同时显示农历和公历日期
- **节假日显示**：自动下载并缓存多国家/语言的节假日数据
- **键盘导航**：支持 Vim 风格（hjkl）和方向键
- **主题化**：通过 TOML 配置文件自定义颜色
- **异步更新**：非阻塞式节假日数据获取
- **XDG 规范**：使用标准目录存储配置和缓存

## 安装

### 系统要求

**字体推荐**：本应用使用 Nerd Font 图标以增强视觉显示效果。为获得最佳体验，请在终端中安装并配置 [Nerd Font](https://www.nerdfonts.com/)。

### 从发布版安装

1. 从 [Releases](https://github.com/alinGmail/riqi/releases) 页面下载适合您系统的 ZIP 压缩包
2. 解压 ZIP 文件
3. 将 `riqi` 可执行文件移动到系统 PATH 中的目录：
   - **Linux/macOS**：`sudo mv riqi /usr/local/bin/`
   - **Windows**：将 `riqi.exe` 移动到 PATH 中的目录，或添加自定义目录到 PATH

### 从源码构建

```bash
cargo build --release
```

编译后的二进制文件位于 `target/release/riqi`。

## 使用方法

```bash
riqi
```

### 命令行参数

| 参数 | 简写 | 完整形式 | 类型 | 默认值 | 描述 |
|------|------|---------|------|--------|------|
| 国家 | `-c` | `--country` | `String` | 系统语言环境或 `cn` | 节假日数据的国家代码 |
| 语言 | `-l` | `--language` | `String` | 系统语言环境 | 显示语言代码 |
| 列数 | | `--column` | `u32` | -- | 日历网格的列数 |
| 行数 | | `--row` | `u32` | -- | 日历网格的行数 |
| 显示农历 | | `--show-lunar` | `bool` | false | 显示/隐藏农历日期 |
| 显示节假日 | | `--show-holiday` | `bool` | false | 显示/隐藏节假日信息 |

**示例：**

```bash
# 设置国家和语言
riqi --country us --language en --show-holoday

# 自定义网格布局
riqi --column 7 --row 6

# 启用农历显示
riqi --show-lunar
# 或禁用
riqi --show-lunar=false

```

**配置优先级（从高到低）：**
1. 命令行参数
2. 配置文件
3. 系统语言环境
4. 默认值

### 键盘操作

#### 导航
| 按键 | 替代键 | 操作 |
|-----|--------|------|
| `h` | `←` | 向左移动（前一天） |
| `j` | `↓` | 向下移动（下一周） |
| `k` | `↑` | 向上移动（上一周） |
| `l` | `→` | 向右移动（后一天） |

#### 时间跳转
| 按键 | 操作   |
|-----|------|
| `d` | 下个月  |
| `u` | 上个月  |
| `f` | 下一年  |
| `b` | 上一年  |
| `t` | 跳转到今天 |
| `g` | 跳转到  |

#### 应用程序
| 按键 | 操作 |
|-----|------|
| `q` | 退出应用 |

## 配置

### 配置文件位置

Riqi 遵循各平台的配置文件约定：

| 平台 | 配置目录 | 配置文件 |
|------|---------|---------|
| **Linux** | `$XDG_CONFIG_HOME/riqi/` 或 `~/.config/riqi/` | `~/.config/riqi/config.toml` |
| **macOS** | `~/Library/Application Support/riqi/` | `~/Library/Application Support/riqi/config.toml` |
| **Windows** | `%APPDATA%\riqi\` | `C:\Users\<用户名>\AppData\Roaming\riqi\config.toml` |

### 缓存和数据目录

| 平台 | 缓存目录 | 用途 |
|------|---------|------|
| **Linux** | `$XDG_CACHE_HOME/riqi/` 或 `~/.cache/riqi/` | 节假日数据缓存 |
| **macOS** | `~/Library/Caches/riqi/` | 节假日数据缓存 |
| **Windows** | `%LOCALAPPDATA%\riqi\` 或 `C:\Users\<用户名>\AppData\Local\riqi\` | 节假日数据缓存 |

### 配置文件格式

配置文件使用 TOML 格式。所有字段都是可选的；如果未指定，应用将使用系统默认值或命令行参数。

**可用配置选项：**

| 选项 | 类型 | 描述 | 默认值 |
|-----|------|-----|--------|
| `language` | `string` | 语言代码（如 `en`、`zh`） | 系统语言环境 |
| `country` | `string` | 节假日数据的国家代码（如 `us`、`cn`） | 系统语言环境或 `cn` |
| `show_lunar` | `boolean` | 显示农历日期 | `false` |
| `show_holiday` | `boolean` | 显示节假日信息 | `false` |
| `hide_bg` | `boolean` | 隐藏背景颜色 | `false` |
| `column` | `integer` | 日历网格列数 | 主题默认值（7） |
| `row` | `integer` | 日历网格行数 | 主题默认值（6） |

### 配置文件示例

**完整示例（`config.toml`）：**

```toml
language = "zh"
country = "cn"
show_lunar = true
show_holiday = true
hide_bg = false
column = 7
row = 6
```

**最小示例（中文/中国，带农历）：**

```toml
language = "zh"
country = "cn"
show_lunar = true
```

**最小示例（英文/美国，无农历）：**

```toml
language = "en"
country = "us"
show_lunar = false
```

**自定义网格布局：**

```toml
column = 7
row = 6
hide_bg = true
```

### 创建配置文件

首次运行 Riqi 时会自动创建配置目录。要创建自己的配置：

**在 Linux/macOS 上：**
```bash
mkdir -p ~/.config/riqi  # Linux
mkdir -p ~/Library/Application\ Support/riqi  # macOS
nano ~/.config/riqi/config.toml  # 使用您喜欢的编辑器
```

**在 Windows 上（PowerShell）：**
```powershell
New-Item -ItemType Directory -Force -Path "$env:APPDATA\riqi"
notepad "$env:APPDATA\riqi\config.toml"
```

### 配置优先级

配置值按以下顺序解析（优先级从高到低）：

1. **命令行参数**（如 `--country us`）
2. **配置文件**（`config.toml`）
3. **系统语言环境**（自动检测）
4. **默认值**（硬编码后备值）

### 主题配置

- **主题文件**：位于 `resources/theme/` 目录
- 主题使用 TOML 格式定义
- 自定义日历元素、高亮和边框的颜色


## 常见问题

* **问：我启用了 `show-holiday` 参数，但看不到节假日数据。**
  * **答：** 您必须设置正确的语言和国家。目前仅支持 `zh_cn` 和 `en_cn`。


## 开发
### 项目结构

```
riqi/
├── src/
│   ├── main.rs              # 入口点和主循环
│   ├── state.rs             # 应用状态管理
│   ├── config/              # 配置加载和解析
│   ├── data/                # 日历数据结构
│   │   └── calendar.rs      # MonthCalendar 和 CalendarDay 类型
│   ├── holiday/             # 节假日数据系统
│   │   ├── manager.rs       # 节假日数据管理
│   │   ├── load.rs          # 缓存加载
│   │   ├── update.rs        # 远程数据获取
│   │   └── downloader.rs    # HTTP 客户端
│   ├── ui/                  # UI 组件
│   │   ├── month_component.rs
│   │   ├── day_component.rs
│   │   └── bottom_line_component.rs
│   ├── theme/               # 主题系统
│   └── events.rs            # 异步通信的事件总线
├── resources/
│   └── theme/               # 主题 TOML 文件
└── AGENTS.md                # 开发指南
```

### 构建命令

```bash
cargo build                 # 调试构建
cargo build --release       # 发布构建
cargo run                   # 运行应用
```

### 测试

```bash
cargo test                  # 运行所有测试
cargo test --lib            # 仅运行单元测试
cargo test <test_name>      # 运行特定测试
```

### 日志

调试日志写入项目根目录下的 `debug.log`。

## 依赖项

- **ratatui**：TUI 框架
- **crossterm**：终端控制
- **tokio**：异步运行时
- **chrono**：日期时间处理
- **tyme4rs**：农历计算
- **reqwest**：HTTP 客户端用于节假日下载
- **serde/serde_json**：配置序列化
