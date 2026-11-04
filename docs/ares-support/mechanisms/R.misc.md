# `R.misc` 全局杂项：随机箱钱数、显示延迟、盘状激光动画

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CanMakeStuffUp` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:176 | check_0A_use | — |
| `DisplayCreditsDelay` | Rules | Valueable<double> | 0.02 | src/Ext/Rules/Body.cpp:215 | no_use_found | src/rules/ares_bounty.rs |
| `RandomCrateMoney` | Rules | Valueable<int> | 900 | src/Ext/Rules/Body.cpp:221 | ok | — |
| `ReturnStructures` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:187 | check_0A_use | — |

## 钩子（12）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 481C6C | `CellClass_CrateBeingCollected_Armor1` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F840.c | src/Misc/Crates.cpp:19 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 481CE1 | `CellClass_CrateBeingCollected_Speed1` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F9D0.c | src/Misc/Crates.cpp:29 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 481D0E | `CellClass_CrateBeingCollected_Firepower1` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F940.c | src/Misc/Crates.cpp:39 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 481D3D | `CellClass_CrateBeingCollected_Cloak1` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F8D0.c | src/Misc/Crates.cpp:49 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 48248D | `CellClass_CrateBeingCollected_MoneyRandom` | new_in_3.0 | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1002C8A0.c | src/Recovered/CellClass.cpp:35 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 48294F | `CellClass_CrateBeingCollected_Cloak2` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F920.c | src/Misc/Crates.cpp:70 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 482E57 | `CellClass_CrateBeingCollected_Armor2` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F870.c | src/Misc/Crates.cpp:79 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 48303A | `CellClass_CrateBeingCollected_Speed2` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005FA00.c | src/Misc/Crates.cpp:94 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 483226 | `CellClass_CrateBeingCollected_Firepower2` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F970.c | src/Misc/Crates.cpp:108 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 4FC731 | `HouseClass_DestroyAll_ReturnStructures` | kept | 004FC6D0 `HouseClass::HouseClass__Blowup_All` | decompiled/functions/10024150.c | src/Ext/House/Hooks.cpp:461 | src/sim/temporal.rs, src/sim/world/house_defeat.rs |
| 55AFB3 | `LogicClass_Update_1000` | kept | 0055AFB0 `LogicClass::LogicClass__PerTickUpdate` | decompiled/functions/100680F0.c | src/Misc/Invalidators.cpp:437 | src/sim/crates/runtime.rs, src/sim/world/master_frame.rs, src/sim/world/mod.rs |
| 687C16 | `INIClass_ReadScenario_ValidateThings` | kept ⚠改过 | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/100677B0.c | src/Misc/Invalidators.cpp:65 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1000B780 | BuildingTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B780.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 10064F70 | GetSectionCached [translated,inferred] | 使用键 | decompiled/functions/10064F70.c |
| 10070230 | FUN_10070230 [source_0.A,inferred] | 使用键 | decompiled/functions/10070230.c |
| 10023D60 | `BuildingClass::KillOccupants` | 被钩子调用 | decompiled/functions/10023D60.c |
| 10020F20 | `HouseClass::FindCivilianSide` | 被钩子调用 | decompiled/functions/10020F20.c |
| 10023D00 | `BuildingClass::DisableStuff` | 被钩子调用 | decompiled/functions/10023D00.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100460A0 | `TechnoExt::ExtData::RecalculateStats` | 被钩子调用 | decompiled/functions/100460A0.c |
| 10043450 | `TechnoExt::CanICloakByDefault` | 被钩子调用 | decompiled/functions/10043450.c |
| 10078000 | `Exception::PrepareSnapshotDirectory` | 被钩子调用 | decompiled/functions/10078000.c |
| 10061A30 | `RadarClass::First_Time` | 被钩子调用 | decompiled/functions/10061A30.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00481A00 `CellClass::CellClass__PickupCrate` | 9 | R.misc, recovered:CellClass |
