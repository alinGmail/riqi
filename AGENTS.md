# AGENTS.md

Guidance for OpenCode sessions working in this repo.

## What this is

`riqi` is a Rust TUI calendar (lunar dates, holidays, vim/arrow navigation) built on `ratatui` 0.30 + `crossterm`, async via `tokio`. Single binary; no lib target.

**Active code is `src/`. `src-bak/` is the pre-refactor original and is NOT compiled** (nothing in `Cargo.toml` or `src/main.rs` references it). Do not edit or draw conclusions from `src-bak/`. `GEMINI.md` is stale — it describes `src-bak/` as active and `src/main.rs` as a placeholder; ignore it.

## Commands

```bash
cargo build                 # debug
cargo build --release       # release binary at target/release/riqi
cargo run -- --show-holiday # run with flags
cargo test                  # all tests
cargo test --lib            # unit tests only
cargo test <test_name>      # single test
```

No lint/format/typecheck config, no CI test job. `.github/workflows/build.yml` only builds release binaries for 3 targets on `v*` tags (macOS signing/notarization needs repo secrets).

## Wiring you'd otherwise guess wrong

- `src/main.rs` owns the loop. Input is polled on a thread and sent over a `std::sync::mpsc` channel; the main loop blocks on `rx.recv()` and matches `AppEvent` (`src/events.rs`: `TerminalEvent`, `Quit`, `UpdateHoliday`, `AddNotification`, `RemoveNotification`). Holiday fetches run as `tokio` tasks and report back via `AppEvent::UpdateHoliday`.
- Components implement `render(area, &mut Buffer)` and are drawn with `f.buffer_mut()`, not `Frame` (enabled by ratatui's `unstable-widget-ref` feature). Follow that pattern for new widgets.
- Pressing `Enter` in normal mode prints the selected day formatted by `--output` and exits — this is the scripting interface; don't break it.
- Keys: `hjkl`/arrows move days, `d`/`u` month, `f`/`b` year, `t` today, `g` goto panel, `q`/`Esc` quit. (README's `y`/`x` year keys are wrong; code uses `f`/`b`.)
- `src/state.rs` (`RiqiState`, `RiqiMode`) is the central mutable state; `src/data/calendar.rs` builds the always-6-week grid.

## Configuration

Resolution order: CLI args > `<config_dir>/riqi/config.toml` > system locale > defaults (`src/config/config_main.rs`). `config_dir` is XDG per-OS (`src/config/xdg.rs`). Boolean flags use `num_args(0..=1)`, so both `--show-lunar` and `--show-lunar=false` work.

## Themes

Theme TOMLs in `resources/theme/` are **embedded at compile time** with `include_dir!` (`src/theme/theme_loader.rs`). Adding/editing a theme requires a rebuild. An unknown `--theme` name panics (`expect` in `src/main.rs:95`), not a graceful error. 8 themes; default `ningmen`.

## Holidays

- Fetched from hardcoded GitHub/Gitee raw URLs (`src/holiday/manager.rs:47`); `--source github|gitee` picks one. There is no bundled fallback.
- Cache: `<cache_dir>/riqi/holidays/<year>/<language>_<country>.json`. Refresh TTL is **10 days** (`is_need_update`, `src/holiday/manager.rs:62`).
- Only `zh_cn` and `en_cn` actually have data. `resources/holidays/` holds the files served by the remote repo, not compiled into the binary.

## Logging

`debug.log` is written to the **XDG cache dir** (`<cache_dir>/riqi/debug.log`), not the project root (`src/main.rs:51`). Logger is initialized with `.is_test(true)`.

## Tests

Inline `#[cfg(test)]` modules only, in `src/data/calendar.rs` and `src/ui/lunar.rs`. No `tests/` directory.
