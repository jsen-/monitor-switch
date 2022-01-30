# monitor-switch
allows you to switch between multiple *screen & audio output* profiles with single keystroke


## Installation
either download a pre-build binary from [project releases](Releases) or build manually:
```sh
cargo install -Z "build-std=std,panic_abort" -Z "build-std-features=panic_immediate_abort" --target "x86_64-unknown-linux-gnu" --path .
```


## Configuration
`${XDG_CONFIG_HOME:-${HOME}/.config}/monitor-switch/config.toml`

```toml
[profile."work"]
pulseaudio-sink = "alsa_output.usb-SteelSeries_Arctis_Pro_Wireless-00.iec958-stereo"
monitors ="""
<monitors version="2">
[...]
</monitors>"""

[profile."movie"]
pulseaudio-sink = "alsa_output.pci-0000_03_00.1.hdmi-stereo"
monitors ="""
<monitors version="2">
[...]
</monitors>"""

[profile."disco"]
...
```
[full example of config.toml](config.example.toml)


## State file
`${XDG_CACHE_HOME:-${HOME}/.cache}/monitor-switch/state{,.tmp}`

just fyi, feel free to ignore those


## Usage
```
monitor-switch
```
Will read the last applied profile from state file and apply the next profile defined in config.
If current profile cannot be determined the first profile will be applied.


## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
