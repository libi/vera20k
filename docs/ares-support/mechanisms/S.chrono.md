# `S.chrono` ChronoSphere/ChronoWarp

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（10）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Chronosphere.AffectsIronCurtain` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:71 | no_use_found | — |
| `Chronosphere.AffectsUndeployable` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:73 | no_use_found | — |
| `Chronosphere.AffectsUnwarpable` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:72 | ok | — |
| `Chronosphere.BlastDest` | SWType | Nullable<AnimTypeClass *> | — | src/Misc/SWTypes/ChronoSphere.cpp:78 | ok | — |
| `Chronosphere.BlastSrc` | SWType | Nullable<AnimTypeClass *> | — | src/Misc/SWTypes/ChronoSphere.cpp:77 | ok | — |
| `Chronosphere.BlowUnplaceable` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:74 | ok | — |
| `Chronosphere.KillCargo` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:70 | no_use_found | — |
| `Chronosphere.KillOrganic` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:68 | ok | — |
| `Chronosphere.KillTeleporters` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:69 | no_use_found | — |
| `Chronosphere.ReconsiderBuildings` | SWType | Valueable<bool> | false | src/Misc/SWTypes/ChronoSphere.cpp:75 | no_use_found | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1006FB60 | SW_ChronoWarp::Activate [translated] | 使用键 | decompiled/functions/1006FB60.c |
| 10070230 | ChronoWarpStateMachine::Update [translated] | 使用键 | decompiled/functions/10070230.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 100323C0 | `SWTypeExt::ClearChronoAnim` | 被钩子调用 | decompiled/functions/100323C0.c |
| 10070050 | `CellClass::GetCoordsWithBridge` | 被钩子调用 | decompiled/functions/10070050.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10032980 | `SWTypeExt::ExtData::GetRange` | 被钩子调用 | decompiled/functions/10032980.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
