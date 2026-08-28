# Development setup

## Current status

The Tauri + React + TypeScript foundation runs on Nobara. Before starting the desktop app on a Wayland/Nvidia system, use the documented development workaround below.

## Target environment

- Nobara Linux with KDE Plasma
- Rust and Cargo
- Node.js and a package manager compatible with the selected Tauri template
- Tauri with React and TypeScript
- FFmpeg, `v4l2-utils` and `v4l2loopback` for camera-pipeline verification

Install the Tauri toolchain dependencies following the [official Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/). For the camera-specific one-time setup, see [First-time camera setup](setup.md).

## Run locally

```bash
npm install
WEBKIT_DISABLE_DMABUF_RENDERER=1 npm run tauri dev
```

`WEBKIT_DISABLE_DMABUF_RENDERER=1` is a development workaround for a known WebKitGTK/Wayland GPU-rendering problem seen on the target Nobara system. It is not an application requirement on every Linux desktop.

## First implementation milestone

After the desktop shell launches, the first slice is deliberately narrow:

```text
select physical webcam → choose 180 degrees → start pipeline → virtual camera in Teams → stop → camera released
```

See the [MVP scope](mvp.md) and the [Project board](https://github.com/users/MrBig83/projects/36) for the tracked work order.
