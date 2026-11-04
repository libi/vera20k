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
| `Factory.ExplicitOnly` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:285 | check_0A_use | src/rules/native_processing.rs |
| `BuiltAt` | TechnoType | ValueableVector<BuildingTypeClass const*> | — | src/Ext/TechnoType/Body.cpp:353 | check_0A_use | src/rules/native_processing.rs |
| `FactoryOwners` | TechnoType | ValueableVector<HouseTypeClass *> | — | src/Ext/TechnoType/Body.cpp:366 | check_0A_use | src/rules/native_processing.rs |
| `FactoryOwners.Forbidden` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:367 | unscanned | src/rules/native_processing.rs |
| `FactoryOwners.HasAllPlans` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:371 | no_use_found | src/rules/native_processing.rs |
| `FactoryOwners.HaveAllPlans` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:369 | ok | src/rules/native_processing.rs |
| `FactoryOwners.Permanent` | TechnoType | Valueable<bool>::Read | — | src/Ext/TechnoType/Body.cpp:370 | ok | src/rules/native_processing.rs |

## 钩子（7）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 443CCA | `BuildingClass_KickOutUnit_AircraftType` | kept | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024D60.c | src/Ext/House/Hooks.Queue.cpp:106 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 444119 | `BuildingClass_KickOutUnit_UnitType` | kept | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024DC0.c | src/Ext/House/Hooks.Queue.cpp:75 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 444131 | `BuildingClass_KickOutUnit_InfantryType` | kept | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024DA0.c | src/Ext/House/Hooks.Queue.cpp:90 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 44531F | `BuildingClass_KickOutUnit_BuildingType` | kept | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10024D80.c | src/Ext/House/Hooks.Queue.cpp:98 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 4502F4 | `BuildingClass_Update_Factory` | kept | 004500F0 `BuildingClass::BuildingClass__Factory_AI` | decompiled/functions/10024E00.c | src/Ext/House/Hooks.Queue.cpp:11 | src/sim/production/factory.rs, src/sim/production/factory_ai.rs |
| 4CA07A | `FactoryClass_AbandonProduction` | kept | 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | decompiled/functions/10024EE0.c | src/Ext/House/Hooks.Queue.cpp:48 | src/sim/production/factory.rs, src/sim/production/factory_ai.rs, src/sim/production/factory_lifecycle.rs |
| 50B370 | `HouseClass_ShouldDisableCameo` | kept | 0050B370 `HouseClass::HouseClass__CheckBuildLimit` | decompiled/functions/10024F80.c | src/Ext/House/Hooks.Queue.cpp:115 | src/sim/production/can_build.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10021FF0 | HouseExt::PrerequisitesMet [source_0.A] | 使用键 | decompiled/functions/10021FF0.c |
| 1003D9A0 | TechnoTypeExt::ExtData::CanBeBuiltAt [source_0.A] | 使用键 | decompiled/functions/1003D9A0.c |
| 100449D0 | FUN_100449d0 [translated] | 使用键 | decompiled/functions/100449D0.c |
| 10021A00 | `Prereqs::HouseOwnsAll` | 被钩子调用 | decompiled/functions/10021A00.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100159C0 | `HouseClass::GetPrimaryFactory` | 被钩子调用 | decompiled/functions/100159C0.c |
| 10024D20 | `FactoryClass::CountTotal` | 被钩子调用 | decompiled/functions/10024D20.c |
| 10020870 | `HouseExt::BuildLimitRemaining` | 被钩子调用 | decompiled/functions/10020870.c |
| 10020D40 | `HouseExt::HasFactory` | 被钩子调用 | decompiled/functions/10020D40.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | 11 | B.firestorm, T.factory, ext:Building, ext:TechnoType |
| 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | 2 | T.factory, recovered:FactoryClass |
