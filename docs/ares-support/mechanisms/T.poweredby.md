# `T.poweredby` PoweredBy 供电单位

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M6 | T.emp | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DeactivateDimPowered` | Rules | Valueable<double> | 0.5 | src/Ext/Rules/Body.cpp:193 | check_0A_use | — |
| `PoweredBy` | TechnoType | ValueableVector<BuildingTypeClass*> | — | src/Ext/TechnoType/Body.cpp:348 | check_0A_use | — |

## 钩子（8）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 51D0DD | `InfantryClass_Scatter` | kept | 0051D0D0 `InfantryClass::InfantryClass__Scatter` | decompiled/functions/1004CC60.c | src/Ext/Techno/Hooks.Deactivated.cpp:43 | src/sim/infantry.rs, src/sim/movement/infantry_scatter.rs, src/sim/movement/scatter.rs … |
| 5200B3 | `InfantryClass_UpdatePanic` | kept | 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | decompiled/functions/1004CC80.c | src/Ext/Techno/Hooks.Deactivated.cpp:31 | src/sim/infantry.rs |
| 687C16 | `INIClass_ReadScenario_ValidateThings` | kept ⚠改过 | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/100677B0.c | src/Misc/Invalidators.cpp:65 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |
| 70FBE0 | `TechnoClass_Activate` | kept | 0070FBE0 `TechnoClass::TechnoClass__Reactivate` | decompiled/functions/1004CCB0.c | src/Ext/Techno/Hooks.Deactivated.cpp:107 | src/sim/powered_unit.rs |
| 70FC90 | `TechnoClass_Deactivate` | kept | 0070FC90 `TechnoClass::TechnoClass__OnDeployBegin` | decompiled/functions/1004CDC0.c | src/Ext/Techno/Hooks.Deactivated.cpp:157 | src/sim/mod.rs, src/sim/powered_unit.rs |
| 736135 | `UnitClass_Update_Deactivated` | kept | 007360C0 `UnitClass::UnitClass__AI` | decompiled/functions/1004CFD0.c | src/Ext/Techno/Hooks.Deactivated.cpp:70 | src/sim/mission/authority.rs, src/sim/world/object_turn.rs, src/sim/world/unit_repair_seek.rs |
| 73C143 | `UnitClass_DrawVXL_Deactivated` | kept | 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | decompiled/functions/1004CEA0.c | src/Ext/Techno/Hooks.Deactivated.cpp:79 | src/app/presentation/instances/units.rs, src/render/vxl_native.rs, src/rules/voxel_frame_catalog.rs |
| 73DBF9 | `UnitClass_Mi_Unload_Decactivated` | kept | 0073D630 `UnitClass::UnitClass__Mission_Unload` | decompiled/functions/1004CF80.c | src/Ext/Techno/Hooks.Deactivated.cpp:53 | src/rules/object_type.rs, src/sim/components.rs, src/sim/deploy.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1002D410.c |
| 1006A6F0 | TechnoExt::Destroy [source_0.A] | 使用键 | decompiled/functions/1006A6F0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 100454F0 | `TechnoExt::ExtData::IsPowered` | 被钩子调用 | decompiled/functions/100454F0.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10042C10 | `BuildingLightClass::BuildingLightClass` | 被钩子调用 | decompiled/functions/10042C10.c |
| 10046420 | `TechnoExt::ExtData::SetSpotlight` | 被钩子调用 | decompiled/functions/10046420.c |
| 10013250 | `BuildingExt::UpdateFactoryQueues` | 被钩子调用 | decompiled/functions/10013250.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 1004CAC0 | `UnitClass::IsDeactivated` | 被钩子调用 | decompiled/functions/1004CAC0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10061A30 | `RadarClass::First_Time` | 被钩子调用 | decompiled/functions/10061A30.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10062D60 | `EMPulse::IsDeactivationAdvisable` | 被钩子调用 | decompiled/functions/10062D60.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0073D630 `UnitClass::UnitClass__Mission_Unload` | 5 | T.poweredby, U.bugfixes, ext:House, recovered:UnitClass |
| 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | 4 | T.ammo, T.poweredby, W.death, ext:Infantry |
| 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | 7 | T.misc, T.poweredby, recovered:UnitClass |
