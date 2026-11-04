# `B.prism` Prism 转发

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（13）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `PrismForwarding` | BuildingType | — | — | src/Ext/BuildingType/PrismForward.cpp:62 | unscanned | — |
| `PrismForwarding.EliteSupportWeapon` | BuildingType | — | — | src/Ext/BuildingType/PrismForward.cpp:141 | unscanned | — |
| `PrismForwarding.SupportWeapon` | BuildingType | — | — | src/Ext/BuildingType/PrismForward.cpp:140 | unscanned | — |
| `PrismForwarding.BreakSupport` | HouseType | Valueable<bool>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.ChargeDelay` | HouseType | Valueable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.DamageAdd` | HouseType | Valueable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.Intensity` | HouseType | Valueable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.MaxChainLength` | HouseType | Valueable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.MaxFeeds` | HouseType | Nullable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.MaxNetworkSize` | HouseType | Nullable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.SupportModifier` | HouseType | Nullable<int>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.Targets` | HouseType | ValueableVector<BuildingTypeClass*>::Read | — | 10010680 | no_use_found | — |
| `PrismForwarding.ToAllies` | HouseType | Valueable<bool>::Read | — | 10010680 | no_use_found | — |

## 钩子（10）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4424EF | `BuildingClass_ReceiveDamage_PrismForward` | kept | 00442230 `BuildingClass::BuildingClass__ReceiveDamage` | decompiled/functions/10016950.c | src/Ext/Building/Hooks.Prism.cpp:247 | src/audio/events.rs, src/rules/ruleset.rs, src/rules/warhead_type.rs … |
| 447113 | `BuildingClass_Sell_PrismForward` | kept | 00447110 `BuildingClass::BuildingClass__Sell_Back` | decompiled/functions/10016970.c | src/Ext/Building/Hooks.Prism.cpp:255 | src/rules/buildup_asset_catalog.rs, src/rules/ruleset.rs, src/sim/production/production_sell.rs … |
| 448277 | `BuildingClass_ChangeOwner_PrismForwardAndLeaveBomb` | kept | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/100163B0.c | src/Ext/Building/Hooks.Prism.cpp:267 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 44ABD0 | `BuildingClass_FireLaser` | kept | 0044ABD0 `BuildingClass::BuildingClass__FirePrismSupportBeam` | decompiled/functions/10016490.c | src/Ext/Building/Hooks.Prism.cpp:137 | src/sim/world/techno_ai/building_missions.rs |
| 44B2FE | `BuildingClass_Mi_Attack_IsPrism` | kept | 0044ACF0 `BuildingClass::BuildingClass__Mission_Attack` | decompiled/functions/100167E0.c | src/Ext/Building/Hooks.Prism.cpp:13 | src/rules/art_data.rs, src/rules/mission_data.rs, src/sim/world/techno_ai/building_missions.rs |
| 44EBF0 | `BuildingClass_Disappear_PrismForward` | kept | 0044EBF0 `BuildingClass::BuildingClass__Detach_All` | decompiled/functions/10016470.c | src/Ext/Building/Hooks.Prism.cpp:359 | src/sim/production/factory_ai.rs, src/sim/world/lifecycle.rs |
| 4503F0 | `BuildingClass_Update_Prism` | kept | 004503F0 `BuildingClass::BuildingClass__ProcessDelayedFire` | decompiled/functions/100169A0.c | src/Ext/Building/Hooks.Prism.cpp:77 | src/sim/world/techno_ai/building_missions.rs |
| 454B3D | `BuildingClass_UpdatePowered_PrismForward` | kept | 004549B0 `BuildingClass::BuildingClass__UpdateGapAndSpecialEffects` | decompiled/functions/10016950.c | src/Ext/Building/Hooks.Prism.cpp:349 | src/sim/building_art_power.rs, src/sim/capture_manager.rs, src/sim/powered_unit.rs … |
| 70FD9A | `TechnoClass_Drain_PrismForward` | kept | 0070FD70 `TechnoClass::TechnoClass__Install_Drain_Link` | decompiled/functions/10016B90.c | src/Ext/Building/Hooks.Prism.cpp:336 | src/rules/combat_damage.rs, src/sim/combat/mod.rs, src/sim/combat/world_fire.rs … |
| 71AF76 | `TemporalClass_Fire_PrismForwardAndWarpable` | kept | 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | decompiled/functions/10016BD0.c | src/Ext/Building/Hooks.Prism.cpp:315 | src/sim/projectile.rs, src/sim/temporal.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10012920 | `BuildingExt::KickOutHospitalArmory` | 被钩子调用 | decompiled/functions/10012920.c |
| 10016280 | `RadBeam::Allocate` | 被钩子调用 | decompiled/functions/10016280.c |
| 10055210 | `WeaponTypeExt::ExtData::GetBeamColor` | 被钩子调用 | decompiled/functions/10055210.c |
| 100162A0 | `EBolt::Fire` | 被钩子调用 | decompiled/functions/100162A0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 100162C0 | `BuildingClass::FireLaser` | 被钩子调用 | decompiled/functions/100162C0.c |
| 1000D3C0 | `BuildingClass::DestroyNthAnim` | 被钩子调用 | decompiled/functions/1000D3C0.c |
| 10016300 | `ObjectClass::IsGreenHP` | 被钩子调用 | decompiled/functions/10016300.c |
| 10016340 | `BuildingClass::BuildingClass__SetAnimSlotImage` | 被钩子调用 | decompiled/functions/10016340.c |
| 10016320 | `SpawnManagerClass::KillNodes` | 被钩子调用 | decompiled/functions/10016320.c |
| 100162E0 | `CaptureManagerClass::FreeAll` | 被钩子调用 | decompiled/functions/100162E0.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00448260 `BuildingClass::BuildingClass__ChangeOwner` | 9 | B.academy, B.prism, B.spy, T.jammer, ext:Building |
| 004549B0 `BuildingClass::BuildingClass__UpdateGapAndSpecialEffects` | 5 | B.prism, T.jammer, ext:Building, recovered:BuildingClass |
| 00442230 `BuildingClass::BuildingClass__ReceiveDamage` | 4 | B.firestorm, B.prism, recovered:BuildingClass |
| 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | 3 | B.prism, U.bugfixes, recovered:TemporalClass |
| 00447110 `BuildingClass::BuildingClass__Sell_Back` | 2 | B.prism, U.bugfixes |
