# AGENTS.md

This file provides guidance to Qoder (qoder.com) when working with code in this repository.

## Project Overview

Riqi is a terminal-based calendar application (TUI) written in Rust. It displays monthly calendars with lunar dates, holidays, and navigation support. The app uses `ratatui` for the TUI interface and `crossterm` for terminal management.

## Build and Development Commands

### Building
```bash
cargo build                 # Debug build
cargo build --release       # Release build
```

### Running
```bash
cargo run                   # Run the main binary
```

### Testing
```bash
cargo test                  # Run all tests
cargo test --lib            # Run unit tests only
cargo test <test_name>      # Run specific test
```

## Architecture

### Core Application Flow (src/main.rs)
- Entry point initializes logging to `debug.log`
- Sets up terminal in raw mode with alternate screen
- Main loop handles:
  - Rendering via `ratatui`
  - Async event handling through `MessageBus`
  - Keyboard input (hjkl/arrow keys for navigation, d/u for month navigation, y/x for year navigation, t for today, q to quit)

### State Management (src/state.rs)
`RiqiState` is the central application state containing:
- `select_day`: Currently selected date
- `holiday_map`: Holiday data loaded from cache/remote
- `today`: Today's date reference
- `config`: User configuration
- `theme`: UI theme colors
- `message_bus`: Event communication channel

### Calendar System (src/types/calendar.rs)
- `CalendarDay`: Represents a single day with solar and lunar dates
- `MonthCalendar`: Generates 6-week calendar grid for any month
- Handles edge cases like month boundaries, weekday calculations
- Uses `tyme4rs` for lunar calendar conversions

### Holiday System (src/holiday/)
Implements a caching system for holiday data:
- Downloads holiday metadata from GitHub
- Caches holiday files locally (XDG cache directory on Unix)
- Validates cache freshness (1 day TTL for meta_cache.json)
- Updates asynchronously without blocking UI
- Structure: `holidays/<year>/<country>_<language>.json`

Key files:
- `load.rs`: Loads holidays from cache or triggers updates
- `update.rs`: Downloads meta and holiday data files
- `downloader.rs`: HTTP client for fetching remote data
- `utils.rs`: Cache path resolution and parsing

### Component System (src/component/)
UI components render calendar views:
- `month_component.rs`: Main calendar grid display
- `day_component.rs`: Individual day cell rendering
- `bottom_line_component.rs`: Status/help bar

### Configuration (src/config/)
- Loads from file or uses system defaults
- Detects system locale (language/country)
- Theme loading from TOML files in `resources/theme/`
- XDG-compliant path resolution for config files

### Event System (src/events.rs)
`MessageBus` provides async communication:
- `AppEvent::Input`: Keyboard events
- `AppEvent::RequestResult`: Background task completions (e.g., holiday updates)

## Key Dependencies

- `ratatui`: TUI framework
- `crossterm`: Terminal control
- `tokio`: Async runtime
- `chrono`: Date/time handling
- `tyme4rs`: Lunar calendar calculations
- `reqwest`: HTTP client for holiday downloads
- `serde`/`serde_json`: Configuration and data serialization

## Development Notes

- Logging is written to `debug.log` in the project root
- The app uses XDG directories for caching holiday data
- Tests are located inline with implementation (see `src/types/calendar.rs` for examples)
- Calendar always renders 6 weeks for consistent layout
