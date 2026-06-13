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

## Layout

- `boot/boot.S` - assembly entry point
- `c/runtime.c` - C helper runtime
- `rust/kernel.rs` - Rust kernel entry and VGA writer
- `rust/apps.rs` - app registry and renderers
- `Makefile` - build scaffold

## Notes

This repo is intentionally minimal and freestanding. The Rust compiler is not installed in the current environment, so the build is a scaffold ready for a real cross-toolchain setup.