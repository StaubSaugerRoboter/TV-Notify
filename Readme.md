# Command Output to MQTT

The new Libreelec image does not support `vcgencmd display_power`
anymore. Therefore, if you want to notify Home Assistant about the
state of the display output on a Raspberry Pi you need to parse `modetest`.
This application does so and sends the state to an MQTT broker during startup
or during changes. You can configure the behaviour through the config and also adapt
it to other commands such as `vcgencmd display_power`.


## Configure
Copy either `modetest_config.toml` or `vcgencmd_config.toml` to `/etc/tv-notify/config.toml`
change the content to match you setup (MQTT broker, topic..).

## Compile
You will need [cross](https://github.com/cross-rs/cross). And the
nightly toolchain for musl, as glibc will fail to run on
Libreelec due to linking issues.

```bash
export RUSTFLAGS="-C link-arg=-lgcc -Clink-arg=-static-libgcc"
cross +nightly build --target aarch64-unknown-linux-musl -Z build-std --release
```

## Installation
Copy the compiled binary to `/storage/.bin/tv-notify` and create
a systemd service: `/storage/.config/system.d/tv-notify.service`:
```systemd
[Unit]
Description=TV Display to MQTT
After=network.target

[Service]
LogLevelMax=6
ExecStart=/storage/.bin/tv-notify
TimeoutStopSec=2
Restart=always
RestartSec=10

[Install]
WantedBy=default.target
```