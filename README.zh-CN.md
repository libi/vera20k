# VERA Mod Engine

[English](README.md) · **简体中文** · [Deutsch](README.de.md)

VERA Mod Engine 是 YuriPlanet 基于 VERA20K 独立维护的 Rust 引擎分支，面向《红色警戒 2》的 MOD 和任务地图作者。项目的目标是让制作组保留某一版游戏的规则、科技树和战役，通过 INI 单独改写一种玩法。例如，RA2 MOD 可以只给一个弹头添加 Ares 风格的减速效果，其余武器仍按 RA2 原有的规则结算。

**开发状态。** 仓库已提供 VERA20K 引擎源码、设计规格和三份可供工具查询的索引，分别覆盖零售版 INI、原版程序读取的键名和 Ares 手册条目。下文介绍的兼容预设和新 INI 语法处于设计阶段。

## 兼容预设

每套预设规定本局加载哪些文件、如何读取 INI 标签，以及任务地图可用的事件和动作编号。按设计，三套预设共用一套单位运动、武器发射、伤害、附加效果（AE）和地图触发系统。

| 预设 | 规则与任务 | 兼容性对照 |
|---|---|---|
| 红色警戒 2（RA2） | `rules.ini`、`art.ini`、`ai.ini` 及 RA2 地图 | 零售版 RA2 |
| 尤里的复仇（YR） | `rulesmd.ini`、`artmd.ini`、`aimd.ini` 及 YR 地图，按模式和地图的覆盖顺序读取 | 零售版 YR 战役与遭遇战 |
| Ares | 在 YR 定义上补入指定 Ares 版本的 INI 标签、触发编号和默认行为修订 | 分别对照 YR 1.001 搭配 Ares 3.0、3.0p1 的表现 |

在拟议的 INI 里，一条触发规则由事件、过滤器和效果器组成。例如，电场可以给进入区域的步兵和弹体临时挂上 `Tesla` 标记。检测区的另一条触发只看这个标记，在标记有效期间便能选中这两类对象。

首批制作工具包括手写 INI 所用的字段字典和织网者（INIWeaver）模板。MOD 自编 WebAssembly 模块安排在后续阶段。

## 示例 · 指定炮弹的飞行时间

第一个里程碑在同一张 YR 地图上对照两门炮。一门使用灰熊坦克的原版 `105mm`，另一门保留原弹头，为弹体指定以下拟议的运动方案。

```ini
[Motion demo.HighArc]
Goal=SnapshotAtStart
Path=Line
Pace=FixedTicks(60)
Height=PeakAboveHigherEnd(192lepton)
Facing=AlongPath
```

在没有遮挡的情况下，炮弹应在第 60 个逻辑帧到达开火时记录的落点。修改 `192lepton` 可以单独调整弧顶高出较高端点的距离，飞行时间仍为 60 帧。提前撞桥时应只引爆一次。半空存档再读回，命中帧应与连续运行一致。[完整武器与弹体配置](docs/zh-CN/mod-engine-product-guide.md#配置示例--分别设定飞行时间与弧高)

## 项目文档

当前引擎的 MOD 配置、构建与测试工具已有英文指南。[MOD 制作指南](docs/modding.md) · [开发工具](tools/README.md)

以下为本项目的中文设计文档。

- [产品说明书与 INI 配置示例](docs/zh-CN/mod-engine-product-guide.md)
- [INI 兼容手册与来源索引](docs/zh-CN/ini-preset-wrapping.md)
- [运动、规则和效果系统规格](docs/zh-CN/composable-gameplay-design.md)
- [实施路线图与验收条件](docs/zh-CN/composable-gameplay-roadmap.md)
- [上游 RFC #4653 参与者建议、作者回复与待定问题](docs/zh-CN/rfc-4653-feedback.md)

## 编译与运行

当前桌面客户端从本机《尤里的复仇》安装目录读取游戏素材。构建需要 Git 和通过 rustup 安装的稳定版 Rust。Windows 使用 Visual Studio C++ 生成工具，macOS 使用 Xcode 命令行工具。Debian 或 Ubuntu 需安装 `build-essential`、`pkg-config`、`libasound2-dev` 和 `libxkbcommon-x11-0`。[Rust 工具链](rust-toolchain.toml)

```sh
git clone https://github.com/YuriPlanet/vera-mod-engine.git
cd vera-mod-engine
```

将 `config.toml.example` 复制为 `config.toml`，在 `[paths]` 节把 `ra2_dir` 设为游戏安装目录。然后从仓库根目录启动客户端。

```sh
cargo run --locked --profile fast --bin vera20k
```

## 参与贡献

报告兼容问题时，请标明游戏和扩展版本，附上能复现问题的最小 INI 片段或 FinalAlert 2（FA2）触发，并说明原版与本分支各自的实际表现。提交引擎改动时，请写明改动影响的行为及用于验证的场景。[项目 Issues](https://github.com/YuriPlanet/vera-mod-engine/issues)

## 许可证与署名

版权所有 © 2026 Halvor Nikolai Rusten 及 VERA20K 贡献者。引擎代码采用 GPLv3 及项目附加条款。Command & Conquer 和 Red Alert 为 Electronic Arts Inc. 的商标。[上游项目](https://github.com/YuriPlanet/vera20k) · [GPLv3 许可证](LICENSE-GPL) · [法律声明与附加条款](LEGAL.md#additional-terms)

This product includes software developed by the Ares Contributors.
This product includes software developed by the Phobos Developers. [Ares 许可证](LICENSES/Ares.txt) · [Phobos 开发者名单](docs/phobos.md#original-project-and-authors)
