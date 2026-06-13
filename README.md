# QLite-OS

QLite-OS is a tiny freestanding OS scaffold built around three pieces:

- `asm` boot entry to transfer control into the kernel
- `rust` kernel logic and app rendering
- `c` runtime helpers for low-level string and memory work

It currently includes a text-mode launcher plus a small app suite:

- `QSearch`, a custom search surface inspired by Google
- `QStudio`, a VS Code-style editor shell
- `QLpaint`, a canvas app with brush and fill demos
- `QLcalc`, a calculator with expression evaluation
- `QLfiles`, a file browser view for the OS tree
- `QLnotes`, a pinned note board with tasks and tags
- `QLmusic`, a lightweight player with a queue and EQ view
- `QLsettings`, a system panel for theme, input, and power
- `QLfetch`, a Fastfetch-style banner with logo and system stats
- `QLterm`, a terminal with commands and sample shell output

The terminal demo now accepts commands such as `help`, `ls /apps`, `echo hello`, and `stats pc` for the logo-based PC stats view.

## Layout

- `boot/boot.S` - assembly entry point
- `c/runtime.c` - C helper runtime
- `rust/kernel.rs` - Rust kernel entry and VGA writer
- `rust/apps.rs` - app registry and renderers
- `Makefile` - build scaffold
- `docs/` - expanded project notes and app docs
- `fonts/` - text-based font assets and samples
- `icons/` - icon assets placeholder

## Docs

- [Architecture](docs/architecture.md)
- [Apps](docs/apps.md)
- [Terminal](docs/terminal.md)
- [Roadmap](docs/roadmap.md)

## Project Docs

- [Contributing](CONTRIBUTING.md)
- [Changelog](CHANGELOG.md)
- [Security](SECURITY.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

## Icons

The repository now includes [icons/qlite-logo.svg](icons/qlite-logo.svg), [icons/qlite-terminal.svg](icons/qlite-terminal.svg), and a per-app icon set in [icons/README.md](icons/README.md).

## Fonts

The current font asset lives in [fonts/qlite-5x7.font.txt](fonts/qlite-5x7.font.txt), with usage notes in [fonts/README.md](fonts/README.md).

## Notes

This repo is intentionally minimal and freestanding. The Rust compiler is not installed in the current environment, so the build is a scaffold ready for a real cross-toolchain setup.