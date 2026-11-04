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
| `ImmuneToSaboteurs` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:308 | ok | — |
| `SpyEffect.AircraftVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:246 | ok | — |
| `SpyEffect.BuildingVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:247 | ok | — |
| `SpyEffect.Custom` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:236 | ok | — |
| `SpyEffect.InfantryVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:243 | ok | — |
| `SpyEffect.KeepRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:241 | ok | — |
| `SpyEffect.NavalVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:245 | ok | — |
| `SpyEffect.PowerOutageDuration` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:248 | ok | — |
| `SpyEffect.ResetRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:239 | ok | — |
| `SpyEffect.ResetSuperweapons` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:238 | ok | — |
| `SpyEffect.RevealProduction` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:237 | ok | — |
| `SpyEffect.RevealRadar` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:240 | ok | — |
| `SpyEffect.SabotageDelay` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:249 | ok | — |
| `SpyEffect.StolenMoneyAmount` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:250 | ok | — |
| `SpyEffect.StolenMoneyPercentage` | BuildingType | Valueable<float> | 0.0f | src/Ext/BuildingType/Body.cpp:251 | ok | — |
| `SpyEffect.StolenTechIndex` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:259 | unscanned | — |
| `SpyEffect.SuperWeapon` | BuildingType | Valueable<SuperWeaponTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:253 | ok | — |
| `SpyEffect.SuperWeaponPermanent` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:254 | ok | — |
| `SpyEffect.UnitVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:242 | ok | — |
| `SpyEffect.VehicleVeterancy` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:244 | ok | — |

## 钩子（14）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 43E7B0 | `BuildingClass_DrawVisible` | kept | ✓ | 0043E7B0 `BuildingClass::BuildingClass__DrawFactoryQueuePreview` | decompiled/functions/10015AB0.c | src/Ext/Building/Hooks.Infiltrate.cpp:133 | — |
| 44161C | `BuildingClass_Destroy_OldSpy1` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10015A90.c | src/Ext/Building/Hooks.Infiltrate.cpp:193 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 448312 | `BuildingClass_ChangeOwnership_OldSpy1` | kept | ✓ | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10015A20.c | src/Ext/Building/Hooks.Infiltrate.cpp:202 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 448D95 | `BuildingClass_ChangeOwnership_OldSpy2` | kept | ✓ | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10015A60.c | src/Ext/Building/Hooks.Infiltrate.cpp:215 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 44F7A0 | `BuildingClass_UpdateDisplayTo` | kept | ✓ | 0044F7A0 `FUN_0044f7a0` | decompiled/functions/10015EC0.c | src/Ext/Building/Hooks.Infiltrate.cpp:227 | — |
| 4571E0 | `BuildingClass_Infiltrate` | kept | ✓ | 004571E0 `BuildingClass::BuildingClass__OnSpyInfiltrate` | decompiled/functions/10015E40.c | src/Ext/Building/Hooks.Infiltrate.cpp:38 | src/app/match_runtime/eva_producers.rs, src/app/match_runtime/sound_dispatch.rs, src/sim/house_state.rs … |
| 457533 | `BuildingClass_Infiltrate_Standard` | kept | ✓ | 004571E0 `BuildingClass::BuildingClass__OnSpyInfiltrate` | decompiled/functions/10015E70.c | src/Ext/Building/Hooks.Infiltrate.cpp:52 | src/app/match_runtime/eva_producers.rs, src/app/match_runtime/sound_dispatch.rs, src/sim/house_state.rs … |
| 459ED0 | `BuildingClass_GetUIName` | new_in_3.0 | ✓ | 00459ED0 `BuildingClass::BuildingClass__GetUIName` | decompiled/functions/10015DA0.c | src/Ext/Building/Hooks.Infiltrate.cpp:247 | src/app/input/tooltips.rs |
| 509303 | `HouseClass_AllyWith_unused` | kept | ✓ | 005092F0 `FUN_005092f0` | decompiled/functions/10015EE0.c | src/Ext/Building/Hooks.Infiltrate.cpp:234 | — |
| 519FF8 | `InfantryClass_UpdatePosition_Saboteur` | replaces_0.A:InfantryClass_UpdatePosition_PreInfiltrate | ✓ | 00519630 `InfantryClass::InfantryClass__PerCellProcess` | decompiled/functions/100260D0.c | src/Ext/Infantry/Hooks.Saboteur.cpp:94 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/house_tracking.rs … |
| 51B2CB | `InfantryClass_SetTarget_Saboteur` | kept | ✓ | 0051B1F0 `InfantryClass::InfantryClass__Assign_Target` | decompiled/functions/10026080.c | src/Ext/Infantry/Hooks.Saboteur.cpp:75 | src/sim/combat/world_receiver.rs, src/sim/crew_survival.rs, src/sim/movement/foot_path.rs … |
| 51EE6B | `InfantryClass_GetActionOnObject_Saboteur` | replaces_0.A:InfantryClass_GetCursorOverObject_Saboteur | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10025FC0.c | src/Ext/Infantry/Hooks.Saboteur.cpp:45 | src/app/input/context_order.rs |
| 7004AD | `TechnoClass_GetActionOnObject_Saboteur` | replaces_0.A:TechnoClass_GetCursorOverObject_Saboteur | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/100261F0.c | src/Ext/Infantry/Hooks.Saboteur.cpp:24 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 70AA60 | `TechnoClass_DrawExtraInfo` | new_in_3.0 | ✓ | 0070AA60 `DrawExtraInfo` | decompiled/functions/10015F10.c | src/Ext/Building/Hooks.Infiltrate.cpp:63 | src/app/presentation/render/object_extras.rs |

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
| 1000D6B0 | `BuildingTypeClass::GetFoundationWidth` | 被钩子调用 | decompiled/functions/1000D6B0.c |
| 100252D0 | `InfantryExt::GetInfiltrateAction` | 被钩子调用 | decompiled/functions/100252D0.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 10008FE0 | `FootClass::AbortMotion` | 被钩子调用 | decompiled/functions/10008FE0.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | 13 | B.spy, T.emp, T.poweredby, W.ivanbomb, ext:Building, ext:Infantry, ext:Techno |
| 00448260 `BuildingClass::BuildingClass__ChangeOwner` | 9 | B.academy, B.prism, B.spy, T.jammer, ext:Building |
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 00519630 `InfantryClass::InfantryClass__PerCellProcess` | 7 | B.spy, ext:Building, ext:Infantry, ext:Techno |
| 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | 7 | B.spy, W.ivanbomb, ext:Techno |
