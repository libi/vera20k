# `H.country` Country 展示、载入画面、权重、ParentCountry

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | docs/ares-country-presentation.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（15）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AIRandomSelectionWeight` | HouseType | Nullable<int> | (empty) | src/Ext/HouseType/Body.cpp:321 | check_0A_use | src/rules/ares_country_presentation.rs |
| `File.Flag` | HouseType | — | — | src/Ext/HouseType/Body.cpp:263 | unscanned | src/rules/ares_country_presentation.rs |
| `File.LoadScreen` | HouseType | — | — | src/Ext/HouseType/Body.cpp:267 | unscanned | src/rules/country_loading.rs |
| `File.LoadScreenPAL` | HouseType | — | — | src/Ext/HouseType/Body.cpp:268 | unscanned | src/rules/country_loading.rs |
| `File.ObserverBackground` | HouseType | — | — | src/Ext/HouseType/Body.cpp:265 | unscanned | src/rules/ares_country_presentation.rs |
| `File.ObserverFlag` | HouseType | — | — | src/Ext/HouseType/Body.cpp:264 | unscanned | src/rules/ares_country_presentation.rs |
| `File.ObserverFlagAltPalette` | HouseType | Valueable<bool> | false | src/Ext/HouseType/Body.cpp:284 | ok | src/rules/ares_country_presentation.rs |
| `File.Taunt` | HouseType | — | — | src/Ext/HouseType/Body.cpp:269 | unscanned | src/rules/ares_country_presentation.rs |
| `ListIndex` | HouseType | Valueable<int> | 100 | src/Ext/HouseType/Body.cpp:322 | no_use_found | src/rules/ares_country_presentation.rs |
| `LoadScreenText.Brief` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:281 | no_use_found | src/rules/country_loading.rs |
| `LoadScreenText.Color` | HouseType | ValueableIdx<ColorScheme> | -1 | src/Ext/HouseType/Body.cpp:318 | ok | src/rules/country_loading.rs |
| `LoadScreenText.Name` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:279 | no_use_found | src/rules/country_loading.rs |
| `LoadScreenText.SpecialName` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:280 | no_use_found | src/rules/country_loading.rs |
| `MenuText.Status` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:282 | no_use_found | src/rules/ares_country_presentation.rs |
| `RandomSelectionWeight` | HouseType | Valueable<int> | 1 | src/Ext/HouseType/Body.cpp:320 | check_0A_use | src/rules/ares_country_presentation.rs |

## 钩子（22）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4E3792 | `HTExt_Unlimit1` | kept | 004E3690 `FUN_004e3690` | decompiled/functions/1001DD70.c | src/Ext/HouseType/Hooks.cpp:106 | — |
| 4E3A6A | `hWnd_PopulateWithCountryNames` | kept | 004E3A00 `FUN_004e3a00` | decompiled/functions/1001E2C0.c | src/Ext/HouseType/Hooks.cpp:175 | src/app/shell_wol/my_info.rs, src/rules/ruleset.rs |
| 4E3A9C | `HTExt_Unlimit2` | kept | 004E3A00 `FUN_004e3a00` | decompiled/functions/1001DD80.c | src/Ext/HouseType/Hooks.cpp:109 | src/app/shell_wol/my_info.rs, src/rules/ruleset.rs |
| 4E3F31 | `HTExt_Unlimit3` | kept | 004E3CE0 `FUN_004e3ce0` | decompiled/functions/1001DD90.c | src/Ext/HouseType/Hooks.cpp:112 | — |
| 4E412C | `HTExt_Unlimit4` | kept | 004E3F70 `FUN_004e3f70` | decompiled/functions/1001DDA0.c | src/Ext/HouseType/Hooks.cpp:115 | src/app/shell_network_setup/cooperative.rs |
| 4E41A7 | `HTExt_Unlimit5` | kept | 004E4170 `FUN_004e4170` | decompiled/functions/1001DDB0.c | src/Ext/HouseType/Hooks.cpp:118 | — |
| 4FE782 | `HouseClass_AI_BaseConstructionUpdate_PickPowerplant` | kept | 004FE3E0 `HouseClass::HouseClass__AI_Choose_Building` | decompiled/functions/1001DDC0.c | src/Ext/HouseType/Hooks.cpp:137 | src/sim/ai_base_building.rs |
| 511635 | `HouseTypeClass_CTOR_1` | kept | 005113F0 `HouseTypeClass::HouseTypeClass__Constructor` | decompiled/functions/1001D0A0.c | src/Ext/HouseType/Body.cpp:546 | — |
| 511643 | `HouseTypeClass_CTOR_2` | kept | 005113F0 `HouseTypeClass::HouseTypeClass__Constructor` | decompiled/functions/1001D0A0.c | src/Ext/HouseType/Body.cpp:553 | — |
| 51214F | `HouseTypeClass_LoadFromINI` | kept | 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | decompiled/functions/1001D110.c | src/Ext/HouseType/Body.cpp:588 | src/app/frontend/skirmish_shell_render/text.rs |
| 512290 | `HouseTypeClass_SaveLoad_Prefix` | kept | 00512480 `HouseTypeClass__Save` | decompiled/functions/1001D210.c | src/Ext/HouseType/Body.cpp:567 | — |
| 51246D | `HouseTypeClass_Load_Suffix` | kept | 00512290 `HouseTypeClass__Load` | decompiled/functions/1001D130.c | src/Ext/HouseType/Body.cpp:578 | — |
| 51255C | `HouseTypeClass_Save_Suffix` | kept | 00512480 `HouseTypeClass__Save` | decompiled/functions/1001D250.c | src/Ext/HouseType/Body.cpp:583 | — |
| 5127CF | `HouseTypeClass_DTOR` | kept | 00512760 `HouseTypeClass::HouseTypeClass__ScalarDeletingDestructor` | decompiled/functions/1001D0D0.c | src/Ext/HouseType/Body.cpp:560 | — |
| 553412 | `LoadProgressMgr_Draw_LSFile` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001DFD0.c | src/Ext/HouseType/Hooks.cpp:9 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 5536DA | `LoadProgressMgr_Draw_LSName` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001E010.c | src/Ext/HouseType/Hooks.cpp:32 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553A05 | `LoadProgressMgr_Draw_LSSpecialName` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001E070.c | src/Ext/HouseType/Hooks.cpp:55 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553D06 | `LoadProgressMgr_Draw_LSBrief` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001DF70.c | src/Ext/HouseType/Hooks.cpp:79 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 6847B7 | `ScenarioClass_PrepareMapAndUDP` | replaces_0.A:Sides_LoadTextColor_CacheMP | 00684620 `ScenarioClass__Read_Scenario` | decompiled/functions/1002F8F0.c | src/Recovered/ScenarioClass.cpp:16 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/app/loading/pump.rs … |
| 69B670 | `HTExt_PickRandom_AI` | kept | 0069B670 `FUN_0069b670` | decompiled/functions/1001DD30.c | src/Ext/HouseType/Hooks.cpp:131 | — |
| 69B774 | `HTExt_PickRandom_Human` | kept | 0069B760 `FUN_0069b760` | decompiled/functions/1001DD50.c | src/Ext/HouseType/Hooks.cpp:123 | src/app/frontend/skirmish_session.rs, src/app/shell_wol/lobby.rs |
| 6AA164 | `StripClass_Draw_DrawObserverFlag` | replaces_0.A:TabCameoListClass_Draw_DrawObserverFlag | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1001E1E0.c | src/Recovered/StripClass.cpp:117 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001C9A0 | HouseTypeExt::PickRandomCountry [source_0.A] | 使用键 | decompiled/functions/1001C9A0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10021B00 | `HouseExt::PrereqValidate` | 被钩子调用 | decompiled/functions/10021B00.c |
| 10021CA0 | `HouseTypeExt::ExtData::GetPowerplants` | 被钩子调用 | decompiled/functions/10021CA0.c |
| 1001DC90 | `PCX::BlitToSurface` | 被钩子调用 | decompiled/functions/1001DC90.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | 7 | H.country, H.sidepresent |
| 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | 3 | H.country, recovered:HouseTypeClass |
