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
| `HSBuilding` | Rules | ValueableVector<BuildingTypeClass*>::Read | — | src/Ext/Rules/Body.cpp:222 | ok | src/rules/native_processing.rs |
| `HunterSeekerAscentSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:225 | ok | — |
| `HunterSeekerDescendProximity` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:224 | ok | — |
| `HunterSeekerDescentSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:226 | ok | — |
| `HunterSeekerDetonateProximity` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:223 | ok | — |
| `HunterSeekerEmergeSpeed` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:227 | ok | — |
| `HunterSeeker` | Side | Valueable<UnitTypeClass*> | nullptr | src/Ext/Side/Body.cpp:137 | ok | src/rules/native_processing.rs, src/rules/object_type.rs, src/rules/superweapon_targeting.rs … |
| `HunterSeeker.AscentSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:442 | ok | — |
| `HunterSeeker.DescendProximity` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:441 | ok | — |
| `HunterSeeker.DescentSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:443 | ok | — |
| `HunterSeeker.DetonateProximity` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:440 | ok | — |
| `HunterSeeker.EmergeSpeed` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:444 | ok | — |
| `HunterSeeker.Ignore` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:445 | ok | src/rules/superweapon_influence.rs |

## 钩子（8）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4CCB84 | `FlyLocomotionClass_ILocomotion_Process_HunterSeeker` | kept | ✓ | 004CCB40 `FlyLocomotionClass__Process` | decompiled/functions/1004D7A0.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:10 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CD9C8 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_UpdateTarget` | kept | ✓ | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004DA90.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:191 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CDE64 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_Ascent` | kept | ✓ | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004D930.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:233 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CDF54 | `FlyLocomotionClass_sub_4CD600_HunterSeeker_Descent` | kept | ✓ | 004CD600 `FlyLocomotionClass::FlyLocomotionClass__UpdateFlightMotion` | decompiled/functions/1004DA20.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:267 | src/sim/movement/air_movement.rs, src/sim/movement/bump_crush.rs, src/sim/world/crash.rs … |
| 4CE85A | `FlyLocomotionClass_UpdateLanding` | kept | ✓ | 004CE840 `FlyLocomotionClass::FlyLocomotionClass__Process_Landing` | decompiled/functions/1004D8A0.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:45 | src/sim/movement/fly_height.rs, src/sim/world/object_entry.rs |
| 4CF3D0 | `FlyLocomotionClass_sub_4CEFB0_HunterSeeker` | kept | ✓ | 004CEFB0 `FlyLocomotionClass::FlyLocomotionClass__Horizontal_Step` | decompiled/functions/1004DC30.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:73 | src/sim/world/fly_process.rs |
| 4CFE80 | `FlyLocomotionClass_ILocomotion_AcquireHunterSeekerTarget` | kept | ✓ | 004CFE80 `FlyLocomotionClass__Acquire_Hunter_Seeker_Target` | decompiled/functions/1004D770.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:288 | — |
| 4D8D95 | `FootClass_UpdatePosition_HunterSeeker` | kept | ✓ | 004D85D0 `FootClass::FootClass__PerCellProcess` | decompiled/functions/1004E050.c | src/Ext/Techno/Hooks.HunterSeeker.cpp:301 | src/sim/movement/per_cell.rs, src/sim/world/techno_ai_cloak.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F490.c |
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002D410.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002E060.c |
| 10030570 | SWTypeExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10030570.c |
| 10072850 | SW_HunterSeeker::Activate [translated] | 使用键 | decompiled/functions/10072850.c |
| 10009730 | `TunnelData::Save` | 被钩子调用 | decompiled/functions/10009730.c |
| 10076390 | `CustomPalette::Save` | 被钩子调用 | decompiled/functions/10076390.c |
| 10076370 | `AresPCXFile::Save` | 被钩子调用 | decompiled/functions/10076370.c |
| 10042E40 | `TechnoExt::ExtData::AcquireHunterSeekerTarget` | 被钩子调用 | decompiled/functions/10042E40.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |
| 10009CC0 | `FacingStruct::current` | 被钩子调用 | decompiled/functions/10009CC0.c |
| 1000ACB0 | `Math::cos` | 被钩子调用 | decompiled/functions/1000ACB0.c |
| 1000ACC0 | `Math::sin` | 被钩子调用 | decompiled/functions/1000ACC0.c |
| 1004D750 | `MapClass::MapClass__CoordinatesLegal` | 被钩子调用 | decompiled/functions/1004D750.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004D85D0 `FootClass::FootClass__PerCellProcess` | 2 | T.hunterseeker, ext:Techno |
