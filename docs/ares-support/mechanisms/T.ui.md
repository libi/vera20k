# `T.ui` 敌方显示名、范围指示圈、透明度、扳手图标

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（5）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AlliedSolidTransparency` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:164 | ok | — |
| `EnemyWrench` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:220 | ok | — |
| `CivilianEnemy` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:447 | ok | — |
| `EnemyUIName` | TechnoType | Nullable<CSFText> | (empty) | src/Ext/TechnoType/Body.cpp:499 | ok | — |
| `RadialIndicatorRadius` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:567 | ok | — |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 41BE80 | `ObjectClass_DrawRadialIndicator` | new_in_3.0 | ✓ | 0041BE80 `ObjectClass::ObjectClass__DrawRadialIndicator` | decompiled/functions/10049A70.c | src/Ext/Techno/Hooks.cpp:3161 | — |
| 4566B0 | `BuildingClass_GetRangeOfRadial_Radius` | new_in_3.0 | ✓ | 004566B0 `BuildingClass::BuildingClass__GetSensorRange` | decompiled/functions/10048E60.c | src/Ext/Techno/Hooks.cpp:3143 | src/sim/world/gap_generator.rs |
| 6F526C | `TechnoClass_DrawExtras_PowerOff` | removed_in_3.0 | — | 006F5190 `TechnoClass::TechnoClass__DrawExtras` | decompiled/functions/10049FE0.c | src/Ext/Techno/Hooks.cpp:2116 | src/app/presentation/render/draw_plan_lowering.rs |
| 6F826E | `TechnoClass_CanAutoTargetObject_CivilianEnemy` | kept | ✓ | 006F7CA0 `TechnoClass::TechnoClass__Evaluate_Candidate` | decompiled/functions/10049D00.c | src/Ext/Techno/Hooks.cpp:1964 | src/sim/combat/greatest_threat.rs, src/sim/combat/threat_range.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1006E540 | AresTrajectoryHelper::GetObstacle [source_0.A,3.0✓] | 使用键 | decompiled/functions/1006E540.c |
| 10048790 | `CellClass::IsShrouded` | 被钩子调用 | decompiled/functions/10048790.c |
| 10009EE0 | `TacticalClass::CoordsToClient` | 被钩子调用 | decompiled/functions/10009EE0.c |
| 10016C80 | `GameOptionsClass::GetAnimSpeed` | 被钩子调用 | decompiled/functions/10016C80.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1006E7C0 | `BuildingClass::IsTraversable` | 被钩子调用 | decompiled/functions/1006E7C0.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |
