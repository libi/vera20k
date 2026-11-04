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
| `IsTrench` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:212 | unscanned | — |
| `SubjectToTrenches` | BulletType | Valueable<bool> | true | src/Ext/BulletType/Body.cpp:29 | check_0A_use | — |

## 钩子（9）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 441F12 | `BuildingClass_Destroy_RubbleYell` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017160.c | src/Ext/Building/Hooks.Trenches.cpp:86 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441F2C | `BuildingClass_Destroy_KickOutOfRubble` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017130.c | src/Ext/Building/Hooks.Trenches.cpp:103 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 4494D2 | `BuildingClass_IsSellable` | kept | 004494C0 `BuildingClass::BuildingClass__CanSell` | decompiled/functions/10017200.c | src/Ext/Building/Hooks.Trenches.cpp:123 | src/rules/buildup_asset_catalog.rs, src/sim/production/production_sell.rs |
| 457D58 | `BuildingClass_CanBeOccupied_SpecificOccupiers` | kept | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10017060.c | src/Ext/Building/Hooks.Trenches.cpp:17 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |
| 457DB7 | `BuildingClass_CanBeOccupied_SpecificAssaulters` | kept | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10016FD0.c | src/Ext/Building/Hooks.Trenches.cpp:58 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |
| 4581CD | `BuildingClass_UnloadOccupants_AllOccupantsHaveLeft` | kept | 00457DE0 `BuildingClass::BuildingClass__SellBuilding` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:213 | src/sim/combat/object_health.rs, src/sim/passenger/garrison_eject.rs, src/sim/production/production_sell.rs |
| 4586CA | `BuildingClass_KillOccupiers_EachOccupierKilled` | kept | 004585C0 `BuildingClass::BuildingClass__KillOccupants` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:199 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/temporal.rs |
| 458729 | `BuildingClass_KillOccupiers_AllOccupantsKilled` | kept | 004585C0 `BuildingClass::BuildingClass__KillOccupants` | decompiled/functions/10017220.c | src/Ext/Building/Hooks.Trenches.cpp:226 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/temporal.rs |
| 52297F | `InfantryClass_GarrisonBuilding_OccupierEntered` | kept | 00522910 `InfantryClass::InfantryClass__AddGarrisonOccupant` | decompiled/functions/10017240.c | src/Ext/Building/Hooks.Trenches.cpp:143 | src/sim/passenger.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10019AC0 | FUN_10019ac0 [source_0.A] | 使用键 | decompiled/functions/10019AC0.c |
| 1000D2F0 | `BuildingTypeExt::ExtData::CanBeOccupiedBy` | 被钩子调用 | decompiled/functions/1000D2F0.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 10012970 | `BuildingExt::ExtData::KickOutOfRubble` | 被钩子调用 | decompiled/functions/10012970.c |
| 1000F2B0 | `BuildingExt::ExtData::RubbleYell` | 被钩子调用 | decompiled/functions/1000F2B0.c |
| 100136C0 | `BuildingExt::ExtData::evalRaidStatus` | 被钩子调用 | decompiled/functions/100136C0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10019D00 | `TechnoClass::UpdateThreatInCell` | 被钩子调用 | decompiled/functions/10019D00.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 004585C0 `BuildingClass::BuildingClass__KillOccupants` | 3 | B.trench, recovered:BuildingClass |
| 004494C0 `BuildingClass::BuildingClass__CanSell` | 2 | B.trench, recovered:BuildingClass |
