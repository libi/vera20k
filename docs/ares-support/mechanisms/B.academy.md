# `B.academy` Academy 老兵加成与 VeteranBuildings

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | T.promotion | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Academy.AircraftVeterancy` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:295 | no_use_found | — |
| `Academy.BuildingVeterancy` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:297 | no_use_found | — |
| `Academy.Ignore` | BuildingType | ValueableVector<TechnoTypeClass*> | (empty) | src/Ext/BuildingType/Body.cpp:293 | check_0A_use | — |
| `Academy.InfantryVeterancy` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:294 | no_use_found | — |
| `Academy.Types` | BuildingType | ValueableVector<TechnoTypeClass*> | (empty) | src/Ext/BuildingType/Body.cpp:292 | check_0A_use | — |
| `Academy.VehicleVeterancy` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:296 | no_use_found | — |
| `VeteranBuildings` | HouseType | ValueableVector<BuildingTypeClass*> | (empty) | src/Ext/HouseType/Body.cpp:324 | check_0A_use | src/app/presentation/sidebar_render.rs, src/rules/native_processing.rs |

## 钩子（8）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 413FD2 | `AircraftClass_Init_Academy` | kept | 00413F80 `AircraftClass::AircraftClass__InitFromType` | decompiled/functions/10023600.c | src/Ext/House/Hooks.Academy.cpp:98 | — |
| 442D1B | `BuildingClass_Init_Academy` | kept | 00442C40 `BuildingClass::BuildingClass__Init_Managers` | decompiled/functions/100236D0.c | src/Ext/House/Hooks.Academy.cpp:109 | — |
| 445905 | `BuildingClass_Remove_Academy` | kept | 00445880 `BuildingClass::BuildingClass__Limbo` | decompiled/functions/10023690.c | src/Ext/House/Hooks.Academy.cpp:24 | src/sim/base_plan.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs |
| 446366 | `BuildingClass_Place_Academy` | kept | 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | decompiled/functions/10023750.c | src/Ext/House/Hooks.Academy.cpp:10 | src/app/match_runtime/eva_producers.rs, src/sim/building_art_storage.rs, src/sim/building_art_super.rs … |
| 448AB2 | `BuildingClass_ChangeOwnership_Remove_Academy` | kept | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10023690.c | src/Ext/House/Hooks.Academy.cpp:38 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 4491D5 | `BuildingClass_ChangeOwnership_Add_Academy` | kept | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10023650.c | src/Ext/House/Hooks.Academy.cpp:52 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 517D51 | `InfantryClass_Init_Academy` | kept | 00517CC0 `InfantryClass::InfantryClass__InitFromType` | decompiled/functions/10023790.c | src/Ext/House/Hooks.Academy.cpp:69 | — |
| 74689B | `UnitClass_Init_Academy` | kept | 00746810 `UnitClass::UnitClass__InitFromType` | decompiled/functions/100237C0.c | src/Ext/House/Hooks.Academy.cpp:80 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10020750 | HouseExt::ExtData::ApplyAcademy [source_0.A] | 使用键 | decompiled/functions/10020750.c |
| 1003D800 | TechnoTypeExt::ExtData::CameoIsElite [source_0.A] | 使用键 | decompiled/functions/1003D800.c |
| 10022760 | `HouseExt::ExtData::UpdateAcademy` | 被钩子调用 | decompiled/functions/10022760.c |

## 涉及的热点函数（6）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | 9 | B.academy, S.core, ext:Building, ext:BuildingType, ext:Techno, recovered:BuildingClass |
| 00448260 `BuildingClass::BuildingClass__ChangeOwner` | 9 | B.academy, B.prism, B.spy, T.jammer, ext:Building |
| 00413F80 `AircraftClass::AircraftClass__InitFromType` | 3 | B.academy, U.bugfixes, recovered:AircraftClass |
| 00445880 `BuildingClass::BuildingClass__Limbo` | 3 | B.academy, B.firestorm, recovered:BuildingClass |
| 00442C40 `BuildingClass::BuildingClass__Init_Managers` | 2 | B.academy, U.bugfixes |
| 007353C0 `UnitClass::UnitClass__Constructor` | 2 | B.academy, recovered:UnitClass |
