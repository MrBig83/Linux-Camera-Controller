# Development setup

## Current status

The repository is documentation-first. No Tauri application code has been added yet.

## Target environment

- Nobara Linux with KDE Plasma
- Rust and Cargo
- Node.js and a package manager compatible with the selected Tauri template
- Tauri with React and TypeScript
- FFmpeg, `v4l2-utils` and `v4l2loopback` for camera-pipeline verification

The exact package and toolchain verification is tracked in [the Tauri scaffold issue](https://github.com/MrBig83/Linux-Camera-Controller/issues/4). It will be documented here once tested on the target system instead of guessing commands in advance.

## First implementation milestone

After the desktop shell launches, the first slice is deliberately narrow:

```text
select physical webcam → choose 180 degrees → start pipeline → virtual camera in Teams → stop → camera released
```

See the [MVP scope](mvp.md) and the [Project board](https://github.com/users/MrBig83/projects/36) for the tracked work order.
