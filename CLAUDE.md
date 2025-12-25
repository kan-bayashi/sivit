# sivit - Project Guide

## Overview

sivit (Simple Image Viewer In Terminal) - A terminal-based image viewer with sxiv-like keybindings.

## Tech Stack

- Language: Rust (Edition 2024)
- TUI Framework: ratatui + ratatui-image
- Terminal Backend: crossterm
- Image Processing: image crate
- CLI: clap (derive)

## Project Structure

```
sivit/
├── Cargo.toml
├── docs/
│   ├── architecture.md
│   └── development.md
├── src/
│   ├── main.rs    # Entry point, CLI parsing, event loop
│   ├── app.rs     # App state, navigation, cache/prefetch orchestration
│   ├── fit.rs     # Fit mode (Normal/Fit)
│   ├── kgp.rs     # Kitty Graphics Protocol helpers (encode/place/erase)
│   ├── sender.rs  # TerminalWriter (single stdout writer, status priority, cancel)
│   └── worker.rs  # ImageWorker (decode/resize/encode)
```

## Development Commands

```bash
cargo build          # Build
cargo run            # Run
cargo test           # Test
cargo fmt            # Format
cargo clippy         # Lint
```

## Keybindings

- `q` - Quit
- `j` / `Space` / `l` - Next image
- `k` / `Backspace` / `h` - Previous image
- `g` - First image
- `G` - Last image
- `f` - Toggle fit
- `r` - Reload (clear cache)
- Counts supported (e.g. `5j`, `10G`)

## Environment Variables

- `SIVIT_NAV_LATCH_MS` - Navigation latch (ms) before drawing images (default: 150)
- `SIVIT_RENDER_CACHE_SIZE` - Render cache entries (default: 15)
- `SIVIT_TMUX_KITTY_MAX_PIXELS` - Max pixels for tmux+kitty in `Normal` mode (default: 1500000)
- `SIVIT_FORCE_ALT_SCREEN` - Force alternate screen mode
- `SIVIT_NO_ALT_SCREEN` - Disable alternate screen mode
- `SIVIT_DEBUG` - Enable debug info in status bar
- `SIVIT_TRACE_WORKER` - Write worker timing logs to `/tmp/sivit_worker.log`

## Coding Conventions

- Follow Rust standard naming conventions (snake_case for functions/variables, PascalCase for types)
- Use `anyhow::Result` for error handling
- Keep functions small and focused
- Write tests for public functions

### Critical Invariants

- **stdout is written by `TerminalWriter` only** (`src/sender.rs`)
- **Image output must be chunked at safe boundaries**
  - KGP chunk boundaries for transmit (`encode_chunks`)
  - per-row boundaries for placement/erase (`place_rows` / `erase_rows`)
- **Navigation must stay responsive**
  - cancel in-flight image output on navigation
  - avoid blocking the main loop on decode/encode or stdout I/O

## Architecture Notes

- See `docs/architecture.md` and `docs/development.md`.

## Contributing

See `CONTRIBUTING.md`.

## References

- [ratatui-image](https://github.com/benjajaja/ratatui-image)
- [ratatui](https://ratatui.rs/)
- [Kitty Graphics Protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/)
