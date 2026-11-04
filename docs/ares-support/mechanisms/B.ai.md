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
| `AIBaseNormal` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:326 | ok | — |
| `AIBuildCounts` | BuildingType | ValueableVector<int> | (empty) | src/Ext/BuildingType/Body.cpp:314 | check_0A_use | — |
| `AIExtraCounts` | BuildingType | ValueableVector<int> | (empty) | src/Ext/BuildingType/Body.cpp:315 | check_0A_use | — |
| `AIInnerBase` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:325 | ok | — |
| `AI.BaseDefenseCounts` | Side | NullableVector<int> | — | src/Ext/Side/Body.cpp:101 | no_use_found | src/rules/native_processing.rs |
| `AI.BaseDefenses` | Side | NullableVector<BuildingTypeClass*> | — | src/Ext/Side/Body.cpp:103 | no_use_found | src/rules/native_processing.rs |

## 钩子（11）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4F65BF | `HouseClass_CanAffordBase` | kept | 004F6540 `HouseClass::HouseClass__Can_Make_Money` | decompiled/functions/10023840.c | src/Ext/House/Hooks.BasePlan.cpp:148 | src/sim/ai_build_need.rs |
| 5051E0 | `HouseClass_FirstBuildableFromArray` | kept | 005051E0 `HouseClass::HouseClass__FirstBuildableFromArray` | decompiled/functions/100238E0.c | src/Ext/House/Hooks.BasePlan.cpp:120 | src/rules/object_type.rs, src/sim/ai_build_need.rs, src/sim/ai_buildable.rs … |
| 5054B0 | `HouseClass_GenerateAIBuildList_EnsureSanity` | kept | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023AF0.c | src/Ext/House/Hooks.BasePlan.cpp:10 | src/sim/base_plan_generation.rs |
| 5055D8 | `HouseClass_GenerateAIBuildList_SWAllowed` | kept | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B90.c | src/Ext/House/Hooks.BasePlan.cpp:24 | src/sim/base_plan_generation.rs |
| 505B58 | `HouseClass_GenerateAIBuildList_SkipManualCopy` | kept | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023BD0.c | src/Ext/House/Hooks.BasePlan.cpp:46 | src/sim/base_plan_generation.rs |
| 505C34 | `HouseClass_GenerateAIBuildList_FullAutoCopy` | kept | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B10.c | src/Ext/House/Hooks.BasePlan.cpp:54 | src/sim/base_plan_generation.rs |
| 505C95 | `HouseClass_GenerateAIBuildList_CountExtra` | kept ⚠改过 | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023940.c | src/Ext/House/Hooks.BasePlan.cpp:60 | src/sim/base_plan_generation.rs |
| 505CF1 | `HouseClass_GenerateAIBuildList_PadWithN1` | kept ⚠改过 | 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | decompiled/functions/10023B30.c | src/Ext/House/Hooks.BasePlan.cpp:106 | src/sim/base_plan_generation.rs |
| 50610E | `HouseClass_FindPositionForBuilding_FixShipyard` | kept | 005060B0 `HouseClass::HouseClass__FindBaseBuildingSite` | decompiled/functions/100238A0.c | src/Ext/House/Hooks.BasePlan.cpp:134 | src/sim/ai_base_site.rs, src/sim/naval_base_placement.rs |
| 506306 | `HouseClass_FindPlaceToBuild_Evaluate` | new_in_3.0 | 005060B0 `HouseClass::HouseClass__FindBaseBuildingSite` | decompiled/functions/10023860.c | src/Recovered/HouseClass.cpp:74 | src/sim/ai_base_site.rs, src/sim/naval_base_placement.rs |
| 688B37 | `MPGameModeClass_CreateStartingUnits_B` | kept | 006886B0 `ScenarioClass__Generate_Random_Units` | decompiled/functions/10023BF0.c | src/Ext/House/Hooks.BasePlan.cpp:160 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10011810 | BuildingExt::ExtData::IsBaseNormal [translated] | 使用键 | decompiled/functions/10011810.c |
| 1000D6B0 | `BuildingTypeClass::GetFoundationWidth` | 被钩子调用 | decompiled/functions/1000D6B0.c |
| 1000D690 | `BuildingTypeClass::GetFoundationHeight` | 被钩子调用 | decompiled/functions/1000D690.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020CC0 | `HouseExt::FindBuildable` | 被钩子调用 | decompiled/functions/10020CC0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 100208F0 | `HouseExt::ExtData::CheckBasePlanSanity` | 被钩子调用 | decompiled/functions/100208F0.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 005060B0 `HouseClass::HouseClass__FindBaseBuildingSite` | 2 | B.ai, recovered:HouseClass |
| 005054B0 `HouseClass::HouseClass__AI_RecalcBuildOptions` | 6 | B.ai |
