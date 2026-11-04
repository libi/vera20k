# `B.ai` AI 基地建造计数与内圈

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AIBaseNormal` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:322 | ok | — |
| `AIBuildCounts` | BuildingType | ValueableVector<int> | (empty) | src/Ext/BuildingType/Body.cpp:310 | ok | — |
| `AIExtraCounts` | BuildingType | ValueableVector<int> | (empty) | src/Ext/BuildingType/Body.cpp:311 | ok | — |
| `AIInnerBase` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:321 | ok | — |
| `AI.BaseDefenseCounts` | Side | NullableVector<int> | — | src/Ext/Side/Body.cpp:106 | no_use_found | src/rules/native_processing.rs |
| `AI.BaseDefenses` | Side | NullableVector<BuildingTypeClass*> | — | src/Ext/Side/Body.cpp:108 | no_use_found | src/rules/native_processing.rs |

## 钩子（12）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4F65BF | `HouseClass_CanAffordBase` | kept | ✓ | 004F6540 `HouseClass::HouseClass__Can_Make_Money` | decompiled/functions/10023840.c | src/Ext/House/Hooks.BasePlan.cpp:187 | src/sim/ai_build_need.rs |
| 5051E0 | `HouseClass_FirstBuildableFromArray` | kept | ✓ | 005051E0 `HouseClass::HouseClass__FirstBuildableFromArray` | decompiled/functions/100238E0.c | src/Ext/House/Hooks.BasePlan.cpp:145 | src/rules/object_type.rs, src/sim/ai_build_need.rs, src/sim/ai_buildable.rs … |
| 5054B0 | `HouseClass_GenerateAIBuildList_EnsureSanity` | kept | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023AF0.c | src/Ext/House/Hooks.BasePlan.cpp:19 | src/sim/base_plan_generation.rs |
| 5055D8 | `HouseClass_GenerateAIBuildList_SWAllowed` | kept | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B90.c | src/Ext/House/Hooks.BasePlan.cpp:33 | src/sim/base_plan_generation.rs |
| 505B58 | `HouseClass_GenerateAIBuildList_SkipManualCopy` | kept | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023BD0.c | src/Ext/House/Hooks.BasePlan.cpp:55 | src/sim/base_plan_generation.rs |
| 505C34 | `HouseClass_GenerateAIBuildList_FullAutoCopy` | kept | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B10.c | src/Ext/House/Hooks.BasePlan.cpp:63 | src/sim/base_plan_generation.rs |
| 505C95 | `HouseClass_GenerateAIBuildList_CountExtra` | kept ⚠改过 | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023940.c | src/Ext/House/Hooks.BasePlan.cpp:69 | src/sim/base_plan_generation.rs |
| 505CF1 | `HouseClass_GenerateAIBuildList_PadWithN1` | kept ⚠改过 | ✓ | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B30.c | src/Ext/House/Hooks.BasePlan.cpp:131 | src/sim/base_plan_generation.rs |
| 50610E | `HouseClass_FindPositionForBuilding_FixShipyard` | kept | ✓ | 005060B0 `HouseClass::HouseClass__FindBaseBuildingSite` | decompiled/functions/100238A0.c | src/Ext/House/Hooks.BasePlan.cpp:159 | src/sim/ai_base_site.rs, src/sim/naval_base_placement.rs |
| 506306 | `HouseClass_FindPlaceToBuild_Evaluate` | new_in_3.0 | ✓ | 005060B0 `HouseClass::HouseClass__FindBaseBuildingSite` | decompiled/functions/10023860.c | src/Ext/House/Hooks.BasePlan.cpp:176 | src/sim/ai_base_site.rs, src/sim/naval_base_placement.rs |
| 5D705E | `MPGameMode_SpawnBaseUnit_BaseUnit` | replaces_0.A:MPGameMode_SpawnBaseUnit | ✓ | 005D7030 `MultiplayerGameMode__Create_Starting_Base_Unit` | decompiled/functions/10023C80.c | src/Ext/House/Hooks.BasePlan.cpp:203 | src/app/frontend/skirmish.rs, src/sim/scenario_bootstrap.rs |
| 688B37 | `MPGameModeClass_CreateStartingUnits_B` | kept | ✓ | 006886B0 `ScenarioClass__Generate_Random_Units` | decompiled/functions/10023BF0.c | src/Ext/House/Hooks.BasePlan.cpp:220 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10011810 | BuildingExt::ExtData::IsBaseNormal [translated] | 使用键 | decompiled/functions/10011810.c |
| 1000D6B0 | `BuildingTypeClass::GetFoundationWidth` | 被钩子调用 | decompiled/functions/1000D6B0.c |
| 1000D690 | `BuildingTypeClass::GetFoundationHeight` | 被钩子调用 | decompiled/functions/1000D690.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020CC0 | `HouseExt::FindBuildableIndex` | 被钩子调用 | decompiled/functions/10020CC0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1002DD40 | `SideExt::ExtData::GetBaseDefenseCounts` | 被钩子调用 | decompiled/functions/1002DD40.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 100208F0 | `HouseExt::ExtData::CheckBasePlanSanity` | 被钩子调用 | decompiled/functions/100208F0.c |
| 100210E0 | `HouseExt::FindOwnedIndex` | 被钩子调用 | decompiled/functions/100210E0.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 005D7030 `MultiplayerGameMode__Create_Starting_Base_Unit` | 2 | B.ai, H.country |
| 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | 6 | B.ai |
