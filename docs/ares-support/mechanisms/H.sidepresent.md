# `H.sidepresent` Side 展示：计分、对话框背景、图形文字、侧栏文件、颜色、载入主题

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | H.side | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（22）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CampaignScore.Animation` | Side | — | — | src/Ext/Side/Body.cpp:137 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Background` | Side | — | — | src/Ext/Side/Body.cpp:135 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.OverParTheme` | Side | — | — | src/Ext/Side/Body.cpp:151 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Palette` | Side | — | — | src/Ext/Side/Body.cpp:138 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.Transition` | Side | — | — | src/Ext/Side/Body.cpp:136 | unscanned | src/rules/native_processing.rs |
| `CampaignScore.UnderParTheme` | Side | — | — | src/Ext/Side/Body.cpp:150 | unscanned | src/rules/native_processing.rs |
| `DialogBackground.Image` | Side | — | — | src/Ext/Side/Body.cpp:158 | unscanned | src/rules/native_processing.rs |
| `DialogBackground.Palette` | Side | — | — | src/Ext/Side/Body.cpp:159 | unscanned | src/rules/native_processing.rs |
| `GraphicalText.Image` | Side | — | — | src/Ext/Side/Body.cpp:155 | unscanned | src/rules/native_processing.rs |
| `GraphicalText.Palette` | Side | — | — | src/Ext/Side/Body.cpp:156 | unscanned | src/rules/native_processing.rs |
| `LoadingTheme` | Side | — | — | src/Ext/Side/Hooks.cpp:450 | unscanned | src/app/loading/pump.rs, src/rules/process_owner.rs |
| `MultiplayerScore.Background` | Side | — | — | src/Ext/Side/Body.cpp:139 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.Bars` | Side | — | — | src/Ext/Side/Body.cpp:141 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.LoseTheme` | Side | — | — | src/Ext/Side/Body.cpp:153 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.Palette` | Side | — | — | src/Ext/Side/Body.cpp:140 | unscanned | src/rules/native_processing.rs |
| `MultiplayerScore.WinTheme` | Side | — | — | src/Ext/Side/Body.cpp:152 | unscanned | src/rules/native_processing.rs |
| `SCORE` | Side | AresCRT::strCopy | — | 1002D9F0 | no_use_found | src/app/browser_lifecycle.rs, src/app/diagnostics/shell_capture/movies/campaign_restart.rs, src/app/diagnostics/shell_capture/movies/lan/live/online.rs … |
| `SCORE` | Side | AresCRT::strCopy | — | 1002D9F0 | no_use_found | src/app/browser_lifecycle.rs, src/app/diagnostics/shell_capture/movies/campaign_restart.rs, src/app/diagnostics/shell_capture/movies/lan/live/online.rs … |
| `SCORE` | Side | AresCRT::strCopy | — | 1002D9F0 | no_use_found | src/app/browser_lifecycle.rs, src/app/diagnostics/shell_capture/movies/campaign_restart.rs, src/app/diagnostics/shell_capture/movies/lan/live/online.rs … |
| `SCORE` | Side | AresCRT::strCopy | — | 1002D9F0 | no_use_found | src/app/browser_lifecycle.rs, src/app/diagnostics/shell_capture/movies/campaign_restart.rs, src/app/diagnostics/shell_capture/movies/lan/live/online.rs … |
| `Sidebar.MixFileIndex` | Side | Valueable<int> | 0 | src/Ext/Side/Body.cpp:126 | check_0A_use | src/rules/native_processing.rs |
| `Sidebar.YuriFileNames` | Side | Valueable<bool> | false | src/Ext/Side/Body.cpp:127 | check_0A_use | src/rules/native_processing.rs |

## 钩子（34）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 41E893 | `AITriggerTypeClass_ConditionMet_SideIndex` | kept | 0041E720 `AITriggerTypeClass::AITriggerTypeClass__Is_Eligible` | decompiled/functions/1002F2D0.c | src/Ext/Side/Hooks.cpp:187 | src/sim/ai_team_creation.rs |
| 52267D | `InfantryClass_GetDisguise_Disguise` | kept | 00522640 `InfantryClass::InfantryClass__GetDisguise` | decompiled/functions/1002F780.c | src/Ext/Side/Hooks.cpp:14 | src/app/input/tooltips.rs, src/sim/cloak_disguise.rs |
| 5227A3 | `Sides_Disguise` | kept | 006F3F40 `TechnoClass::TechnoClass__Init_Managers` | decompiled/functions/1002F9C0.c | src/Ext/Side/Hooks.cpp:26 | src/sim/capture_manager.rs, src/sim/world/lifecycle.rs, src/sim/world/world_spawn.rs |
| 534FB1 | `Sides_MixFileIndex` | kept ⚠改过 | 00534FA0 `InitSideMixFiles` | decompiled/functions/1002FA00.c | src/Ext/Side/Hooks.cpp:119 | src/app/presentation/sidebar_render.rs, src/render/mod.rs |
| 53534C | `Game_LoadUI_LoadSideData` | kept | 00534FA0 `InitSideMixFiles` | decompiled/functions/1002F4A0.c | src/Ext/Side/Hooks.cpp:334 | src/app/presentation/sidebar_render.rs, src/render/mod.rs |
| 553820 | `LoadProgressMgr_Draw_SkipShadowOnNullString2` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F830.c | src/Ext/Side/Hooks.cpp:97 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 553E54 | `LoadProgressMgr_Draw_SkipShadowOnNullString` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F810.c | src/Ext/Side/Hooks.cpp:86 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 55403D | `LoadProgressMgr_Draw_SkipShadowOnNullString3` | kept | 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | decompiled/functions/1002F850.c | src/Ext/Side/Hooks.cpp:108 | src/app/loading/campaign_presentation.rs, src/app/loading/composition.rs, src/render/loading_screen_chrome.rs |
| 5C9B75 | `Global_DrawScoreScreen_ScoreTheme` | kept | 005C9B10 `ScoreDialog__WndProc` | decompiled/functions/1002F4B0.c | src/Ext/Side/Hooks.cpp:367 | src/app/audio_runtime.rs, src/app/match_runtime/frame_pacer.rs, src/ui/score_shell.rs |
| 5CA110 | `Game_GetMultiplayerScoreScreenBar` | kept | 005CA110 `ScoreDialog__BandImage` | decompiled/functions/1002F300.c | src/Ext/Side/Hooks.cpp:317 | src/rules/ares_side_extension.rs, src/rules/ruleset.rs, src/ui/score_shell.rs |
| 622223 | `sub_621E90_DialogBackground` | kept | 00621E90 `WM_PAINT_Handler` | decompiled/functions/1002FB50.c | src/Ext/Side/Hooks.cpp:353 | src/app/frontend/campaign_score_shell_render.rs, src/app/frontend/score_shell_render.rs, src/app/frontend/skirmish_shell_render/connection.rs … |
| 679A10 | `SideClass_LoadAllFromINI` | kept | 00679A10 `RulesClass::RulesClass__ReadTypeData` | decompiled/functions/1002EFC0.c | src/Ext/Side/Body.cpp:561 | src/rules/mod.rs, src/rules/native_processing.rs |
| 67D315 | `SaveGame_EarlySaveSides` | kept | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1002F8B0.c | src/Ext/Side/Hooks.cpp:145 | — |
| 67E09A | `SaveGame_LateSkipSides` | kept | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1002F8D0.c | src/Ext/Side/Hooks.cpp:154 | — |
| 67E74A | `LoadGame_EarlyLoadSides` | kept | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1002F7A0.c | src/Ext/Side/Hooks.cpp:164 | — |
| 67F281 | `LoadGame_LateSkipSides` | kept | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1002F800.c | src/Ext/Side/Hooks.cpp:182 | — |
| 683C70 | `sub_683AB0_LoadingScoreA` | kept | 00683AB0 `ScenarioClass__Start_Scenario` | decompiled/functions/1002FB80.c | src/Ext/Side/Hooks.cpp:431 | src/app/campaign_victory.rs, src/app/loading/pump.rs, src/app/loading/transitions.rs … |
| 683D05 | `sub_683AB0_LoadingScoreB` | kept | 00683AB0 `ScenarioClass__Start_Scenario` | decompiled/functions/1002FC20.c | src/Ext/Side/Hooks.cpp:461 | src/app/campaign_victory.rs, src/app/loading/pump.rs, src/app/loading/transitions.rs … |
| 6A4609 | `SideClass_CTOR` | kept | 006A4550 `SideClass::SideClass__Constructor` | decompiled/functions/1002EF60.c | src/Ext/Side/Body.cpp:522 | src/rules/native_processing.rs |
| 6A4780 | `SideClass_SaveLoad_Prefix` | kept | 006A48A0 `SideClass__Save` | decompiled/functions/1002F150.c | src/Ext/Side/Body.cpp:538 | — |
| 6A488B | `SideClass_Load_Suffix` | kept | 006A4780 `SideClass__Load` | decompiled/functions/1002F010.c | src/Ext/Side/Body.cpp:549 | — |
| 6A48FC | `SideClass_Save_Suffix` | kept | 006A48A0 `SideClass__Save` | decompiled/functions/1002F190.c | src/Ext/Side/Body.cpp:555 | — |
| 6A499F | `SideClass_SDDTOR` | kept | 006A4930 `SideClass::SideClass__ScalarDeletingDestructor` | decompiled/functions/1002F110.c | src/Ext/Side/Body.cpp:530 | — |
| 6C922C | `ScoreDialog_Handle_ScoreThemeA` | kept | 006C8FF0 `Dialog0x103__DlgProc` | decompiled/functions/1002F920.c | src/Ext/Side/Hooks.cpp:387 | src/ui/campaign_score_shell.rs |
| 6C935C | `ScoreDialog_Handle_ScoreThemeB` | kept | 006C8FF0 `Dialog0x103__DlgProc` | decompiled/functions/1002F9A0.c | src/Ext/Side/Hooks.cpp:417 | src/ui/campaign_score_shell.rs |
| 6D4E79 | `TacticalClass_DrawOverlay_GraphicalText` | kept | 006D4E20 `Tactical::FUN_006d4e20` | decompiled/functions/1002FAD0.c | src/Ext/Side/Hooks.cpp:341 | src/app/presentation/outcome_banner.rs, src/app/presentation/render/draw_passes.rs |
| 6DE0D3 | `TActionClass_Execute_MessageColor` | kept | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/1002FA80.c | src/Ext/Side/Hooks.cpp:220 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 72D300 | `Game_LoadCampaignScoreAssets` | kept | 0072D300 `FUN_0072d300` | decompiled/functions/1002F3A0.c | src/Ext/Side/Hooks.cpp:253 | src/app/cooperative_round.rs, src/app/shell_main_menu.rs, src/render/main_menu_shell_chrome.rs … |
| 72D730 | `Game_LoadMultiplayerScoreAssets` | kept | 0072D730 `ScoreArt__Load` | decompiled/functions/1002F430.c | src/Ext/Side/Hooks.cpp:289 | src/app/frontend/state.rs, src/app/shell_main_menu.rs, src/render/main_menu_shell_chrome.rs … |
| 72F370 | `Sides_MixFileYuriFiles2` | kept | 0072F350 `PaletteLoad` | decompiled/functions/1002FA60.c | src/Ext/Side/Hooks.cpp:138 | src/assets/asset_manager.rs |
| 72F440 | `Game_InitializeToolTipColor` | kept | 0072F440 `SetSidebarTextColor` | decompiled/functions/1002F360.c | src/Ext/Side/Hooks.cpp:235 | src/render/sidebar_text.rs |
| 72FA1A | `Sides_MixFileYuriFiles1` | kept | 0072FA10 `MIX_LoadNeutral` | decompiled/functions/1002FA50.c | src/Ext/Side/Hooks.cpp:135 | src/render/sidebar_chrome_in_game_shell.rs |
| 72FBC0 | `Sides_MixFileYuriFiles3` | kept | 0072FBC0 `LeftPanel__LoadPalettes` | decompiled/functions/1002FA70.c | src/Ext/Side/Hooks.cpp:141 | — |
| 7534E0 | `VoxClass_SetEVAIndex` | kept | 007534E0 `VoxClass__SetSide` | decompiled/functions/1002FB00.c | src/Ext/Side/Hooks.cpp:204 | src/rules/sound_ini.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002EAA0 | SideExt::MixFileYuriFiles [source_0.A] | 使用键 | decompiled/functions/1002EAA0.c |
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1002DF20 | `SideExt::ExtData::GetMultiplayerScoreBarFilename` | 被钩子调用 | decompiled/functions/1002DF20.c |
| 1002E110 | `PCX::GetSurface` | 被钩子调用 | decompiled/functions/1002E110.c |
| 1002F270 | `FileSystem::LoadWholeFileEx` | 被钩子调用 | decompiled/functions/1002F270.c |
| 1001DCB0 | `ConvertClass::CreateFromFile` | 被钩子调用 | decompiled/functions/1001DCB0.c |
| 1002ECC0 | `SideExt::UpdateGlobalFiles` | 被钩子调用 | decompiled/functions/1002ECC0.c |
| 10021270 | `HouseExt::ExtData::GetDisguise` | 被钩子调用 | decompiled/functions/10021270.c |
| 1002F2B0 | `Game::Save_Sides` | 被钩子调用 | decompiled/functions/1002F2B0.c |
| 1002DF00 | `SideExt::GetGraphicalTextConvert` | 被钩子调用 | decompiled/functions/1002DF00.c |
| 1002DF10 | `SideExt::GetGraphicalTextImage` | 被钩子调用 | decompiled/functions/1002DF10.c |
| 1002F290 | `INIClass::INIClass__ReadTheme` | 被钩子调用 | decompiled/functions/1002F290.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 10051730 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10051730.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |

## 涉及的热点函数（6）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00679A10 `RulesClass::RulesClass__ReadTypeData` | 4 | H.sidepresent, ext:Rules, ext:TechnoType |
| 00552D60 `LoadProgressMgr::LoadProgressMgr__DrawLoadingScreen` | 7 | H.country, H.sidepresent |
| 0067D300 `Save_Game_Content_To_Stream` | 4 | F.save, H.sidepresent |
| 0067E730 `Load_Game_Content_From_Stream` | 4 | F.save, H.sidepresent |
| 006F3F40 `TechnoClass::TechnoClass__Init_Managers` | 3 | H.sidepresent, ext:Techno |
| 006DD8B0 `TActionClass::TActionClass__Execute` | 2 | H.sidepresent, U.triggers |
