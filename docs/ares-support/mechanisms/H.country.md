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
| `AIRandomSelectionWeight` | HouseType | Nullable<int> | (empty) | src/Ext/HouseType/Body.cpp:324 | ok | src/rules/ares_country_presentation.rs |
| `File.Flag` | HouseType | — | — | src/Ext/HouseType/Body.cpp:266 | unscanned | src/rules/ares_country_presentation.rs |
| `File.LoadScreen` | HouseType | — | — | src/Ext/HouseType/Body.cpp:270 | unscanned | src/rules/country_loading.rs |
| `File.LoadScreenPAL` | HouseType | — | — | src/Ext/HouseType/Body.cpp:271 | unscanned | src/rules/country_loading.rs |
| `File.ObserverBackground` | HouseType | — | — | src/Ext/HouseType/Body.cpp:268 | unscanned | src/rules/ares_country_presentation.rs |
| `File.ObserverFlag` | HouseType | — | — | src/Ext/HouseType/Body.cpp:267 | unscanned | src/rules/ares_country_presentation.rs |
| `File.ObserverFlagAltPalette` | HouseType | Valueable<bool> | false | src/Ext/HouseType/Body.cpp:287 | ok | src/rules/ares_country_presentation.rs |
| `File.Taunt` | HouseType | — | — | src/Ext/HouseType/Body.cpp:272 | unscanned | src/rules/ares_country_presentation.rs |
| `ListIndex` | HouseType | Valueable<int> | 100 | src/Ext/HouseType/Body.cpp:325 | no_use_found | src/rules/ares_country_presentation.rs |
| `LoadScreenText.Brief` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:284 | no_use_found | src/rules/country_loading.rs |
| `LoadScreenText.Color` | HouseType | ValueableIdx<ColorScheme> | -1 | src/Ext/HouseType/Body.cpp:321 | ok | src/rules/country_loading.rs |
| `LoadScreenText.Name` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:282 | no_use_found | src/rules/country_loading.rs |
| `LoadScreenText.SpecialName` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:283 | no_use_found | src/rules/country_loading.rs |
| `MenuText.Status` | HouseType | Valueable<CSFText> | — | src/Ext/HouseType/Body.cpp:285 | no_use_found | src/rules/ares_country_presentation.rs |
| `RandomSelectionWeight` | HouseType | Valueable<int> | 1 | src/Ext/HouseType/Body.cpp:323 | ok | src/rules/ares_country_presentation.rs |

## 钩子（34）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 48DA3B | `sub_48D1E0_PlayTaunt` | new_in_3.0 | ✓ | 0048D1E0 `Network__ProcessGlobalPackets` | decompiled/functions/1001E400.c | src/Ext/HouseType/Hooks.cpp:233 | src/app/diagnostics/shell_capture/movies/lan/live/cooperative.rs, src/app/match_runtime/beacons.rs, src/net/lan_messages/ingame.rs … |
| 4E3562 | `Game_GetFlagSurface` | new_in_3.0 | ✓ | 004E3560 `Get_Country_Insignia_Surface` | decompiled/functions/1001DCE0.c | src/Ext/HouseType/Hooks.cpp:124 | src/ui/shell/diplomacy.rs |
| 4E3792 | `HTExt_Unlimit1` | kept | ✓ | 004E3690 `FUN_004e3690` | decompiled/functions/1001DD70.c | src/Ext/HouseType/Hooks.cpp:268 | — |
| 4E38D8 | `LoadPlayerCountryString` | replaces_0.A:HTExt_GetSTT | ✓ | 004E38A0 `FUN_004e38a0` | decompiled/functions/1001DF10.c | src/Ext/HouseType/Hooks.cpp:185 | — |
| 4E3A6A | `hWnd_PopulateWithCountryNames` | kept | ✓ | 004E3A00 `FUN_004e3a00` | decompiled/functions/1001E2C0.c | src/Ext/HouseType/Hooks.cpp:339 | src/app/shell_wol/my_info.rs, src/rules/ruleset.rs |
| 4E3A9C | `HTExt_Unlimit2` | kept | ✓ | 004E3A00 `FUN_004e3a00` | decompiled/functions/1001DD80.c | src/Ext/HouseType/Hooks.cpp:271 | src/app/shell_wol/my_info.rs, src/rules/ruleset.rs |
| 4E3F31 | `HTExt_Unlimit3` | kept | ✓ | 004E3CE0 `FUN_004e3ce0` | decompiled/functions/1001DD90.c | src/Ext/HouseType/Hooks.cpp:274 | — |
| 4E412C | `HTExt_Unlimit4` | kept | ✓ | 004E3F70 `FUN_004e3f70` | decompiled/functions/1001DDA0.c | src/Ext/HouseType/Hooks.cpp:277 | src/app/shell_network_setup/cooperative.rs |
| 4E41A7 | `HTExt_Unlimit5` | kept | ✓ | 004E4170 `FUN_004e4170` | decompiled/functions/1001DDB0.c | src/Ext/HouseType/Hooks.cpp:280 | — |
| 4FE782 | `HouseClass_AI_BaseConstructionUpdate_PickPowerplant` | kept | ✓ | 004FE3E0 `HouseClass::HouseClass__AI_Choose_Building` | decompiled/functions/1001DDC0.c | src/Ext/HouseType/Hooks.cpp:301 | src/sim/ai_base_building.rs |
| 511635 | `HouseTypeClass_CTOR_1` | kept | ✓ | 005113F0 `HouseTypeClass::HouseTypeClass__Constructor` | decompiled/functions/1001D0A0.c | src/Ext/HouseType/Body.cpp:574 | — |
| 511643 | `HouseTypeClass_CTOR_2` | kept | ✓ | 005113F0 `HouseTypeClass::HouseTypeClass__Constructor` | decompiled/functions/1001D0A0.c | src/Ext/HouseType/Body.cpp:581 | — |
| 51214F | `HouseTypeClass_LoadFromINI` | kept | ✓ | 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | decompiled/functions/1001D110.c | src/Ext/HouseType/Body.cpp:616 | src/app/frontend/skirmish_shell_render/text.rs |
| 512290 | `HouseTypeClass_SaveLoad_Prefix` | kept | ✓ | 00512480 `HouseTypeClass__Save` | decompiled/functions/1001D210.c | src/Ext/HouseType/Body.cpp:595 | — |
| 51246D | `HouseTypeClass_Load_Suffix` | kept | ✓ | 00512290 `HouseTypeClass__Load` | decompiled/functions/1001D130.c | src/Ext/HouseType/Body.cpp:606 | — |
| 51255C | `HouseTypeClass_Save_Suffix` | kept | ✓ | 00512480 `HouseTypeClass__Save` | decompiled/functions/1001D250.c | src/Ext/HouseType/Body.cpp:611 | — |
| 5127CF | `HouseTypeClass_DTOR` | kept | ✓ | 00512760 `HouseTypeClass::HouseTypeClass__ScalarDeletingDestructor` | decompiled/functions/1001D0D0.c | src/Ext/HouseType/Body.cpp:588 | — |
| 536438 | `TauntCommandClass_Execute` | new_in_3.0 | ✓ | 00536350 `TauntCommandClass__Execute` | decompiled/functions/1001E290.c | src/Ext/HouseType/Hooks.cpp:215 | src/app/input/dispatch.rs, src/app/input/taunt.rs, src/app/match_runtime/session.rs … |
| 553412 | `LoadProgressMgr_Draw_LSFile` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001DFD0.c | src/Ext/HouseType/Hooks.cpp:19 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 5536DA | `LoadProgressMgr_Draw_LSName` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001E010.c | src/Ext/HouseType/Hooks.cpp:42 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553A05 | `LoadProgressMgr_Draw_LSSpecialName` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001E070.c | src/Ext/HouseType/Hooks.cpp:65 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553D06 | `LoadProgressMgr_Draw_LSBrief` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1001DF70.c | src/Ext/HouseType/Hooks.cpp:90 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 5D6D9A | `MPGameModeClass_CreateStartingUnits_UnitCost` | new_in_3.0 | ✓ | 005D6D80 `MultiplayerGameMode__Generate_Starting_Units` | decompiled/functions/1001EE10.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:141 | src/sim/scenario_bootstrap.rs |
| 5D6F61 | `MPGameModeClass_CreateStartingUnits_BaseCenter` | new_in_3.0 | ✓ | 005D6D80 `MultiplayerGameMode__Generate_Starting_Units` | decompiled/functions/1001ED60.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:217 | src/sim/scenario_bootstrap.rs |
| 5D7048 | `MPGameMode_SpawnBaseUnit_BuildConst` | new_in_3.0 | ✓ | 005D7030 `MultiplayerGameMode__Create_Starting_Base_Unit` | decompiled/functions/1001F0B0.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:32 | src/app/frontend/skirmish.rs, src/sim/scenario_bootstrap.rs |
| 5D7163 | `MPGameMode_SpawnStartingUnits_Types` | new_in_3.0 | ✓ | 005D70F0 `MultiplayerGameMode__GenerateStartingUnits` | decompiled/functions/1001F220.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:92 | src/sim/scenario_bootstrap.rs |
| 5D7337 | `MPGameMode_SpawnStartingUnits_NoInfantry` | new_in_3.0 | ✓ | 005D70F0 `MultiplayerGameMode__GenerateStartingUnits` | decompiled/functions/1001F200.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:127 | src/sim/scenario_bootstrap.rs |
| 6847B7 | `ScenarioClass_PrepareMapAndUDP` | replaces_0.A:Sides_LoadTextColor_CacheMP | ✓ | 00684620 `ScenarioClass__Read_Scenario` | decompiled/functions/1002F8F0.c | src/Ext/Side/Hooks.cpp:172 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/app/loading/pump.rs … |
| 69B670 | `HTExt_PickRandom_AI` | kept | ✓ | 0069B670 `FUN_0069b670` | decompiled/functions/1001DD30.c | src/Ext/HouseType/Hooks.cpp:293 | — |
| 69B774 | `HTExt_PickRandom_Human` | kept | ✓ | 0069B760 `FUN_0069b760` | decompiled/functions/1001DD50.c | src/Ext/HouseType/Hooks.cpp:285 | src/app/frontend/skirmish_session.rs, src/app/shell_wol/lobby.rs |
| 6AA0CA | `StripClass_Draw_DrawObserverBackground` | replaces_0.A:TabCameoListClass_Draw_DrawObserverBackground | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1001E160.c | src/Ext/HouseType/Hooks.cpp:380 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6AA164 | `StripClass_Draw_DrawObserverFlag` | replaces_0.A:TabCameoListClass_Draw_DrawObserverFlag | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1001E1E0.c | src/Ext/HouseType/Hooks.cpp:409 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 72B690 | `LoadScreenPal_Load` | replaces_0.A:HTExt_LSPAL | ✓ | 0072B530 `LoadingScreen__LoadPaletteForCountry` | decompiled/functions/1001E0E0.c | src/Ext/HouseType/Hooks.cpp:150 | — |
| 752B70 | `PlayTaunt` | new_in_3.0 | ✓ | 00752B70 `SpeechSystem__PlayTaunt` | decompiled/functions/1001E130.c | src/Ext/HouseType/Hooks.cpp:211 | src/audio/sfx.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001C9A0 | HouseTypeExt::PickRandomCountry [source_0.A,3.0✓] | 使用键 | decompiled/functions/1001C9A0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 100617C0 | `Debug::FatalErrorAndExit` | 被钩子调用 | decompiled/functions/100617C0.c |
| 10057D10 | `ContainerMapBase::remove` | 被钩子调用 | decompiled/functions/10057D10.c |
| 1006B770 | `AresStreamReader::RegisterChange` | 被钩子调用 | decompiled/functions/1006B770.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1001BD20 | `HouseTypeExt::ExtData::GetPowerplants` | 被钩子调用 | decompiled/functions/1001BD20.c |
| 10021B00 | `HouseExt::PrereqValidate` | 被钩子调用 | decompiled/functions/10021B00.c |
| 10021CA0 | `HouseExt::ExtData::PrerequisitesMet` | 被钩子调用 | decompiled/functions/10021CA0.c |
| 1001DCD0 | `FixedString::data` | 被钩子调用 | decompiled/functions/1001DCD0.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |
| 1001DCB0 | `ConvertClass::CreateFromFile` | 被钩子调用 | decompiled/functions/1001DCB0.c |
| 1001CAD0 | `HouseTypeExt::PlayTaunt` | 被钩子调用 | decompiled/functions/1001CAD0.c |
| 1001DC90 | `PCX::BlitToSurface` | 被钩子调用 | decompiled/functions/1001DC90.c |
| 1001E5C0 | `MPGameModeClass_CreateStartingUnits_UnitCost::AddTypes` | 被钩子调用 | decompiled/functions/1001E5C0.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020CC0 | `HouseExt::FindBuildableIndex` | 被钩子调用 | decompiled/functions/10020CC0.c |
| 1000D6B0 | `BuildingTypeClass::GetFoundationWidth` | 被钩子调用 | decompiled/functions/1000D6B0.c |
| 1000D690 | `BuildingTypeClass::GetFoundationHeight` | 被钩子调用 | decompiled/functions/1000D690.c |
| 1001E630 | `HouseClass::HouseClass__RefreshBuildOptionsAfterDeploy` | 被钩子调用 | decompiled/functions/1001E630.c |

## 涉及的热点函数（4）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006A9540 `StripClass::StripClass__Draw` | 18 | H.country, S.core, T.cameo, T.factory, ext:Rules |
| 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | 7 | H.country, H.sidepresent |
| 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | 3 | H.country, U.bugfixes |
| 005D7030 `MultiplayerGameMode__Create_Starting_Base_Unit` | 2 | B.ai, H.country |
