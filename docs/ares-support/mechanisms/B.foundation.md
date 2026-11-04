# `B.foundation` 自定义地基与实体高度

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Foundation` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:106 | unscanned | src/app/input/dispatch.rs, src/app/presentation/selection_brackets.rs, src/headless_scenario.rs … |
| `Foundation.X` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:113 | unscanned | src/rules/foundation.rs |
| `Foundation.Y` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:114 | unscanned | src/rules/foundation.rs |
| `FoundationOutline.Length` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:115 | unscanned | src/rules/foundation.rs |
| `SolidHeight` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:98 | ok | — |
| `SolidLevel` | BuildingType | Valueable<int> | 1 | src/Ext/BuildingType/Body.cpp:102 | ok | — |
| `SolidLevel` | BulletType | Valueable<int> | 0 | src/Ext/BulletType/Body.cpp:26 | ok | — |

## 钩子（12）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 45EC90 | `BuildingTypeClass_GetFoundationWidth` | replaces_0.A:Foundations_GetFoundationWidth | ✓ | 0045EC90 `BuildingTypeClass::BuildingTypeClass__GetFoundationWidth` | decompiled/functions/10015310.c | src/Ext/Building/Hooks.Foundation.cpp:76 | src/render/radar/visibility.rs, src/sim/ai_base_site.rs, src/sim/combat/combat_weapon.rs … |
| 45ECA0 | `BuildingTypeClass_GetFoundationHeight` | replaces_0.A:Foundations_GetFoundationHeight | ✓ | 0045ECA0 `BuildingTypeClass::BuildingTypeClass__GetFoundationHeight` | decompiled/functions/100152D0.c | src/Ext/Building/Hooks.Foundation.cpp:92 | src/render/radar/visibility.rs, src/rules/foundation.rs, src/sim/ai_base_site.rs … |
| 4A8C77 | `DisplayClass_ProcessFoundation1_UnlimitBuffer` | replaces_0.A:MapClass_ProcessFoundation1_UnlimitBuffer | ✓ | 004A8BF0 `DisplayClass::DisplayClass__SetActiveFoundation` | decompiled/functions/10015340.c | src/Ext/Building/Hooks.Foundation.cpp:217 | src/app/cooperative_round.rs |
| 4A8DD7 | `DisplayClass_ProcessFoundation2_UnlimitBuffer` | replaces_0.A:MapClass_ProcessFoundation2_UnlimitBuffer | ✓ | 004A8D50 `DisplayClass::DisplayClass__SetPendingFoundation` | decompiled/functions/100153B0.c | src/Ext/Building/Hooks.Foundation.cpp:240 | — |
| 4CC310 | `TrajectoryHelper_FindFirstImpenetrableObstacle` | kept | ✓ | 004CC310 `TrajectoryHelper__FindFirstImpenetrableObstacle` | decompiled/functions/1006E7E0.c | src/Misc/TrajectoryHelper.cpp:229 | src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs, src/sim/world/techno_ai/target_scan.rs |
| 568411 | `MapClass_AddContentAt_Foundation_P1` | kept | ✓ | 005683C0 `MapClass::MapClass__Place_Down` | decompiled/functions/10015490.c | src/Ext/Building/Hooks.Foundation.cpp:151 | src/sim/combat/shrapnel.rs, src/sim/movement/foot_mark.rs, src/sim/occupancy.rs … |
| 568565 | `MapClass_AddContentAt_Foundation_OccupyHeight` | kept | ✓ | 005683C0 `MapClass::MapClass__Place_Down` | decompiled/functions/10015420.c | src/Ext/Building/Hooks.Foundation.cpp:160 | src/sim/combat/shrapnel.rs, src/sim/movement/foot_mark.rs, src/sim/occupancy.rs … |
| 568841 | `MapClass_RemoveContentAt_Foundation_P1` | kept | ✓ | 005687F0 `MapClass::MapClass__Pick_Up` | decompiled/functions/10015520.c | src/Ext/Building/Hooks.Foundation.cpp:180 | src/sim/movement/foot_mark.rs, src/sim/occupancy.rs, src/sim/passenger.rs … |
| 568997 | `MapClass_RemoveContentAt_Foundation_OccupyHeight` | kept | ✓ | 005687F0 `MapClass::MapClass__Pick_Up` | decompiled/functions/100154B0.c | src/Ext/Building/Hooks.Foundation.cpp:189 | src/sim/movement/foot_mark.rs, src/sim/occupancy.rs, src/sim/passenger.rs … |
| 6563B0 | `RadarClass_UpdateFoundationShapes_Custom` | kept | ✓ | 006563B0 `RadarClass::RadarClass__GenerateBrushShapes` | decompiled/functions/10015580.c | src/Ext/Building/Hooks.Foundation.cpp:140 | src/render/radar/tracker.rs |
| 656584 | `RadarClass_GetFoundationShape` | replaces_0.A:MapClass_GetFoundationShape | ✓ | 00656580 `RadarClass::RadarClass__GetFoundationPixels` | decompiled/functions/10015540.c | src/Ext/Building/Hooks.Foundation.cpp:117 | — |
| 6D50FB | `sub_6D5030_CustomFoundation` | new_in_3.0 | ✓ | 006D5030 `Tactical::BuildingPlacement_OverlayRenderer` | decompiled/functions/100155C0.c | src/Ext/Building/Hooks.Foundation.cpp:265 | src/app/presentation/render/draw_passes.rs, src/render/selection_overlay.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DFB0 | FUN_1002dfb0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002DFB0.c |
| 10057C60 | thunk_FUN_10057be0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10057C60.c |
| 1006E1F0 | FUN_1006e1f0 [source_0.A,3.0✓] | 使用键 | decompiled/functions/1006E1F0.c |
| 1006E540 | AresTrajectoryHelper::GetObstacle [source_0.A,3.0✓] | 使用键 | decompiled/functions/1006E540.c |
| 100152B0 | `DisplayClass::FoundationBoundsSize` | 被钩子调用 | decompiled/functions/100152B0.c |
| 10011B00 | `BuildingExt::GetCoveredCells` | 被钩子调用 | decompiled/functions/10011B00.c |
| 1000F510 | `BuildingTypeExt::ExtData::UpdateFoundationRadarShape` | 被钩子调用 | decompiled/functions/1000F510.c |
| 1006E2B0 | `AresTrajectoryHelper::FindFirstObstacle` | 被钩子调用 | decompiled/functions/1006E2B0.c |
| 10026410 | `CellClass::ConnectsToOverlay` | 被钩子调用 | decompiled/functions/10026410.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1006E7C0 | `BuildingClass::IsTraversable` | 被钩子调用 | decompiled/functions/1006E7C0.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006D5030 `Tactical::BuildingPlacement_OverlayRenderer` | 3 | B.firestorm, B.foundation |
