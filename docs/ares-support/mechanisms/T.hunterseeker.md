# `T.hunterseeker` HunterSeeker 单位参数

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | docs/ares-hunter-seeker.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（13）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `HSBuilding` | Rules | ValueableVector<BuildingTypeClass*>::Read | — | src/Ext/Rules/Body.cpp:267 | check_0A_use | src/rules/native_processing.rs |
| `HunterSeekerAscentSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:270 | check_0A_use | — |
| `HunterSeekerDescendProximity` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:269 | check_0A_use | — |
| `HunterSeekerDescentSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:271 | check_0A_use | — |
| `HunterSeekerDetonateProximity` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:268 | check_0A_use | — |
| `HunterSeekerEmergeSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:272 | check_0A_use | — |
| `HunterSeeker` | Side | Valueable<UnitTypeClass*> | nullptr | src/Ext/Side/Body.cpp:132 | check_0A_use | src/rules/native_processing.rs, src/rules/object_type.rs, src/rules/superweapon_targeting.rs … |
| `HunterSeeker.AscentSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:438 | check_0A_use | — |
| `HunterSeeker.DescendProximity` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:437 | check_0A_use | — |
| `HunterSeeker.DescentSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:439 | check_0A_use | — |
| `HunterSeeker.DetonateProximity` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:436 | check_0A_use | — |
| `HunterSeeker.EmergeSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:440 | check_0A_use | — |
| `HunterSeeker.Ignore` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:441 | no_use_found | src/rules/superweapon_influence.rs |

## 钩子（8）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4CCB84 | `FlyLocomotionClass_ILocomotion_Process_HunterSeeker` | kept | 004CCB40 `FlyLocomotionClass__Process` | decompiled/functions/1004D7A0.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:10 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CD9C8 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_UpdateTarget` | kept | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004DA90.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:181 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CDE64 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_Ascent` | kept | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004D930.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:222 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CDF54 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_Descent` | kept | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004DA20.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:256 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CE85A | `FlyLocomotionClass_UpdateLanding` | kept | 004CE840 `FlyLocomotionClass::FlyLocomotionClass__Process_Landing` | decompiled/functions/1004D8A0.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:35 | src/sim/movement/fly_height.rs, src/sim/world/object_entry.rs |
| 4CF3D0 | `FlyLocomotionClass_sub_4CEFB0_HunterSeeker` | kept | 004CEFB0 `FlyLocomotionClass::FlyLocomotionClass__Horizontal_Step` | decompiled/functions/1004DC30.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:63 | src/sim/world/fly_process.rs |
| 4CFE80 | `FlyLocomotionClass_ILocomotion_AcquireHunterSeekerTarget` | kept | 004CFE80 `FlyLocomotionClass__Acquire_Hunter_Seeker_Target` | decompiled/functions/1004D770.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:277 | — |
| 4D8D95 | `FootClass_UpdatePosition_HunterSeeker` | kept | 004D85D0 `FootClass::FootClass__PerCellProcess` | decompiled/functions/1004E050.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:290 | src/sim/movement/per_cell.rs, src/sim/world/techno_ai_cloak.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F490.c |
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1002D410.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred] | 使用键 | decompiled/functions/1002E060.c |
| 10030570 | SWTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/10030570.c |
| 10072850 | FUN_10072850 [source_0.A] | 使用键 | decompiled/functions/10072850.c |
| 10042E40 | `TechnoExt::ExtData::AcquireHunterSeekerTarget` | 被钩子调用 | decompiled/functions/10042E40.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |
| 1000ACB0 | `Math::cos` | 被钩子调用 | decompiled/functions/1000ACB0.c |
| 1000ACC0 | `Math::sin` | 被钩子调用 | decompiled/functions/1000ACC0.c |
| 1004D750 | `MapClass::MapClass__CoordinatesLegal` | 被钩子调用 | decompiled/functions/1004D750.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004D85D0 `FootClass::FootClass__PerCellProcess` | 2 | T.hunterseeker, ext:Techno |
