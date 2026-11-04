# `T.spotlight` 探照灯

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（8）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `HasSpotlight` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:205 | check_0A_use | src/rules/native_processing.rs, src/rules/object_type.rs |
| `Spotlight.AttachedTo` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:208 | unscanned | src/rules/ares_spotlight.rs |
| `Spotlight.DisableBlue` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:222 | ok | src/rules/ares_spotlight.rs |
| `Spotlight.DisableColor` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:223 | ok | src/rules/ares_spotlight.rs |
| `Spotlight.DisableGreen` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:221 | ok | src/rules/ares_spotlight.rs |
| `Spotlight.DisableRed` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:220 | ok | src/rules/ares_spotlight.rs |
| `Spotlight.Distance` | TechnoType | int | 1024 | src/Ext/TechnoType/Body.cpp:207 | no_use_found | src/rules/ares_spotlight.rs |
| `Spotlight.StartHeight` | TechnoType | int | 430 | src/Ext/TechnoType/Body.cpp:206 | check_0A_use | src/rules/ares_spotlight.rs |

## 钩子（14）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 435820 | `BuildingLightClass_CTOR` | kept | 00435820 `BuildingLightClass::BuildingLightClass__Constructor` | decompiled/functions/1004EA20.c | src/Ext/Techno/Hooks.Spotlights.cpp:47 | — |
| 435BFA | `BuildingLightClass_Draw_Start` | removed_in_3.0 | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EB30.c | src/Ext/Techno/Hooks.Spotlights.cpp:112 | — |
| 435CD3 | `BuildingLightClass_Draw_Spotlight` | replaces_0.A:SpotlightClass_CTOR | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EAC0.c | src/Recovered/BuildingLightClass.cpp:64 | — |
| 436072 | `BuildingLightClass_Draw_430` | kept | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EAA0.c | src/Ext/Techno/Hooks.Spotlights.cpp:113 | — |
| 4360D8 | `BuildingLightClass_Draw_400` | kept | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EA70.c | src/Ext/Techno/Hooks.Spotlights.cpp:121 | — |
| 4360FF | `BuildingLightClass_Draw_250` | kept | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EA40.c | src/Ext/Techno/Hooks.Spotlights.cpp:129 | — |
| 436459 | `BuildingLightClass_Update` | kept ⚠改过 | 004361D0 `BuildingLightClass::BuildingLightClass__AI` | decompiled/functions/1004EC10.c | src/Ext/Techno/Hooks.Spotlights.cpp:73 | src/sim/world/spotlight.rs |
| 4370C0 | `BuildingLightClass_SDDTOR` | kept | 004370C0 `BuildingLightClass::BuildingLightClass__ScalarDeletingDestructor` | decompiled/functions/1004EBE0.c | src/Ext/Techno/Hooks.Spotlights.cpp:58 | — |
| 441163 | `BuildingClass_Put_DontSpawnSpotlight` | kept | 00440580 `BuildingClass::BuildingClass__Unlimbo` | decompiled/functions/1004EA10.c | src/Ext/Techno/Hooks.Spotlights.cpp:42 | src/sim/base_plan.rs, src/sim/world/lifecycle.rs |
| 5F4FE7 | `ObjectClass_Put` | removed_in_3.0 | 005F4EC0 `ObjectClass::ObjectClass__Unlimbo` | decompiled/functions/1001ADA0.c | src/Ext/Techno/Hooks.Spotlights.cpp | src/sim/production/production_refinery.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs … |
| 6F6D0E | `TechnoClass_Put_1` | kept | 006F6CA0 `TechnoClass::TechnoClass__Unlimbo` | decompiled/functions/1004EDE0.c | src/Ext/Techno/Hooks.Spotlights.cpp:14 | src/sim/house_threat.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs |
| 6F6F20 | `TechnoClass_Put_2` | kept | 006F6CA0 `TechnoClass::TechnoClass__Unlimbo` | decompiled/functions/1004EDE0.c | src/Ext/Techno/Hooks.Spotlights.cpp:28 | src/sim/house_threat.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs |
| 70FBE0 | `TechnoClass_Activate` | kept | 0070FBE0 `TechnoClass::TechnoClass__Reactivate` | decompiled/functions/1004CCB0.c | src/Ext/Techno/Hooks.Deactivated.cpp:107 | src/sim/powered_unit.rs |
| 70FC90 | `TechnoClass_Deactivate` | kept | 0070FC90 `TechnoClass::TechnoClass__OnDeployBegin` | decompiled/functions/1004CDC0.c | src/Ext/Techno/Hooks.Deactivated.cpp:157 | src/sim/mod.rs, src/sim/powered_unit.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1004F300 | TechnoExt::CanWarpTarget [translated,inferred] | 使用键 | decompiled/functions/1004F300.c |
| 1006A6F0 | TechnoExt::Destroy [source_0.A] | 使用键 | decompiled/functions/1006A6F0.c |
| 10019A50 | `BulletExt::ExtData::CreateAttachedSystem` | 被钩子调用 | decompiled/functions/10019A50.c |
| 100454F0 | `TechnoExt::ExtData::IsPowered` | 被钩子调用 | decompiled/functions/100454F0.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10042C10 | `BuildingLightClass::BuildingLightClass` | 被钩子调用 | decompiled/functions/10042C10.c |
| 10046420 | `TechnoExt::ExtData::SetSpotlight` | 被钩子调用 | decompiled/functions/10046420.c |
| 10013250 | `BuildingExt::UpdateFactoryQueues` | 被钩子调用 | decompiled/functions/10013250.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1000ACB0 | `Math::cos` | 被钩子调用 | decompiled/functions/1000ACB0.c |
| 1000ACC0 | `Math::sin` | 被钩子调用 | decompiled/functions/1000ACC0.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |

## 涉及的热点函数（4）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00440580 `BuildingClass::BuildingClass__Unlimbo` | 5 | B.firestorm, B.gates, T.spotlight, ext:Building, recovered:BuildingClass |
| 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | 5 | T.spotlight, recovered:BuildingLightClass |
| 004361D0 `BuildingLightClass::BuildingLightClass__AI` | 2 | T.spotlight, recovered:BuildingLightClass |
| 005F4EC0 `ObjectClass::ObjectClass__Unlimbo` | 2 | T.spotlight, ext:Terrain |
