# `T.carryall` Carryall 吊运限制

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Carryall.Allowed` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:360 | check_0A_use | — |
| `Carryall.SizeLimit` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:361 | check_0A_use | — |

## 钩子（12）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 435CD3 | `BuildingLightClass_Draw_Spotlight` | replaces_0.A:SpotlightClass_CTOR | 00435BE0 `BuildingLightClass::BuildingLightClass__Draw_It` | decompiled/functions/1004EAC0.c | src/Recovered/BuildingLightClass.cpp:64 | — |
| 4DA8B2 | `FootClass_Update_AnimRate` | kept ⚠改过 | 004DA530 `FootClass::FootClass__AI` | decompiled/functions/100493D0.c | src/Ext/Techno/Hooks.cpp:66 | src/sim/aircraft/mod.rs |
| 5F5ADD | `ObjectClass_SpawnParachuted_Animation` | replaces_0.A:Parachute_Animation | 005F5940 `ObjectClass::ObjectClass__Paradrop` | decompiled/functions/10047C60.c | src/Recovered/ObjectClass.cpp:76 | src/app/presentation/lighting.rs, src/sim/aircraft/drop_payload.rs, src/sim/game_entity.rs … |
| 5F8084 | `ObjectTypeClass_UnloadTurretArt` | new_in_3.0 | 005F8080 `ObjectTypeClass::ObjectTypeClass__Free_Indexed_Voxels` | decompiled/functions/100423B0.c | src/Recovered/ObjectTypeClass.cpp:270 | — |
| 6B1065 | `SlaveManagerClass_ShouldWakeUp_ShortScan` | new_in_3.0 | 006B1020 `SlaveManagerClass::SlaveManagerClass__ShouldRecallSlaves` | decompiled/functions/1004D3F0.c | src/Recovered/SlaveManagerClass.cpp:34 | src/sim/slave_manager.rs |
| 6FFEC0 | `TechnoClass_GetActionOnObject_Cursors` | new_in_3.0 | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004C990.c | src/Recovered/TechnoClass.cpp:761 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 7000CD | `TechnoClass_GetActionOnObject_SelfDeployCursor` | new_in_3.0 | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004CA20.c | src/Recovered/TechnoClass.cpp:747 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 700600 | `TechnoClass_GetActionOnCell_Cursors` | new_in_3.0 | 00700600 `TechnoClass::TechnoClass__What_Action_OnCell` | decompiled/functions/1004C920.c | src/Recovered/TechnoClass.cpp:783 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/c4_plant.rs … |
| 702E9D | `TechnoClass_RegisterDestruction_Veterancy` | kept | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/1004EEF0.c | src/Ext/Techno/Hooks.Veterancy.cpp:12 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |
| 707D20 | `TechnoClass_GetCrew` | kept | 00707D20 `TechnoClass::TechnoClass__GetCrew` | decompiled/functions/1004C780.c | src/Ext/Techno/Hooks.Crew.cpp:9 | src/rules/ruleset.rs, src/sim/crew_survival.rs |
| 70A990 | `TechnoClass_DrawVeterancy` | kept | 0070A990 `DrawVeterancyPips` | decompiled/functions/1004E0C0.c | src/Ext/Techno/Hooks.Insignia.cpp:7 | src/app/presentation/render/object_extras.rs |
| 7418AA | `UnitClass_CrushCell_CrushDamage` | kept | 007416A0 `UnitClass::UnitClass__Crush_Or_Prescatter_Cell` | decompiled/functions/1004BCE0.c | src/Ext/Techno/Hooks.cpp:1548 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/combat/techno_death.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1003D9E0 | TechnoTypeExt::ExtData::CarryallCanLift [source_0.A] | 使用键 | decompiled/functions/1003D9E0.c |
| 100434C0 | TechnoExt::ExtData::CanKillDriver [translated] | 使用键 | decompiled/functions/100434C0.c |
| 100443E0 | PlayDigEffects [translated] | 使用键 | decompiled/functions/100443E0.c |
| 10044FF0 | TechnoExt::ExtData::GetActionHijack [source_0.A] | 使用键 | decompiled/functions/10044FF0.c |
| 10045440 | TechnoExt::ExtData::IsOperated [source_0.A] | 使用键 | decompiled/functions/10045440.c |
| 100456D0 | TechnoExt::ExtData::KillDriver [translated] | 使用键 | decompiled/functions/100456D0.c |
| 10045B60 | TechnoExt::ExtData::PerformActionHijack [source_0.A] | 使用键 | decompiled/functions/10045B60.c |
| 10046AF0 | TechnoExt::UpdateVeterancy [translated] | 使用键 | decompiled/functions/10046AF0.c |
| 1006A6F0 | TechnoExt::Destroy [source_0.A] | 使用键 | decompiled/functions/1006A6F0.c |
| 1003D7B0 | `MotLib::~MotLib` | 被钩子调用 | decompiled/functions/1003D7B0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 10046340 | `FootClass::RemovePassenger` | 被钩子调用 | decompiled/functions/10046340.c |
| 10046320 | `FootClass::RemoveFirstPassenger` | 被钩子调用 | decompiled/functions/10046320.c |
| 100448A0 | `TechnoClass::ExitedOpenTopped` | 被钩子调用 | decompiled/functions/100448A0.c |
| 10009180 | `CaptureManagerClass::FreeUnit` | 被钩子调用 | decompiled/functions/10009180.c |
| 100162E0 | `CaptureManagerClass::FreeAll` | 被钩子调用 | decompiled/functions/100162E0.c |
| 100092F0 | `TeamClass::LiberateMember` | 被钩子调用 | decompiled/functions/100092F0.c |
| 10016320 | `SpawnManagerClass::KillNodes` | 被钩子调用 | decompiled/functions/10016320.c |
| 10046380 | `SpawnManagerClass::ResetTarget` | 被钩子调用 | decompiled/functions/10046380.c |
| 10045A00 | `SlaveManagerClass::Killed` | 被钩子调用 | decompiled/functions/10045A00.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 100464A0 | `TagClass::ShouldReplace` | 被钩子调用 | decompiled/functions/100464A0.c |
| 10046360 | `ObjectClass::ReplaceTag` | 被钩子调用 | decompiled/functions/10046360.c |
| 10043630 | `CaptureManagerClass::CaptureUnit` | 被钩子调用 | decompiled/functions/10043630.c |
| 10008FE0 | `FootClass::AbortMotion` | 被钩子调用 | decompiled/functions/10008FE0.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 10021520 | `HouseExt::ExtData::GetTechnician` | 被钩子调用 | decompiled/functions/10021520.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10021230 | `HouseExt::ExtData::GetCrew` | 被钩子调用 | decompiled/functions/10021230.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |
