---
layout: default
title: VERA Mod Engine 产品说明书
lang: zh-CN
---

# VERA Mod Engine 产品说明书

VERA Mod Engine 是 YuriPlanet 面向《红色警戒 2》MOD 和任务地图制作的 VERA20K 开发分支。它的设计目标是让 RA2、尤里的复仇（YR）和 Ares 三套预设从同一套引擎派生。制作组选择预设以确定原版单位、规则和剧本的读法，再用 INI 配置自己的单位运动、弹道、附加效果（AE）和地图触发。

**当前状态为设计草案。** 仓库已收录 VERA 源码、玩法规格、实施路线图和三份 INI 来源索引。下文的具名节、模板、导入报告及验收结果描述计划交付的功能。项目沿用上游 GPL-3.0 许可证，原版游戏素材由玩家的本地安装提供。[源码起点](https://github.com/YuriPlanet/vera20k/commit/4b9af402654fd61ca6338aca9502071e6e2d4063) · [许可证](../../LICENSE-GPL)

## 选择基础预设

| 制作目标 | 预设与输入 | 验收基准 |
|---|---|---|
| 延续 RA2 的国家、科技树和任务 | 选择 `ra2`，使用 RA2 的 `rules.ini`、`art.ini`、`ai.ini` 和对应地图。自定义 MOD 可单独选用已经接入的扩展玩法 | 用 RA2 原版程序和相同地图核对。给自定义坦克加减速 AE，另开纯 RA2 局检查原有单位速度 |
| 制作 YR 遭遇战或战役 | 选择 `yr`，使用 `rulesmd.ini`、`artmd.ini`、`aimd.ini` 和 YR 地图。模式和地图按该产品的读取顺序覆盖规则 | 优先验证零售 YR 的遭遇战和战役，包括任务胜负、援军、存读档 |
| 迁移既有 Ares MOD | 选择相应版本的 Ares 预设。复用 YR 的共同定义，加上该版新增的标签、事件、动作及默认生效的行为修订 | 以 YR 1.001 加对应版本的 Ares 为参照。3.0 与 3.0p1 分别记录差异 |

Phobos 弹道和 AE、KratosPP 的单位与弹体挂载提供额外的旧写法和行为参考。每项兼容都需注明目标版本和已验证场景。第三方地图与战役采用逐项导入报告。[INI 兼容性手册](ini-preset-wrapping.md)

## 配置入口

一个新包声明自己的名称、格式和基础预设。以下为拟议语法，示例包名为 `demo`。

```ini
[Package]
Id=demo
Format=1
Base=yr
```

| 配置对象 | 主要用途 | 制作时需要决定的内容 |
|---|---|---|
| 运动方案 `Motion` | 描述一段位移，单位和在途弹体可引用同一方案 | 目标何时取样、路径、速度或耗时、高度、朝向。使用方另定碰撞、落地或引爆 |
| 对象标签 `Flag` | 供过滤器筛选当前对象，例如 `Infantry`、`Hero`、`Tesla` | 类型可带默认标签。每个效果记录自己授予或屏蔽的标签，到期时撤销该来源的贡献 |
| 规则 `Rule` | 将一次事件、过滤条件和效果器接在一起 | `On` 选触发时机，`Where` 筛对象，`Do` 指定命中条件后执行的动作 |
| 持续行为 `Behavior` | 表达有时长、来源、刷新或叠层规则的效果，包括 AE | 写明持续帧数、同一来源重复施加的处理，以及对象离场时如何清理 |
| FA2 触发与 Tag | 保留老地图的事件、动作、触发实例和绑定 | 使用该预设的数字编号。FA2 的 `[Tags]` 绑定触发，`Flag` 描述对象当前可用于筛选的属性 |

新定义按包名和节名发现，可以先引用后定义。手写 INI 和织网者模板共用字段字典。导入旧包时按旧读法解释 `[InfantryTypes]` 等注册表，保留原编号、注册顺序以及地图中的引用。[玩法系统规格](composable-gameplay-design.md)

## 配置示例 · 分别设定飞行时间与弧高

以 YR 灰熊坦克的 `105mm` 为基准。在测试图保留使用原炮的单位，给第二个单位绑定示例武器 `demo.ArcGun`。新武器继承原弹头，使用新的弹体。运动方案在开火时记录目标坐标。

```ini
[Motion demo.HighArc]
Goal=SnapshotAtStart
Path=Line
Pace=FixedTicks(60)
Height=PeakAboveHigherEnd(192lepton)
Facing=AlongPath

[Actor demo.SlowShell]
From=yr.Projectile.Cannon
Flag.Add=Interceptable
Motion=demo.HighArc
Motion.Contact=Sweep(Terrain,Bridge)
Motion.OnContact=DetonatePayload
Motion.OnArrive=DetonatePayload

[Weapon demo.ArcGun]
From=yr.Weapon.105mm
Projectile=demo.SlowShell
```

| 参数或工况 | 规格要求 |
|---|---|
| `FixedTicks(60)` | 无遮挡时从起点飞至采样落点共 60 个逻辑帧。目标中途离开，落点仍取开火时的坐标 |
| `PeakAboveHigherEnd(192lepton)` | 每格 256 lepton。起点高度 0、终点高度 128 时，第 30 帧顶高为 320。将参数改为 64 后，顶高为 192，抵达仍为第 60 帧 |
| 途中与桥面接触 | 提前在接触点引爆。接触与到达落在同一帧只引爆一次 |
| 飞行中存档 | 读档后保留起点、采样落点和进度，首次接触、引爆帧和伤害与连续运行一致 |

地图作者可以把同一 `demo.HighArc` 用于步兵跳跃。能力负责预留落点，步兵落地后交回占格和原任务，炮弹沿用自己的引信、接触和弹头。完整示例包含两个独立电场、绝缘区、减速弹头和 FA2 援军触发。[实施路线图](composable-gameplay-roadmap.md)

## 导入旧包与核对差异

1. **确定参照版本。** 记录基础游戏、Ares 等扩展的版本、规则文件、模式和地图。相同键的含义取决于节和读取位置，例如武器节的 `ROF` 是开火间隔，难度节的 `ROF` 是倍率。
2. **核对本局生效值。** 导入报告应定位到文件、节、键或地图触发编号，给出覆盖它的规则层、最终值和使用它的子系统。遇到未知地图动作时，报告应指出所属触发和动作序号。
3. **在目标场景验收。** 武器需检查开火、引爆和掉血。任务需检查触发、援军和胜负。兼容清单分别记录已在场景核对、仅可读取或保留、已知差异、待实现或待验证的条目。
4. **按目标格式导出。** 原样导回同版本时，应按字节保留未改的原文及地图压缩块。导回旧弹道时，应列出原格式无法表达的独立弧高和飞行时长，供作者决定是否采用近似配置。

三份可查询的索引分别收录零售档案的字段与记录、两款原版程序的字面键读点，以及 Ares 手册的字段与数字指令。具体标记为支持之前，还要核对其上下文、默认值和实际效果。[零售来源](ini-preset-wrapping/retail-index.json) · [RA2/YR 读点](ini-preset-wrapping/native-reader-index.json) · [Ares 条目](ini-preset-wrapping/ares-index.json)

## 开发范围与反馈

首阶段以一张 YR 遭遇战地图验证原版炮和可调慢弧线炮。后续里程碑覆盖步兵跳跃、标签与持续效果、零售任务和版本化预设。模组自编 Wasm 安排在 INI 玩法验收之后。上游 RFC 与独立 fork 分别维护。模块的连锁超限和脚本失败处理仍需对齐。[里程碑与验收](composable-gameplay-roadmap.md) · [上游 RFC 参与者建议](rfc-4653-feedback.md)

报告兼容问题时，请附目标产品及版本、最小 INI 或 FA2 触发片段、原版表现和实际表现。涉及地图覆盖或存读档时，请注明模式、触发顺序或存档时点。[项目 Issues](https://github.com/YuriPlanet/vera-mod-engine/issues)
