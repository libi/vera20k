# VERA Mod Engine

**English** · [简体中文](README.zh-CN.md) · [Deutsch](README.de.md)

VERA Mod Engine is a Rust fork of VERA20K for *Red Alert 2* mod and mission authors, maintained independently by YuriPlanet. Its goal is to let authors keep a game’s rules, tech tree and missions while changing individual mechanics in INI. A mod based on RA2 could, for example, add an Ares-style slowing effect to one warhead and keep the rest of its combat rules intact.

**Project status.** The repository contains the VERA20K engine, design specifications and three machine-readable indexes covering retail INI files, keys read by the original executables, and the Ares manual. The new presets and INI syntax described below are at the design stage.

## Compatibility targets

Each preset defines which files to load, how to interpret INI keys, and which numbered events and actions a mission can use. All three are designed to share movement, weapon fire, damage, attached effects (AE) and map triggers.

| Preset | Rules and missions | Compatibility reference |
|---|---|---|
| Red Alert 2 | `rules.ini`, `art.ini`, `ai.ini` and RA2 maps | Retail RA2 |
| Yuri’s Revenge | `rulesmd.ini`, `artmd.ini`, `aimd.ini` and YR maps, including mode and map overrides | Retail YR campaigns and skirmishes |
| Ares | YR definitions plus the selected Ares version’s INI keys, trigger numbers and changes to default behavior | YR 1.001 with Ares 3.0 or 3.0p1, each a separate reference |

In the proposed INI format, a trigger combines an event, a filter and an effect. For example, an electric field can give infantry and projectiles a temporary `Tesla` flag. A second trigger can then select either while the flag is active.

The first authoring tools are planned around hand-written INI, INIWeaver templates and a field dictionary. Mod-authored WebAssembly modules are a later extension.

## A timed shell trajectory

The first milestone compares the Grizzly Tank’s stock `105mm` gun with a second gun on the same YR map. The second gun keeps the original warhead and gives its projectile the following proposed motion definition.

```ini
[Motion demo.HighArc]
Goal=SnapshotAtStart
Path=Line
Pace=FixedTicks(60)
Height=PeakAboveHigherEnd(192lepton)
Facing=AlongPath
```

With a clear path, the shell should reach the position recorded at launch after 60 simulation ticks. Changing `192lepton` adjusts the peak above the higher endpoint while preserving the flight time. To pass this milestone, a shell that strikes a bridge early must detonate exactly once. Loading a mid-flight save must reproduce the original impact tick. [Full weapon and projectile example](docs/zh-CN/mod-engine-product-guide.md#配置示例--分别设定飞行时间与弧高)

## Documentation

For the current engine, the English guides cover mod setup, local builds and testing. [Modding guide](docs/modding.md) · [Developer tools](tools/README.md)

The design specifications are in Simplified Chinese.

- [Product manual and authoring examples](docs/zh-CN/mod-engine-product-guide.md)
- [INI compatibility reference and source indexes](docs/zh-CN/ini-preset-wrapping.md)
- [Gameplay specification for motion, rules and effects](docs/zh-CN/composable-gameplay-design.md)
- [Implementation roadmap and acceptance criteria](docs/zh-CN/composable-gameplay-roadmap.md)
- [Upstream RFC #4653 proposals, author responses and open questions](docs/zh-CN/rfc-4653-feedback.md)

## Build and run

The current desktop client reads assets from a local installation of *Yuri’s Revenge*. Install Git and stable Rust via rustup. Windows builds use the Visual Studio C++ Build Tools, and macOS builds use the Xcode Command Line Tools. On Debian or Ubuntu, install `build-essential`, `pkg-config`, `libasound2-dev` and `libxkbcommon-x11-0`. [Rust toolchain](rust-toolchain.toml)

```sh
git clone https://github.com/YuriPlanet/vera-mod-engine.git
cd vera-mod-engine
```

Copy `config.toml.example` to `config.toml`. Under `[paths]`, set `ra2_dir` to your game installation directory, then start the client from the repository root.

```sh
cargo run --locked --profile fast --bin vera20k
```

## Contributing

For a compatibility report, include the game and extension versions, the smallest INI section or FinalAlert 2 trigger that reproduces the issue, and what happens in the original game and in this fork. For an engine change, identify the affected behavior and the scenario that verifies it. [Project issues](https://github.com/YuriPlanet/vera-mod-engine/issues)

## License and credits

Copyright © 2026 Halvor Nikolai Rusten and VERA20K contributors. The engine code is distributed under GPLv3 with the project’s additional terms. Command & Conquer and Red Alert are trademarks of Electronic Arts Inc. [Upstream project](https://github.com/YuriPlanet/vera20k) · [GPLv3](LICENSE-GPL) · [Legal notice and additional terms](LEGAL.md#additional-terms)

This product includes software developed by the Ares Contributors.
This product includes software developed by the Phobos Developers. [Ares license](LICENSES/Ares.txt) · [Phobos credits](docs/phobos.md#original-project-and-authors)
