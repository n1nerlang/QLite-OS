# Architecture

QLite-OS is split into three layers:

- `boot/boot.S` transfers execution into the kernel entry point.
- `c/runtime.c` provides small freestanding helpers used by the kernel.
- `rust/` contains the kernel renderer and the app showcase.

The current runtime is text-mode only, which keeps the project simple while the app surface grows.
