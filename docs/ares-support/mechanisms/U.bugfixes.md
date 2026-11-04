# `U.bugfixes` 无条件 Bugfix 钩子（无键）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M7 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（113）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 413FA3 | `AircraftClass_Init_Cloakable` | kept | 00413F80 `AircraftClass::AircraftClass__InitFromType` | decompiled/functions/1005C2D0.c | src/Misc/Bugfixes.cpp:607 | — |
| 414D36 | `AACombat` | kept | 00414BB0 `AircraftClass::AircraftClass__AI` | decompiled/functions/1005C190.c | src/Misc/Bugfixes.cpp:198 | src/sim/aircraft/mod.rs, src/sim/world/techno_ai.rs |
| 418478 | `AircraftClass_Mi_Attack_Untarget1` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C2F0.c | src/Misc/Bugfixes.cpp:631 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 4186D7 | `AircraftClass_Mi_Attack_Untarget2` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C310.c | src/Misc/Bugfixes.cpp:640 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418826 | `AircraftClass_Mi_Attack_Untarget3` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C330.c | src/Misc/Bugfixes.cpp:649 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418935 | `AircraftClass_Mi_Attack_Untarget4` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C350.c | src/Misc/Bugfixes.cpp:658 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418A44 | `AircraftClass_Mi_Attack_Untarget5` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C370.c | src/Misc/Bugfixes.cpp:667 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418B40 | `AircraftClass_Mi_Attack_Untarget6` | kept | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C390.c | src/Misc/Bugfixes.cpp:676 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 41D887 | `AirstrikeClass_Fire` | kept | 0041D860 `AirstrikeClass::AirstrikeClass__StartStrikeOnTarget` | decompiled/functions/1005C3B0.c | src/Misc/Bugfixes.cpp:620 | — |
| 422FCC | `AnimClass_Draw_Details` | kept ⚠改过 | 00422CA0 `AnimClass::AnimClass__DrawIt` | decompiled/functions/1005E020.c | src/Misc/Bugfixes.Perf.cpp:20 | src/app/presentation/instances/overlays.rs |
| 4242CA | `AnimClass_Update_FixIE_TrailerSeperation` | kept | 00423AC0 `AnimClass::AnimClass__AI` | decompiled/functions/1000B210.c | src/Misc/Bugfixes.cpp:427 | src/sim/anim_class.rs, src/sim/bounce.rs, src/sim/radiation.rs |
| 424B23 | `AnimClass_Update` | kept | 00423AC0 `AnimClass::AnimClass__AI` | decompiled/functions/1005C3E0.c | src/Misc/Bugfixes.cpp:99 | src/sim/anim_class.rs, src/sim/bounce.rs, src/sim/radiation.rs |
| 441C21 | `BuildingClass_Destroy_ShakeScreenZero` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/1005C4D0.c | src/Misc/Bugfixes.cpp:438 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441D25 | `BuildingClass_Destroy` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/1005C4C0.c | src/Misc/Bugfixes.cpp:82 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 442CE0 | `BuildingClass_Init_Cloakable` | kept | 00442C40 `BuildingClass::BuildingClass__Init_Managers` | decompiled/functions/1005C4F0.c | src/Misc/Bugfixes.cpp:596 | — |
| 4430E8 | `BuildingClass_Demolish_LogCrash` | kept | 00442D90 `BuildingClass::BuildingClass__SpawnSurvivors` | decompiled/functions/1005C470.c | src/Misc/Bugfixes.cpp:915 | src/sim/combat/smudge_dispatch.rs, src/sim/crew_survival.rs, src/sim/smudge_grid.rs |
| 4471D5 | `BuildingClass_Sell_DetonateNoBuildup` | kept | 00447110 `BuildingClass::BuildingClass__Sell_Back` | decompiled/functions/10056AD0.c | src/Misc/Bugfixes.cpp:781 | src/rules/buildup_asset_catalog.rs, src/rules/ruleset.rs, src/sim/production/production_sell.rs … |
| 449FF8 | `BuildingClass_Mi_Selling_PutMcv` | kept ⚠改过 | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C5D0.c | src/Misc/Bugfixes.cpp:1124 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 44A1FF | `BuildingClass_Mi_Selling_DetonatePostBuildup` | kept | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/10056AB0.c | src/Misc/Bugfixes.cpp:791 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 44A5F0 | `BuildingClass_Mi_Selling_EngineerFreeze` | kept | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C550.c | src/Misc/Bugfixes.cpp:887 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 44A8A2 | `BuildingClass_Mi_Selling_Crew` | kept | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C530.c | src/Misc/Bugfixes.cpp:1145 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 455E4C | `HouseClass_FindRepairBay` | kept ⚠改过 | 00455DD0 `BuildingClass__FindNearestRadioCompatibleRepairStation` | decompiled/functions/1005CB50.c | src/Misc/Bugfixes.cpp:394 | src/sim/world/unit_repair_seek.rs |
| 4692A2 | `BulletClass_DetonateAt_RaiseAttackedByHouse` | kept | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C630.c | src/Misc/Bugfixes.cpp:756 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 4693B0 | `BulletClass_Fire_Overpower` | kept | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C650.c | src/Misc/Bugfixes.cpp:151 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 469467 | `BulletClass_DetonateAt_CanTemporalTarget` | kept | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C600.c | src/Misc/Bugfixes.cpp:581 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 472198 | `CaptureManagerClass_DrawLinks` | kept | 00472160 `CaptureManagerClass::CaptureManagerClass__DrawLinks` | decompiled/functions/1005C6C0.c | src/Misc/Bugfixes.cpp:498 | — |
| 47243F | `CaptureManagerClass_DecideUnitFate_BuildingFate` | kept | 004723B0 `CaptureManagerClass::CaptureManagerClass__DecideUnitFate` | decompiled/functions/1005C680.c | src/Misc/Bugfixes.cpp:770 | src/rules/mind_control_rules.rs, src/sim/capture_manager.rs |
| 4748A0 | `INIClass_GetPipIdx` | kept ⚠改过 | 004748A0 `INIClass::INIClass__ReadPip` | decompiled/functions/1005CC40.c | src/Misc/Bugfixes.cpp:946 | src/rules/ini_value.rs |
| 4750EC | `INIClass_ReadHouseTypesList` | kept | 004750D0 `INIClass::INIClass__ReadHouseTypesList` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:287 | src/rules/ini_value.rs, src/rules/object_type.rs |
| 475107 | `INIClass_ReadHouseTypesList_Strtok` | kept | 004750D0 `INIClass::INIClass__ReadHouseTypesList` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:294 | src/rules/ini_value.rs, src/rules/object_type.rs |
| 47527C | `INIClass_GetAlliesBitfield` | kept | 00475260 `INIClass::INIClass__ReadHousesList` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:300 | src/map/houses.rs |
| 475297 | `INIClass_GetAlliesBitfield_Strtok` | kept | 00475260 `INIClass::INIClass__ReadHousesList` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:307 | src/map/houses.rs |
| 4759D4 | `INIClass_WriteEdge` | kept | 004759D0 `INIClass::INIClass__WriteEdge` | decompiled/functions/1005CD00.c | src/Misc/Bugfixes.cpp:1108 | — |
| 489562 | `DamageArea_DestroyCliff` | kept | 00489280 `Apply_area_damage` | decompiled/functions/1005BE20.c | src/Misc/Bugfixes.Buffer.cpp:319 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 48A4F9 | `SelectDamageAnimation_FixNegatives` | removed_in_3.0 | 0048A4F0 `Warhead__SelectExplosionAnim` | decompiled/functions/1005D070.c | src/Misc/Bugfixes.cpp | src/rules/effect_asset_catalog.rs, src/sim/combat/detonation_anim.rs, src/sim/crates/effects.rs … |
| 48A634 | `FlashbangWarheadAt_Details` | kept ⚠改过 | 0048A620 `WarheadFlash__CreateAt` | decompiled/functions/1005E090.c | src/Misc/Bugfixes.Perf.cpp:13 | src/sim/anim_class.rs, src/sim/crates/effects.rs, src/sim/movement/rocket_movement.rs … |
| 4B5EB0 | `DropPodLocomotionClass_ILocomotion_Process_Smoke` | kept | 004B5B70 `DropPodLocomotionClass__Process` | decompiled/functions/1005C8B0.c | src/Misc/Bugfixes.cpp:1003 | — |
| 4B5F9E | `DropPodLocomotionClass_ILocomotion_Process_Report` | kept | 004B5B70 `DropPodLocomotionClass__Process` | decompiled/functions/1005C890.c | src/Misc/Bugfixes.cpp:1029 | — |
| 4C6DDB | `Networking_RespondToEvent_Selling` | kept | 004C6CB0 `EventClass::EventClass__Execute` | decompiled/functions/1005CE40.c | src/Misc/Bugfixes.cpp:1153 | src/sim/command.rs, src/sim/radio/tether.rs, src/sim/world/world_commands.rs |
| 4CA437 | `FactoryClass_GetCRC` | kept | 004CA430 `FactoryClass::FactoryClass__ComputeCRC` | decompiled/functions/1005C9A0.c | src/Misc/Bugfixes.cpp:693 | — |
| 4D9F7B | `FootClass_Sell_Detonate` | kept | 004D9F70 `FootClass::FootClass__OnSold` | decompiled/functions/10056AD0.c | src/Misc/Bugfixes.cpp:800 | src/app/match_runtime/eva_producers.rs, src/sim/docking/building_dock.rs, src/sim/production/production_sell.rs |
| 4FA2E0 | `HouseClass_SetThreat_Bounds` | kept ⚠改过 | 004FA2E0 `HouseClass::HouseClass__Adjust_Threat` | decompiled/functions/1005CC10.c | src/Misc/Bugfixes.cpp:1097 | src/sim/house_threat.rs |
| 50928C | `HouseClass_Update_Factories_Queues_SkipBrokenDTOR` | kept | 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | decompiled/functions/1005CC30.c | src/Misc/Bugfixes.cpp:463 | src/sim/production/factory_lifecycle.rs |
| 51DF27 | `InfantryClass_Remove_Teleport` | kept | 0051DF10 `InfantryClass::InfantryClass__Limbo` | decompiled/functions/1005CD50.c | src/Misc/Bugfixes.cpp:824 | src/sim/slave_manager.rs, src/sim/world/lifecycle.rs |
| 52070F | `InfantryClass_UpdateFiringState_Uncloak` | kept | 005206B0 `InfantryClass::InfantryClass__Fire_At_Target` | decompiled/functions/1005CDD0.c | src/Misc/Bugfixes.cpp:1036 | — |
| 52BB64 | `Expand_MIX_Deorg` | kept | 0052BA60 `Init_Game` | decompiled/functions/1005C940.c | src/Misc/Bugfixes.cpp:58 | src/app/frontend/startup_splash.rs, src/app/loading/init_helpers.rs, src/app/mod.rs … |
| 53029E | `Load_Bootstrap_AresMIX` | kept | 005301A0 `Init_Base_Mix_Files` | decompiled/functions/1005CE30.c | src/Misc/Bugfixes.cpp:64 | — |
| 531726 | `StupidPips1` | kept | 00531680 `Show_Loading_Screen` | decompiled/functions/1005D130.c | src/Misc/Bugfixes.cpp:115 | — |
| 53173F | `StupidPips2` | kept | 00531680 `Show_Loading_Screen` | decompiled/functions/1005D140.c | src/Misc/Bugfixes.cpp:121 | — |
| 547043 | `IsometricTileTypeClass_ReadFromFile` | kept | 00547020 `IsometricTileTypeClass::TMP_Loader` | decompiled/functions/1005E270.c | src/Misc/Bugfixes.Safeguards.cpp:6 | — |
| 550BCA | `LaserDrawClass_Draw_InHouseColor_Details` | kept ⚠改过 | 005509F0 `LaserDrawClass__DrawBeamSpecial` | decompiled/functions/1005E0B0.c | src/Misc/Bugfixes.Perf.cpp:26 | — |
| 56017A | `OptionsDlg_WndProc_RemoveResLimit` | kept | 0055FDB0 `OptionsLauncherDialog__Proc_D5` | decompiled/functions/1005CF10.c | src/Misc/Bugfixes.cpp:372 | src/app/persistence/options/launcher.rs, src/ui/main_menu_dialogs/options.rs |
| 5601E3 | `OptionsDlg_WndProc_RemoveHiResCheck` | kept | 0055FDB0 `OptionsLauncherDialog__Proc_D5` | decompiled/functions/1005CF00.c | src/Misc/Bugfixes.cpp:377 | src/app/persistence/options/launcher.rs, src/ui/main_menu_dialogs/options.rs |
| 56BC54 | `ThreatPosedEstimates_GetIndex` | kept | 0056BC50 `CalculateSpatialThreatPaddedIndex` | decompiled/functions/1005D4A0.c | src/Misc/Bugfixes.cpp:1080 | src/sim/house_threat.rs |
| 5CB0B1 | `QueryPerformance` | kept | 005CAFC0 `FUN_005cafc0` | decompiled/functions/1005D020.c | src/Misc/Bugfixes.cpp:44 | — |
| 5F698F | `ObjectClass_GetCell` | kept | 005F6960 `ObjectClass::ObjectClass__GetOccupiedCell` | decompiled/functions/1005CE70.c | src/Misc/Bugfixes.cpp:127 | src/sim/aircraft/retreat_mission.rs, src/sim/anim_class.rs, src/sim/cloak_disguise.rs … |
| 5F77F0 | `ObjectTypeClass_UnloadPipsSHP` | kept | 005F77F0 `ObjectTypeClass__FreePipAssets` | decompiled/functions/1005CE80.c | src/Misc/Bugfixes.cpp:978 | — |
| 615BD3 | `Handle_Static_Messages_LoopingMovie` | kept | 006153E0 `OwnerDraw_Static_006153E0` | decompiled/functions/1005CB20.c | src/Misc/Bugfixes.cpp:863 | src/app/frontend/skirmish_shell_render/restate.rs, src/render/native_surface_format.rs, src/ui/shell/restate.rs |
| 621B80 | `DSurface_FillRectWithColor` | kept | 00621B80 `AlphaBlendRect` | decompiled/functions/1005C720.c | src/Misc/Bugfixes.cpp:560 | src/render/main_menu_shell_chrome.rs |
| 62A2F8 | `ParasiteClass_PointerGotInvalid` | kept | 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | decompiled/functions/1005CF20.c | src/Misc/Bugfixes.cpp:514 | src/sim/combat/parasite.rs |
| 62C2ED | `ParticleClass_Update_Gas` | kept | 0062BD50 `ParticleClass::ParticleClass__AI_Gas` | decompiled/functions/1005CFE0.c | src/Misc/Bugfixes.cpp:742 | src/sim/particles/system_ai.rs |
| 62CDE8 | `ParticleClass_Update_Fire` | kept | 0062CB10 `ParticleClass::ParticleClass__AI_Fire` | decompiled/functions/1005CFA0.c | src/Misc/Bugfixes.cpp:730 | — |
| 62CEC9 | `ParticleClass_Draw_Details` | kept ⚠改过 | 0062CEC0 `ParticleClass::ParticleClass__DrawIt` | decompiled/functions/1005E170.c | src/Misc/Bugfixes.Perf.cpp:32 | — |
| 65D8FB | `TeamTypeClass_ValidateHouse` | kept | 0065EC30 `ChronoSphere__WarpUnitsAtCell` | decompiled/functions/1005D170.c | src/Misc/Bugfixes.cpp:707 | src/sim/team_script_vm/reinforcements.rs |
| 66BC71 | `Buf_CombatDamage` | kept | 0066BBB0 `RulesClass::RulesClass__ReadCombatDamage` | decompiled/functions/1005BA50.c | src/Misc/Bugfixes.Buffer.cpp:132 | src/rules/combat_damage.rs, src/rules/native_processing.rs, src/rules/rules_type_references.rs |
| 66D55E | `Buf_General` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BB00.c | src/Misc/Bugfixes.Buffer.cpp:45 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66DB93 | `Buf_BridgeExplosions` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BA40.c | src/Misc/Bugfixes.Buffer.cpp:126 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66DD13 | `Buf_WeatherArt` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BE10.c | src/Misc/Bugfixes.Buffer.cpp:121 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F34B | `Buf_RepairBay` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDD0.c | src/Misc/Bugfixes.Buffer.cpp:111 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F589 | `Buf_Shipyard` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BE00.c | src/Misc/Bugfixes.Buffer.cpp:105 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F7C0 | `Buf_PPA` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDB0.c | src/Misc/Bugfixes.Buffer.cpp:95 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66FA13 | `Buf_SecretBoons` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDF0.c | src/Misc/Bugfixes.Buffer.cpp:90 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 67062F | `Buf_AnimToInf_Paradrop` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BA30.c | src/Misc/Bugfixes.Buffer.cpp:85 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 671152 | `RulesClass_Addition_General_PrismSupportModifier` | kept | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005D040.c | src/Misc/Bugfixes.cpp:142 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 672B0E | `Buf_AI` | kept | 00672AE0 `RulesClass::RulesClass__ReadAI` | decompiled/functions/1005B830.c | src/Misc/Bugfixes.Buffer.cpp:152 | src/rules/object_type.rs, src/rules/rules_type_references.rs, src/sim/house_state.rs … |
| 6873AB | `INIClass_ReadScenario_EarlyLoadRules` | kept | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/1002EF30.c | src/Misc/Bugfixes.cpp:352 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |
| 69281E | `DisplayClass_ChooseAction_TogglePower` | kept | 00692610 `DisplayClass::DisplayClass__DetermineAction` | decompiled/functions/1005C790.c | src/Misc/Bugfixes.cpp:1048 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/entity_pick.rs … |
| 699C1C | `Game_ParsePKTs_ClearFile` | kept | 00699980 `SessionClass__ScanMultiplayerMapFiles` | decompiled/functions/1005CA80.c | src/Misc/Bugfixes.cpp:446 | src/assets/asset_manager.rs, src/map/scenario_sources.rs |
| 6BB9DD | `WinMain_LogNonsense` | kept | 006BB9A0 `WinMain` | decompiled/functions/1005D7C0.c | src/Misc/Bugfixes.cpp:109 | src/app/persistence/options_profile.rs, src/util/config.rs |
| 6BD7E3 | `Expand_MIX_Reorg` | kept | 006BB9A0 `WinMain` | decompiled/functions/1005C950.c | src/Misc/Bugfixes.cpp:52 | src/app/persistence/options_profile.rs, src/util/config.rs |
| 6BE9BD | `sub_6BE1C0` | kept | 006BE1C0 `AtExit_Free_Game_Resources` | decompiled/functions/1005D840.c | src/Misc/Bugfixes.cpp:70 | — |
| 6D7847 | `TacticalClass_DrawPixelEffects_Details` | kept ⚠改过 | 006D7840 `Tactical::DrawPixelFXSparkles` | decompiled/functions/1005E1B0.c | src/Misc/Bugfixes.Perf.cpp:38 | src/app/presentation/render/build_instances.rs |
| 6F661D | `TechnoClass_DrawHealthBar_DestroyedBuilding_RedPip` | kept | 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | decompiled/functions/1005D280.c | src/Misc/Bugfixes.cpp:764 | src/app/presentation/render/object_extras.rs, src/app/presentation/ui_overlays.rs |
| 6F7561 | `Arcing_Aircraft` | kept | 006F7220 `TechnoClass::TechnoClass__InRange` | decompiled/functions/1005C450.c | src/Misc/Bugfixes.cpp:206 | src/rules/weapon_type.rs, src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs … |
| 6FA2C7 | `TechnoClass_Update_DrawHidden` | kept ⚠改过 | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/1005D330.c | src/Misc/Bugfixes.cpp:1166 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 6FCA30 | `TechnoClass_GetWeaponState` | kept | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/1005D2B0.c | src/Misc/Bugfixes.cpp:134 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 700E47 | `TechnoClass_CanDeploySlashUnload_Immobile` | kept | 00700D50 `TechnoClass::TechnoClass__CanDeploySlashUnload` | decompiled/functions/1005D1C0.c | src/Misc/Bugfixes.cpp:869 | src/sim/combat/rof.rs, src/sim/deploy.rs, src/sim/world/world_commands.rs |
| 707B09 | `TechnoClass_PointerGotInvalid_ResetMindControl` | removed_in_3.0 | 007077C0 `TechnoClass::TechnoClass__PointerExpired` | decompiled/functions/1005D300.c | src/Misc/Bugfixes.cpp | src/app/input/entity_pick.rs, src/sim/combat/aim_coord.rs, src/sim/combat/attack_target.rs … |
| 70CAD8 | `TechnoClass_DealParticleDamage_DontDestroyCliff` | kept | 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | decompiled/functions/1005BF80.c | src/Misc/Bugfixes.Buffer.cpp:314 | — |
| 70CBDA | `TechnoClass_DealParticleDamage` | kept | 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | decompiled/functions/1005D260.c | src/Misc/Bugfixes.cpp:723 | — |
| 7125DF | `TechnoTypeClass_LoadFromINI_ListLength` | kept | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005BF90.c | src/Misc/Bugfixes.Buffer.cpp:186 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 713171 | `TechnoTypeClass_LoadFromINI_SkipLists1` | kept | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005C020.c | src/Misc/Bugfixes.Buffer.cpp:204 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 713C10 | `TechnoTypeClass_LoadFromINI_SkipLists2` | kept | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005C040.c | src/Misc/Bugfixes.Buffer.cpp:214 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 715857 | `TechnoTypeClass_LoadFromINI_LimitPalettes` | kept | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005D380.c | src/Misc/Bugfixes.cpp:76 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 71810D | `TeleportLocomotionClass_ILocomotion_MoveTo_Deactivated` | kept | 00718100 `TeleportLocomotionClass__Move_To` | decompiled/functions/1005D3E0.c | src/Misc/Bugfixes.cpp:813 | src/sim/movement/infantry_scatter.rs, src/sim/movement/teleport_movement.rs, src/sim/movement/track_path.rs |
| 718871 | `TeleportLocomotionClass_UnfreezeObject_SinkOrSwim` | kept | 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | decompiled/functions/1005D420.c | src/Misc/Bugfixes.cpp:538 | src/sim/movement/teleport_chrono.rs |
| 71AA52 | `TemporalClass_Update_AnnounceInvalidPointer` | kept | 0071A760 `TemporalClass::TemporalClass__Update` | decompiled/functions/1005D470.c | src/Misc/Bugfixes.cpp:686 | src/sim/temporal.rs |
| 71AF2B | `TemporalClass_Fire_UnwarpableA` | kept | 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | decompiled/functions/1005D460.c | src/Misc/Bugfixes.cpp:89 | src/sim/projectile.rs, src/sim/temporal.rs |
| 720A61 | `sub_7209D0_NoLog` | kept | 007209D0 `ThemeClass::ThemeClass__AI` | decompiled/functions/1005D850.c | src/Misc/Bugfixes.cpp:857 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 720DE8 | `Theme_Stop_NoLog` | kept | 00720BB0 `ThemeClass::ThemeClass__Play` | decompiled/functions/1005D490.c | src/Misc/Bugfixes.cpp:846 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 720F37 | `sub_720EA0_NoLog` | kept | 00720EA0 `ThemeClass::ThemeClass__Stop` | decompiled/functions/1005D860.c | src/Misc/Bugfixes.cpp:852 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 7274AF | `TriggerTypeClass_LoadFromINI_Read_Events` | kept | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:260 | src/map/triggers.rs |
| 7274C8 | `TriggerTypeClass_LoadFromINI_Strtok_Events` | kept | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:267 | src/map/triggers.rs |
| 727529 | `TriggerTypeClass_LoadFromINI_Read_Actions` | kept | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:274 | src/map/triggers.rs |
| 727544 | `TriggerTypeClass_LoadFromINI_Strtok_Actions` | kept | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005C070.c | src/Misc/Bugfixes.Buffer.cpp:281 | src/map/triggers.rs |
| 730DB0 | `GuardCommandClass_Execute` | kept | 00730D60 `FUN_00730d60` | decompiled/functions/1005CAF0.c | src/Misc/Bugfixes.cpp:488 | src/sim/world/world_commands.rs |
| 730EE5 | `StopCommandClass_Execute_Berzerk` | kept | 00730EA0 `Selection__IssueStopCommand` | decompiled/functions/1005D110.c | src/Misc/Bugfixes.cpp:937 | src/app/input/dispatch.rs |
| 737CE4 | `UnitClass_ReceiveDamage_ShipyardRepair` | kept | 00737C90 `UnitClass::UnitClass__ReceiveDamage` | decompiled/functions/1005D5E0.c | src/Misc/Bugfixes.cpp:997 | src/sim/combat/destruction_effects.rs, src/sim/combat/receiver_call.rs, src/sim/combat/unit_damage_response.rs … |
| 73D81E | `UnitClass_Mi_Unload_LastPassenger` | kept | 0073D630 `UnitClass::UnitClass__Mission_Unload` | decompiled/functions/1005D5B0.c | src/Misc/Bugfixes.cpp:928 | src/rules/object_type.rs, src/sim/components.rs, src/sim/deploy.rs … |
| 7440BD | `UnitClass_Remove` | kept | 007440B0 `UnitClass::UnitClass__Limbo` | decompiled/functions/1005D610.c | src/Misc/Bugfixes.cpp:453 | src/sim/production/production_sell.rs |
| 75D660 | `WarheadTypeClass_LoadFromINI_ListLength` | kept | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/1005C080.c | src/Misc/Bugfixes.Buffer.cpp:226 | src/rules/warhead_type.rs |
| 75DAE6 | `WarheadTypeClass_LoadFromINI_SkipLists` | kept | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/1005C0E0.c | src/Misc/Bugfixes.Buffer.cpp:240 | src/rules/warhead_type.rs |
| 772462 | `WeaponTypeClass_LoadFromINI_ListLength` | kept | 00772080 `WeaponTypeClass::WeaponTypeClass__ReadINI` | decompiled/functions/1005C0F0.c | src/Misc/Bugfixes.Buffer.cpp:247 | src/rules/mod.rs, src/rules/native_processing.rs, src/rules/ruleset.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002EC80 | `RulesClass::Read_Sides` | 被钩子调用 | decompiled/functions/1002EC80.c |
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |
| 10061960 | `Debug::INIParseFailed` | 被钩子调用 | decompiled/functions/10061960.c |
| 1005B810 | `CellClass::CellClass__Tile_Is_DestroyableCliff` | 被钩子调用 | decompiled/functions/1005B810.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1005B700 | `MapClass::DestroyCliff` | 被钩子调用 | decompiled/functions/1005B700.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 1005C120 | `MixFileClass::Bootstrap` | 被钩子调用 | decompiled/functions/1005C120.c |
| 10003880 | `INIClass::Clear` | 被钩子调用 | decompiled/functions/10003880.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 10003660 | `Ares::InitOwnResources` | 被钩子调用 | decompiled/functions/10003660.c |
| 100097B0 | `TechnoClass::SetFocus` | 被钩子调用 | decompiled/functions/100097B0.c |
| 1002C9E0 | `SHPStruct::~SHPStruct` | 被钩子调用 | decompiled/functions/1002C9E0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 1005C170 | `TeamTypeClass::GetHouse` | 被钩子调用 | decompiled/functions/1005C170.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 1005C130 | `BuildingClass::ClearBunker` | 被钩子调用 | decompiled/functions/1005C130.c |
| 100038B0 | `Ares::UninitOwnResources` | 被钩子调用 | decompiled/functions/100038B0.c |
| 1002A6C0 | `RulesExt::DetailsCurrentlyEnabled` | 被钩子调用 | decompiled/functions/1002A6C0.c |

## 涉及的热点函数（44）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00489280 `Apply_area_damage` | 14 | R.tiberium, U.bugfixes, W.filter, ext:Rules, recovered:DamageArea |
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno, recovered:TechnoClass |
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | 7 | U.bugfixes, W.death, ext:Bullet, ext:WarheadType, recovered:BulletClass |
| 0071A760 `TemporalClass::TemporalClass__Update` | 6 | U.bugfixes, ext:Techno, ext:WarheadType, recovered:TemporalClass |
| 0052BA60 `Init_Game` | 5 | U.bugfixes, U.shell, ext:Ares, recovered:YR |
| 0073D630 `UnitClass::UnitClass__Mission_Unload` | 5 | T.poweredby, U.bugfixes, ext:House, recovered:UnitClass |
| 00686B20 `CCINIClass::InitializeScenarioFromINI` | 4 | U.bugfixes, ext:Debug, ext:Invalidators, recovered:INIClass |
| 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | 9 | U.bugfixes, ext:TechnoType, recovered:TechnoTypeClass |
| 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | 5 | F.armor, U.bugfixes, ext:WarheadType |
| 00423AC0 `AnimClass::AnimClass__AI` | 4 | T.misc, U.bugfixes, recovered:AnimClass |
| 006BB9A0 `WinMain` | 4 | F.csf, U.bugfixes, recovered:LoadRA2MD |
| 00413F80 `AircraftClass::AircraftClass__InitFromType` | 3 | B.academy, U.bugfixes, recovered:AircraftClass |
| 004C6CB0 `EventClass::EventClass__Execute` | 3 | U.bugfixes, ext:Network, recovered:Networking |
| 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | 3 | B.firestorm, U.bugfixes, ext:Techno |
| 007077C0 `TechnoClass::TechnoClass__PointerExpired` | 3 | U.bugfixes, ext:Techno, recovered:TechnoClass |
| 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | 3 | B.prism, U.bugfixes, recovered:TemporalClass |
| 00737C90 `UnitClass::UnitClass__ReceiveDamage` | 3 | U.bugfixes, ext:Techno, ext:WarheadType |
| 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | 7 | U.bugfixes, recovered:BuildingClass |
| 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | 7 | U.bugfixes, recovered:TechnoClass |
| 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | 5 | U.bugfixes, recovered:TriggerTypeClass |
| 00772080 `WeaponTypeClass::WeaponTypeClass__ReadINI` | 4 | U.bugfixes, ext:WeaponType |
| 006153E0 `OwnerDraw_Static_006153E0` | 3 | U.bugfixes, U.shell |
| 006F7220 `TechnoClass::TechnoClass__InRange` | 3 | U.bugfixes, W.weaponmisc |
| 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | 3 | U.bugfixes, recovered:TechnoClass |
| 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | 3 | S.core, U.bugfixes |
| 00414BB0 `AircraftClass::AircraftClass__AI` | 2 | T.misc, U.bugfixes |
| 0041D860 `AirstrikeClass::AirstrikeClass__StartStrikeOnTarget` | 2 | U.bugfixes, ext:Techno |
| 00422CA0 `AnimClass::AnimClass__DrawIt` | 2 | U.bugfixes, ext:AnimType |
| 00442C40 `BuildingClass::BuildingClass__Init_Managers` | 2 | B.academy, U.bugfixes |
| 00442D90 `BuildingClass::BuildingClass__SpawnSurvivors` | 2 | U.bugfixes, recovered:BuildingClass |
| 00447110 `BuildingClass::BuildingClass__Sell_Back` | 2 | B.prism, U.bugfixes |
| 0048A4F0 `Warhead__SelectExplosionAnim` | 2 | S.core, U.bugfixes |
| 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | 2 | U.bugfixes, recovered:HouseClass |
| 0051DF10 `InfantryClass::InfantryClass__Limbo` | 2 | U.bugfixes, ext:Infantry |
| 005206B0 `InfantryClass::InfantryClass__Fire_At_Target` | 2 | U.bugfixes, recovered:InfantryClass |
| 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | 2 | T.cloak, U.bugfixes |
| 0062BD50 `ParticleClass::ParticleClass__AI_Gas` | 2 | U.bugfixes, recovered:ParticleClass |
| 0062CEC0 `ParticleClass::ParticleClass__DrawIt` | 2 | U.bugfixes, recovered:ParticleClass |
| 0065D8E0 `TeamTypeClass__SpawnReinforcements` | 2 | U.bugfixes, recovered:TeamTypeClass |
| 00692610 `DisplayClass::DisplayClass__DetermineAction` | 2 | F.cursors, U.bugfixes |
| 00700D50 `TechnoClass::TechnoClass__CanDeploySlashUnload` | 2 | U.bugfixes, ext:Techno |
| 0066D530 `RulesClass::RulesClass__ReadGeneral` | 9 | U.bugfixes |
| 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | 6 | U.bugfixes |
