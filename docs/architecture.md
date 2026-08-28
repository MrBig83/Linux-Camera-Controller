# Architecture

Linux Camera Controller is a desktop interface and process manager around established Linux video components. It does not attempt to implement a new video engine.

```text
Physical V4L2 webcam
        ↓
Linux Camera Controller
  Tauri UI + Rust backend
        ↓
FFmpeg transform pipeline
        ↓
v4l2loopback virtual camera
        ↓
Teams, Discord, Google Meet, and similar applications
```

## Components

- **React and TypeScript:** desktop UI, form state and feedback.
- **Tauri and Rust:** camera discovery, capability lookup, controlled process lifecycle and error handling.
- **V4L2:** Linux camera interface.
- **FFmpeg:** frame transformation.
- **v4l2loopback:** virtual camera consumed by conferencing applications.

## Safety boundaries

- The frontend does not construct or execute arbitrary shell commands.
- The Rust backend owns the restricted FFmpeg process and its arguments.
- Normal runtime should not need repeated sudo prompts.
- Kernel-module setup is an installation concern, not a per-click runtime concern.
- Video remains local; the application has no cloud account or telemetry requirement.

## Lifecycle rule

Opening the application must not keep the physical camera active. Starting the pipeline acquires the selected camera; stopping it terminates FFmpeg cleanly and releases that camera.
