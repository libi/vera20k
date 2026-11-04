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

## 钩子（154）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 41088D | `AbstractTypeClass_CTOR_IDTooLong` | new_in_3.0 | ✓ | 00410800 `AbstractTypeClass::AbstractTypeClass__Constructor` | decompiled/functions/1005E230.c | src/Misc/Bugfixes.Safeguards.cpp:57 | src/rules/mod.rs, src/rules/native_processing.rs, src/rules/ruleset.rs |
| 413FA3 | `AircraftClass_Init_Cloakable` | kept | ✓ | 00413F80 `AircraftClass::AircraftClass__InitFromType` | decompiled/functions/1005C2D0.c | src/Misc/Bugfixes.cpp:681 | — |
| 414D36 | `AACombat` | kept | ✓ | 00414BB0 `AircraftClass::AircraftClass__AI` | decompiled/functions/1005C190.c | src/Misc/Bugfixes.cpp:234 | src/sim/aircraft/mod.rs, src/sim/world/techno_ai.rs |
| 418478 | `AircraftClass_Mi_Attack_Untarget1` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C2F0.c | src/Misc/Bugfixes.cpp:737 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 4186D7 | `AircraftClass_Mi_Attack_Untarget2` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C310.c | src/Misc/Bugfixes.cpp:746 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418826 | `AircraftClass_Mi_Attack_Untarget3` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C330.c | src/Misc/Bugfixes.cpp:755 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418935 | `AircraftClass_Mi_Attack_Untarget4` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C350.c | src/Misc/Bugfixes.cpp:764 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418A44 | `AircraftClass_Mi_Attack_Untarget5` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C370.c | src/Misc/Bugfixes.cpp:773 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 418B40 | `AircraftClass_Mi_Attack_Untarget6` | kept | ✓ | 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | decompiled/functions/1005C390.c | src/Misc/Bugfixes.cpp:782 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/mod.rs, src/sim/combat/aircraft_release.rs … |
| 41D887 | `AirstrikeClass_Fire` | kept | ✓ | 0041D860 `AirstrikeClass::AirstrikeClass__StartStrikeOnTarget` | decompiled/functions/1005C3B0.c | src/Misc/Bugfixes.cpp:708 | — |
| 420F40 | `AlphaShapeClass_DrawAll_Details` | replaces_0.A:Spotlights_UpdateFoo | ✓ | 00420F40 `AlphaShapeClass__DrawAll_WithMask` | decompiled/functions/1005E000.c | src/Misc/Bugfixes.Perf.cpp:77 | — |
| 422FCC | `AnimClass_Draw_Details` | kept ⚠改过 | ✓ | 00422CA0 `AnimClass::AnimClass__DrawIt` | decompiled/functions/1005E020.c | src/Misc/Bugfixes.Perf.cpp:47 | src/app/presentation/instances/overlays.rs |
| 4239F0 | `AnimClass_UpdateBounce_Damage` | new_in_3.0 | ✓ | 00423930 `AnimClass::AnimClass__ProcessBounceResult` | decompiled/functions/1005C410.c | src/Misc/Bugfixes.cpp:1494 | src/sim/anim_class.rs, src/sim/bounce.rs |
| 424B23 | `AnimClass_Update` | kept | ✓ | 00423AC0 `AnimClass::AnimClass__AI` | decompiled/functions/1005C3E0.c | src/Misc/Bugfixes.cpp:135 | src/sim/anim_class.rs, src/sim/bounce.rs, src/sim/radiation.rs |
| 441C21 | `BuildingClass_Destroy_ShakeScreenZero` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/1005C4D0.c | src/Misc/Bugfixes.cpp:454 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441D25 | `BuildingClass_Destroy` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/1005C4C0.c | src/Misc/Bugfixes.cpp:118 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 442CE0 | `BuildingClass_Init_Cloakable` | kept | ✓ | 00442C40 `BuildingClass::BuildingClass__Init_Managers` | decompiled/functions/1005C4F0.c | src/Misc/Bugfixes.cpp:670 | — |
| 4430E8 | `BuildingClass_Demolish_LogCrash` | kept | ✓ | 00442D90 `BuildingClass::BuildingClass__SpawnSurvivors` | decompiled/functions/1005C470.c | src/Misc/Bugfixes.cpp:1097 | src/sim/combat/smudge_dispatch.rs, src/sim/crew_survival.rs, src/sim/smudge_grid.rs |
| 449FF8 | `BuildingClass_Mi_Selling_PutMcv` | kept ⚠改过 | ✓ | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C5D0.c | src/Misc/Bugfixes.cpp:1333 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 44A5F0 | `BuildingClass_Mi_Selling_EngineerFreeze` | kept | ✓ | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C550.c | src/Misc/Bugfixes.cpp:1069 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 44A8A2 | `BuildingClass_Mi_Selling_Crew` | kept | ✓ | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/1005C530.c | src/Misc/Bugfixes.cpp:1349 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 455E4C | `HouseClass_FindRepairBay` | kept ⚠改过 | ✓ | 00455DD0 `BuildingClass__FindNearestRadioCompatibleRepairStation` | decompiled/functions/1005CB50.c | src/Misc/Bugfixes.cpp:421 | src/sim/world/unit_repair_seek.rs |
| 4586D6 | `BuildingClass_KillOccupiers` | new_in_3.0 | ✓ | 004585C0 `BuildingClass::BuildingClass__KillOccupants` | decompiled/functions/1005C510.c | src/Misc/Bugfixes.cpp:1480 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/temporal.rs |
| 458E1E | `BuildingClass_GetOccupyRangeBonus_Demacroize` | new_in_3.0 | ✓ | 00458E00 `BuildingClass::BuildingClass__GetHalfFoundationSize` | decompiled/functions/1005E070.c | src/Misc/Bugfixes.Perf.cpp:176 | src/sim/combat/combat_weapon.rs |
| 4692A2 | `BulletClass_DetonateAt_RaiseAttackedByHouse` | kept | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C630.c | src/Misc/Bugfixes.cpp:939 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 4693B0 | `BulletClass_Fire_Overpower` | kept | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C650.c | src/Misc/Bugfixes.cpp:187 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 469467 | `BulletClass_DetonateAt_CanTemporalTarget` | kept | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1005C600.c | src/Misc/Bugfixes.cpp:655 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 472198 | `CaptureManagerClass_DrawLinks` | kept | ✓ | 00472160 `CaptureManagerClass::CaptureManagerClass__DrawLinks` | decompiled/functions/1005C6C0.c | src/Misc/Bugfixes.cpp:523 | — |
| 47243F | `CaptureManagerClass_DecideUnitFate_BuildingFate` | kept | ✓ | 004723B0 `CaptureManagerClass::CaptureManagerClass__DecideUnitFate` | decompiled/functions/1005C680.c | src/Misc/Bugfixes.cpp:953 | src/rules/mind_control_rules.rs, src/sim/capture_manager.rs |
| 4748A0 | `INIClass_GetPipIdx` | kept ⚠改过 | ✓ | 004748A0 `INIClass::INIClass__ReadPip` | decompiled/functions/1005CC40.c | src/Misc/Bugfixes.cpp:1128 | src/rules/ini_value.rs |
| 4750EC | `INIClass_ReadHouseTypesList` | kept | ✓ | 004750D0 `INIClass::INIClass__ReadHouseTypesList` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:314 | src/rules/ini_value.rs, src/rules/object_type.rs |
| 475107 | `INIClass_ReadHouseTypesList_Strtok` | kept | ✓ | 004750D0 `INIClass::INIClass__ReadHouseTypesList` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:321 | src/rules/ini_value.rs, src/rules/object_type.rs |
| 47527C | `INIClass_GetAlliesBitfield` | kept | ✓ | 00475260 `INIClass::INIClass__ReadHousesList` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:327 | src/map/houses.rs |
| 475297 | `INIClass_GetAlliesBitfield_Strtok` | kept | ✓ | 00475260 `INIClass::INIClass__ReadHousesList` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:334 | src/map/houses.rs |
| 4759D4 | `INIClass_WriteEdge` | kept | ✓ | 004759D0 `INIClass::INIClass__WriteEdge` | decompiled/functions/1005CD00.c | src/Misc/Bugfixes.cpp:1317 | — |
| 47F9A4 | `CellClass_DrawOverlay_WallRemap` | replaces_0.A:DrawOverlay_WallRemap | ✓ | 0047F6A0 `CellClass::CellClass__DrawOverlay_Body` | decompiled/functions/1005C6F0.c | src/Misc/Bugfixes.cpp:722 | src/app/presentation/instances/bridges.rs, src/app/presentation/instances/cell_bodies.rs, src/app/presentation/instances/overlays.rs … |
| 489562 | `DamageArea_DestroyCliff` | kept | ✓ | 00489280 `Apply_area_damage` | decompiled/functions/1005BE20.c | src/Misc/Bugfixes.Buffer.cpp:370 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 48A4F9 | `SelectDamageAnimation_FixNegatives` | removed_in_3.0 | — | 0048A4F0 `Warhead__SelectExplosionAnim` | decompiled/functions/1005D070.c | src/Misc/Bugfixes.cpp:693 | src/rules/effect_asset_catalog.rs, src/sim/combat/detonation_anim.rs, src/sim/crates/effects.rs … |
| 48A634 | `FlashbangWarheadAt_Details` | kept ⚠改过 | ✓ | 0048A620 `WarheadFlash__CreateAt` | decompiled/functions/1005E090.c | src/Misc/Bugfixes.Perf.cpp:30 | src/sim/anim_class.rs, src/sim/crates/effects.rs, src/sim/movement/rocket_movement.rs … |
| 4B5EB0 | `DropPodLocomotionClass_ILocomotion_Process_Smoke` | kept | ✓ | 004B5B70 `DropPodLocomotionClass__Process` | decompiled/functions/1005C8B0.c | src/Misc/Bugfixes.cpp:1184 | — |
| 4B5F9E | `DropPodLocomotionClass_ILocomotion_Process_Report` | kept | ✓ | 004B5B70 `DropPodLocomotionClass__Process` | decompiled/functions/1005C890.c | src/Misc/Bugfixes.cpp:1210 | — |
| 4B619F | `DropPodLocomotionClass_ILocomotion_MoveTo_AtmosphereEntry` | new_in_3.0 | ✓ | 004B6040 `DropPodLocomotionClass__Move_To` | decompiled/functions/1005C870.c | src/Misc/Bugfixes.cpp:1221 | — |
| 4C6DDB | `Networking_RespondToEvent_Selling` | kept | ✓ | 004C6CB0 `EventClass::EventClass__Execute` | decompiled/functions/1005CE40.c | src/Misc/Bugfixes.cpp:1357 | src/sim/command.rs, src/sim/radio/tether.rs, src/sim/world/world_commands.rs |
| 4CA0E3 | `FactoryClass_AbandonProduction_Invalidate` | new_in_3.0 | ✓ | 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | decompiled/functions/1005C960.c | src/Misc/Bugfixes.cpp:815 | src/sim/production/factory.rs, src/sim/production/factory_ai.rs, src/sim/production/factory_lifecycle.rs |
| 4CA437 | `FactoryClass_GetCRC` | kept | ✓ | 004CA430 `FactoryClass::FactoryClass__ComputeCRC` | decompiled/functions/1005C9A0.c | src/Misc/Bugfixes.cpp:799 | — |
| 4D5782 | `FootClass_ApproachTarget_Passive` | new_in_3.0 | ✓ | 004D5690 `FootClass::FootClass__Approach_Target` | decompiled/functions/1005C9F0.c | src/Misc/Bugfixes.cpp:1620 | src/sim/team_script_vm/actions.rs, src/sim/world/jumpjet_cruise.rs, src/sim/world/techno_ai/mission_handlers.rs |
| 4D9EE1 | `FootClass_CanBeSold_Dock` | new_in_3.0 | ✓ | 004D9E70 `FootClass::FootClass__CanBeSold` | decompiled/functions/1005CA30.c | src/Misc/Bugfixes.cpp:1605 | — |
| 4DB87E | `FootClass_SetLocation_Parasite` | replaces_0.A:FootClass_SetCoords | ✓ | 004DB810 `FootClass::FootClass__SetLocation` | decompiled/functions/1005CA50.c | src/Misc/Bugfixes.cpp:567 | src/sim/movement/ground_pose.rs, src/sim/movement/rocket_movement.rs, src/sim/passenger.rs … |
| 4FA2E0 | `HouseClass_SetThreat_Bounds` | kept ⚠改过 | ✓ | 004FA2E0 `HouseClass::HouseClass__Adjust_Threat` | decompiled/functions/1005CC10.c | src/Misc/Bugfixes.cpp:1283 | src/sim/house_threat.rs |
| 5005CC | `HouseClass_SetFactoryCreatedManually` | new_in_3.0 | ✓ | 005005B0 `FUN_005005b0` | decompiled/functions/1005CBD0.c | src/Misc/Bugfixes.cpp:838 | — |
| 50067C | `HouseClass_ClearFactoryCreatedManually` | new_in_3.0 | ✓ | 00500660 `FUN_00500660` | decompiled/functions/1005CB30.c | src/Misc/Bugfixes.cpp:850 | — |
| 5007BE | `HouseClass_SetFactoryCreatedManually2` | new_in_3.0 | ✓ | 005007A0 `HouseClass::HouseClass__SetProductionCategoryFlag` | decompiled/functions/1005CBF0.c | src/Misc/Bugfixes.cpp:861 | src/app/sidebar_projection.rs |
| 50928C | `HouseClass_Update_Factories_Queues_SkipBrokenDTOR` | kept | ✓ | 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | decompiled/functions/1005CC30.c | src/Misc/Bugfixes.cpp:488 | src/sim/production/factory_lifecycle.rs |
| 511D16 | `HouseTypeClass_LoadFromINI_Buffer_CountryVeteran` | replaces_0.A:Buf_CountryVeteran | ✓ | 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | decompiled/functions/1005BE70.c | src/Misc/Bugfixes.Buffer.cpp:59 | src/app/frontend/skirmish_shell_render/text.rs |
| 51DF27 | `InfantryClass_Remove_Teleport` | kept | ✓ | 0051DF10 `InfantryClass::InfantryClass__Limbo` | decompiled/functions/1005CD50.c | src/Misc/Bugfixes.cpp:979 | src/sim/slave_manager.rs, src/sim/world/lifecycle.rs |
| 51F716 | `InfantryClass_Mi_Unload_Undeploy` | new_in_3.0 | ✓ | 0051F6E0 `InfantryClass::InfantryClass__Mission_Unload` | decompiled/functions/1005CD20.c | src/Misc/Bugfixes.cpp:1703 | — |
| 52070F | `InfantryClass_UpdateFiringState_Uncloak` | kept | ✓ | 005206B0 `InfantryClass::InfantryClass__Fire_At_Target` | decompiled/functions/1005CDD0.c | src/Misc/Bugfixes.cpp:1226 | — |
| 52BA78 | `YR_GameInit_Pre` | replaces_0.A:_YR_GameInit_Pre | ✓ | 0052BA60 `Init_Game` | decompiled/functions/1005D7F0.c | src/Misc/Bugfixes.cpp:627 | src/app/frontend/startup_splash.rs, src/app/loading/init_helpers.rs, src/app/mod.rs … |
| 52BB64 | `Expand_MIX_Deorg` | kept | ✓ | 0052BA60 `Init_Game` | decompiled/functions/1005C940.c | src/Misc/Bugfixes.cpp:94 | src/app/frontend/startup_splash.rs, src/app/loading/init_helpers.rs, src/app/mod.rs … |
| 53029E | `Load_Bootstrap_AresMIX` | kept | ✓ | 005301A0 `Init_Base_Mix_Files` | decompiled/functions/1005CE30.c | src/Misc/Bugfixes.cpp:100 | — |
| 531726 | `StupidPips1` | kept | ✓ | 00531680 `Show_Loading_Screen` | decompiled/functions/1005D130.c | src/Misc/Bugfixes.cpp:151 | — |
| 53173F | `StupidPips2` | kept | ✓ | 00531680 `Show_Loading_Screen` | decompiled/functions/1005D140.c | src/Misc/Bugfixes.cpp:157 | — |
| 534F89 | `Game_ReloadNeutralMIX_NewLine` | new_in_3.0 | ✓ | 00534E50 `InitNeutralMixFiles` | decompiled/functions/1005CAA0.c | src/Misc/Bugfixes.cpp:1585 | src/assets/asset_manager.rs |
| 547043 | `IsometricTileTypeClass_ReadFromFile` | kept | ✓ | 00547020 `IsometricTileTypeClass::TMP_Loader` | decompiled/functions/1005E270.c | src/Misc/Bugfixes.Safeguards.cpp:9 | — |
| 550BCA | `LaserDrawClass_Draw_InHouseColor_Details` | kept ⚠改过 | ✓ | 005509F0 `LaserDrawClass__DrawBeamSpecial` | decompiled/functions/1005E0B0.c | src/Misc/Bugfixes.Perf.cpp:53 | — |
| 551A30 | `LayerClass_YSortReorder` | new_in_3.0 | ✓ | 00551A30 `LayerClass__AdjacentYSortPass` | decompiled/functions/1005E0D0.c | src/Misc/Bugfixes.Perf.cpp:90 | src/app/presentation/render/draw_plan_lowering.rs |
| 56017A | `OptionsDlg_WndProc_RemoveResLimit` | kept | ✓ | 0055FDB0 `OptionsLauncherDialog__Proc_D5` | decompiled/functions/1005CF10.c | src/Misc/Bugfixes.cpp:399 | src/app/persistence/options/launcher.rs, src/ui/main_menu_dialogs/options.rs |
| 5601E3 | `OptionsDlg_WndProc_RemoveHiResCheck` | kept | ✓ | 0055FDB0 `OptionsLauncherDialog__Proc_D5` | decompiled/functions/1005CF00.c | src/Misc/Bugfixes.cpp:404 | src/app/persistence/options/launcher.rs, src/ui/main_menu_dialogs/options.rs |
| 56BC54 | `ThreatPosedEstimates_GetIndex` | kept | ✓ | 0056BC50 `CalculateSpatialThreatPaddedIndex` | decompiled/functions/1005D4A0.c | src/Misc/Bugfixes.cpp:1266 | src/sim/house_threat.rs |
| 5C9A6E | `Global_CollectScoreScreenData` | new_in_3.0 | ✓ | 005C98A0 `ScoreDialog__FillEntries` | decompiled/functions/1005CAC0.c | src/Misc/Bugfixes.cpp:1566 | src/sim/score.rs |
| 5CB0B1 | `QueryPerformance` | kept | ✓ | 005CAFC0 `FUN_005cafc0` | decompiled/functions/1005D020.c | src/Misc/Bugfixes.cpp:80 | — |
| 5F6515 | `AbstractClass_Distance2DSquared_1` | new_in_3.0 | ✓ | 005F6500 `ObjectClass::ObjectClass__DistanceSquaredXY_To_Object` | decompiled/functions/1005C1A0.c | src/Misc/Bugfixes.cpp:1419 | src/sim/team_script_vm/team_ai.rs |
| 5F6560 | `AbstractClass_Distance2DSquared_2` | new_in_3.0 | ✓ | 005F6560 `TechnoClass::TechnoClass__DistanceSquaredTo` | decompiled/functions/1005C240.c | src/Misc/Bugfixes.cpp:1441 | src/sim/movement/bump_crush.rs, src/sim/team_script_vm/membership.rs, src/sim/team_script_vm/team_ai.rs |
| 5F6612 | `ObjectClass_UnInit_SkipInvalidation` | new_in_3.0 | ✓ | 005F65F0 `ObjectClass::ObjectClass__UnInit` | decompiled/functions/1005E140.c | src/Misc/Bugfixes.Perf.cpp:115 | src/sim/combat/aim_coord.rs, src/sim/combat/combat_aoe.rs, src/sim/combat/world_receiver.rs … |
| 5F698F | `ObjectClass_GetCell` | kept | ✓ | 005F6960 `ObjectClass::ObjectClass__GetOccupiedCell` | decompiled/functions/1005CE70.c | src/Misc/Bugfixes.cpp:163 | src/sim/aircraft/retreat_mission.rs, src/sim/anim_class.rs, src/sim/cloak_disguise.rs … |
| 5F77F0 | `ObjectTypeClass_UnloadPipsSHP` | kept | ✓ | 005F77F0 `ObjectTypeClass__FreePipAssets` | decompiled/functions/1005CE80.c | src/Misc/Bugfixes.cpp:1159 | — |
| 5FDDA4 | `IsOverlayIdxTiberium_Log` | new_in_3.0 | ✓ | 005FDD20 `OverlayIndex__GetTiberiumIndex` | decompiled/functions/1005CE10.c | src/Misc/Bugfixes.cpp:1657 | src/map/tiberium_cell.rs, src/sim/build_site.rs |
| 5FF86E | `SpotLightClass_Draw_Details` | replaces_0.A:SpotlightClass_Draw_Details | ✓ | 005FF850 `SpotLightClass::SpotLightClass__Draw` | decompiled/functions/1005E190.c | src/Misc/Bugfixes.Perf.cpp:40 | — |
| 615BD3 | `Handle_Static_Messages_LoopingMovie` | kept | ✓ | 006153E0 `OwnerDraw_Static_006153E0` | decompiled/functions/1005CB20.c | src/Misc/Bugfixes.cpp:1047 | src/app/frontend/skirmish_shell_render/restate.rs, src/render/native_surface_format.rs, src/ui/shell/restate.rs |
| 621B80 | `DSurface_FillRectWithColor` | kept | ✓ | 00621B80 `AlphaBlendRect` | decompiled/functions/1005C720.c | src/Misc/Bugfixes.cpp:600 | src/render/main_menu_shell_chrome.rs |
| 62A2F8 | `ParasiteClass_PointerGotInvalid` | kept | ✓ | 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | decompiled/functions/1005CF20.c | src/Misc/Bugfixes.cpp:539 | src/sim/combat/parasite.rs |
| 62C2ED | `ParticleClass_Update_Gas` | kept | ✓ | 0062BD50 `ParticleClass::ParticleClass__AI_Gas` | decompiled/functions/1005CFE0.c | src/Misc/Bugfixes.cpp:924 | src/sim/particles/system_ai.rs |
| 62CDE8 | `ParticleClass_Update_Fire` | kept | ✓ | 0062CB10 `ParticleClass::ParticleClass__AI_Fire` | decompiled/functions/1005CFA0.c | src/Misc/Bugfixes.cpp:911 | — |
| 62CEC9 | `ParticleClass_Draw_Details` | kept ⚠改过 | ✓ | 0062CEC0 `ParticleClass::ParticleClass__DrawIt` | decompiled/functions/1005E170.c | src/Misc/Bugfixes.Perf.cpp:59 | — |
| 65D8FB | `TeamTypeClass_ValidateHouse` | kept | ✓ | 0065EC30 `ChronoSphere__WarpUnitsAtCell` | decompiled/functions/1005D170.c | src/Misc/Bugfixes.cpp:888 | src/sim/team_script_vm/reinforcements.rs |
| 66BC71 | `Buf_CombatDamage` | kept | ✓ | 0066BBB0 `RulesClass::RulesClass__ReadCombatDamage` | decompiled/functions/1005BA50.c | src/Misc/Bugfixes.Buffer.cpp:159 | src/rules/combat_damage.rs, src/rules/native_processing.rs, src/rules/rules_type_references.rs |
| 66D55E | `Buf_General` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BB00.c | src/Misc/Bugfixes.Buffer.cpp:55 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66DB93 | `Buf_BridgeExplosions` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BA40.c | src/Misc/Bugfixes.Buffer.cpp:153 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66DD13 | `Buf_WeatherArt` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BE10.c | src/Misc/Bugfixes.Buffer.cpp:148 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F34B | `Buf_RepairBay` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDD0.c | src/Misc/Bugfixes.Buffer.cpp:138 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F589 | `Buf_Shipyard` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BE00.c | src/Misc/Bugfixes.Buffer.cpp:132 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66F7C0 | `Buf_PPA` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDB0.c | src/Misc/Bugfixes.Buffer.cpp:122 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 66FA13 | `Buf_SecretBoons` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BDF0.c | src/Misc/Bugfixes.Buffer.cpp:117 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 67062F | `Buf_AnimToInf_Paradrop` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005BA30.c | src/Misc/Bugfixes.Buffer.cpp:112 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 671152 | `RulesClass_Addition_General_PrismSupportModifier` | kept | ✓ | 0066D530 `RulesClass::RulesClass__ReadGeneral` | decompiled/functions/1005D040.c | src/Misc/Bugfixes.cpp:178 | src/rules/missile_spawn.rs, src/rules/rules_type_references.rs, src/rules/ruleset.rs |
| 672B0E | `Buf_AI` | kept | ✓ | 00672AE0 `RulesClass::RulesClass__ReadAI` | decompiled/functions/1005B830.c | src/Misc/Bugfixes.Buffer.cpp:179 | src/rules/object_type.rs, src/rules/rules_type_references.rs, src/sim/house_state.rs … |
| 69281E | `DisplayClass_ChooseAction_TogglePower` | kept | ✓ | 00692610 `DisplayClass::DisplayClass__DetermineAction` | decompiled/functions/1005C790.c | src/Misc/Bugfixes.cpp:1238 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/entity_pick.rs … |
| 699C1C | `Game_ParsePKTs_ClearFile` | kept | ✓ | 00699980 `SessionClass__ScanMultiplayerMapFiles` | decompiled/functions/1005CA80.c | src/Misc/Bugfixes.cpp:471 | src/assets/asset_manager.rs, src/map/scenario_sources.rs |
| 6A9348 | `StripClass_GetTip_FixLength` | replaces_0.A:CameoClass_GetTip_FixLength | ✓ | 006A92E0 `StripClass::StripClass__GetCameoTooltip` | decompiled/functions/1005BF00.c | src/Misc/Bugfixes.Buffer.cpp:345 | — |
| 6B7D50 | `SpawnManagerClass_CountDockedSpawns` | new_in_3.0 | ✓ | 006B7D50 `SpawnManagerClass::SpawnManagerClass__CountDockedSpawns` | decompiled/functions/1005D0A0.c | src/Misc/Bugfixes.cpp:1679 | src/app/presentation/instances/units.rs, src/sim/spawn_manager.rs |
| 6BB9DD | `WinMain_LogNonsense` | kept | ✓ | 006BB9A0 `WinMain` | decompiled/functions/1005D7C0.c | src/Misc/Bugfixes.cpp:145 | src/app/persistence/options_profile.rs, src/util/config.rs |
| 6BD7E3 | `Expand_MIX_Reorg` | kept | ✓ | 006BB9A0 `WinMain` | decompiled/functions/1005C950.c | src/Misc/Bugfixes.cpp:88 | src/app/persistence/options_profile.rs, src/util/config.rs |
| 6BE9BD | `sub_6BE1C0` | kept | ✓ | 006BE1C0 `AtExit_Free_Game_Resources` | decompiled/functions/1005D840.c | src/Misc/Bugfixes.cpp:106 | — |
| 6D7847 | `TacticalClass_DrawPixelEffects_Details` | kept ⚠改过 | ✓ | 006D7840 `Tactical::DrawPixelFXSparkles` | decompiled/functions/1005E1B0.c | src/Misc/Bugfixes.Perf.cpp:65 | src/app/presentation/render/build_instances.rs |
| 6E20D8 | `TActionClass_DestroyAttached_Loop` | new_in_3.0 | ✓ | 006E2050 `TActionClass::TActionClass__DestroyAttached` | decompiled/functions/1005D150.c | src/Misc/Bugfixes.cpp:1730 | src/sim/trigger_runtime.rs |
| 6F661D | `TechnoClass_DrawHealthBar_DestroyedBuilding_RedPip` | kept | ✓ | 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | decompiled/functions/1005D280.c | src/Misc/Bugfixes.cpp:947 | src/app/presentation/render/object_extras.rs, src/app/presentation/ui_overlays.rs |
| 6F7561 | `Arcing_Aircraft` | kept | ✓ | 006F7220 `TechnoClass::TechnoClass__InRange` | decompiled/functions/1005C450.c | src/Misc/Bugfixes.cpp:242 | src/rules/weapon_type.rs, src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs … |
| 6F90F8 | `TechnoClass_SelectAutoTarget_Demacroize` | new_in_3.0 | ✓ | 006F8DF0 `TechnoClass::TechnoClass__Greatest_Threat` | decompiled/functions/1005E210.c | src/Misc/Bugfixes.Perf.cpp:141 | src/map/cell_index.rs, src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs … |
| 6FA2C7 | `TechnoClass_Update_DrawHidden` | kept ⚠改过 | ✓ | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/1005D330.c | src/Misc/Bugfixes.cpp:1370 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 6FC339 | `TechnoClass_CanFire_OpenToppedGunnerTemporal` | replaces_0.A:TechnoClass_GetFireError_OpenToppedGunnerTemporal | ✓ | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/1005D200.c | src/Misc/Bugfixes.cpp:1299 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 6FCA30 | `TechnoClass_GetWeaponState` | kept | ✓ | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/1005D2B0.c | src/Misc/Bugfixes.cpp:170 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 6FCF53 | `TechnoClass_SetTarget_Burst` | new_in_3.0 | ✓ | 006FCDB0 `TechnoClass::TechnoClass__Assign_Target` | decompiled/functions/1005D320.c | src/Misc/Bugfixes.cpp:1719 | src/sim/aircraft/attack_mission.rs, src/sim/aircraft/idle_entry.rs, src/sim/aircraft/mod.rs … |
| 700E47 | `TechnoClass_CanDeploySlashUnload_Immobile` | kept | ✓ | 00700D50 `TechnoClass::TechnoClass__CanDeploySlashUnload` | decompiled/functions/1005D1C0.c | src/Misc/Bugfixes.cpp:1053 | src/sim/combat/rof.rs, src/sim/deploy.rs, src/sim/world/world_commands.rs |
| 70133E | `TechnoClass_GetWeaponRange_Demacroize` | new_in_3.0 | ✓ | 007012C0 `TechnoClass::TechnoClass__Get_Weapon_Range` | decompiled/functions/1005E1F0.c | src/Misc/Bugfixes.Perf.cpp:164 | src/sim/combat/combat_weapon.rs, src/sim/combat/threat_range.rs, src/sim/combat/world_fire.rs … |
| 707A47 | `TechnoClass_PointerGotInvalid_LastTarget` | new_in_3.0 | ✓ | 007077C0 `TechnoClass::TechnoClass__PointerExpired` | decompiled/functions/1005D2E0.c | src/Misc/Bugfixes.cpp:1001 | src/app/input/entity_pick.rs, src/sim/combat/aim_coord.rs, src/sim/combat/attack_target.rs … |
| 707B09 | `TechnoClass_PointerGotInvalid_ResetMindControl` | removed_in_3.0 | — | 007077C0 `TechnoClass::TechnoClass__PointerExpired` | decompiled/functions/1005D300.c | src/Misc/Bugfixes.cpp:1013 | src/app/input/entity_pick.rs, src/sim/combat/aim_coord.rs, src/sim/combat/attack_target.rs … |
| 707EEA | `TechnoClass_GetGuardRange_Demacroize` | new_in_3.0 | ✓ | 00707E60 `TechnoClass::TechnoClass__Threat_Range` | decompiled/functions/1005E1D0.c | src/Misc/Bugfixes.Perf.cpp:152 | src/sim/combat/greatest_threat.rs, src/sim/combat/threat_range.rs |
| 70CAD8 | `TechnoClass_DealParticleDamage_DontDestroyCliff` | kept | ✓ | 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | decompiled/functions/1005BF80.c | src/Misc/Bugfixes.Buffer.cpp:365 | — |
| 70CBDA | `TechnoClass_DealParticleDamage` | kept | ✓ | 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | decompiled/functions/1005D260.c | src/Misc/Bugfixes.cpp:904 | — |
| 7119D5 | `TechnoTypeClass_CTOR_NoInit_Particles` | new_in_3.0 | ✓ | 00711840 `TechnoTypeClass::TechnoTypeClass__Constructor_NoInit` | decompiled/functions/1005D350.c | src/Misc/Bugfixes.cpp:1383 | — |
| 7125DF | `TechnoTypeClass_LoadFromINI_ListLength` | kept | ✓ | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005BF90.c | src/Misc/Bugfixes.Buffer.cpp:213 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 713171 | `TechnoTypeClass_LoadFromINI_SkipLists1` | kept | ✓ | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005C020.c | src/Misc/Bugfixes.Buffer.cpp:231 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 713C10 | `TechnoTypeClass_LoadFromINI_SkipLists2` | kept | ✓ | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005C040.c | src/Misc/Bugfixes.Buffer.cpp:241 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 715857 | `TechnoTypeClass_LoadFromINI_LimitPalettes` | kept | ✓ | 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | decompiled/functions/1005D380.c | src/Misc/Bugfixes.cpp:112 | src/rules/art_data.rs, src/rules/ini_parser.rs, src/rules/ini_value.rs … |
| 716D98 | `TechnoTypeClass_Load_Palette` | new_in_3.0 | ✓ | 007162F0 `TechnoTypeClass::TechnoTypeClass__Load` | decompiled/functions/1005D390.c | src/Misc/Bugfixes.cpp:1644 | src/rules/art_data.rs |
| 717823 | `TechnoTypeClass_UpdatePalette_Reset` | new_in_3.0 | ✓ | 00717840 `FUN_00717840` | decompiled/functions/1005D3C0.c | src/Misc/Bugfixes.cpp:1632 | — |
| 71810D | `TeleportLocomotionClass_ILocomotion_MoveTo_Deactivated` | kept | ✓ | 00718100 `TeleportLocomotionClass__Move_To` | decompiled/functions/1005D3E0.c | src/Misc/Bugfixes.cpp:968 | src/sim/movement/infantry_scatter.rs, src/sim/movement/teleport_movement.rs, src/sim/movement/track_path.rs |
| 718871 | `TeleportLocomotionClass_UnfreezeObject_SinkOrSwim` | kept | ✓ | 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | decompiled/functions/1005D420.c | src/Misc/Bugfixes.cpp:578 | src/sim/movement/teleport_chrono.rs |
| 71AA52 | `TemporalClass_Update_AnnounceInvalidPointer` | kept | ✓ | 0071A760 `TemporalClass::TemporalClass__Update` | decompiled/functions/1005D470.c | src/Misc/Bugfixes.cpp:792 | src/sim/temporal.rs |
| 71AF2B | `TemporalClass_Fire_UnwarpableA` | kept | ✓ | 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | decompiled/functions/1005D460.c | src/Misc/Bugfixes.cpp:125 | src/sim/projectile.rs, src/sim/temporal.rs |
| 720A61 | `sub_7209D0_NoLog` | kept | ✓ | 007209D0 `ThemeClass::ThemeClass__AI` | decompiled/functions/1005D850.c | src/Misc/Bugfixes.cpp:1041 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 720DE8 | `Theme_Stop_NoLog` | kept | ✓ | 00720BB0 `ThemeClass::ThemeClass__Play` | decompiled/functions/1005D490.c | src/Misc/Bugfixes.cpp:1030 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 720F37 | `sub_720EA0_NoLog` | kept | ✓ | 00720EA0 `ThemeClass::ThemeClass__Stop` | decompiled/functions/1005D860.c | src/Misc/Bugfixes.cpp:1036 | src/app/audio_runtime.rs, src/audio/theme.rs |
| 725A1F | `AnnounceInvalidPointer_SkipBehind` | new_in_3.0 | ✓ | 007258D0 `DispatchPointerExpiredCleanup` | decompiled/functions/1005E040.c | src/Misc/Bugfixes.Perf.cpp:132 | src/sim/combat/combat_aoe.rs, src/sim/team_script_vm.rs, src/sim/trigger_runtime.rs … |
| 7272B5 | `TriggerTypeClass_LoadFromINI_House` | new_in_3.0 | ✓ | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005E2B0.c | src/Misc/Bugfixes.Safeguards.cpp:32 | src/map/triggers.rs |
| 7274AF | `TriggerTypeClass_LoadFromINI_Read_Events` | kept | ✓ | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:287 | src/map/triggers.rs |
| 7274C8 | `TriggerTypeClass_LoadFromINI_Strtok_Events` | kept | ✓ | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BEF0.c | src/Misc/Bugfixes.Buffer.cpp:294 | src/map/triggers.rs |
| 727529 | `TriggerTypeClass_LoadFromINI_Read_Actions` | kept | ✓ | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005BED0.c | src/Misc/Bugfixes.Buffer.cpp:301 | src/map/triggers.rs |
| 727544 | `TriggerTypeClass_LoadFromINI_Strtok_Actions` | kept | ✓ | 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | decompiled/functions/1005C070.c | src/Misc/Bugfixes.Buffer.cpp:308 | src/map/triggers.rs |
| 72920C | `TunnelLocomotionClass_Turning` | new_in_3.0 | ✓ | 007291F0 `TunnelLocomotionClass::TunnelLocomotionClass__ProcessPreDigIn` | decompiled/functions/1005D500.c | src/Misc/Bugfixes.cpp:1461 | — |
| 730DB0 | `GuardCommandClass_Execute` | kept | ✓ | 00730D60 `FUN_00730d60` | decompiled/functions/1005CAF0.c | src/Misc/Bugfixes.cpp:513 | src/sim/world/world_commands.rs |
| 730EE5 | `StopCommandClass_Execute_Berzerk` | kept | ✓ | 00730EA0 `Selection__IssueStopCommand` | decompiled/functions/1005D110.c | src/Misc/Bugfixes.cpp:1119 | src/app/input/dispatch.rs |
| 737CE4 | `UnitClass_ReceiveDamage_ShipyardRepair` | kept | ✓ | 00737C90 `UnitClass::UnitClass__ReceiveDamage` | decompiled/functions/1005D5E0.c | src/Misc/Bugfixes.cpp:1178 | src/sim/combat/destruction_effects.rs, src/sim/combat/receiver_call.rs, src/sim/combat/unit_damage_response.rs … |
| 7387D1 | `UnitClass_Destroy_ShakeScreenZero` | new_in_3.0 | ✓ | 00738680 `UnitClass::UnitClass__Death_Explosion` | decompiled/functions/1005D530.c | src/Misc/Bugfixes.cpp:466 | src/rules/effect_asset_catalog.rs, src/rules/object_type.rs, src/sim/combat/destruction_effects.rs … |
| 73D81E | `UnitClass_Mi_Unload_LastPassenger` | kept | ✓ | 0073D630 `UnitClass::UnitClass__Mission_Unload` | decompiled/functions/1005D5B0.c | src/Misc/Bugfixes.cpp:1110 | src/rules/object_type.rs, src/sim/components.rs, src/sim/deploy.rs … |
| 73F7DD | `UnitClass_IsCellOccupied_Bib` | new_in_3.0 | ✓ | 0073F0A0 `UnitClass::UnitClass__Can_Enter_Cell` | decompiled/functions/1005D550.c | src/Misc/Bugfixes.cpp:1397 | src/sim/combat/combat_weapon.rs, src/sim/movement/foot_path.rs, src/sim/movement/fresh_oracle_seam.rs … |
| 7440BD | `UnitClass_Remove` | kept | ✓ | 007440B0 `UnitClass::UnitClass__Limbo` | decompiled/functions/1005D610.c | src/Misc/Bugfixes.cpp:478 | src/sim/production/production_sell.rs |
| 749088 | `FixedWidthCounter_ResetWithGivenCount` | replaces_0.A:Count_ResetWithGivenCount | ✓ | 00749060 `UnitTrackerClass__PopulateUnitCount` | decompiled/functions/1005C9C0.c | src/Misc/Bugfixes.cpp:873 | — |
| 74A884 | `VoxelAnimClass_Update_Damage` | new_in_3.0 | ✓ | 00749F30 `VoxelAnimClass::VoxelAnimClass__AI` | decompiled/functions/1005D640.c | src/Misc/Bugfixes.cpp:1521 | src/sim/voxel_anim.rs, src/sim/world/world_hash.rs |
| 75D660 | `WarheadTypeClass_LoadFromINI_ListLength` | kept | ✓ | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/1005C080.c | src/Misc/Bugfixes.Buffer.cpp:253 | src/rules/warhead_type.rs |
| 75DAE6 | `WarheadTypeClass_LoadFromINI_SkipLists` | kept | ✓ | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/1005C0E0.c | src/Misc/Bugfixes.Buffer.cpp:267 | src/rules/warhead_type.rs |
| 75F46E | `WaveClass_DamageCell_Wall` | new_in_3.0 | ✓ | 0075F330 `WaveClass::WaveClass__DamageArea` | decompiled/functions/1005D7A0.c | src/Misc/Bugfixes.cpp:1669 | src/sim/world/mod.rs |
| 772462 | `WeaponTypeClass_LoadFromINI_ListLength` | kept | ✓ | 00772080 `WeaponTypeClass::WeaponTypeClass__ReadINI` | decompiled/functions/1005C0F0.c | src/Misc/Bugfixes.Buffer.cpp:274 | src/rules/mod.rs, src/rules/native_processing.rs, src/rules/ruleset.rs … |
| 7BB445 | `XSurface_20` | new_in_3.0 | ✓ | 007BB350 `XSurface::XSurface__DrawEllipse` | decompiled/functions/1005D7D0.c | src/Misc/Bugfixes.cpp:1595 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |
| 10003950 | `CCINIClass::IsBlank` | 被钩子调用 | decompiled/functions/10003950.c |
| 10061960 | `Debug::INIParseFailed` | 被钩子调用 | decompiled/functions/10061960.c |
| 1005B810 | `CellClass::CellClass__Tile_Is_DestroyableCliff` | 被钩子调用 | decompiled/functions/1005B810.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1005B700 | `MapClass::DestroyCliff` | 被钩子调用 | decompiled/functions/1005B700.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 1005C120 | `MixFileClass::Bootstrap` | 被钩子调用 | decompiled/functions/1005C120.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |
| 10003880 | `INIClass::Clear` | 被钩子调用 | decompiled/functions/10003880.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 10003660 | `Ares::InitOwnResources` | 被钩子调用 | decompiled/functions/10003660.c |
| 100097B0 | `TechnoClass::SetFocus` | 被钩子调用 | decompiled/functions/100097B0.c |
| 1002C9E0 | `SHPStruct::~SHPStruct` | 被钩子调用 | decompiled/functions/1002C9E0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 1005C170 | `TeamTypeClass::GetHouse` | 被钩子调用 | decompiled/functions/1005C170.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 1005C130 | `BuildingClass::ClearBunker` | 被钩子调用 | decompiled/functions/1005C130.c |
| 1005C150 | `BounceClass::GetCoords` | 被钩子调用 | decompiled/functions/1005C150.c |
| 100263F0 | `TacticalClass::AdjustForZ` | 被钩子调用 | decompiled/functions/100263F0.c |
| 10007130 | `MouseCursorTypes::InitDefaults` | 被钩子调用 | decompiled/functions/10007130.c |
| 100038B0 | `Ares::UninitOwnResources` | 被钩子调用 | decompiled/functions/100038B0.c |
| 1002A6C0 | `RulesExt::DetailsCurrentlyEnabled` | 被钩子调用 | decompiled/functions/1002A6C0.c |
| 1005DFE0 | `AbstractClass::AnnounceExpiredPointer` | 被钩子调用 | decompiled/functions/1005DFE0.c |
| 100617A0 | `Debug::FatalError` | 被钩子调用 | decompiled/functions/100617A0.c |
| 100617C0 | `Debug::FatalErrorAndExit` | 被钩子调用 | decompiled/functions/100617C0.c |

## 涉及的热点函数（56）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00489280 `Apply_area_damage` | 14 | R.tiberium, U.bugfixes, W.filter, ext:Rules, ext:WarheadType |
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | 7 | T.emp, U.bugfixes, W.ivanbomb, ext:Building, ext:Techno |
| 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | 7 | U.bugfixes, W.death, W.ivanbomb, ext:Bullet, ext:WarheadType |
| 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | 7 | F.armor, T.ammo, U.bugfixes, W.ivanbomb, ext:Techno |
| 0071A760 `TemporalClass::TemporalClass__Update` | 6 | U.bugfixes, ext:Building, ext:Techno, ext:WarheadType, ext:WeaponType |
| 0073D630 `UnitClass::UnitClass__Mission_Unload` | 5 | T.poweredby, U.bugfixes, ext:Building, ext:House, ext:Techno |
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno |
| 007258D0 `DispatchPointerExpiredCleanup` | 4 | U.bugfixes, W.ivanbomb, ext:Ares.Ext, ext:ParticleSystem |
| 0052BA60 `Init_Game` | 5 | U.bugfixes, U.shell, ext:Ares |
| 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | 5 | F.armor, U.bugfixes, ext:WarheadType |
| 00423AC0 `AnimClass::AnimClass__AI` | 4 | T.misc, U.bugfixes, ext:AnimType |
| 006BB9A0 `WinMain` | 4 | F.csf, U.bugfixes, ext:Ares.GlobalControls |
| 00413F80 `AircraftClass::AircraftClass__InitFromType` | 3 | B.academy, U.bugfixes, ext:Techno |
| 004C6CB0 `EventClass::EventClass__Execute` | 3 | S.core, U.bugfixes, ext:Network |
| 006A92E0 `StripClass::StripClass__GetCameoTooltip` | 3 | S.core, T.cameo, U.bugfixes |
| 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | 3 | B.firestorm, U.bugfixes, ext:Techno |
| 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | 3 | B.prism, U.bugfixes, ext:WarheadType |
| 00737C90 `UnitClass::UnitClass__ReceiveDamage` | 3 | U.bugfixes, ext:Techno, ext:WarheadType |
| 0073F0A0 `UnitClass::UnitClass__Can_Enter_Cell` | 3 | B.firestorm, U.bugfixes, ext:Building |
| 00712170 `TechnoTypeClass::TechnoTypeClass__ReadINI` | 9 | U.bugfixes, ext:TechnoType |
| 00420F40 `AlphaShapeClass__DrawAll_WithMask` | 4 | T.misc, U.bugfixes |
| 00772080 `WeaponTypeClass::WeaponTypeClass__ReadINI` | 4 | U.bugfixes, ext:WeaponType |
| 004585C0 `BuildingClass::BuildingClass__KillOccupants` | 3 | B.trench, U.bugfixes |
| 00511850 `HouseTypeClass::HouseTypeClass__ReadINI` | 3 | H.country, U.bugfixes |
| 006153E0 `OwnerDraw_Static_006153E0` | 3 | U.bugfixes, U.shell |
| 006F7220 `TechnoClass::TechnoClass__InRange` | 3 | U.bugfixes, W.weaponmisc |
| 006F8DF0 `TechnoClass::TechnoClass__Greatest_Threat` | 3 | U.bugfixes, ext:Techno |
| 007077C0 `TechnoClass::TechnoClass__PointerExpired` | 3 | U.bugfixes, ext:Techno |
| 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | 3 | U.bugfixes, W.weaponmisc |
| 007162F0 `TechnoTypeClass::TechnoTypeClass__Load` | 3 | U.bugfixes, ext:TechnoType |
| 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | 3 | S.core, U.bugfixes |
| 00414BB0 `AircraftClass::AircraftClass__AI` | 2 | T.misc, U.bugfixes |
| 0041D860 `AirstrikeClass::AirstrikeClass__StartStrikeOnTarget` | 2 | U.bugfixes, ext:Techno |
| 00422CA0 `AnimClass::AnimClass__DrawIt` | 2 | U.bugfixes, ext:AnimType |
| 00442C40 `BuildingClass::BuildingClass__Init_Managers` | 2 | B.academy, U.bugfixes |
| 00442D90 `BuildingClass::BuildingClass__SpawnSurvivors` | 2 | U.bugfixes, ext:Building |
| 0048A4F0 `Warhead__SelectExplosionAnim` | 2 | S.core, U.bugfixes |
| 004C9FF0 `FactoryClass::FactoryClass__AbandonProduction` | 2 | T.factory, U.bugfixes |
| 004D9E70 `FootClass::FootClass__CanBeSold` | 2 | U.bugfixes, ext:Techno |
| 00509140 `HouseClass::HouseClass__Update_Factory_Queue` | 2 | T.factory, U.bugfixes |
| 0051DF10 `InfantryClass::InfantryClass__Limbo` | 2 | U.bugfixes, ext:Infantry |
| 0051F6E0 `InfantryClass::InfantryClass__Mission_Unload` | 2 | U.bugfixes, ext:Techno |
| 005206B0 `InfantryClass::InfantryClass__Fire_At_Target` | 2 | U.bugfixes, ext:Techno |
| 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | 2 | T.cloak, U.bugfixes |
| 0062BD50 `ParticleClass::ParticleClass__AI_Gas` | 2 | U.bugfixes, ext:ParticleType |
| 0062CEC0 `ParticleClass::ParticleClass__DrawIt` | 2 | U.bugfixes, ext:ParticleType |
| 0065D8E0 `TeamTypeClass__SpawnReinforcements` | 2 | U.bugfixes, ext:Team |
| 00692610 `DisplayClass::DisplayClass__DetermineAction` | 2 | F.cursors, U.bugfixes |
| 00700D50 `TechnoClass::TechnoClass__CanDeploySlashUnload` | 2 | U.bugfixes, ext:Techno |
| 007291F0 `TunnelLocomotionClass::TunnelLocomotionClass__ProcessPreDigIn` | 2 | T.misc, U.bugfixes |
| 00738680 `UnitClass::UnitClass__Death_Explosion` | 2 | U.bugfixes, ext:Techno |
| 0075F330 `WaveClass::WaveClass__DamageArea` | 2 | U.bugfixes, W.beams |
| 0066D530 `RulesClass::RulesClass__ReadGeneral` | 9 | U.bugfixes |
| 00417FE0 `AircraftClass::AircraftClass__Mission_Attack` | 6 | U.bugfixes |
| 00727240 `TriggerTypeClass::TriggerTypeClass__ReadINI` | 5 | U.bugfixes |
