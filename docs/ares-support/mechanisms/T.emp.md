# `T.emp` EMP（单位/弹头/建筑/超武共用状态）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | F.readers | done | docs/ares-emp.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（10）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DeactivateDimEMP` | Rules | Valueable<double> | 0.8 | src/Ext/Rules/Body.cpp:149 | ok | — |
| `EMPAIRecoverMission` | Rules | Nullable<Mission> | (empty) | src/Ext/Rules/Body.cpp:183 | ok | src/rules/native_processing.rs |
| `EMP.Modifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:285 | ok | src/rules/ares_emp.rs |
| `EMP.Sparkles` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:286 | ok | src/rules/native_processing.rs |
| `EMP.Threshold` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:290 | unscanned | src/rules/ares_emp.rs |
| `empimmune` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:77 | unscanned | src/rules/ares_abilities.rs, src/sim/game_entity/emp.rs |
| `ImmuneToEMP` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:284 | ok | src/rules/ares_emp.rs |
| `EMP.Cap` | WarheadType | int | -1 | src/Ext/WarheadType/Body.cpp:101 | ok | src/rules/ares_warhead.rs |
| `EMP.Duration` | WarheadType | int | 0 | src/Ext/WarheadType/Body.cpp:100 | ok | src/rules/ares_warhead.rs |
| `EMP.Sparkles` | WarheadType | Valueable<AnimTypeClass*> | nullptr | src/Ext/WarheadType/Body.cpp:102 | ok | src/rules/native_processing.rs |

## 钩子（5）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4456E5 | `BuildingClass_UpdateConstructionOptions_ExcludeDisabled` | kept | ✓ | 004456D0 `BuildingClass::BuildingClass__UpdateConstructionOptions` | decompiled/functions/10053D80.c | src/Ext/WarheadType/Hooks.EMP.cpp:87 | src/sim/production/production_tech.rs |
| 44A04C | `BuildingClass_Mi_Selling_EMPDuration` | replaces_0.A:BuildingClass_Unload_CopyEMPDuration | ✓ | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/10053D60.c | src/Ext/WarheadType/Hooks.EMP.cpp:44 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 51E3B0 | `InfantryClass_GetActionOnObject_EMP` | replaces_0.A:InfantryClass_GetCursorOverObject_EMP | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10053DB0.c | src/Ext/WarheadType/Hooks.EMP.cpp:59 | src/app/input/context_order.rs |
| 6FAF0D | `TechnoClass_Update_EMPLock` | kept | ✓ | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/10053DF0.c | src/Ext/WarheadType/Hooks.EMP.cpp:13 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 73D219 | `UnitClass_Draw_OreGatherAnim` | kept | ✓ | 0073CEC0 `UnitClass::UnitClass__DrawIt` | decompiled/functions/10053E50.c | src/Ext/WarheadType/Hooks.EMP.cpp:75 | src/app/presentation/instances/units.rs, src/map/lighting.rs, src/render/sprite_atlas.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10003BB0 | SyringeHandshake [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10003BB0.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F490.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002E060.c |
| 10052870 | WarheadTypeExt::ExtData::applyEMP [translated] | 使用键 | decompiled/functions/10052870.c |
| 10062320 | FUN_10062320 [translated,inferred] | 使用键 | decompiled/functions/10062320.c |
| 100627F0 | EMPulse::DisableEMPEffect [source_0.A,3.0✓] | 使用键 | decompiled/functions/100627F0.c |
| 10062D20 | FUN_10062d20 [source_0.A,3.0✓] | 使用键 | decompiled/functions/10062D20.c |
| 10062D90 | FUN_10062d90 [translated] | 使用键 | decompiled/functions/10062D90.c |
| 10062F70 | FUN_10062f70 [source_0.A,3.0✓] | 使用键 | decompiled/functions/10062F70.c |
| 10070BA0 | PsychicDominatorStateMachine::Update [translated,inferred] | 使用键 | decompiled/functions/10070BA0.c |
| 10075920 | SW_UnitDelivery::Activate [translated,inferred] | 使用键 | decompiled/functions/10075920.c |
| 10009730 | `TunnelData::Save` | 被钩子调用 | decompiled/functions/10009730.c |
| 10062240 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10062240.c |
| 10062FD0 | `EMPulse::UpdateSparkleAnim` | 被钩子调用 | decompiled/functions/10062FD0.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |
| 10048690 | `TechnoClass::Deactivate` | 被钩子调用 | decompiled/functions/10048690.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10062F50 | `TechnoExt::ExtData::IsOperated_Inline` | 被钩子调用 | decompiled/functions/10062F50.c |
| 10062B40 | `EMPulse::enableEMPEffect` | 被钩子调用 | decompiled/functions/10062B40.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 10044300 | `TechnoExt::Destroy` | 被钩子调用 | decompiled/functions/10044300.c |
| 10062CE0 | `BuildingClass::EnableStuff` | 被钩子调用 | decompiled/functions/10062CE0.c |
| 100487D0 | `TechnoClass::Reactivate` | 被钩子调用 | decompiled/functions/100487D0.c |
| 1003DC50 | `TechnoTypeExt::ExtData::GetWeaponSlots` | 被钩子调用 | decompiled/functions/1003DC50.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 100708B0 | `PsyDom::Fire` | 被钩子调用 | decompiled/functions/100708B0.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | 13 | B.spy, T.emp, T.poweredby, W.ivanbomb, ext:Building, ext:Infantry, ext:Techno |
| 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | 7 | T.emp, U.bugfixes, W.ivanbomb, ext:Building, ext:Techno |
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno |
