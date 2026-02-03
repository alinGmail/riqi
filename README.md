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

| Argument | Short | Long | Default | Description |
|----------|-------|------|---------|-------------|
| Country | `-c` | `--country` | System locale or `cn` | Country code for holiday data |
| Language | `-l` | `--language` | System locale | Language code for display |

**Example:**

```bash
riqi --country us --language en
```

**Configuration Priority (highest to lowest):**
1. Command-line arguments
2. Configuration file
3. System locale
4. Default values

### Keyboard Controls

- **hjkl** / **Arrow Keys**: Navigate between days
- **d/u**: Next/previous month
- **y/x**: Next/previous year
- **t**: Jump to today
- **q**: Quit

## Configuration

Riqi uses XDG-compliant directories for configuration:

- **Config Files**: `$XDG_CONFIG_HOME/riqi/` (Unix-like systems)
- **Cache**: `$XDG_CACHE_HOME/riqi/` (for holiday data)
- **Themes**: Located in `resources/theme/` directory

The application automatically detects your system locale for language and country settings.

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

