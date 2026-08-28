# Linux Camera Controller documentation

This documentation is the public, navigable companion to the repository README. It describes the product direction without duplicating every internal working note.

## Start here

- [MVP scope](mvp.md) — the first useful version and its deliberately narrow boundary.
- [Architecture](architecture.md) — how the application fits around Linux's existing video stack.
- [Development setup](development.md) — the planned local development environment and first implementation step.
- [GitHub Project board](https://github.com/users/MrBig83/projects/36) — the current public work queue.

## Project principles

- Fix one common webcam-orientation problem well; do not build a mini-OBS.
- Keep all video processing local.
- Release the physical camera whenever the pipeline is stopped.
- Keep the UI free of raw shell commands and implementation-specific device paths.
