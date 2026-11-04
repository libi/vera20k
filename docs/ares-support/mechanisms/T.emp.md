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
| `DeactivateDimEMP` | Rules | Valueable<double> | 0.8 | src/Ext/Rules/Body.cpp:194 | check_0A_use | — |
| `EMPAIRecoverMission` | Rules | Nullable<Mission> | (empty) | src/Ext/Rules/Body.cpp:228 | check_0A_use | src/rules/native_processing.rs |
| `EMP.Modifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:281 | no_use_found | src/rules/ares_emp.rs |
| `EMP.Sparkles` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:282 | check_0A_use | src/rules/native_processing.rs |
| `EMP.Threshold` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:286 | unscanned | src/rules/ares_emp.rs |
| `empimmune` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:30 | unscanned | src/rules/ares_abilities.rs, src/sim/game_entity/emp.rs |
| `ImmuneToEMP` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:280 | ok | src/rules/ares_emp.rs |
| `EMP.Cap` | WarheadType | int | -1 | src/Ext/WarheadType/Body.cpp:85 | check_0A_use | src/rules/ares_warhead.rs |
| `EMP.Duration` | WarheadType | int | 0 | src/Ext/WarheadType/Body.cpp:84 | check_0A_use | src/rules/ares_warhead.rs |
| `EMP.Sparkles` | WarheadType | Valueable<AnimTypeClass*> | nullptr | src/Ext/WarheadType/Body.cpp:86 | check_0A_use | src/rules/native_processing.rs |

## 钩子（3）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4456E5 | `BuildingClass_UpdateConstructionOptions_ExcludeDisabled` | kept | 004456D0 `BuildingClass::BuildingClass__UpdateConstructionOptions` | decompiled/functions/10053D80.c | src/Ext/WarheadType/Hooks.EMP.cpp:46 | src/sim/production/production_tech.rs |
| 6FAF0D | `TechnoClass_Update_EMPLock` | kept | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/10053DF0.c | src/Ext/WarheadType/Hooks.EMP.cpp:5 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 73D219 | `UnitClass_Draw_OreGatherAnim` | kept | 0073CEC0 `UnitClass::UnitClass__DrawIt` | decompiled/functions/10053E50.c | src/Ext/WarheadType/Hooks.EMP.cpp:34 | src/app/presentation/instances/units.rs, src/map/lighting.rs, src/render/sprite_atlas.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10003BB0 | SyringeHandshake [source_0.A,inferred] | 使用键 | decompiled/functions/10003BB0.c |
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1000B780 | BuildingTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B780.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F490.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred] | 使用键 | decompiled/functions/1002E060.c |
| 10052870 | WarheadTypeExt::ExtData::applyKillDriver [source_0.A] | 使用键 | decompiled/functions/10052870.c |
| 10055920 | WeaponTypeExt::ModifyWaveColor [translated,inferred] | 使用键 | decompiled/functions/10055920.c |
| 10060540 | CSFLoader::FindDynamic [source_0.A,inferred] | 使用键 | decompiled/functions/10060540.c |
| 10062320 | FUN_10062320 [translated,inferred] | 使用键 | decompiled/functions/10062320.c |
| 100627F0 | EMPulse::DisableEMPEffect [source_0.A] | 使用键 | decompiled/functions/100627F0.c |
| 10062D20 | FUN_10062d20 [source_0.A] | 使用键 | decompiled/functions/10062D20.c |
| 10062D90 | FUN_10062d90 [translated] | 使用键 | decompiled/functions/10062D90.c |
| 10062F70 | FUN_10062f70 [source_0.A] | 使用键 | decompiled/functions/10062F70.c |
| 10070BA0 | FUN_10070ba0 [source_0.A,inferred] | 使用键 | decompiled/functions/10070BA0.c |
| 10075920 | FUN_10075920 [source_0.A,inferred] | 使用键 | decompiled/functions/10075920.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10062240 | `TechnoExt::EjectPassengers` | 被钩子调用 | decompiled/functions/10062240.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |
| 10048690 | `TechnoClass::Deactivate` | 被钩子调用 | decompiled/functions/10048690.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 10062FD0 | `EMPulse::updateRadarBlackout` | 被钩子调用 | decompiled/functions/10062FD0.c |
| 10062CE0 | `BuildingClass::EnableStuff` | 被钩子调用 | decompiled/functions/10062CE0.c |
| 100487D0 | `TechnoClass::Reactivate` | 被钩子调用 | decompiled/functions/100487D0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 100708B0 | `PsyDom::Fire` | 被钩子调用 | decompiled/functions/100708B0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno, recovered:TechnoClass |
