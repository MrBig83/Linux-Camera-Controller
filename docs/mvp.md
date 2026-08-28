# MVP scope

The MVP is a Linux desktop application that makes a webcam available to other apps in the correct orientation.

## First acceptance scenario

1. Launch the app on Nobara Linux.
2. Select a Logitech StreamCam.
3. Select 180-degree rotation.
4. Start the virtual camera.
5. Select the virtual camera in Microsoft Teams.
6. Confirm that the image is correctly oriented.
7. Stop the virtual camera and confirm that the physical camera is released.

## In scope

- V4L2 camera discovery without hard-coded physical device paths.
- Rotation: 0, 90, 180 and 270 degrees.
- Horizontal and vertical flips.
- Resolution and frame-rate choices based on the selected camera's capabilities.
- A v4l2loopback-backed virtual camera.
- Clear active/inactive state and useful errors.

## Not in scope

Recording, streaming, background effects, AI features, microphone processing, multi-camera scenes, accounts, telemetry, Windows support and macOS support are intentionally excluded from the MVP.

## Delivery plan

The tracked implementation order lives on the [GitHub Project board](https://github.com/users/MrBig83/projects/36). The first issue creates the Tauri foundation; the first technical milestone proves the complete camera pipeline from the app to Teams.
