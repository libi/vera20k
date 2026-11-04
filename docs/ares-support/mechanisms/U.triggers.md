# `U.triggers` 新触发动作/事件（TAction/TEvent）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M7 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（21）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4368C9 | `BuildingLightClass_Update_Trigger` | new_in_3.0 | ✓ | 004361D0 `BuildingLightClass::BuildingLightClass__AI` | decompiled/functions/10050550.c | src/Ext/TEvent/Hooks.Raise.cpp:24 | src/sim/world/spotlight.rs |
| 5F57B5 | `ObjectClass_ReceiveDamage_Trigger` | new_in_3.0 | ✓ | 005F5390 `ObjectClass::ObjectClass__ReceiveDamage` | decompiled/functions/100505A0.c | src/Ext/TEvent/Hooks.Raise.cpp:47 | src/sim/combat/damage/receive.rs, src/sim/combat/receiver_call.rs, src/sim/terrain_object.rs |
| 6DD8D7 | `TActionClass_Execute` | kept ⚠改过 | ✓ | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/10039A20.c | src/Ext/TAction/Hooks.cpp:8 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 6E1780 | `TActionClass_PlayAudioAtRandomWP` | kept | ✓ | 006E1780 `TActionClass::TActionClass__PlayAudioAtRandomWP` | decompiled/functions/10039B60.c | src/Ext/TAction/Hooks.Replacements.cpp:11 | — |
| 6E3B60 | `TActionClass_GetMode` | new_in_3.0 | ✓ | 006E3B60 `TActionClass__GetMode` | decompiled/functions/10039AD0.c | src/Ext/TAction/Hooks.cpp:57 | — |
| 6E3EE0 | `TActionClass_GetFlags` | new_in_3.0 | ✓ | 006E3EE0 `TActionClass__GetFlags` | decompiled/functions/10039A90.c | src/Ext/TAction/Hooks.cpp:39 | — |
| 702DD6 | `TechnoClass_RegisterDestruction_Trigger` | new_in_3.0 | ✓ | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/10050650.c | src/Ext/TEvent/Hooks.Raise.cpp:96 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |
| 7032B0 | `TechnoClass_RegisterLoss_Trigger` | new_in_3.0 | ✓ | 00703230 `TechnoClass::TechnoClass__RegisterKill` | decompiled/functions/10050690.c | src/Ext/TEvent/Hooks.Raise.cpp:76 | src/sim/combat/record_kill.rs, src/sim/locomotor_lift.rs |
| 71E7F8 | `TEventClass_CTOR` | kept | ✓ | 0071E6A0 `TEventClass::TEventClass__Constructor` | decompiled/functions/100500F0.c | src/Ext/TEvent/Body.cpp:464 | — |
| 71E949 | `TEventClass_HasOccured` | replaces_0.A:EventClass_HasOccured | ✓ | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/100503E0.c | src/Ext/TEvent/Hooks.cpp:14 | src/sim/world/world_commands.rs |
| 71ED01 | `TEventClass_HasOccured_PlayerAtX2` | replaces_0.A:EventClass_HasOccured_PlayerAtX2 | ✓ | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050460.c | src/Ext/TEvent/Hooks.cpp:57 | src/sim/world/world_commands.rs |
| 71EE79 | `TEventClass_HasOccured_PlayerAtX3` | replaces_0.A:EventClass_HasOccured_PlayerAtX3 | ✓ | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050480.c | src/Ext/TEvent/Hooks.cpp:72 | src/sim/world/world_commands.rs |
| 71F06C | `TEventClass_HasOccured_PlayerAtX1` | replaces_0.A:EventClass_HasOccured_PlayerAtX1 | ✓ | 0071E940 `TEventClass::TEventClass__HasOccurred` | decompiled/functions/10050430.c | src/Ext/TEvent/Hooks.cpp:39 | src/sim/world/world_commands.rs |
| 71F39B | `TEventClass_SaveToINI` | new_in_3.0 | ✓ | 0071F390 `TEventClass::TEventClass__PrepareSaveToINI` | decompiled/functions/10050500.c | src/Ext/TEvent/Hooks.cpp:110 | — |
| 71F683 | `TEventClass_GetFlags` | new_in_3.0 | ✓ | 0071F680 `TEventClass__GetFlags` | decompiled/functions/10050390.c | src/Ext/TEvent/Hooks.cpp:91 | — |
| 71F8C0 | `TEventClass_SaveLoad_Prefix` | kept | ✓ | 0071F930 `TEventClass__Save` | decompiled/functions/10050280.c | src/Ext/TEvent/Body.cpp:480 | — |
| 71F92B | `TEventClass_Load_Suffix` | kept | ✓ | 0071F8C0 `TEventClass__Load` | decompiled/functions/10050150.c | src/Ext/TEvent/Body.cpp:74 | — |
| 71F94A | `TEventClass_Save_Suffix` | kept | ✓ | 0071F930 `TEventClass__Save` | decompiled/functions/100502C0.c | src/Ext/TEvent/Body.cpp:67 | — |
| 71F9C0 | `TEventClass_Persistable` | new_in_3.0 | ✓ | 0071F9C0 `TEventClass::TEventClass__GetStateB` | decompiled/functions/100504D0.c | src/Ext/TEvent/Hooks.cpp:126 | src/sim/trigger_runtime.rs |
| 71FAA6 | `TEventClass_SDDTOR` | kept | ✓ | 0071FA80 `TEventClass::TEventClass__ScalarDeletingDestructor` | decompiled/functions/10050250.c | src/Ext/TEvent/Body.cpp:472 | — |
| 744745 | `UnitClass_RegisterDestruction_Trigger` | new_in_3.0 | ✓ | 00744720 `UnitClass::UnitClass__RecordKill` | decompiled/functions/100506D0.c | src/Ext/TEvent/Hooks.Raise.cpp:117 | src/sim/combat/record_kill.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100396C0 | `TActionExt::Execute` | 被钩子调用 | decompiled/functions/100396C0.c |
| 10039930 | `TActionExt::GetFlags` | 被钩子调用 | decompiled/functions/10039930.c |
| 10039990 | `TActionExt::GetMode` | 被钩子调用 | decompiled/functions/10039990.c |
| 10039B40 | `ScenarioClass::IsDefinedWaypoint` | 被钩子调用 | decompiled/functions/10039B40.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10039B20 | `ScenarioClass::GetWaypointCoords` | 被钩子调用 | decompiled/functions/10039B20.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10057CF0 | `ContainerMapBase::insert` | 被钩子调用 | decompiled/functions/10057CF0.c |
| 1006B770 | `AresStreamReader::RegisterChange` | 被钩子调用 | decompiled/functions/1006B770.c |
| 100617C0 | `Debug::FatalErrorAndExit` | 被钩子调用 | decompiled/functions/100617C0.c |
| 10057D10 | `ContainerMapBase::remove` | 被钩子调用 | decompiled/functions/10057D10.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1004F6A0 | `TEventExt::GetFlags` | 被钩子调用 | decompiled/functions/1004F6A0.c |
| 1004F7A0 | `TEventExt::HasOccured` | 被钩子调用 | decompiled/functions/1004F7A0.c |
| 1004FE20 | `TEventExt::ResolveHouseParam` | 被钩子调用 | decompiled/functions/1004FE20.c |
| 1004FE00 | `HouseClass::Index_IsMP` | 被钩子调用 | decompiled/functions/1004FE00.c |
| 1004F610 | `TEventExt::GetPersistable` | 被钩子调用 | decompiled/functions/1004F610.c |
| 1004F720 | `TEventExt::GetSaveParamKind` | 被钩子调用 | decompiled/functions/1004F720.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 005F5390 `ObjectClass::ObjectClass__ReceiveDamage` | 3 | U.triggers, ext:WarheadType |
| 00702D40 `TechnoClass::TechnoClass__RecordKill` | 3 | U.triggers, ext:Techno |
| 004361D0 `BuildingLightClass::BuildingLightClass__AI` | 2 | T.spotlight, U.triggers |
| 006DD8B0 `TActionClass::TActionClass__Execute` | 2 | H.sidepresent, U.triggers |
| 0071E940 `TEventClass::TEventClass__HasOccurred` | 7 | U.triggers |
