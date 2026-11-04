# `B.spy` 间谍渗透效果

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | B.engineer | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（20）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `ImmuneToSaboteurs` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:312 | ok | — |
| `SpyEffect.AircraftVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:250 | no_use_found | — |
| `SpyEffect.BuildingVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:251 | no_use_found | — |
| `SpyEffect.Custom` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:240 | no_use_found | — |
| `SpyEffect.InfantryVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:247 | no_use_found | — |
| `SpyEffect.KeepRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:245 | no_use_found | — |
| `SpyEffect.NavalVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:249 | no_use_found | — |
| `SpyEffect.PowerOutageDuration` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:252 | no_use_found | — |
| `SpyEffect.ResetRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:243 | no_use_found | — |
| `SpyEffect.ResetSuperweapons` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:242 | ok | — |
| `SpyEffect.RevealProduction` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:241 | check_0A_use | — |
| `SpyEffect.RevealRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:244 | no_use_found | — |
| `SpyEffect.SabotageDelay` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:253 | no_use_found | — |
| `SpyEffect.StolenMoneyAmount` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:254 | no_use_found | — |
| `SpyEffect.StolenMoneyPercentage` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:255 | no_use_found | — |
| `SpyEffect.StolenTechIndex` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:263 | unscanned | — |
| `SpyEffect.SuperWeapon` | BuildingType | Valueable<SuperWeaponTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:257 | no_use_found | — |
| `SpyEffect.SuperWeaponPermanent` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:258 | no_use_found | — |
| `SpyEffect.UnitVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:246 | no_use_found | — |
| `SpyEffect.VehicleVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:248 | no_use_found | — |

## 钩子（9）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 43E7B0 | `BuildingClass_DrawVisible` | kept | 0043E7B0 `BuildingClass::BuildingClass__DrawFactoryQueuePreview` | decompiled/functions/10015AB0.c | src/Ext/Building/Hooks.Infiltrate.cpp:33 | — |
| 44161C | `BuildingClass_Destroy_OldSpy1` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10015A90.c | src/Ext/Building/Hooks.Infiltrate.cpp:86 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 448312 | `BuildingClass_ChangeOwnership_OldSpy1` | kept | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10015A20.c | src/Ext/Building/Hooks.Infiltrate.cpp:95 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 448D95 | `BuildingClass_ChangeOwnership_OldSpy2` | kept | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10015A60.c | src/Ext/Building/Hooks.Infiltrate.cpp:108 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 44F7A0 | `BuildingClass_UpdateDisplayTo` | kept | 0044F7A0 `FUN_0044f7a0` | decompiled/functions/10015EC0.c | src/Ext/Building/Hooks.Infiltrate.cpp:120 | — |
| 4571E0 | `BuildingClass_Infiltrate` | kept | 004571E0 `BuildingClass::BuildingClass__OnSpyInfiltrate` | decompiled/functions/10015E40.c | src/Ext/Building/Hooks.Infiltrate.cpp:12 | src/app/match_runtime/eva_producers.rs, src/app/match_runtime/sound_dispatch.rs, src/sim/house_state.rs … |
| 457533 | `BuildingClass_Infiltrate_Standard` | kept | 004571E0 `BuildingClass::BuildingClass__OnSpyInfiltrate` | decompiled/functions/10015E70.c | src/Ext/Building/Hooks.Infiltrate.cpp:26 | src/app/match_runtime/eva_producers.rs, src/app/match_runtime/sound_dispatch.rs, src/sim/house_state.rs … |
| 509303 | `HouseClass_AllyWith_unused` | kept | 005092F0 `FUN_005092f0` | decompiled/functions/10015EE0.c | src/Ext/Building/Hooks.Infiltrate.cpp:127 | — |
| 51B2CB | `InfantryClass_SetTarget_Saboteur` | kept | 0051B1F0 `InfantryClass::InfantryClass__Assign_Target` | decompiled/functions/10026080.c | src/Ext/Infantry/Hooks.Saboteur.cpp:11 | src/sim/combat/world_receiver.rs, src/sim/crew_survival.rs, src/sim/movement/foot_path.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10009220 | TunnelData::IsLastEntrance [translated] | 使用键 | decompiled/functions/10009220.c |
| 1000D790 | FUN_1000d790 [translated] | 使用键 | decompiled/functions/1000D790.c |
| 100131D0 | `BuildingExt::UpdateDisplayTo` | 被钩子调用 | decompiled/functions/100131D0.c |
| 100159C0 | `HouseClass::GetPrimaryFactory` | 被钩子调用 | decompiled/functions/100159C0.c |
| 10012000 | `BuildingExt::ExtData::InfiltratedBy` | 被钩子调用 | decompiled/functions/10012000.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 100117F0 | `SidebarClass::Draw` | 被钩子调用 | decompiled/functions/100117F0.c |
| 100252D0 | `BuildingTypeExt::IsSabotagable` | 被钩子调用 | decompiled/functions/100252D0.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00448260 `BuildingClass::BuildingClass__ChangeOwner` | 9 | B.academy, B.prism, B.spy, T.jammer, ext:Building |
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
