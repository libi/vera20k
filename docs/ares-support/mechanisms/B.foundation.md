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
| `Foundation` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:112 | unscanned | src/app/input/dispatch.rs, src/app/presentation/selection_brackets.rs, src/headless_scenario.rs … |
| `Foundation.X` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:119 | unscanned | src/rules/foundation.rs |
| `Foundation.Y` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:120 | unscanned | src/rules/foundation.rs |
| `FoundationOutline.Length` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:121 | unscanned | src/rules/foundation.rs |
| `SolidHeight` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:104 | check_0A_use | — |
| `SolidLevel` | BuildingType | Valueable<int> | 1 | src/Ext/BuildingType/Body.cpp:108 | check_0A_use | — |
| `SolidLevel` | BulletType | Valueable<int> | 0 | src/Ext/BulletType/Body.cpp:26 | check_0A_use | — |

## 钩子（6）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4CC310 | `TrajectoryHelper_FindFirstImpenetrableObstacle` | kept | 004CC310 `TrajectoryHelper__FindFirstImpenetrableObstacle` | decompiled/functions/1006E7E0.c | src/Misc/TrajectoryHelper.cpp:226 | src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs, src/sim/world/techno_ai/target_scan.rs |
| 568411 | `MapClass_AddContentAt_Foundation_P1` | kept | 005683C0 `MapClass::MapClass__Place_Down` | decompiled/functions/10015490.c | src/Ext/Building/Hooks.Foundation.cpp:22 | src/sim/combat/shrapnel.rs, src/sim/movement/foot_mark.rs, src/sim/occupancy.rs … |
| 568565 | `MapClass_AddContentAt_Foundation_OccupyHeight` | kept | 005683C0 `MapClass::MapClass__Place_Down` | decompiled/functions/10015420.c | src/Ext/Building/Hooks.Foundation.cpp:31 | src/sim/combat/shrapnel.rs, src/sim/movement/foot_mark.rs, src/sim/occupancy.rs … |
| 568841 | `MapClass_RemoveContentAt_Foundation_P1` | kept | 005687F0 `MapClass::MapClass__Pick_Up` | decompiled/functions/10015520.c | src/Ext/Building/Hooks.Foundation.cpp:51 | src/sim/movement/foot_mark.rs, src/sim/occupancy.rs, src/sim/passenger.rs … |
| 568997 | `MapClass_RemoveContentAt_Foundation_OccupyHeight` | kept | 005687F0 `MapClass::MapClass__Pick_Up` | decompiled/functions/100154B0.c | src/Ext/Building/Hooks.Foundation.cpp:60 | src/sim/movement/foot_mark.rs, src/sim/occupancy.rs, src/sim/passenger.rs … |
| 6563B0 | `RadarClass_UpdateFoundationShapes_Custom` | kept | 006563B0 `RadarClass::RadarClass__GenerateBrushShapes` | decompiled/functions/10015580.c | src/Ext/Building/Hooks.Foundation.cpp:10 | src/render/radar/tracker.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DFB0 | FUN_1002dfb0 [source_0.A,inferred] | 使用键 | decompiled/functions/1002DFB0.c |
| 10057C60 | thunk_FUN_10057be0 [source_0.A,inferred] | 使用键 | decompiled/functions/10057C60.c |
| 1006E1F0 | FUN_1006e1f0 [source_0.A] | 使用键 | decompiled/functions/1006E1F0.c |
| 1006E540 | AresTrajectoryHelper::GetObstacle [source_0.A] | 使用键 | decompiled/functions/1006E540.c |
| 10011B00 | `BuildingExt::GetCoveredCells` | 被钩子调用 | decompiled/functions/10011B00.c |
| 1000F510 | `BuildingTypeExt::ExtData::UpdateFoundationRadarShape` | 被钩子调用 | decompiled/functions/1000F510.c |
| 1006E2B0 | `AresTrajectoryHelper::FindFirstObstacle` | 被钩子调用 | decompiled/functions/1006E2B0.c |
| 10026410 | `CellClass::ConnectsToOverlay` | 被钩子调用 | decompiled/functions/10026410.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1006E7C0 | `BuildingClass::IsTraversable` | 被钩子调用 | decompiled/functions/1006E7C0.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |
