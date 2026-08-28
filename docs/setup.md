# First-time camera setup

Linux Camera Controller expects the video stack to be prepared once by the system owner. The app checks readiness and explains what is missing, but it does not install packages or ask for sudo during normal use.

These steps are tested for Nobara/Fedora-style systems. Other distributions use equivalent packages and configuration locations.

## 1. Install the camera tools

```bash
sudo dnf install ffmpeg-free v4l-utils v4l2loopback
```

`ffmpeg-free` is Nobara's usual FFmpeg package. Confirm the installed build with:

```bash
ffmpeg -version
```

Do not install a different FFmpeg package over Nobara's existing FFmpeg build without checking package conflicts first.

## 2. Configure a persistent virtual camera

Create `/etc/modprobe.d/v4l2loopback.conf` with this content. The example uses an otherwise unused `/dev/video10` and gives the camera a friendly name for conferencing apps.

```text
options v4l2loopback devices=1 video_nr=10 card_label="StreamCam Rotated" exclusive_caps=1
```

Create `/etc/modules-load.d/v4l2loopback.conf` with:

```text
v4l2loopback
```

Restart the computer so the configured module loads with the selected options.

## 3. Confirm readiness

Open Linux Camera Controller. The **System readiness** card should show all three checks as ready:

- FFmpeg
- V4L2 tools
- Virtual camera

The virtual camera may be assigned a different `/dev/videoX` number if your chosen number is already in use. The app detects the loopback camera rather than relying on a fixed device path.

## Troubleshooting

- **FFmpeg missing:** install the FFmpeg package appropriate for your distribution, then refresh readiness.
- **V4L2 tools missing:** install `v4l-utils`, then refresh readiness.
- **v4l2loopback missing:** install `v4l2loopback`, complete the configuration above and restart.
- **Virtual camera missing after restart:** verify that the module loaded and review the two configuration files. Do this setup from a terminal once; normal Start and Stop in the app will not request sudo.

The app does not open the physical camera while checking readiness.
