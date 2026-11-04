# `H.sidepresent` Side 展示：计分、对话框背景、图形文字、侧栏文件、颜色、载入主题

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | H.side | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（19）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CampaignScore.Animation` | Side | — | — | src/Ext/Side/Body.cpp:142 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Background` | Side | — | — | src/Ext/Side/Body.cpp:140 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.OverParTheme` | Side | — | — | src/Ext/Side/Body.cpp:156 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Palette` | Side | — | — | src/Ext/Side/Body.cpp:143 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Transition` | Side | — | — | src/Ext/Side/Body.cpp:141 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.UnderParTheme` | Side | — | — | src/Ext/Side/Body.cpp:155 | unscanned | src/rules/native_processing.rs |
| `DialogBackground.Image` | Side | — | — | src/Ext/Side/Body.cpp:163 | unscanned | src/rules/native_processing.rs |
| `DialogBackground.Palette` | Side | — | — | src/Ext/Side/Body.cpp:164 | unscanned | src/rules/native_processing.rs |
| `GraphicalText.Image` | Side | — | — | src/Ext/Side/Body.cpp:160 | unscanned | src/rules/native_processing.rs |
| `GraphicalText.Palette` | Side | — | — | src/Ext/Side/Body.cpp:161 | unscanned | src/rules/native_processing.rs |
| `LoadingTheme` | Side | — | — | src/Ext/Side/Hooks.cpp:634 | unscanned | src/app/loading/pump.rs, src/rules/process_owner.rs |
| `MultiplayerScore.Background` | Side | — | — | src/Ext/Side/Body.cpp:144 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.Bars` | Side | — | — | src/Ext/Side/Body.cpp:146 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.LoseTheme` | Side | — | — | src/Ext/Side/Body.cpp:158 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.Palette` | Side | — | — | src/Ext/Side/Body.cpp:145 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.WinTheme` | Side | — | — | src/Ext/Side/Body.cpp:157 | unscanned | src/rules/native_processing.rs |
| `SCORE` | Side | AresCRT::strCopy | — | src/Ext/Side/Body.h:78 | no_use_found | src/app/browser_lifecycle.rs, src/app/diagnostics/shell_capture/movies/campaign_restart.rs, src/app/diagnostics/shell_capture/movies/lan/live/online.rs … |
| `Sidebar.MixFileIndex` | Side | Valueable<int> | 0 | src/Ext/Side/Body.cpp:131 | ok | src/rules/native_processing.rs |
| `Sidebar.YuriFileNames` | Side | Valueable<bool> | false | src/Ext/Side/Body.cpp:132 | ok | src/rules/native_processing.rs |

## 钩子（43）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 41E893 | `AITriggerTypeClass_ConditionMet_SideIndex` | kept | ✓ | 0041E720 `AITriggerTypeClass::AITriggerTypeClass__Is_Eligible` | decompiled/functions/1002F2D0.c | src/Ext/Side/Hooks.cpp:361 | src/sim/ai_team_creation.rs |
| 4F8C97 | `HouseClass_Update_BuildConst` | replaces_0.A:Sides_BuildConst | ✓ | 004F8440 `HouseClass::HouseClass__Update` | decompiled/functions/1002F5A0.c | src/Ext/Side/Hooks.cpp:33 | src/app/match_runtime/scenario_exit.rs, src/app/presentation/sidebar_gadgets.rs, src/sim/house_eva.rs … |
| 4F8F54 | `HouseClass_Update_SlaveMinerCheck` | replaces_0.A:Sides_SlaveMinerCheck | ✓ | 004F8440 `HouseClass::HouseClass__Update` | decompiled/functions/1002F610.c | src/Ext/Side/Hooks.cpp:59 | src/app/match_runtime/scenario_exit.rs, src/app/presentation/sidebar_gadgets.rs, src/sim/house_eva.rs … |
| 507BCA | `HouseClass_BaseDefenses` | replaces_0.A:Sides_BaseDefenses | ✓ | 00507F60 `HouseClass::HouseClass__AI_AntiInfantryDefenseCandidates` | decompiled/functions/1002F510.c | src/Ext/Side/Hooks.cpp:83 | src/sim/ai_base_defense.rs |
| 52267D | `InfantryClass_GetDisguise_Disguise` | kept | ✓ | 00522640 `InfantryClass::InfantryClass__GetDisguise` | decompiled/functions/1002F780.c | src/Ext/Side/Hooks.cpp:107 | src/app/input/tooltips.rs, src/sim/cloak_disguise.rs |
| 5227A3 | `Sides_Disguise` | kept | ✓ | 006F3F40 `TechnoClass::TechnoClass__Init_Managers` | decompiled/functions/1002F9C0.c | src/Ext/Side/Hooks.cpp:117 | src/sim/capture_manager.rs, src/sim/world/lifecycle.rs, src/sim/world/world_spawn.rs |
| 534FB1 | `Sides_MixFileIndex` | kept ⚠改过 | ✓ | 00534FA0 `InitSideMixFiles` | decompiled/functions/1002FA00.c | src/Ext/Side/Hooks.cpp:288 | src/app/presentation/sidebar_render.rs, src/render/mod.rs |
| 53534C | `Game_LoadUI_LoadSideData` | kept | ✓ | 00534FA0 `InitSideMixFiles` | decompiled/functions/1002F4A0.c | src/Ext/Side/Hooks.cpp:516 | src/app/presentation/sidebar_render.rs, src/render/mod.rs |
| 553820 | `LoadProgressMgr_Draw_SkipShadowOnNullString2` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F830.c | src/Ext/Side/Hooks.cpp:266 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553E54 | `LoadProgressMgr_Draw_SkipShadowOnNullString` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F810.c | src/Ext/Side/Hooks.cpp:255 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 55403D | `LoadProgressMgr_Draw_SkipShadowOnNullString3` | kept | ✓ | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F850.c | src/Ext/Side/Hooks.cpp:277 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 5C9B75 | `Global_DrawScoreScreen_ScoreTheme` | kept | ✓ | 005C9B10 `ScoreDialog__WndProc` | decompiled/functions/1002F4B0.c | src/Ext/Side/Hooks.cpp:549 | src/app/audio_runtime.rs, src/app/match_runtime/frame_pacer.rs, src/ui/score_shell.rs |
| 5CA110 | `Game_GetMultiplayerScoreScreenBar` | kept | ✓ | 005CA110 `ScoreDialog__BandImage` | decompiled/functions/1002F300.c | src/Ext/Side/Hooks.cpp:493 | src/rules/ares_side_extension.rs, src/rules/ruleset.rs, src/ui/score_shell.rs |
| 622223 | `sub_621E90_DialogBackground` | kept | ✓ | 00621E90 `WM_PAINT_Handler` | decompiled/functions/1002FB50.c | src/Ext/Side/Hooks.cpp:535 | src/app/frontend/campaign_score_shell_render.rs, src/app/frontend/score_shell_render.rs, src/app/frontend/skirmish_shell_render/connection.rs … |
| 642B36 | `ProgressScreenClass_GetLoadTextColor` | replaces_0.A:Sides_LoadTextColor1 | ✓ | 00642B30 `ProgressScreenClass::ProgressScreenClass__Get_Side_ColorScheme` | decompiled/functions/1002F870.c | src/Ext/Side/Hooks.cpp:142 | — |
| 642B91 | `ProgressScreenClass_GetSideColor` | replaces_0.A:Sides_LoadTextColor3 | ✓ | 00642B60 `FUN_00642b60` | decompiled/functions/1002F870.c | src/Ext/Side/Hooks.cpp:161 | — |
| 643BB9 | `ProgressScreenClass_UpdateSingleProgressBar` | replaces_0.A:Sides_LoadTextColor2 | ✓ | 00643AE0 `ProgressScreenClass::ProgressScreenClass__Redraw` | decompiled/functions/1002F890.c | src/Ext/Side/Hooks.cpp:151 | — |
| 679A10 | `SideClass_LoadAllFromINI` | kept | ✓ | 00679A10 `RulesClass::RulesClass__ReadTypeData` | decompiled/functions/1002EFC0.c | src/Ext/Side/Body.cpp:573 | src/rules/mod.rs, src/rules/native_processing.rs |
| 67D315 | `SaveGame_EarlySaveSides` | kept | ✓ | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1002F8B0.c | src/Ext/Side/Hooks.cpp:319 | — |
| 67E09A | `SaveGame_LateSkipSides` | kept | ✓ | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1002F8D0.c | src/Ext/Side/Hooks.cpp:328 | — |
| 67E74A | `LoadGame_EarlyLoadSides` | kept | ✓ | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1002F7A0.c | src/Ext/Side/Hooks.cpp:338 | — |
| 67F281 | `LoadGame_LateSkipSides` | kept | ✓ | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1002F800.c | src/Ext/Side/Hooks.cpp:356 | — |
| 683C70 | `sub_683AB0_LoadingScoreA` | kept | ✓ | 00683AB0 `ScenarioClass__Start_Scenario` | decompiled/functions/1002FB80.c | src/Ext/Side/Hooks.cpp:615 | src/app/campaign_victory.rs, src/app/loading/pump.rs, src/app/loading/transitions.rs … |
| 683D05 | `sub_683AB0_LoadingScoreB` | kept | ✓ | 00683AB0 `ScenarioClass__Start_Scenario` | decompiled/functions/1002FC20.c | src/Ext/Side/Hooks.cpp:645 | src/app/campaign_victory.rs, src/app/loading/pump.rs, src/app/loading/transitions.rs … |
| 6847B7 | `ScenarioClass_PrepareMapAndUDP` | replaces_0.A:Sides_LoadTextColor_CacheMP | ✓ | 00684620 `ScenarioClass__Read_Scenario` | decompiled/functions/1002F8F0.c | src/Ext/Side/Hooks.cpp:172 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/app/loading/pump.rs … |
| 686D7F | `INIClass_ReadScenario_CacheSP` | replaces_0.A:Sides_LoadTextColor_CacheSP | ✓ | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/1002F680.c | src/Ext/Side/Hooks.cpp:190 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |
| 6873AB | `INIClass_ReadScenario_EarlyLoadRules` | kept | ✓ | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/1002EF30.c | src/Ext/Side/Body.cpp:581 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |
| 6A4609 | `SideClass_CTOR` | kept | ✓ | 006A4550 `SideClass::SideClass__Constructor` | decompiled/functions/1002EF60.c | src/Ext/Side/Body.cpp:534 | src/rules/native_processing.rs |
| 6A4780 | `SideClass_SaveLoad_Prefix` | kept | ✓ | 006A48A0 `SideClass__Save` | decompiled/functions/1002F150.c | src/Ext/Side/Body.cpp:550 | — |
| 6A488B | `SideClass_Load_Suffix` | kept | ✓ | 006A4780 `SideClass__Load` | decompiled/functions/1002F010.c | src/Ext/Side/Body.cpp:561 | — |
| 6A48FC | `SideClass_Save_Suffix` | kept | ✓ | 006A48A0 `SideClass__Save` | decompiled/functions/1002F190.c | src/Ext/Side/Body.cpp:567 | — |
| 6A499F | `SideClass_SDDTOR` | kept | ✓ | 006A4930 `SideClass::SideClass__ScalarDeletingDestructor` | decompiled/functions/1002F110.c | src/Ext/Side/Body.cpp:542 | — |
| 6C922C | `ScoreDialog_Handle_ScoreThemeA` | kept | ✓ | 006C8FF0 `Dialog0x103__DlgProc` | decompiled/functions/1002F920.c | src/Ext/Side/Hooks.cpp:570 | src/ui/campaign_score_shell.rs |
| 6C935C | `ScoreDialog_Handle_ScoreThemeB` | kept | ✓ | 006C8FF0 `Dialog0x103__DlgProc` | decompiled/functions/1002F9A0.c | src/Ext/Side/Hooks.cpp:601 | src/ui/campaign_score_shell.rs |
| 6D4E79 | `TacticalClass_DrawOverlay_GraphicalText` | kept | ✓ | 006D4E20 `Tactical::FUN_006d4e20` | decompiled/functions/1002FAD0.c | src/Ext/Side/Hooks.cpp:523 | src/app/presentation/outcome_banner.rs, src/app/presentation/render/draw_passes.rs |
| 6DE0D3 | `TActionClass_Execute_MessageColor` | kept | ✓ | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/1002FA80.c | src/Ext/Side/Hooks.cpp:394 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 72D300 | `Game_LoadCampaignScoreAssets` | kept | ✓ | 0072D300 `FUN_0072d300` | decompiled/functions/1002F3A0.c | src/Ext/Side/Hooks.cpp:427 | src/app/cooperative_round.rs, src/app/shell_main_menu.rs, src/render/main_menu_shell_chrome.rs … |
| 72D730 | `Game_LoadMultiplayerScoreAssets` | kept | ✓ | 0072D730 `ScoreArt__Load` | decompiled/functions/1002F430.c | src/Ext/Side/Hooks.cpp:464 | src/app/frontend/state.rs, src/app/shell_main_menu.rs, src/render/main_menu_shell_chrome.rs … |
| 72F370 | `Sides_MixFileYuriFiles2` | kept | ✓ | 0072F350 `PaletteLoad` | decompiled/functions/1002FA60.c | src/Ext/Side/Hooks.cpp:312 | src/assets/asset_manager.rs |
| 72F440 | `Game_InitializeToolTipColor` | kept | ✓ | 0072F440 `SetSidebarTextColor` | decompiled/functions/1002F360.c | src/Ext/Side/Hooks.cpp:409 | src/render/sidebar_text.rs |
| 72FA1A | `Sides_MixFileYuriFiles1` | kept | ✓ | 0072FA10 `MIX_LoadNeutral` | decompiled/functions/1002FA50.c | src/Ext/Side/Hooks.cpp:309 | src/render/sidebar_chrome_in_game_shell.rs |
| 72FBC0 | `Sides_MixFileYuriFiles3` | kept | ✓ | 0072FBC0 `LeftPanel__LoadPalettes` | decompiled/functions/1002FA70.c | src/Ext/Side/Hooks.cpp:315 | — |
| 7534E0 | `VoxClass_SetEVAIndex` | kept | ✓ | 007534E0 `VoxClass__SetSide` | decompiled/functions/1002FB00.c | src/Ext/Side/Hooks.cpp:378 | src/rules/sound_ini.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002EAA0 | SideExt::MixFileYuriFiles [source_0.A,3.0✓] | 使用键 | decompiled/functions/1002EAA0.c |
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1002EC80 | `RulesClass::Read_Sides` | 被钩子调用 | decompiled/functions/1002EC80.c |
| 10057CF0 | `ContainerMapBase::insert` | 被钩子调用 | decompiled/functions/10057CF0.c |
| 1006B770 | `AresStreamReader::RegisterChange` | 被钩子调用 | decompiled/functions/1006B770.c |
| 100617C0 | `Debug::FatalErrorAndExit` | 被钩子调用 | decompiled/functions/100617C0.c |
| 10057D10 | `ContainerMapBase::remove` | 被钩子调用 | decompiled/functions/10057D10.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1002DF20 | `SideExt::ExtData::GetMultiplayerScoreBarFilename` | 被钩子调用 | decompiled/functions/1002DF20.c |
| 1002E110 | `PCX::GetSurface` | 被钩子调用 | decompiled/functions/1002E110.c |
| 1002F270 | `FileSystem::LoadWholeFileEx` | 被钩子调用 | decompiled/functions/1002F270.c |
| 1001DCB0 | `ConvertClass::CreateFromFile` | 被钩子调用 | decompiled/functions/1001DCB0.c |
| 1002ECC0 | `SideExt::UpdateGlobalFiles` | 被钩子调用 | decompiled/functions/1002ECC0.c |
| 1002DDD0 | `SideExt::ExtData::GetBaseDefenses` | 被钩子调用 | decompiled/functions/1002DDD0.c |
| 10022590 | `HouseExt::ExtData::SetFirestormState` | 被钩子调用 | decompiled/functions/10022590.c |
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 10021270 | `HouseExt::ExtData::GetDisguise` | 被钩子调用 | decompiled/functions/10021270.c |
| 1002EA80 | `SideExt::GetLoadTextColor` | 被钩子调用 | decompiled/functions/1002EA80.c |
| 1002F2B0 | `Game::Save_Sides` | 被钩子调用 | decompiled/functions/1002F2B0.c |
| 1002DF00 | `SideExt::GetGraphicalTextConvert` | 被钩子调用 | decompiled/functions/1002DF00.c |
| 1002DF10 | `SideExt::GetGraphicalTextImage` | 被钩子调用 | decompiled/functions/1002DF10.c |
| 1002F290 | `INIClass::INIClass__ReadTheme` | 被钩子调用 | decompiled/functions/1002F290.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |

## 涉及的热点函数（8）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004F8440 `HouseClass::HouseClass__Update` | 7 | H.sidepresent, S.core, ext:House |
| 00679A10 `RulesClass::RulesClass__ReadTypeData` | 4 | H.sidepresent, ext:Rules, ext:TechnoType |
| 00686B20 `CCINIClass::InitializeScenarioFromINI` | 4 | H.sidepresent, ext:Debug, ext:Invalidators |
| 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | 7 | H.country, H.sidepresent |
| 0067D300 `Save_Game_Content_To_Stream` | 4 | F.save, H.sidepresent |
| 0067E730 `Load_Game_Content_From_Stream` | 4 | F.save, H.sidepresent |
| 006F3F40 `TechnoClass::TechnoClass__Init_Managers` | 3 | H.sidepresent, ext:Techno |
| 006DD8B0 `TActionClass::TActionClass__Execute` | 2 | H.sidepresent, U.triggers |
