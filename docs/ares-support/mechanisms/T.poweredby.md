# `T.poweredby` PoweredBy 供电单位

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M6 | T.emp | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DeactivateDimPowered` | Rules | Valueable<double> | 0.5 | src/Ext/Rules/Body.cpp:148 | ok | — |
| `PoweredBy` | TechnoType | ValueableVector<BuildingTypeClass*> | — | src/Ext/TechnoType/Body.cpp:352 | ok | — |

## 钩子（19）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 417CCB | `AircraftClass_GetActionOnObject_Deactivated` | replaces_0.A:AircraftClass_GetCursorOverObject_Deactivated | ✓ | 00417CC0 `AircraftClass::AircraftClass__What_Action_OnObject` | decompiled/functions/1004CB00.c | src/Ext/Techno/Hooks.Deactivated.cpp:166 | src/app/input/context_order.rs, src/sim/world/world_orders.rs |
| 417F83 | `AircraftClass_GetActionOnCell_Deactivated` | replaces_0.A:AircraftClass_GetCursorOverCell_Deactivated | ✓ | 00417F80 `AircraftClass::AircraftClass__What_Action_OnCell` | decompiled/functions/1004CAE0.c | src/Ext/Techno/Hooks.Deactivated.cpp:150 | src/app/input/context_order.rs, src/sim/world/move_cell_input.rs, src/sim/world/world_orders.rs |
| 4436F7 | `BuildingClass_ActionOnCell_Deactivated` | replaces_0.A:BuildingClass_140_Deactivated | ✓ | 004436F0 `BuildingClass::BuildingClass__Active_Click_With_Cell` | decompiled/functions/1004CB30.c | src/Ext/Techno/Hooks.Deactivated.cpp:210 | src/app/input/context_order.rs, src/app/input/planning_mode.rs, src/sim/production/production_sell.rs … |
| 447218 | `BuildingClass_GetActionOnObject_Deactivated` | replaces_0.A:BuildingClass_GetCursorOverObject_Deactivated | ✓ | 00447210 `BuildingClass::BuildingClass__What_Action_OnObject` | decompiled/functions/1004CB70.c | src/Ext/Techno/Hooks.Deactivated.cpp:62 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/world/gap_generator.rs … |
| 447548 | `BuildingClass_GetActionOnCell_Deactivated` | replaces_0.A:BuildingClass_GetCursorOverCell_Deactivated | ✓ | 00447540 `BuildingClass::BuildingClass__What_Action_OnCell` | decompiled/functions/1004CB50.c | src/Ext/Techno/Hooks.Deactivated.cpp:48 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/world/move_cell_input.rs |
| 4D74EC | `FootClass_ActionOnObject_Deactivated` | replaces_0.A:FootClass_ClickedAction_Deactivated | ✓ | 004D74E0 `FootClass::FootClass__ClickedAction_Object` | decompiled/functions/1004CBC0.c | src/Ext/Techno/Hooks.Deactivated.cpp:185 | src/app/input/autoload.rs, src/app/input/commands.rs, src/app/input/context_order.rs … |
| 4D7D58 | `FootClass_ActionOnCell_Deactivated` | replaces_0.A:FootClass_140_Deactivated | ✓ | 004D7D50 `FootClass::FootClass__ClickedAction_Cell` | decompiled/functions/1004CBA0.c | src/Ext/Techno/Hooks.Deactivated.cpp:201 | src/app/input/commands.rs, src/app/input/context_order.rs, src/sim/c4_plant.rs … |
| 51D0DD | `InfantryClass_Scatter` | kept | ✓ | 0051D0D0 `InfantryClass::InfantryClass__Scatter` | decompiled/functions/1004CC60.c | src/Ext/Techno/Hooks.Deactivated.cpp:227 | src/sim/infantry.rs, src/sim/movement/infantry_scatter.rs, src/sim/movement/scatter.rs … |
| 51E440 | `InfantryClass_GetActionOnObject_Deactivated` | replaces_0.A:InfantryClass_GetCursorOverObject_Deactivated | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/1004CC30.c | src/Ext/Techno/Hooks.Deactivated.cpp:132 | src/app/input/context_order.rs |
| 51F808 | `InfantryClass_GetActionOnCell_Deactivated` | replaces_0.A:InfantryClass_GetCursorOverCell_Deactivated | ✓ | 0051F800 `InfantryClass::InfantryClass__What_Action_OnCell` | decompiled/functions/1004CC10.c | src/Ext/Techno/Hooks.Deactivated.cpp:114 | src/sim/c4_plant.rs, src/sim/world/move_cell_input.rs |
| 5200B3 | `InfantryClass_UpdatePanic` | kept | ✓ | 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | decompiled/functions/1004CC80.c | src/Ext/Techno/Hooks.Deactivated.cpp:215 | src/sim/infantry.rs |
| 687C16 | `INIClass_ReadScenario_ValidateThings` | kept ⚠改过 | ✓ | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/100677B0.c | src/Misc/Invalidators.cpp:67 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |
| 70FBE0 | `TechnoClass_Activate` | kept | ✓ | 0070FBE0 `TechnoClass::TechnoClass__Reactivate` | decompiled/functions/1004CCB0.c | src/Ext/Techno/Hooks.Deactivated.cpp:291 | src/sim/powered_unit.rs |
| 70FC90 | `TechnoClass_Deactivate` | kept | ✓ | 0070FC90 `TechnoClass::TechnoClass__OnDeployBegin` | decompiled/functions/1004CDC0.c | src/Ext/Techno/Hooks.Deactivated.cpp:342 | src/sim/mod.rs, src/sim/powered_unit.rs |
| 736135 | `UnitClass_Update_Deactivated` | kept | ✓ | 007360C0 `UnitClass::UnitClass__AI` | decompiled/functions/1004CFD0.c | src/Ext/Techno/Hooks.Deactivated.cpp:254 | src/sim/mission/authority.rs, src/sim/world/object_turn.rs, src/sim/world/unit_repair_seek.rs |
| 73C143 | `UnitClass_DrawVXL_Deactivated` | kept | ✓ | 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | decompiled/functions/1004CEA0.c | src/Ext/Techno/Hooks.Deactivated.cpp:263 | src/app/presentation/instances/units.rs, src/render/vxl_native.rs, src/rules/voxel_frame_catalog.rs |
| 73DBF9 | `UnitClass_Mi_Unload_Decactivated` | kept | ✓ | 0073D630 `UnitClass::UnitClass__Mission_Unload` | decompiled/functions/1004CF80.c | src/Ext/Techno/Hooks.Deactivated.cpp:237 | src/rules/object_type.rs, src/sim/components.rs, src/sim/deploy.rs … |
| 73FD5A | `UnitClass_GetActionOnObject_Deactivated` | replaces_0.A:UnitClass_GetCursorOverObject_Deactivated | ✓ | 0073FD50 `UnitClass::UnitClass__What_Action_OnObject` | decompiled/functions/1004CF50.c | src/Ext/Techno/Hooks.Deactivated.cpp:96 | src/app/input/cursor.rs |
| 7404B9 | `UnitClass_GetActionOnCell_Deactivated` | replaces_0.A:UnitClass_GetCursorOverCell_Deactivated | ✓ | 007404B0 `UnitClass::UnitClass__What_Action_OnCell` | decompiled/functions/1004CF30.c | src/Ext/Techno/Hooks.Deactivated.cpp:80 | src/app/input/context_order.rs, src/sim/world/move_cell_input.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002D410.c |
| 1006A6F0 | TechnoExt::Destroy [source_0.A,3.0✓] | 使用键 | decompiled/functions/1006A6F0.c |
| 10044AD0 | `TechnoExt::GetDeactivatedAction` | 被钩子调用 | decompiled/functions/10044AD0.c |
| 100454F0 | `TechnoExt::ExtData::IsPowered` | 被钩子调用 | decompiled/functions/100454F0.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 10042C10 | `BuildingLightClass::BuildingLightClass` | 被钩子调用 | decompiled/functions/10042C10.c |
| 10046420 | `TechnoExt::ExtData::SetSpotlight` | 被钩子调用 | decompiled/functions/10046420.c |
| 10013250 | `BuildingExt::UpdateFactoryQueues` | 被钩子调用 | decompiled/functions/10013250.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1004CAC0 | `UnitClass::IsDeactivated` | 被钩子调用 | decompiled/functions/1004CAC0.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |
| 100617C0 | `Debug::FatalErrorAndExit` | 被钩子调用 | decompiled/functions/100617C0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1000D430 | `InfantryTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D430.c |
| 100619C0 | `Debug::LogFileClose` | 被钩子调用 | decompiled/functions/100619C0.c |
| 10061A30 | `Debug::LogFileOpen` | 被钩子调用 | decompiled/functions/10061A30.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |
| 10062980 | `EMPulse::EnableEMPEffect2` | 被钩子调用 | decompiled/functions/10062980.c |
| 10062650 | `EMPulse::DisableEMPEffect2` | 被钩子调用 | decompiled/functions/10062650.c |

## 涉及的热点函数（9）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | 13 | B.spy, T.emp, T.poweredby, W.ivanbomb, ext:Building, ext:Infantry, ext:Techno |
| 0073D630 `UnitClass::UnitClass__Mission_Unload` | 5 | T.poweredby, U.bugfixes, ext:Building, ext:House, ext:Techno |
| 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | 4 | T.ammo, T.poweredby, W.death, ext:Infantry |
| 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | 7 | T.misc, T.poweredby, ext:TechnoType |
| 00447210 `BuildingClass::BuildingClass__What_Action_OnObject` | 3 | B.trench, T.poweredby, ext:Building |
| 0073FD50 `UnitClass::UnitClass__What_Action_OnObject` | 5 | T.poweredby, ext:Techno |
| 00417CC0 `AircraftClass::AircraftClass__What_Action_OnObject` | 4 | T.poweredby, ext:Techno |
| 004D74E0 `FootClass::FootClass__ClickedAction_Object` | 2 | F.cursors, T.poweredby |
| 0051F800 `InfantryClass::InfantryClass__What_Action_OnCell` | 2 | T.poweredby, ext:Building |
