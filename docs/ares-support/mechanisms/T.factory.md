# `T.factory` BuiltAt、Factory.ExplicitOnly、FactoryOwners

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | F.prereq | done | tools/rules_oracle/ares_factory_owners.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Factory.ExplicitOnly` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:281 | ok | src/rules/native_processing.rs |
| `BuiltAt` | TechnoType | ValueableVector<BuildingTypeClass const*> | — | src/Ext/TechnoType/Body.cpp:357 | ok | src/rules/native_processing.rs |
| `FactoryOwners` | TechnoType | ValueableVector<HouseTypeClass *> | — | src/Ext/TechnoType/Body.cpp:370 | ok | src/rules/native_processing.rs |
| `FactoryOwners.Forbidden` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:371 | unscanned | src/rules/native_processing.rs |
| `FactoryOwners.HasAllPlans` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:375 | ok | src/rules/native_processing.rs |
| `FactoryOwners.HaveAllPlans` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:373 | no_use_found | src/rules/native_processing.rs |
| `FactoryOwners.Permanent` | TechnoType | Valueable<bool>::Read | — | src/Ext/TechnoType/Body.cpp:374 | ok | src/rules/native_processing.rs |

## 钩子（12）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 443CCA | `BuildingClass_KickOutUnit_AircraftType` | kept | ✓ | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024D60.c | src/Ext/House/Hooks.Queue.cpp:118 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 444119 | `BuildingClass_KickOutUnit_UnitType` | kept | ✓ | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024DC0.c | src/Ext/House/Hooks.Queue.cpp:87 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 444131 | `BuildingClass_KickOutUnit_InfantryType` | kept | ✓ | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024DA0.c | src/Ext/House/Hooks.Queue.cpp:102 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 44531F | `BuildingClass_KickOutUnit_BuildingType` | kept | ✓ | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024D80.c | src/Ext/House/Hooks.Queue.cpp:110 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 4502F4 | `BuildingClass_Update_Factory` | kept | ✓ | 004500F0 `BuildingClass::BuildingClass__Factory_AI` | decompiled/functions/10024E00.c | src/Ext/House/Hooks.Queue.cpp:22 | src/sim/production/factory.rs, src/sim/production/factory_ai.rs |
| 4CA07A | `FactoryClass_AbandonProduction` | kept | ✓ | 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | decompiled/functions/10024EE0.c | src/Ext/House/Hooks.Queue.cpp:59 | src/sim/production/factory.rs, src/sim/production/factory_ai.rs, src/sim/production/factory_lifecycle.rs |
| 509140 | `HouseClass_Update_Factories_Queues` | new_in_3.0 | ✓ | 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | decompiled/functions/10025100.c | src/Ext/House/Hooks.Queue.cpp:200 | src/sim/production/factory_lifecycle.rs |
| 50B370 | `HouseClass_ShouldDisableCameo` | kept | ✓ | 0050B370 `HouseClass::HouseClass__CheckBuildLimit` | decompiled/functions/10024F80.c | src/Ext/House/Hooks.Queue.cpp:127 | src/sim/production/can_build.rs |
| 535DB6 | `SetStructureTabCommandClass_Execute_Power` | new_in_3.0 | ✓ | 00535D90 `SetStructureTabCommandClass__Execute` | decompiled/functions/10025150.c | src/Ext/House/Hooks.Queue.cpp:264 | src/app/input/dispatch.rs |
| 535E76 | `SetDefenseTabCommandClass_Execute_Power` | new_in_3.0 | ✓ | 00535E50 `SetDefenseTabCommandClass__Execute` | decompiled/functions/10025130.c | src/Ext/House/Hooks.Queue.cpp:276 | src/app/input/dispatch.rs |
| 6A9822 | `StripClass_Draw_Power` | new_in_3.0 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/100251D0.c | src/Ext/House/Hooks.Queue.cpp:221 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6AB312 | `SidebarClass_ProcessCameoClick_Power` | new_in_3.0 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/10025170.c | src/Ext/House/Hooks.Queue.cpp:244 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10021FF0 | HouseExt::RequirementsMet [source_0.A,3.0✓] | 使用键 | decompiled/functions/10021FF0.c |
| 1003D9A0 | TechnoTypeExt::ExtData::CanBeBuiltAt [translated] | 使用键 | decompiled/functions/1003D9A0.c |
| 100449D0 | BuildingExt::UpdateFactoryOwnersPlans [translated] | 使用键 | decompiled/functions/100449D0.c |
| 100081E0 | `Prereqs::HouseOwnsAny` | 被钩子调用 | decompiled/functions/100081E0.c |
| 10003620 | `BuildingClass::GetSecretProduction` | 被钩子调用 | decompiled/functions/10003620.c |
| 10020AD0 | `HouseClass::ControlledByHuman` | 被钩子调用 | decompiled/functions/10020AD0.c |
| 10021A00 | `HouseExt::IsDisabledFromShell` | 被钩子调用 | decompiled/functions/10021A00.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100159C0 | `HouseClass::GetPrimaryFactory` | 被钩子调用 | decompiled/functions/100159C0.c |
| 10024D20 | `FactoryClass::CountTotal` | 被钩子调用 | decompiled/functions/10024D20.c |
| 10020870 | `HouseExt::BuildLimitRemaining` | 被钩子调用 | decompiled/functions/10020870.c |
| 10020D40 | `HouseExt::FindFactory` | 被钩子调用 | decompiled/functions/10020D40.c |
| 10013940 | `HouseClass::Update_FactoriesQueues` | 被钩子调用 | decompiled/functions/10013940.c |
| 10024D40 | `FactoryClass::IsDone` | 被钩子调用 | decompiled/functions/10024D40.c |
| 10021130 | `HouseExt::ExtData::GatherPlansOf` | 被钩子调用 | decompiled/functions/10021130.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006A9540 `StripClass::StripClass__Draw` | 18 | H.country, S.core, T.cameo, T.factory, ext:Rules |
| 006AAD00 `SelectClass__Action` | 16 | S.core, T.cameo, T.factory, ext:Gadgets, ext:TechnoType |
| 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | 11 | B.firestorm, T.factory, ext:Building, ext:TechnoType |
| 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | 2 | T.factory, U.bugfixes |
| 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | 2 | T.factory, U.bugfixes |
