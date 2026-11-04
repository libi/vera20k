# `B.engineer` 工程师：多工程师、伤害、修理、占领科技

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AllowMultiEngineer` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:72 | unscanned | — |
| `EngineerRepairable` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:320 | ok | — |
| `EngineerAlwaysCaptureTech` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:218 | ok | — |
| `EngineerDamage` | Rules | Valueable<double> | 0.0 | src/Ext/Rules/Body.cpp:217 | ok | — |
| `RepairStopOnInsufficientFunds` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:178 | ok | — |
| `MultiEngineer` | Skirmish | — | — | src/Misc/Skirmish.cpp:83 | unscanned | src/app/frontend/cncnet_spawn.rs, src/sim/game_options.rs |

## 钩子（5）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4509B4 | `BuildingClass_UpdateRepair_Funds` | new_in_3.0 | ✓ | 00450630 `BuildingClass::BuildingClass__UpdateRepairAndPower` | decompiled/functions/10014630.c | src/Ext/Building/Hooks.cpp:453 | src/sim/production/production_repair.rs |
| 519D9C | `InfantryClass_UpdatePosition_MultiEngineer` | kept | ✓ | 00519630 `InfantryClass::InfantryClass__PerCellProcess` | decompiled/functions/10025880.c | src/Ext/Infantry/Hooks.cpp:159 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/house_tracking.rs … |
| 519FAF | `InfantryClass_UpdatePosition_EngineerRepairsFriendly` | kept ⚠改过 | ✓ | 00519630 `InfantryClass::InfantryClass__PerCellProcess` | decompiled/functions/100257A0.c | src/Ext/Infantry/Hooks.cpp:43 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/house_tracking.rs … |
| 51E4ED | `InfantryClass_GetActionOnObject_EngineerRepairable` | new_in_3.0 | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10014850.c | src/Ext/Building/Hooks.cpp:490 | src/app/input/context_order.rs |
| 51FA82 | `InfantryClass_GetActionOnCell_EngineerRepairable` | new_in_3.0 | ✓ | 0051F800 `InfantryClass::InfantryClass__What_Action_OnCell` | decompiled/functions/10014810.c | src/Ext/Building/Hooks.cpp:504 | src/sim/c4_plant.rs, src/sim/world/move_cell_input.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10025220 | InfantryExt::GetEngineerEnterEnemyBuildingAction [source_0.A,3.0✓] | 使用键 | decompiled/functions/10025220.c |
| 10070230 | ChronoWarpStateMachine::Update [translated,inferred] | 使用键 | decompiled/functions/10070230.c |
| 1000F2B0 | `BuildingTypeExt::SwitchRubble` | 被钩子调用 | decompiled/functions/1000F2B0.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
