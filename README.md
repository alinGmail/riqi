# Riqi

[中文版](README.zh-CN.md)

<p align="center">
  <img src="./assets/demo.gif" alt="描述文字" />
</p>


A terminal-based calendar application (TUI) written in Rust that displays monthly calendars with lunar dates, holidays, and intuitive keyboard navigation.

## Features

- **Monthly Calendar View**: Clean, terminal-based calendar interface
- **Lunar Calendar Support**: Shows lunar dates alongside solar dates using `tyme4rs`
- **Holiday Display**: Automatically downloads and caches holiday data for multiple countries/languages
- **Keyboard Navigation**: Vim-style (hjkl) and arrow key support
- **Themeable**: Customizable colors via TOML configuration files
- **Asynchronous Updates**: Non-blocking holiday data fetching
- **XDG-Compliant**: Uses standard directories for configuration and cache

## Installation

### Requirements

**Font Recommendation**: This application uses Nerd Font icons for enhanced visual display. For the best experience, please install and configure a [Nerd Font](https://www.nerdfonts.com/) in your terminal.

### Install from Release

1. Download the appropriate ZIP package for your system from the [Releases](https://github.com/alinGmail/riqi/releases) page
2. Extract the ZIP file
3. Move the `riqi` executable to a directory in your system's PATH:
   - **Linux/macOS**: `sudo mv riqi /usr/local/bin/`
   - **Windows**: Move `riqi.exe` to a directory in your PATH or add a custom directory to PATH

### Build from Source

```bash
cargo build --release
```

The binary will be available at `target/release/riqi`.

## Usage

```bash
riqi
```

### Command-line Arguments

| Argument     | Short | Long             | Type     | Default | Description                            |
|--------------|-------|------------------|----------|----|----------------------------------------|
| Country      | `-c`  | `--country`      | `String` | System locale or `cn` | Country code for holiday data          |
| Language     | `-l`  | `--language`     | `String` | System locale | Language code for display              |
| Source       |       | `--source`       | `String` | `github` | Holiday data source (`github` or `gitee`) |
| Column       |       | `--column`       | `u32`    | -- | Number of columns in the calendar grid |
| Row          |       | `--row`          | `u32`    | -- | Number of rows in the calendar grid    |
| Show Lunar   |       | `--show-lunar`   | `bool`   | false | Show/hide lunar calendar dates         |
| Show Holiday |       | `--show-holiday` | `bool`   | false | Show/hide holiday information          |
| Theme        |       | `--theme`        | `String` | ningmen | Theme name (see [Theme Configuration](#theme-configuration)) |
| output       | `-o`  | `--output`       | `String` | %Y-%m-%d | the output format of the select day    |

**Examples:**

```bash
# Set country and language
riqi --country us --language en --show-holiday

# Customize grid layout
riqi --column 7 --row 6

# Enable lunar calendar display
riqi --show-lunar
# or disable it
riqi --show-lunar=false

# Use gitee as holiday data source (faster in China)
riqi --source gitee --country cn --language zh --show-holiday

# Use github as holiday data source (default)
riqi --source github --country cn --language zh --show-holiday

# Use forest theme
riqi --theme forest

# Use ocean theme
riqi --theme ocean

# Use lavender theme
riqi --theme lavender

# Use mint theme
riqi --theme mint

# Use sunset theme
riqi --theme sunset

# Use ruby theme
riqi --theme ruby
```

**Configuration Priority (highest to lowest):**
1. Command-line arguments
2. Configuration file
3. System locale
4. Default values

### Keyboard Controls

#### Navigation
| Key | Alternative | Action |
|-----|-------------|--------|
| `h` | `←` | Move left (previous day) |
| `j` | `↓` | Move down (next week) |
| `k` | `↑` | Move up (previous week) |
| `l` | `→` | Move right (next day) |

#### Time Jumping
| Key | Action |
|-----|--------|
| `d` | Next month |
| `u` | Previous month |
| `f` | Next year |
| `b` | Previous year |
| `t` | Jump to today |
| `g` | Jump to |

#### Application
| Key     | Action                                     |
|---------|--------------------------------------------|
| `q`     | Quit application                           |
| `Enter` | output the select day and quit application |

## Configuration

### Configuration File Locations

Riqi follows platform-specific conventions for configuration files:

| Platform | Configuration Directory | Configuration File |
|----------|------------------------|-------------------|
| **Linux** | `$XDG_CONFIG_HOME/riqi/` or `~/.config/riqi/` | `~/.config/riqi/config.toml` |
| **macOS** | `~/Library/Application Support/riqi/` | `~/Library/Application Support/riqi/config.toml` |
| **Windows** | `%APPDATA%\riqi\` | `C:\Users\<YourName>\AppData\Roaming\riqi\config.toml` |

### Cache and Data Directories

| Platform | Cache Directory | Purpose |
|----------|----------------|---------|
| **Linux** | `$XDG_CACHE_HOME/riqi/` or `~/.cache/riqi/` | Holiday data cache |
| **macOS** | `~/Library/Caches/riqi/` | Holiday data cache |
| **Windows** | `%LOCALAPPDATA%\riqi\` or `C:\Users\<YourName>\AppData\Local\riqi\` | Holiday data cache |

### Configuration File Format

The configuration file uses TOML format. All fields are optional; if not specified, the application will use system defaults or command-line arguments.

**Available Configuration Options:**

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `language` | `string` | Language code (e.g., `en`, `zh`) | System locale |
| `country` | `string` | Country code for holiday data (e.g., `us`, `cn`) | System locale or `cn` |
| `source` | `string` | Holiday data source (`github` or `gitee`) | `github` |
| `show_lunar` | `boolean` | Display lunar calendar dates | `false` |
| `show_holiday` | `boolean` | Display holiday information | `false` |
| `hide_bg` | `boolean` | Hide background colors | `false` |
| `column` | `integer` | Number of columns in calendar grid | Theme default (7) |
| `row` | `integer` | Number of rows in calendar grid | Theme default (6) |

### Example Configuration File

**Complete example (`config.toml`):**

```toml
language = "zh"
country = "cn"
source = "gitee"
show_lunar = true
show_holiday = true
hide_bg = false
column = 7
row = 6
```

**Minimal example (Chinese/China with lunar calendar):**

```toml
language = "zh"
country = "cn"
show_lunar = true
```

**Minimal example (English/US without lunar calendar):**

```toml
language = "en"
country = "us"
show_lunar = false
```

**Use gitee source (faster in China):**

```toml
language = "zh"
country = "cn"
source = "gitee"
show_holiday = true
```

### Creating Your Configuration File

The configuration directory will be created automatically when you first run Riqi. To create your own configuration:

**On Linux/macOS:**
```bash
mkdir -p ~/.config/riqi  # Linux
mkdir -p ~/Library/Application\ Support/riqi  # macOS
nano ~/.config/riqi/config.toml  # Edit with your preferred editor
```

**On Windows (PowerShell):**
```powershell
New-Item -ItemType Directory -Force -Path "$env:APPDATA\riqi"
notepad "$env:APPDATA\riqi\config.toml"
```

### Configuration Priority

Configuration values are resolved in the following order (highest priority first):

1. **Command-line arguments** (e.g., `--country us`)
2. **Configuration file** (`config.toml`)
3. **System locale** (auto-detected)
4. **Default values** (hardcoded fallbacks)

### Theme Configuration

Riqi comes with 8 built-in themes. You can switch themes using the `--theme` command-line argument or by setting `theme` in your configuration file.

**Available Themes:**

| Theme Name | Description |
|------------|-------------|
| `ningmen` | Default theme - Lemon yellow-green on dark background |
| `ocean` | Cool blue theme inspired by ocean waves |
| `forest` | Earthy green theme inspired by deep forests |
| `sunset` | Warm orange theme inspired by sunset colors |
| `lavender` | Soft purple theme inspired by lavender fields |
| `mint` | Crisp fresh theme inspired by mint leaves |
| `ruby` | Rich red theme inspired by ruby gemstones |
| `dark` | Classic dark theme (original) |
| `blue` | Classic blue theme (original) |

**Usage:**

Command-line:
```bash
riqi --theme forest
riqi --theme ocean
riqi --theme lavender
riqi --theme mint
riqi --theme sunset
riqi --theme ruby
```

Configuration file (`config.toml`):
```toml
theme = "forest"
```

**Theme Files**: Located in `resources/theme/` directory. Each theme is defined in TOML format with customizable colors for:
- Calendar background/foreground
- Month title and week headers
- Workday and holiday text (current month)
- Adjacent month workday and holiday text
- Selected day highlight
- Bottom status line

For detailed theme information and customization guide, see [THEMES.md](THEMES.md).


## FAQ

* **Q: What is the `--source` parameter and when should I use it?**
  * **A:** The `--source` parameter controls where holiday data is downloaded from. It has two values:
    - `github` (default): Downloads from GitHub (https://github.com/alinGmail/riqi)
    - `gitee`: Downloads from Gitee (https://gitee.com/alinGmail/riqi)
    
    Use `gitee` if you're in China and experiencing slow downloads from GitHub. You can set it via command-line (`--source gitee`) or in the config file (`source = "gitee"`).

* **Q: How can I configure the holiday data source permanently?**
  * **A:** Add `source = "gitee"` (or `source = "github"`) to your `config.toml` file. This will be used as the default unless overridden by the `--source` command-line argument.

* **Q: I enabled the `show-holiday` argument, but cannot see holiday data.**
  * **A:** You must set the correct language and country. Currently, only `zh_cn` and `en_cn` are supported.


## Development
### Project Structure

```
riqi/
├── src/
│   ├── main.rs              # Entry point and main loop
│   ├── state.rs             # Application state management
│   ├── config/              # Configuration loading and parsing
│   ├── data/                # Calendar data structures
│   │   └── calendar.rs      # MonthCalendar and CalendarDay types
│   ├── holiday/             # Holiday data system
│   │   ├── manager.rs       # Holiday data management
│   │   ├── load.rs          # Cache loading
│   │   ├── update.rs        # Remote data fetching
│   │   └── downloader.rs    # HTTP client
│   ├── ui/                  # UI components
│   │   ├── month_component.rs
│   │   ├── day_component.rs
│   │   └── bottom_line_component.rs
│   ├── theme/               # Theme system
│   └── events.rs            # Event bus for async communication
├── resources/
│   └── theme/               # Theme TOML files
└── AGENTS.md                # Development guidelines
```

### Build Commands

```bash
cargo build                 # Debug build
cargo build --release       # Release build
cargo run                   # Run the application
```

### Testing

```bash
cargo test                  # Run all tests
cargo test --lib            # Run unit tests only
cargo test <test_name>      # Run specific test
```

### Logging

Debug logs are written to `debug.log` in the project root.

## Dependencies

- **ratatui**: TUI framework
- **crossterm**: Terminal control
- **tokio**: Async runtime
- **chrono**: Date/time handling
- **tyme4rs**: Lunar calendar calculations
- **reqwest**: HTTP client for holiday downloads
- **serde/serde_json**: Configuration serialization

