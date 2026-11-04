# `B.trench` 战壕

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | B.occupy | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `IsTrench` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:207 | unscanned | — |
| `SubjectToTrenches` | BulletType | Valueable<bool> | true | src/Ext/BulletType/Body.cpp:29 | ok | — |

## 钩子（11）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 441F12 | `BuildingClass_Destroy_RubbleYell` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017160.c | src/Ext/Building/Hooks.Trenches.cpp:98 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441F2C | `BuildingClass_Destroy_KickOutOfRubble` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017130.c | src/Ext/Building/Hooks.Trenches.cpp:117 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 443414 | `BuildingClass_ActionOnObject` | replaces_0.A:BuildingClass_ClickedAction | ✓ | 00443410 `BuildingClass::BuildingClass__ClickedAction_Object` | decompiled/functions/10016F60.c | src/Ext/Building/Hooks.Trenches.cpp:161 | src/app/input/context_order.rs, src/sim/command.rs, src/sim/world/gap_generator.rs … |
| 44725F | `BuildingClass_GetActionOnObject_TargetABuilding` | replaces_0.A:BuildingClass_GetCursorOverObject_TargetABuilding | ✓ | 00447210 `BuildingClass::BuildingClass__What_Action_OnObject` | decompiled/functions/100171B0.c | src/Ext/Building/Hooks.Trenches.cpp:137 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/world/gap_generator.rs … |
| 4494D2 | `BuildingClass_IsSellable` | kept | ✓ | 004494C0 `BuildingClass::BuildingClass__CanSell` | decompiled/functions/10017200.c | src/Ext/Building/Hooks.Trenches.cpp:193 | src/rules/buildup_asset_catalog.rs, src/sim/production/production_sell.rs |
| 457D58 | `BuildingClass_CanBeOccupied_SpecificOccupiers` | kept | ✓ | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10017060.c | src/Ext/Building/Hooks.Trenches.cpp:29 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |
| 457DB7 | `BuildingClass_CanBeOccupied_SpecificAssaulters` | kept | ✓ | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10016FD0.c | src/Ext/Building/Hooks.Trenches.cpp:70 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |
| 4581CD | `BuildingClass_UnloadOccupants_AllOccupantsHaveLeft` | kept | ✓ | 00457DE0 `BuildingClass::BuildingClass__SellBuilding` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:285 | src/sim/combat/object_health.rs, src/sim/passenger/garrison_eject.rs, src/sim/production/production_sell.rs |
| 4586CA | `BuildingClass_KillOccupiers_EachOccupierKilled` | kept | ✓ | 004585C0 `BuildingClass::BuildingClass__KillOccupants` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:271 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/temporal.rs |
| 458729 | `BuildingClass_KillOccupiers_AllOccupantsKilled` | kept | ✓ | 004585C0 `BuildingClass::BuildingClass__KillOccupants` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:298 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/temporal.rs |
| 52297F | `InfantryClass_GarrisonBuilding_OccupierEntered` | kept | ✓ | 00522910 `InfantryClass::InfantryClass__AddGarrisonOccupant` | decompiled/functions/10017240.c | src/Ext/Building/Hooks.Trenches.cpp:214 | src/sim/passenger.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10019AC0 | FUN_10019ac0 [source_0.A,3.0✓] | 使用键 | decompiled/functions/10019AC0.c |
| 1006A170 | `AresNetEvent::Handlers::RaiseTrenchRedirectClick` | 被钩子调用 | decompiled/functions/1006A170.c |
| 1000D2F0 | `BuildingTypeExt::ExtData::CanBeOccupiedBy` | 被钩子调用 | decompiled/functions/1000D2F0.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 10012970 | `BuildingExt::KickOutOfRubble` | 被钩子调用 | decompiled/functions/10012970.c |
| 1000F2B0 | `BuildingTypeExt::SwitchRubble` | 被钩子调用 | decompiled/functions/1000F2B0.c |
| 10011690 | `BuildingExt::ExtData::canTraverseTo` | 被钩子调用 | decompiled/functions/10011690.c |
| 100136C0 | `BuildingExt::ExtData::evalRaidStatus` | 被钩子调用 | decompiled/functions/100136C0.c |
| 10019CD0 | `Randomizer::RandomDouble` | 被钩子调用 | decompiled/functions/10019CD0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10019D00 | `TechnoClass::UpdateThreatInCell` | 被钩子调用 | decompiled/functions/10019D00.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 00447210 `BuildingClass::BuildingClass__What_Action_OnObject` | 3 | B.trench, T.poweredby, ext:Building |
| 004585C0 `BuildingClass::BuildingClass__KillOccupants` | 3 | B.trench, U.bugfixes |
| 00443410 `BuildingClass::BuildingClass__ClickedAction_Object` | 2 | B.trench, ext:Building |
| 004494C0 `BuildingClass::BuildingClass__CanSell` | 2 | B.firestorm, B.trench |
