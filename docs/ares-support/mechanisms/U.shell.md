# `U.shell` 主菜单/战役/影片列表/RMG/遭遇战设置

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M7 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（44）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Campaign1` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:136 | unscanned | — |
| `Campaign2` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:137 | unscanned | — |
| `Campaign3` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:138 | unscanned | — |
| `Campaign4` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:139 | unscanned | — |
| `CampaignButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:84 | unscanned | — |
| `CampaignList` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:92 | unscanned | — |
| `CampaignListSize` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:95 | unscanned | — |
| `Count` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:171 | unscanned | src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/lan/live.rs, src/app/diagnostics/tactical_capture/map_observation.rs … |
| `MoviesAndCreditsButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:83 | unscanned | — |
| `NetworkButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:82 | unscanned | — |
| `Observer` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:175 | unscanned | src/app/frontend/skirmish.rs, src/app/shell_wol/live_service.rs, src/render/draw_state.rs … |
| `PlayMoviesButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:87 | unscanned | — |
| `QuickExit` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:90 | unscanned | — |
| `ShowDebugCampaigns` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:93 | unscanned | — |
| `ShowSummary` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:94 | unscanned | — |
| `SinglePlayerButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:80 | unscanned | — |
| `SkirmishButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:85 | unscanned | — |
| `Slot1` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:176 | unscanned | src/sim/combat/combat_weapon.rs, src/sim/combat/greatest_threat.rs, src/sim/combat/threat_range.rs |
| `Slot10` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:187 | unscanned | — |
| `Slot11` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:188 | unscanned | — |
| `Slot12` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:189 | unscanned | — |
| `Slot13` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:190 | unscanned | — |
| `Slot14` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:191 | unscanned | — |
| `Slot2` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:177 | unscanned | src/sim/combat/greatest_threat.rs |
| `Slot3` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:178 | unscanned | — |
| `Slot4` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:179 | unscanned | — |
| `Slot5` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:180 | unscanned | — |
| `Slot6` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:181 | unscanned | — |
| `Slot7` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:182 | unscanned | — |
| `Slot8` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:183 | unscanned | — |
| `Slot9` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:186 | unscanned | — |
| `SneakPeeksButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:86 | unscanned | — |
| `VersionInfo` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:229 | unscanned | — |
| `ViewCreditsButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:88 | unscanned | — |
| `WWOnlineButton` | Ares.UISettings | — | — | src/Ares.UISettings.cpp:81 | unscanned | — |
| `DebugOnly` | Campaign | — | — | src/Ext/Campaign/Body.cpp:36 | unscanned | src/app/spawn_probe.rs |
| `HoverSound` | Campaign | — | — | src/Ext/Campaign/Body.cpp:38 | unscanned | — |
| `Summary` | Campaign | — | — | src/Ext/Campaign/Body.cpp:40 | unscanned | — |
| `Description` | MoviesList | — | — | src/Misc/MoviesList.cpp:65 | unscanned | src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/lan.rs, src/app/frontend/skirmish_shell_render/keyboard.rs … |
| `DiskRequired` | MoviesList | — | — | src/Misc/MoviesList.cpp:69 | unscanned | — |
| `Movies` | MoviesList | — | — | src/Misc/MoviesList.cpp:46 | unscanned | src/assets/asset_manager.rs, src/rules/movies.rs |
| `Unlockable` | MoviesList | — | — | src/Misc/MoviesList.cpp:70 | unscanned | — |
| `UnlockedMovies` | MoviesList | — | — | src/Misc/MoviesList.cpp:83 | unscanned | — |
| `Urban` | RMG | — | — | src/Misc/RMG.cpp:124 | unscanned | src/asset_tools/names.rs, src/asset_tools/palette.rs, src/asset_tools/verb_art.rs … |

## 钩子（104）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 46CC03 | `CampaignClass_CTOR` | new_in_3.0 | 0046CE10 `CampaignClass__CreateFromINIList` | decompiled/functions/1001AFB0.c | src/Recovered/CampaignClass.cpp:17 | src/rules/campaigns.rs |
| 46CC36 | `CampaignClass_DTOR` | removed_in_3.0 | 0046D090 `CampaignClass::CampaignClass__ScalarDeletingDestructor` | decompiled/functions/1001B020.c | src/Ext/Campaign/Body.cpp | — |
| 46CD56 | `CampaignClass_LoadFromINI` | kept | 0046CCD0 `CampaignClass::CampaignClass__ReadINI` | decompiled/functions/1001B050.c | src/Ext/Campaign/Body.cpp:63 | src/rules/campaigns.rs |
| 4A3B4B | `FetchResource` | kept | 004A3B40 `Game__GetResource` | decompiled/functions/10075E70.c | src/UI/Dialogs.cpp:29 | — |
| 4E42A0 | `GameSetup_GetColorTooltip` | kept | 004E42A0 `FUN_004e42a0` | decompiled/functions/10067000.c | src/Misc/Interface.PlayerColors.cpp:52 | — |
| 4E43C0 | `Game_InitDropdownColors` | kept | 004E43C0 `FUN_004e43c0` | decompiled/functions/10067060.c | src/Misc/Interface.PlayerColors.cpp:9 | src/app/shell_wol/my_info.rs |
| 4E4556 | `hWnd_GetSlotColorIndex` | kept | 004E44E0 `FUN_004e44e0` | decompiled/functions/10067200.c | src/Misc/Interface.PlayerColors.cpp:122 | — |
| 4E4580 | `hWnd_IsAvailableColor` | kept | 004E4580 `FUN_004e4580` | decompiled/functions/10067250.c | src/Misc/Interface.PlayerColors.cpp:139 | — |
| 4E46BB | `hWnd_PopulateWithColors` | kept | 004E45A0 `FUN_004e45a0` | decompiled/functions/10067270.c | src/Misc/Interface.PlayerColors.cpp:70 | src/app/shell_wol/my_info.rs |
| 4E4A41 | `hWnd_SetPlayerColor_A` | kept | 004E49A0 `FUN_004e49a0` | decompiled/functions/10067340.c | src/Misc/Interface.PlayerColors.cpp:98 | — |
| 4E4B47 | `hWnd_SetPlayerColor_B` | kept | 004E49A0 `FUN_004e49a0` | decompiled/functions/10067380.c | src/Misc/Interface.PlayerColors.cpp:113 | — |
| 4E4C9D | `hWnd_UpdatePlayerColors_A` | kept | 004E4C20 `FUN_004e4c20` | decompiled/functions/100673A0.c | src/Misc/Interface.PlayerColors.cpp:146 | — |
| 4E4D67 | `hWnd_UpdatePlayerColors_B` | kept | 004E4C20 `FUN_004e4c20` | decompiled/functions/100673E0.c | src/Misc/Interface.PlayerColors.cpp:161 | — |
| 52C939 | `InitGame_MoviesList` | kept | 0052BA60 `Init_Game` | decompiled/functions/1006A0B0.c | src/Misc/MoviesList.cpp:123 | src/app/frontend/startup_splash.rs, src/app/loading/init_helpers.rs, src/app/mod.rs … |
| 52CA37 | `InitGame_Delay` | kept | 0052BA60 `Init_Game` | decompiled/functions/1006B310.c | src/Misc/Status.cpp:84 | src/app/frontend/startup_splash.rs, src/app/loading/init_helpers.rs, src/app/mod.rs … |
| 52D713 | `SinglePlayer_hDlg_SkirmishButtonClick` | kept | 0052D640 `SinglePlayerDialog0x100__WndProc` | decompiled/functions/10065DB0.c | src/Misc/Interface.Buttons.cpp:238 | src/ui/single_player_shell/layout.rs |
| 52D724 | `SinglePlayer_hDlg_CampaignButtonClick` | kept | 0052D640 `SinglePlayerDialog0x100__WndProc` | decompiled/functions/10065D80.c | src/Misc/Interface.Buttons.cpp:232 | src/ui/single_player_shell/layout.rs |
| 52D7ED | `MoviesAndCredits_hDlg_SneakPeeksButtonClick` | kept | 0052D790 `MoviesCreditsDialog0x101__WndProc` | decompiled/functions/10065D20.c | src/Misc/Interface.Buttons.cpp:244 | src/ui/movies_credits_shell/mod.rs, src/ui/movies_credits_shell/page.rs |
| 52D7FB | `MoviesAndCredits_hDlg_PlayMoviesButtonClick` | kept | 0052D790 `MoviesCreditsDialog0x101__WndProc` | decompiled/functions/10065CF0.c | src/Misc/Interface.Buttons.cpp:250 | src/ui/movies_credits_shell/mod.rs, src/ui/movies_credits_shell/page.rs |
| 52D809 | `MoviesAndCredits_hDlg_ViewCreditsButtonClick` | kept | 0052D790 `MoviesCreditsDialog0x101__WndProc` | decompiled/functions/10065D50.c | src/Misc/Interface.Buttons.cpp:256 | src/ui/movies_credits_shell/mod.rs, src/ui/movies_credits_shell/page.rs |
| 52DDBA | `Frontend_WndProc_MessageBox` | kept ⚠改过 | 0052D9A0 `Main__PrepareSession` | decompiled/functions/10065BD0.c | src/Misc/Interface.Buttons.cpp:184 | src/app/frontend/quit_cascade.rs, src/app/shell_main_menu.rs, src/app/shell_movies_credits.rs … |
| 52E446 | `Frontend_WndProc_JustAfterAction` | kept | 0052D9A0 `Main__PrepareSession` | decompiled/functions/10065BA0.c | src/Misc/Interface.Buttons.cpp:197 | src/app/frontend/quit_cascade.rs, src/app/shell_main_menu.rs, src/app/shell_movies_credits.rs … |
| 52EC18 | `CampaignMenu_hDlg_PreHandleGeneral` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066CF0.c | src/Misc/Interface.cpp:365 | src/ui/campaign_shell/mod.rs |
| 52ED21 | `CampaignMenu_hDlg_ClickedPlay` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066BA0.c | src/Misc/Interface.cpp:411 | src/ui/campaign_shell/mod.rs |
| 52EE04 | `CampaignMenu_hDlg_SelectHoverSound` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066DE0.c | src/Misc/Interface.cpp:448 | src/ui/campaign_shell/mod.rs |
| 52EF39 | `CampaignMenu_hDlg_ImageMouseDown` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066BE0.c | src/Misc/Interface.cpp:432 | src/ui/campaign_shell/mod.rs |
| 52F00B | `CampaignMenu_hDlg_PopulateCampaignList` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066C40.c | src/Misc/Interface.cpp:337 | src/ui/campaign_shell/mod.rs |
| 52F191 | `CampaignMenu_InitializeMoreButtons` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/100669E0.c | src/Misc/Interface.cpp:556 | src/ui/campaign_shell/mod.rs |
| 52F232 | `CampaignMenu_hDlg_StartCampaign` | kept | 0052EC00 `CampaignDialog0x94__WndProc` | decompiled/functions/10066EA0.c | src/Misc/Interface.cpp:489 | src/ui/campaign_shell/mod.rs |
| 531413 | `Game_Start` | kept | 005312A0 `Init_DropPodAssets` | decompiled/functions/1006B140.c | src/Misc/Status.cpp:25 | src/app/frontend/startup_splash.rs, src/assets/pal_file.rs |
| 532017 | `DlgProc_MainMenu_Version` | kept | 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | decompiled/functions/1006B0D0.c | src/Misc/Status.cpp:55 | — |
| 532051 | `Main_hDlg_NetworkButtonClick` | kept | 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | decompiled/functions/10065C60.c | src/Misc/Interface.Buttons.cpp:220 | — |
| 53208D | `Main_hDlg_SinglePlayerButtonClick` | kept | 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | decompiled/functions/10065C90.c | src/Misc/Interface.Buttons.cpp:208 | — |
| 5320AE | `Main_hDlg_MoviesAndCreditsButtonClick` | kept | 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | decompiled/functions/10065C30.c | src/Misc/Interface.Buttons.cpp:226 | — |
| 5320C2 | `Main_hDlg_WWOnlineButtonClick` | kept | 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | decompiled/functions/10065CC0.c | src/Misc/Interface.Buttons.cpp:214 | — |
| 545904 | `IsometricTileTypeClass_CreateFromINIList_MediansFix` | kept | 00545150 `Read_Theater_TileSets_INI` | decompiled/functions/1006A920.c | src/Misc/RMG.cpp:267 | src/map/bridge_rim_tiles.rs, src/map/resolved_terrain/recalc_catalog.rs, src/map/theater.rs … |
| 58FA51 | `RMG_PlaceWEBridge` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006ADB0.c | src/Misc/RMG.cpp:247 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 58FBDD | `RMG_FixPavedRoadEnd_Bridges_East` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006AD50.c | src/Misc/RMG.cpp:242 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 58FCC6 | `RMG_FixPavedRoadEnd_Bridges_West` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006AD80.c | src/Misc/RMG.cpp:237 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 58FE7B | `RMG_PlaceNSBridge` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006AD90.c | src/Misc/RMG.cpp:257 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 59000E | `RMG_FixPavedRoadEnd_Bridges_North` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006AD60.c | src/Misc/RMG.cpp:227 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 5900F7 | `RMG_FixPavedRoadEnd_Bridges_South` | kept | 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | decompiled/functions/1006AD70.c | src/Misc/RMG.cpp:232 | src/map/rmg/phases/bridge_deck.rs, src/sim/overlay_grid.rs |
| 596786 | `MapSeedClass_DialogFunc_SurpriseMe` | kept | 00596300 `RandomMapSetupDialog__Proc` | decompiled/functions/1006A9C0.c | src/Misc/RMG.cpp:53 | — |
| 596C81 | `MapSeedClass_DialogFunc_GetData` | kept | 00596C70 `RandomMapSetupDialog__SyncOptionsFromControls` | decompiled/functions/1006A940.c | src/Misc/RMG.cpp:86 | — |
| 596FFE | `RMG_EnableArchipelago` | kept | 00596E50 `RandomMapSetupDialog__SyncControlsFromOptions` | decompiled/functions/1006ACF0.c | src/Misc/RMG.cpp:21 | src/map/rmg/saved_seeds.rs |
| 5970EA | `RMG_EnableDesert` | kept | 00596E50 `RandomMapSetupDialog__SyncControlsFromOptions` | decompiled/functions/1006AD10.c | src/Misc/RMG.cpp:30 | src/map/rmg/saved_seeds.rs |
| 5971EA | `MapSeedClass_DialogFunc_SetData` | kept | 00596E50 `RandomMapSetupDialog__SyncControlsFromOptions` | decompiled/functions/1006A980.c | src/Misc/RMG.cpp:96 | src/map/rmg/saved_seeds.rs |
| 5982D5 | `MapSeedClass_LoadFromINI` | kept | 005981F0 `MapSeedClass__ReadINI` | decompiled/functions/1006ABF0.c | src/Misc/RMG.cpp:116 | — |
| 5A5C6A | `MapSeedClass_Generate_PlacePavedRoads_RoadEndNE` | kept | 005A5B50 `FUN_005a5b50` | decompiled/functions/1006AA90.c | src/Misc/RMG.cpp:205 | — |
| 5A5D6F | `MapSeedClass_Generate_PlacePavedRoads_RoadEndSW` | kept | 005A5B50 `FUN_005a5b50` | decompiled/functions/1006AAC0.c | src/Misc/RMG.cpp:210 | — |
| 5A5F6A | `MapSeedClass_Generate_PlacePavedRoads_RoadEndNW` | kept | 005A5B50 `FUN_005a5b50` | decompiled/functions/1006AAA0.c | src/Misc/RMG.cpp:215 | — |
| 5A6464 | `MapSeedClass_Generate_PlacePavedRoads_RoadEndSE` | kept | 005A5B50 `FUN_005a5b50` | decompiled/functions/1006AAB0.c | src/Misc/RMG.cpp:220 | — |
| 5A65CA | `MapSeedClass_Generate_PlaceUrbanStructures_Start` | kept | 005A6510 `FUN_005a6510` | decompiled/functions/1006ABA0.c | src/Misc/RMG.cpp:145 | — |
| 5A6619 | `MapSeedClass_Generate_PlaceUrbanStructures_Loop` | kept | 005A6510 `FUN_005a6510` | decompiled/functions/1006AB70.c | src/Misc/RMG.cpp:155 | — |
| 5A66B0 | `MapSeedClass_Generate_PlaceUrbanStructures_SanityCheck` | kept | 005A6510 `FUN_005a6510` | decompiled/functions/1006AB90.c | src/Misc/RMG.cpp:164 | — |
| 5A6998 | `MapSeedClass_Generate_PlaceUrbanFoots` | kept | 005A6920 `FUN_005a6920` | decompiled/functions/1006AAD0.c | src/Misc/RMG.cpp:173 | — |
| 5FAFFB | `Options_SaveToINI_MoviesList` | kept | 005FAD10 `OptionsClass__WriteToINI` | decompiled/functions/1006A0C0.c | src/Misc/MoviesList.cpp:136 | src/app/in_game.rs, src/app/persistence/options.rs, src/app/persistence/options_profile.rs … |
| 5FBF80 | `GameOptionsClass_UnlockMovieIfNeeded_MoviesList` | kept | 005FBF80 `OptionsClass__UnlockCampaignMovie` | decompiled/functions/1006A090.c | src/Misc/MoviesList.cpp:129 | src/app/frontend/fullscreen_movie.rs, src/app/persistence/options_profile.rs, src/ui/movies_credits_shell/movie_list.rs |
| 5FC000 | `GameOptionsClass_PopulateMovieList` | kept ⚠改过 | 005FC000 `OptionsClass__FillMovieList` | decompiled/functions/1006A000.c | src/Misc/MoviesList.cpp:142 | src/app/shell_movies_credits.rs, src/ui/movies_credits_shell/movie_list.rs |
| 5FED00 | `OverlayTypeClass_GetRadarColor` | kept | 005FED00 `OverlayTypeClass::OverlayTypeClass__GetRadarColor` | decompiled/functions/1006ACC0.c | src/Misc/RMG.cpp:107 | src/render/overlay_atlas.rs |
| 60357E | `CampaignMenu_SetAnimationDuration` | kept ⚠改过 | 006033F0 `ShellStatic__AnimationInterval` | decompiled/functions/10066B20.c | src/Misc/Interface.cpp:549 | src/ui/campaign_score_shell.rs, src/ui/campaign_shell/mod.rs |
| 60378B | `CampaignMenu_ChooseButtonPalette` | kept | 006035F0 `ShellStatic__ResolveShapePalette` | decompiled/functions/10066970.c | src/Misc/Interface.cpp:507 | — |
| 603A2E | `CampaignMenu_ChooseButtonImage` | kept | 00603870 `ShellStatic__ResolveShape` | decompiled/functions/10066910.c | src/Misc/Interface.cpp:525 | — |
| 60411B | `Game_DialogFunc_Subtext_Load` | kept | 006040B0 `OwnerDraw__GetTooltipStringLabel` | decompiled/functions/10075EC0.c | src/UI/Dialogs.cpp:64 | src/app/frontend/skirmish_shell_render/pause_menu.rs, src/app/shell_load_saved_game.rs, src/app/shell_wol/buddy.rs … |
| 604136 | `Game_DialogFunc_Subtext_Propagate` | kept | 006040B0 `OwnerDraw__GetTooltipStringLabel` | decompiled/functions/10075F30.c | src/UI/Dialogs.cpp:89 | src/app/frontend/skirmish_shell_render/pause_menu.rs, src/app/shell_load_saved_game.rs, src/app/shell_wol/buddy.rs … |
| 6041ED | `DialogFunc_SubText_CampaignIconA` | kept | 006040B0 `OwnerDraw__GetTooltipStringLabel` | decompiled/functions/10066F20.c | src/Misc/Interface.cpp:612 | src/app/frontend/skirmish_shell_render/pause_menu.rs, src/app/shell_load_saved_game.rs, src/app/shell_wol/buddy.rs … |
| 6041F5 | `DialogFunc_CampaignMenu_CampaignIconB` | kept | 006040B0 `OwnerDraw__GetTooltipStringLabel` | decompiled/functions/10066F10.c | src/Misc/Interface.cpp:628 | src/app/frontend/skirmish_shell_render/pause_menu.rs, src/app/shell_load_saved_game.rs, src/app/shell_wol/buddy.rs … |
| 60A90A | `CampaignMenu_StaticButtonImage` | kept ⚠改过 | 0060A5B0 `ShellStatic__InitKind` | decompiled/functions/10066B60.c | src/Misc/Interface.cpp:542 | src/app/shell_main_menu.rs, src/ui/skirmish_shell/statics.rs |
| 60DDA6 | `Handle_Combobox_Dropdown_Messages_SelectionA` | kept | 0060D540 `ComboDropWin__WndProc` | decompiled/functions/10067480.c | src/Misc/Interface.UIColors.cpp:93 | src/ui/shell/combobox.rs |
| 60DDB6 | `Handle_Combobox_Dropdown_Messages_SelectionB` | kept | 0060D540 `ComboDropWin__WndProc` | decompiled/functions/100674A0.c | src/Misc/Interface.UIColors.cpp:98 | src/ui/shell/combobox.rs |
| 60FAD7 | `Ownerdraw_PostProcessColors` | kept | 0060F9A0 `ShellDialog__SubclassChildByClass` | decompiled/functions/10067680.c | src/Misc/Interface.UIColors.cpp:3 | src/app/input/ime.rs |
| 612DA9 | `Handle_Button_Messages_Color` | kept | 00612B70 `OwnerDraw_Button_00612B70` | decompiled/functions/10067400.c | src/Misc/Interface.UIColors.cpp:23 | src/app/frontend/skirmish_shell_render/launcher_options.rs, src/app/frontend/skirmish_shell_render/multiplayer_setup.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 613072 | `Handle_Button_Messages_DisabledColor` | kept | 00612B70 `OwnerDraw_Button_00612B70` | decompiled/functions/10067420.c | src/Misc/Interface.UIColors.cpp:28 | src/app/frontend/skirmish_shell_render/launcher_options.rs, src/app/frontend/skirmish_shell_render/multiplayer_setup.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 614FF2 | `Handle_NewEdit_Messages_Color` | kept | 00614B30 `OwnerDraw_NewEdit_00614B30` | decompiled/functions/100675C0.c | src/Misc/Interface.UIColors.cpp:118 | src/app/frontend/skirmish_shell_render/controls.rs, src/app/input/ime.rs, src/app/shell_lan.rs … |
| 615AB7 | `Handle_Static_Messages_Disabled` | kept | 006153E0 `OwnerDraw_Static_006153E0` | decompiled/functions/10067660.c | src/Misc/Interface.UIColors.cpp:53 | src/app/frontend/skirmish_shell_render/restate.rs, src/render/native_surface_format.rs, src/ui/shell/restate.rs |
| 615DF7 | `Handle_Static_Messages_Color` | kept | 006153E0 `OwnerDraw_Static_006153E0` | decompiled/functions/10067640.c | src/Misc/Interface.UIColors.cpp:48 | src/app/frontend/skirmish_shell_render/restate.rs, src/render/native_surface_format.rs, src/ui/shell/restate.rs |
| 61664C | `Handle_Checkbox_Messages_Color` | kept | 006163A0 `OwnerDraw_Checkbox_006163A0` | decompiled/functions/10067440.c | src/Misc/Interface.UIColors.cpp:33 | src/app/frontend/skirmish_shell_render/controls.rs, src/app/frontend/skirmish_shell_render/wol_options.rs, src/app/frontend/skirmish_shell_render/wol_persona.rs … |
| 616655 | `Handle_Checkbox_Messages_Disabled` | kept | 006163A0 `OwnerDraw_Checkbox_006163A0` | decompiled/functions/10067460.c | src/Misc/Interface.UIColors.cpp:38 | src/app/frontend/skirmish_shell_render/controls.rs, src/app/frontend/skirmish_shell_render/wol_options.rs, src/app/frontend/skirmish_shell_render/wol_persona.rs … |
| 616AF0 | `Handle_RadioButton_Messages_Color` | kept | 00616980 `OwnerDraw_RadioVariant_00616980` | decompiled/functions/100675E0.c | src/Misc/Interface.UIColors.cpp:43 | — |
| 617A2B | `Handle_Combobox_Messages_Color` | kept | 00617250 `OwnerDraw_ComboBox_00617250` | decompiled/functions/100674C0.c | src/Misc/Interface.UIColors.cpp:83 | src/app/frontend/skirmish_shell_render/wol_login.rs, src/ui/main_menu_dialogs/options.rs, src/ui/shell/combobox.rs … |
| 617A57 | `Handle_Combobox_Messages_Disabled` | kept | 00617250 `OwnerDraw_ComboBox_00617250` | decompiled/functions/100674E0.c | src/Misc/Interface.UIColors.cpp:88 | src/app/frontend/skirmish_shell_render/wol_login.rs, src/ui/main_menu_dialogs/options.rs, src/ui/shell/combobox.rs … |
| 619270 | `Handle_Listbox_Messages_SelectionA` | kept | 00618D40 `OwnerDraw_ListBox_00618D40` | decompiled/functions/10067580.c | src/Misc/Interface.UIColors.cpp:73 | src/app/frontend/skirmish_shell_render/list.rs, src/app/frontend/skirmish_shell_render/modals.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 619288 | `Handle_Listbox_Messages_SelectionB` | kept | 00618D40 `OwnerDraw_ListBox_00618D40` | decompiled/functions/100675A0.c | src/Misc/Interface.UIColors.cpp:78 | src/app/frontend/skirmish_shell_render/list.rs, src/app/frontend/skirmish_shell_render/modals.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 6198D3 | `Handle_Listbox_Messages_DisabledA` | kept | 00618D40 `OwnerDraw_ListBox_00618D40` | decompiled/functions/10067540.c | src/Misc/Interface.UIColors.cpp:63 | src/app/frontend/skirmish_shell_render/list.rs, src/app/frontend/skirmish_shell_render/modals.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 619A4F | `Handle_Listbox_Messages_Color` | kept | 00618D40 `OwnerDraw_ListBox_00618D40` | decompiled/functions/10067520.c | src/Misc/Interface.UIColors.cpp:58 | src/app/frontend/skirmish_shell_render/list.rs, src/app/frontend/skirmish_shell_render/modals.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 619A5F | `Handle_Listbox_Messages_DisabledB` | kept | 00618D40 `OwnerDraw_ListBox_00618D40` | decompiled/functions/10067560.c | src/Misc/Interface.UIColors.cpp:68 | src/app/frontend/skirmish_shell_render/list.rs, src/app/frontend/skirmish_shell_render/modals.rs, src/app/frontend/skirmish_shell_render/text.rs … |
| 61E2A5 | `Handle_Slider_Messages_Color` | kept | 0061D950 `ShellTrackbar__WndProc` | decompiled/functions/10067600.c | src/Misc/Interface.UIColors.cpp:103 | src/app/frontend/campaign_shell_render.rs, src/render/skirmish_shell_chrome.rs, src/ui/campaign_shell/mod.rs … |
| 61E2B1 | `Handle_Slider_Messages_Disabled` | kept | 0061D950 `ShellTrackbar__WndProc` | decompiled/functions/10067620.c | src/Misc/Interface.UIColors.cpp:108 | src/app/frontend/campaign_shell_render.rs, src/render/skirmish_shell_chrome.rs, src/ui/campaign_shell/mod.rs … |
| 61E8A0 | `Handle_GroupBox_Messages_Color` | kept | 0061E700 `OwnerDraw__GroupBoxHandler` | decompiled/functions/10067500.c | src/Misc/Interface.UIColors.cpp:113 | src/app/frontend/movies_credits_render.rs, src/app/frontend/skirmish_shell_render/keyboard.rs, src/app/frontend/wol_welcome_render.rs |
| 62267F | `Dialog_Show_GetTemplate` | kept | 00622650 `UI__BeginDialog` | decompiled/functions/10066F90.c | src/Misc/Interface.cpp:316 | src/ui/campaign_shell/mod.rs, src/ui/skirmish_shell/state/player_name.rs |
| 6226EE | `Dialog_Show_UpdateControls` | kept | 00622650 `UI__BeginDialog` | decompiled/functions/10066FA0.c | src/Misc/Interface.cpp:323 | src/ui/campaign_shell/mod.rs, src/ui/skirmish_shell/state/player_name.rs |
| 69801A | `Game_GetGameTypePrefs_MultiEngineer` | kept | 00697F10 `SessionClass__ReadSkirmishSettings` | decompiled/functions/1006AF40.c | src/Misc/Skirmish.cpp:76 | src/app/frontend/skirmish_session.rs |
| 699043 | `GameMode_SaveGameSettings_MultiEngineer` | kept | 00698F90 `SessionClass__WriteSkirmishSettings` | decompiled/functions/1006AF00.c | src/Misc/Skirmish.cpp:64 | — |
| 69A310 | `SessionClass_GetPlayerColorScheme` | kept | 0069A310 `SessionClass::SessionClass__MapPlayerColorToScheme` | decompiled/functions/10067110.c | src/Misc/Interface.PlayerColors.cpp:20 | src/app/frontend/cncnet_spawn.rs, src/app/frontend/skirmish.rs, src/app/input/messages.rs … |
| 69B69B | `GameModeClass_PickRandomColor_Unlimited` | kept | 0069B690 `FUN_0069b690` | decompiled/functions/10066FC0.c | src/Misc/Interface.PlayerColors.cpp:191 | — |
| 69B7FF | `Session_SetColor_Unlimited` | kept | 0069B7E0 `FUN_0069b7e0` | decompiled/functions/100671D0.c | src/Misc/Interface.PlayerColors.cpp:196 | src/app/frontend/skirmish_session.rs, src/app/shell_wol/lobby.rs |
| 69B949 | `Game_ProcessRandomPlayers_ColorsA` | kept | 0069B8C0 `SessionClass__ProcessRandomAssignments` | decompiled/functions/10067090.c | src/Misc/Interface.PlayerColors.cpp:181 | src/app/cooperative_round.rs, src/app/diagnostics/shell_capture/movies/lan.rs, src/app/input/taunt.rs … |
| 69B97D | `Game_ProcessRandomPlayers_ObserverColor` | kept | 0069B8C0 `SessionClass__ProcessRandomAssignments` | decompiled/functions/100670F0.c | src/Misc/Interface.PlayerColors.cpp:171 | src/app/cooperative_round.rs, src/app/diagnostics/shell_capture/movies/lan.rs, src/app/input/taunt.rs … |
| 69BA13 | `Game_ProcessRandomPlayers_ColorsB` | kept | 0069B8C0 `SessionClass__ProcessRandomAssignments` | decompiled/functions/100670C0.c | src/Misc/Interface.PlayerColors.cpp:186 | src/app/cooperative_round.rs, src/app/diagnostics/shell_capture/movies/lan.rs, src/app/input/taunt.rs … |
| 6ACEE0 | `Skirmish_DialogFunc_UpdateMultiEngineer` | kept | 006ACEE0 `SkirmishDialog__OnCommand` | decompiled/functions/1006B000.c | src/Misc/Skirmish.cpp:9 | src/app/shell_skirmish.rs |
| 6AD8A4 | `Skirmish_DialogFunc_MultiEngineer` | kept | 006ACEE0 `SkirmishDialog__OnCommand` | decompiled/functions/1006AF90.c | src/Misc/Skirmish.cpp:29 | src/app/shell_skirmish.rs |
| 6AEE6A | `Skirmish_InitializeDialog_MultiEngineer` | kept | 006AE6E0 `SkirmishDialog__OnSetup497` | decompiled/functions/1006B070.c | src/Misc/Skirmish.cpp:49 | — |
| 74FAE0 | `GetModuleInternalVersion` | kept | 0074FAE0 `VersionClass__Version_Name` | decompiled/functions/1006B2D0.c | src/Misc/Status.cpp:49 | — |
| 74FDC0 | `GetModuleVersion` | kept | 0074FDC0 `FUN_0074fdc0` | decompiled/functions/1006B2F0.c | src/Misc/Status.cpp:43 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10065740 | `Interface::invokeClickAction` | 被钩子调用 | decompiled/functions/10065740.c |
| 1000D520 | `VocClass::FindIndex` | 被钩子调用 | decompiled/functions/1000D520.c |
| 10010090 | `VocClass::PlayGlobal` | 被钩子调用 | decompiled/functions/10010090.c |
| 10065F50 | `Interface::updateMenu` | 被钩子调用 | decompiled/functions/10065F50.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10069F10 | `INIClass::WriteBool` | 被钩子调用 | decompiled/functions/10069F10.c |
| 10012CE0 | `Randomizer::Random` | 被钩子调用 | decompiled/functions/10012CE0.c |
| 1002DCF0 | `UnitTypeClass::Find` | 被钩子调用 | decompiled/functions/1002DCF0.c |
| 10005FB0 | `INIClass::ReadBool` | 被钩子调用 | decompiled/functions/10005FB0.c |
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |

## 涉及的热点函数（8）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0052BA60 `Init_Game` | 5 | U.bugfixes, U.shell, ext:Ares, recovered:YR |
| 0052D9A0 `Main__PrepareSession` | 3 | U.shell, recovered:Frontend |
| 006153E0 `OwnerDraw_Static_006153E0` | 3 | U.bugfixes, U.shell |
| 006ACEE0 `SkirmishDialog__OnCommand` | 3 | U.shell, ext:Ares |
| 0052EC00 `CampaignDialog0x94__WndProc` | 7 | U.shell |
| 0058F2C0 `RandomMapGenerator__PlaceLowBridgeDeck` | 6 | U.shell |
| 00531F60 `MainMenuDialog0xE2_Proc_00531F60` | 5 | U.shell |
| 00618D40 `OwnerDraw_ListBox_00618D40` | 5 | U.shell |
