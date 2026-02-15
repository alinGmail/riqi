# Riqi

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

### Build from Source

```bash
cargo build --release
```

The binary will be available at `target/release/riqi`.

## Usage

```bash
cargo run
```

### Command-line Arguments

| Argument | Short | Long | Type | Default | Description |
|----------|-------|------|------|---------|-------------|
| Country | `-c` | `--country` | `String` | System locale or `cn` | Country code for holiday data |
| Language | `-l` | `--language` | `String` | System locale | Language code for display |
| Column | | `--column` | `u32` | -- | Number of columns in the calendar grid |
| Row | | `--row` | `u32` | -- | Number of rows in the calendar grid |
| Show Lunar | | `--show-lunar` | `bool` | false | Show/hide lunar calendar dates |
| Show Holiday | | `--show-holiday` | `bool` | false | Show/hide holiday information |

**Examples:**

```bash
# Set country and language
riqi --country us --language en --show-holoday

# Customize grid layout
riqi --column 7 --row 6

# Enable lunar calendar display
riqi --show-lunar
# or disable it
riqi --show-lunar=false

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

#### Application
| Key | Action |
|-----|--------|
| `q` | Quit application |

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

**Custom grid layout:**

```toml
column = 7
row = 6
hide_bg = true
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

- **Theme Files**: Located in `resources/theme/` directory
- Themes are defined in TOML format
- Customize colors for calendar elements, highlights, and borders


## FAQ

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

