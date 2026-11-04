# `R.tiberium` 矿石伤害/治疗/爆炸

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（14）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `OverlayExplodeThreshold` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:183 | check_0A_use | — |
| `TiberiumDamageEnabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:178 | check_0A_use | — |
| `TiberiumExplosiveAnim` | Rules | Valueable<AnimTypeClass*> | nullptr | src/Ext/Rules/Body.cpp:181 | check_0A_use | — |
| `TiberiumExplosiveWarhead` | Rules | — | — | src/Ext/Rules/Body.cpp:180 | unscanned | — |
| `TiberiumHealEnabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:179 | check_0A_use | — |
| `Damage` | Tiberium | Nullable<int> | (empty) | src/Ext/Tiberium/Body.cpp:17 | check_0A_use | src/app/diagnostics/shell_capture/movies/lan/live/battle.rs, src/app/diagnostics/tactical_capture/map_observation.rs, src/app/presentation/combat_lights.rs … |
| `Debris.Chance` | Tiberium | Valueable<int> | 33 | src/Ext/Tiberium/Body.cpp:28 | check_0A_use | — |
| `ExplosionDamage` | Tiberium | Nullable<int> | (empty) | src/Ext/Tiberium/Body.cpp:26 | check_0A_use | — |
| `ExplosionWarhead` | Tiberium | Nullable<WarheadTypeClass*> | (empty) | src/Ext/Tiberium/Body.cpp:25 | check_0A_use | — |
| `Heal.Delay` | Tiberium | Nullable<double> | (empty) | src/Ext/Tiberium/Body.cpp:23 | check_0A_use | — |
| `Heal.IStep` | Tiberium | Nullable<int> | (empty) | src/Ext/Tiberium/Body.cpp:21 | check_0A_use | — |
| `Heal.Step` | Tiberium | Nullable<int> | (empty) | src/Ext/Tiberium/Body.cpp:20 | check_0A_use | — |
| `Heal.UStep` | Tiberium | Nullable<int> | (empty) | src/Ext/Tiberium/Body.cpp:22 | check_0A_use | — |
| `Warhead` | Tiberium | Nullable<WarheadTypeClass*> | (empty) | src/Ext/Tiberium/Body.cpp:18 | check_0A_use | src/app/presentation/combat_lights.rs, src/rules/native_processing.rs, src/rules/ruleset.rs … |

## 钩子（14）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 424DD3 | `AnimClass_ReInit_TiberiumChainReaction_Chance` | kept | 00424CE0 `AnimClass::AnimClass__Start` | decompiled/functions/10050D50.c | src/Ext/Tiberium/Hooks.cpp:84 | src/sim/anim_class.rs, src/sim/world/damage_consequences.rs |
| 424EC5 | `AnimClass_ReInit_TiberiumChainReaction_Damage` | kept | 00424CE0 `AnimClass::AnimClass__Start` | decompiled/functions/10050DA0.c | src/Ext/Tiberium/Hooks.cpp:93 | src/sim/anim_class.rs, src/sim/world/damage_consequences.rs |
| 489270 | `CellChainReact` | kept | 00489270 `CellChainReaction__Request` | decompiled/functions/10050DD0.c | src/Ext/Tiberium/Hooks.cpp:11 | — |
| 48964F | `DamageArea_ChainReaction` | kept | 00489280 `Apply_area_damage` | decompiled/functions/10051000.c | src/Ext/Tiberium/Hooks.cpp:77 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 48A2D9 | `DamageArea_ExplodesThreshold` | kept | 00489280 `Apply_area_damage` | decompiled/functions/1002C8E0.c | src/Ext/Rules/Hooks.cpp:51 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4D85E4 | `FootClass_UpdatePosition_TiberiumDamage` | kept | 004D85D0 `FootClass::FootClass__PerCellProcess` | decompiled/functions/10049280.c | src/Ext/Techno/Hooks.cpp:1088 | src/sim/movement/per_cell.rs, src/sim/world/techno_ai_cloak.rs |
| 4DA53E | `FootClass_Update` | kept | 004DA530 `FootClass::FootClass__AI` | decompiled/functions/10014BA0.c | src/Ext/Building/Hooks.Firewall.cpp:232 | src/sim/aircraft/mod.rs |
| 702216 | `TechnoClass_ReceiveDamage_TiberiumHeal` | kept | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004B050.c | src/Ext/Techno/Hooks.cpp:1064 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |
| 721876 | `TiberiumClass_CTOR` | kept | 007216C0 `TiberiumClass::TiberiumClass__Constructor` | decompiled/functions/100509C0.c | src/Ext/Tiberium/Body.cpp:125 | src/rules/tiberium_type.rs |
| 72193A | `TiberiumClass_DTOR` | kept | 00721880 `TiberiumClass::TiberiumClass__Destructor` | decompiled/functions/100509E0.c | src/Ext/Tiberium/Body.cpp:133 | — |
| 721C7B | `TiberiumClass_LoadFromINI` | kept | 00721A50 `TiberiumClass::TiberiumClass__ReadINI` | decompiled/functions/10050A10.c | src/Ext/Tiberium/Body.cpp:164 | src/rules/native_processing.rs, src/rules/tiberium_type.rs |
| 721E80 | `TiberiumClass_SaveLoad_Prefix` | kept | 007220D0 `TiberiumClass__Save` | decompiled/functions/10050B70.c | src/Ext/Tiberium/Body.cpp:141 | — |
| 72208C | `TiberiumClass_Load_Suffix` | kept | 00721E80 `TiberiumClass__Load` | decompiled/functions/10050A30.c | src/Ext/Tiberium/Body.cpp:152 | src/sim/snapshot.rs |
| 72212C | `TiberiumClass_Save_Suffix` | kept | 007220D0 `TiberiumClass__Save` | decompiled/functions/10050BB0.c | src/Ext/Tiberium/Body.cpp:89 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1000B780 | BuildingTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B780.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002DDD0 | FUN_1002ddd0 [source_0.A,inferred] | 使用键 | decompiled/functions/1002DDD0.c |
| 100507A0 | TiberiumExt::ExtData::GetDamage [source_0.A] | 使用键 | decompiled/functions/100507A0.c |
| 100507D0 | TiberiumExt::ExtData::GetDebrisChance [source_0.A] | 使用键 | decompiled/functions/100507D0.c |
| 100507E0 | TiberiumExt::ExtData::GetExplosionDamage [source_0.A] | 使用键 | decompiled/functions/100507E0.c |
| 10050800 | TiberiumExt::ExtData::GetExplosionWarhead [source_0.A] | 使用键 | decompiled/functions/10050800.c |
| 10050820 | TiberiumExt::ExtData::GetHealDelay [source_0.A] | 使用键 | decompiled/functions/10050820.c |
| 10050840 | TiberiumExt::ExtData::GetHealStep [source_0.A] | 使用键 | decompiled/functions/10050840.c |
| 100508B0 | TiberiumExt::ExtData::GetWarhead [source_0.A] | 使用键 | decompiled/functions/100508B0.c |
| 10064F70 | GetSectionCached [translated,inferred] | 使用键 | decompiled/functions/10064F70.c |
| 10070230 | FUN_10070230 [source_0.A,inferred] | 使用键 | decompiled/functions/10070230.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10012770 | `BuildingExt::IsActiveFirestormWall` | 被钩子调用 | decompiled/functions/10012770.c |
| 10011F30 | `BuildingExt::ExtData::ImmolateVictim` | 被钩子调用 | decompiled/functions/10011F30.c |
| 100149C0 | `CellClass::GetContainedTiberiumIndex` | 被钩子调用 | decompiled/functions/100149C0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 100468B0 | `TechnoExt::SpawnVisceroid` | 被钩子调用 | decompiled/functions/100468B0.c |
| 10011DE0 | `CellClass::GetNeighbourCell` | 被钩子调用 | decompiled/functions/10011DE0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10023D40 | `CellClass::IncreaseTiberium` | 被钩子调用 | decompiled/functions/10023D40.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10050D10 | `CellClass::MarkForRedraw` | 被钩子调用 | decompiled/functions/10050D10.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 10050D30 | `TiberiumClass::RegisterForGrowth` | 被钩子调用 | decompiled/functions/10050D30.c |
| 10078000 | `Exception::PrepareSnapshotDirectory` | 被钩子调用 | decompiled/functions/10078000.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00489280 `Apply_area_damage` | 14 | R.tiberium, U.bugfixes, W.filter, ext:Rules, recovered:DamageArea |
