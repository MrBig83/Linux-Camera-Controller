# Linux Camera Controller

Linux Camera Controller is a lightweight desktop application for Linux that rotates, flips and routes webcams through a virtual camera—without terminal commands, fragile `/dev/videoX` paths or hand-written FFmpeg scripts.

> **Status:** Early development. The first goal is a small, reliable vertical slice on Nobara Linux.

## The problem

A webcam can work perfectly on Linux while its image is upside down, mirrored or otherwise incorrectly oriented in an application such as Microsoft Teams. Many consumer applications do not offer controls to correct this.

Linux Camera Controller turns the existing Linux video pipeline into a focused desktop workflow:

```text
Physical webcam
      ↓
Linux Camera Controller
      ↓
FFmpeg transformation
      ↓
v4l2loopback virtual camera
      ↓
Teams, Discord, Google Meet, and similar apps
```

## MVP

The first working version will:

- discover connected V4L2 webcams without hard-coded device paths;
- let the user select a camera, resolution and frame rate;
- apply 0°, 90°, 180° or 270° rotation;
- apply horizontal and vertical flips;
- start and stop a virtual camera backed by `v4l2loopback`;
- release the physical camera cleanly when stopped; and
- show clear status and useful error messages.

The initial acceptance test is deliberately simple: select a Logitech StreamCam on Nobara, choose 180°, start the pipeline, use the transformed virtual camera in Microsoft Teams, then stop it and confirm that the camera is released.

## Out of scope for the MVP

This is not a replacement for OBS. The first version will not include recording, streaming, AI effects, background replacement, microphone processing, multi-camera scenes, cloud accounts, telemetry, Windows support or macOS support.

## Technical direction

- **Desktop UI:** Tauri, React and TypeScript
- **System integration:** Rust
- **Video pipeline:** FFmpeg, V4L2 and `v4l2loopback`
- **Initial platform:** Nobara Linux with KDE Plasma
- **Privacy:** video is processed locally; no accounts, cloud services or telemetry

The application will not execute arbitrary commands from its UI. Its Rust backend will control the narrowly scoped camera and FFmpeg operations.

## Development status

The underlying manual pipeline has already been proven with a Logitech StreamCam, FFmpeg and `v4l2loopback`. The next milestone is to reproduce that pipeline from a minimal Tauri application before expanding discovery, settings or UI polish.

## Documentation

- [Documentation index](docs/README.md)
- [MVP scope](docs/mvp.md)
- [Architecture](docs/architecture.md)
- [First-time camera setup](docs/setup.md)
- [Development setup](docs/development.md)
- [Project board](https://github.com/users/MrBig83/projects/36)

## Contributing

Contributions are welcome.

1. Fork the repository.
2. Create a branch in your fork.
3. Make and test a focused change.
4. Open a pull request against `main`.

`main` is protected: changes must arrive through pull requests, and the project maintainer reviews and merges them.

## Support

Linux Camera Controller is free and open source. If it saves you time and you want to support its development:

<a href="https://paypal.me/MrBig83"><img src="https://img.shields.io/badge/Donate-PayPal-0070BA?style=for-the-badge&logo=paypal&logoColor=white" alt="Donate with PayPal"></a>

## License

This project is licensed under the [MIT License](LICENSE).

## Credits

Created by [Martin](https://github.com/MrBig83).
