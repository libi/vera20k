# `T.misc` 单位杂项：炮塔速度、Gattling、部署朝向、空速、Assaulter 等

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（15）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AirRate` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:395 | check_0A_use | — |
| `AlternateTheaterArt` | TechnoType | — | — | src/Ext/TechnoType/Hooks.Art.cpp:30 | unscanned | — |
| `Assaulter.Level` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:460 | check_0A_use | — |
| `CanPassiveAquire.Guard` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:477 | check_0A_use | — |
| `DeployDir` | TechnoType | Nullable<Direction::Value> | (empty) | src/Ext/TechnoType/Body.cpp:546 | ok | src/rules/ruleset.rs |
| `EVA.Lost` | TechnoType | ValueableIdx<VoxClass> | -1 | src/Ext/TechnoType/Body.cpp:417 | check_0A_use | — |
| `FakeOf` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:545 | no_use_found | — |
| `Gattling.Cycle` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:374 | check_0A_use | — |
| `Saboteur` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:474 | ok | — |
| `SecretLab.ForbiddenHouses` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:199 | unscanned | — |
| `SecretLab.RequiredHouses` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:196 | unscanned | — |
| `TurretROT` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:528 | ok | src/rules/native_processing.rs |
| `VoiceAirstrikeAbort` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:318 | no_use_found | — |
| `VoiceAirstrikeAttack` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:317 | no_use_found | — |
| `WaterImage` | TechnoType | — | — | src/Ext/TechnoType/Hooks.cpp:64 | unscanned | — |

## 钩子（42）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 413FFA | `AircraftClass_Init_TurretROT` | new_in_3.0 | 00413F80 `AircraftClass::AircraftClass__InitFromType` | decompiled/functions/10048AC0.c | src/Recovered/AircraftClass.cpp:94 | — |
| 415085 | `AircraftClass_Update_DamageSmoke` | kept | 00414BB0 `AircraftClass::AircraftClass__AI` | decompiled/functions/10047B60.c | src/Ext/Techno/Hooks.Art.cpp:156 | src/sim/aircraft/mod.rs, src/sim/world/techno_ai.rs |
| 416C3A | `AircraftClass_Carryall_Unload_Facing` | new_in_3.0 | 00416AF0 `AircraftClass::AircraftClass__Carryall_Pickup` | decompiled/functions/10048920.c | src/Recovered/AircraftClass.cpp:108 | — |
| 420960 | `AlphaShapeClass_CTOR` | kept | 00420960 `AlphaShapeClass::AlphaShapeClass__Constructor` | decompiled/functions/100474B0.c | src/Ext/Techno/Hooks.Alpha.cpp:16 | — |
| 420A71 | `AlphaShapeClass_CTOR_Anims` | kept | 00420960 `AlphaShapeClass::AlphaShapeClass__Constructor` | decompiled/functions/100475A0.c | src/Ext/Techno/Hooks.Alpha.cpp:29 | — |
| 420F75 | `AlphaLightClass_UpdateScreen_ShouldDraw` | kept | 00420F40 `AlphaShapeClass__DrawAll_WithMask` | decompiled/functions/10047470.c | src/Ext/Techno/Hooks.Alpha.cpp:163 | — |
| 4210AC | `AlphaLightClass_UpdateScreen_Header` | kept | 00420F40 `AlphaShapeClass__DrawAll_WithMask` | decompiled/functions/10047440.c | src/Ext/Techno/Hooks.Alpha.cpp:178 | — |
| 4211AC | `AlphaLightClass_UpdateScreen_Body` | kept | 00420F40 `AlphaShapeClass__DrawAll_WithMask` | decompiled/functions/10047400.c | src/Ext/Techno/Hooks.Alpha.cpp:190 | — |
| 421371 | `TacticalClass_UpdateAlphasInRectangle_ShouldDraw` | kept | 00421350 `AlphaShapeClass__DrawAll_NoMask` | decompiled/functions/100477D0.c | src/Ext/Techno/Hooks.Alpha.cpp:204 | — |
| 42146E | `TacticalClass_UpdateAlphasInRectangle_Header` | kept | 00421350 `AlphaShapeClass__DrawAll_NoMask` | decompiled/functions/10047780.c | src/Ext/Techno/Hooks.Alpha.cpp:220 | — |
| 42152C | `TacticalClass_UpdateAlphasInRectangle_Body` | kept | 00421350 `AlphaShapeClass__DrawAll_NoMask` | decompiled/functions/10047740.c | src/Ext/Techno/Hooks.Alpha.cpp:236 | — |
| 421730 | `AlphaShapeClass_SDDTOR` | kept | 00421730 `AlphaShapeClass::AlphaShapeClass__ScalarDeletingDestructor` | decompiled/functions/10047610.c | src/Ext/Techno/Hooks.Alpha.cpp:38 | — |
| 421798 | `AlphaShapeClass_SDDTOR_Anims` | kept | 00421730 `AlphaShapeClass::AlphaShapeClass__ScalarDeletingDestructor` | decompiled/functions/10047670.c | src/Ext/Techno/Hooks.Alpha.cpp:45 | — |
| 423B0B | `AnimClass_Update_AlphaLight` | kept | 00423AC0 `AnimClass::AnimClass__AI` | decompiled/functions/100476C0.c | src/Ext/Techno/Hooks.Alpha.cpp:153 | src/sim/anim_class.rs, src/sim/bounce.rs, src/sim/radiation.rs |
| 44D760 | `BuildingClass_Destroyed_UnitLost` | new_in_3.0 | 0044D760 `BuildingClass::BuildingClass__Death_Announcement` | decompiled/functions/10013F10.c | src/Recovered/BuildingClass.cpp:251 | src/sim/temporal.rs |
| 457DB7 | `BuildingClass_CanBeOccupied_SpecificAssaulters` | kept | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10016FD0.c | src/Ext/Building/Hooks.Trenches.cpp:58 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |
| 4D98C0 | `FootClass_Destroyed` | kept | 004D98C0 `FootClass::FootClass__Destroyed` | decompiled/functions/10049080.c | src/Ext/Techno/Hooks.cpp:1248 | src/rules/object_type.rs, src/sim/combat/effects.rs, src/sim/combat/world_receiver.rs … |
| 4D9920 | `FootClass_SelectAutoTarget_Cloaked` | kept | 004D9920 `FootClass::FootClass__Greatest_Threat` | decompiled/functions/100491E0.c | src/Ext/Techno/Hooks.cpp:1570 | src/sim/aircraft/hunt_mission.rs, src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs … |
| 4DA8B2 | `FootClass_Update_AnimRate` | kept ⚠改过 | 004DA530 `FootClass::FootClass__AI` | decompiled/functions/100493D0.c | src/Ext/Techno/Hooks.cpp:66 | src/sim/aircraft/mod.rs |
| 514A21 | `HoverLocomotionClass_ILocomotion_Process_DeployToLand` | new_in_3.0 | 00514310 `HoverLocomotionClass__Process` | decompiled/functions/10049550.c | src/Recovered/HoverLocomotionClass.cpp:23 | src/sim/movement/hover_process.rs |
| 54C767 | `JumpjetLocomotionClass_State4_54C550_DeployDir` | new_in_3.0 | 0054C550 `JumpjetLocomotionClass::JumpjetLocomotionClass__State4_Descend` | decompiled/functions/100499F0.c | src/Recovered/JumpjetLocomotionClass.cpp:22 | src/sim/movement/jumpjet_flight.rs |
| 5673A0 | `MapClass_RevealArea0` | kept | 005673A0 `MapClass::MapClass__RevealShroud` | decompiled/functions/1004E8C0.c | src/Ext/Techno/Hooks.Sight.cpp:4 | src/sim/vision/mod.rs, src/sim/world/shroud_refresh.rs |
| 5678E0 | `MapClass_RevealArea1` | kept | 005678E0 `MapClass::MapClass__RevealArea` | decompiled/functions/1004E950.c | src/Ext/Techno/Hooks.Sight.cpp:23 | src/rules/ruleset.rs, src/sim/aircraft/fire_at.rs, src/sim/superweapon/psychic_reveal.rs … |
| 567DA0 | `MapClass_RevealArea2` | kept | 00567DA0 `MapClass::MapClass__UpdateFogBorder` | decompiled/functions/1004E9C0.c | src/Ext/Techno/Hooks.Sight.cpp:41 | src/sim/building_art.rs, src/sim/vision/cell_reveal.rs, src/sim/world/fly_landing.rs … |
| 5F3D65 | `ObjectClass_DTOR` | kept | 005F3B80 `ObjectClass::ObjectClass__Destructor` | decompiled/functions/100476E0.c | src/Ext/Techno/Hooks.Alpha.cpp:52 | src/sim/world/lifecycle.rs |
| 5F3E70 | `ObjectClass_Update_AlphaLight` | kept | 005F3E70 `ObjectClass::ObjectClass__AI` | decompiled/functions/10047730.c | src/Ext/Techno/Hooks.Alpha.cpp:146 | src/sim/movement/parachute_descent.rs, src/sim/world/object_sound.rs |
| 5F9070 | `ObjectTypeClass_Load2DArt` | kept | 005F9070 `ObjectTypeClass::ObjectTypeClass__LoadSHPImage` | decompiled/functions/10041040.c | src/Ext/TechnoType/Hooks.Art.cpp:37 | — |
| 5F9634 | `ObjectTypeClass_LoadFromINI` | kept | 005F92D0 `ObjectTypeClass::ObjectTypeClass__ReadINI` | decompiled/functions/100413B0.c | src/Ext/TechnoType/Hooks.Art.cpp:20 | src/rules/alpha_image_assets.rs, src/rules/ini_value.rs, src/rules/mod.rs … |
| 5F96B0 | `ObjectTypeClass_TheaterSpecificID` | kept | 005F96B0 `Theater__ApplyNamingConvention` | decompiled/functions/10041400.c | src/Ext/TechnoType/Hooks.Art.cpp:113 | src/render/sprite_atlas.rs, src/rules/buildup_asset_catalog.rs |
| 6FF2D1 | `TechnoClass_Fire_Facings` | kept | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10047CF0.c | src/Ext/Techno/Hooks.Art.cpp:206 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 705FF3 | `TechnoClass_Draw_A_SHP_File_SkipUnitShadow` | kept | 00705E00 `TechnoClass::TechnoClass_DrawSHP` | decompiled/functions/10047CD0.c | src/Ext/Techno/Hooks.Art.cpp:69 | src/app/presentation/instances/fogged.rs, src/app/presentation/instances/foot_depth.rs, src/app/presentation/instances/shp.rs … |
| 70DEBA | `TechnoClass_UpdateGattling_Cycle` | kept | 0070DE70 `TechnoClass::TechnoClass__IncreaseGattlingStage` | decompiled/functions/1004B6C0.c | src/Ext/Techno/Hooks.cpp:1032 | src/sim/combat/gattling.rs |
| 71944E | `TeleportLocomotionClass_ILocomotion_Process` | kept | 007192F0 `TeleportLocomotionClass__Process` | decompiled/functions/10047810.c | src/Ext/Techno/Hooks.Alpha.cpp:249 | src/render/draw_state.rs, src/sim/movement/teleport_chrono.rs, src/sim/movement/teleport_movement.rs … |
| 735584 | `UnitClass_CTOR_TurretROT` | new_in_3.0 | 007353C0 `UnitClass::UnitClass__Constructor` | decompiled/functions/1004BC40.c | src/Recovered/UnitClass.cpp:341 | — |
| 739B8A | `UnitClass_SimpleDeploy_Facing` | new_in_3.0 | 00739AC0 `UnitClass::UnitClass__UpdateSimpleDeployState` | decompiled/functions/1004C2D0.c | src/Recovered/UnitClass.cpp:308 | src/sim/unit_simple_deploy.rs, src/sim/world/mod.rs |
| 73B4A0 | `UnitClass_DrawVXL_WaterType` | kept | 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | decompiled/functions/100481E0.c | src/Ext/Techno/Hooks.Art.cpp:96 | src/app/presentation/instances/units.rs, src/render/vxl_native.rs, src/rules/voxel_frame_catalog.rs |
| 73C5FC | `UnitClass_DrawSHP_WaterType` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10048190.c | src/Ext/Techno/Hooks.Art.cpp:123 | — |
| 73C613 | `UnitClass_DrawSHP_FacingsA` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10047FE0.c | src/Ext/Techno/Hooks.Art.cpp:177 | — |
| 73C655 | `UnitClass_DrawSHP_ChangeType1` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10047E80.c | src/Ext/Techno/Hooks.Art.cpp:141 | — |
| 73C725 | `UnitClass_DrawSHP_DrawShadowEarlier` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10047EA0.c | src/Ext/Techno/Hooks.Art.cpp:12 | — |
| 73C733 | `UnitClass_DrawSHP_SkipTurretedShadow` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10048180.c | src/Ext/Techno/Hooks.Art.cpp:64 | — |
| 73CD01 | `UnitClass_DrawSHP_FacingsB` | kept | 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | decompiled/functions/10048150.c | src/Ext/Techno/Hooks.Art.cpp:194 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100252D0 | BuildingTypeExt::IsSabotagable [translated] | 使用键 | decompiled/functions/100252D0.c |
| 1003DEC0 | TechnoTypeExt::ExtMap.LoadFromINI [source_0.A,inferred] | 使用键 | decompiled/functions/1003DEC0.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 100091C0 | `INIClass::GetSection` | 被钩子调用 | decompiled/functions/100091C0.c |
| 100039A0 | `AresCRT::strCopy` | 被钩子调用 | decompiled/functions/100039A0.c |
| 1002C9E0 | `SHPStruct::~SHPStruct` | 被钩子调用 | decompiled/functions/1002C9E0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 100431A0 | `TechnoExt::ExtData::AlphaFrame` | 被钩子调用 | decompiled/functions/100431A0.c |
| 100445B0 | `TechnoExt::ExtData::DrawVisualFX` | 被钩子调用 | decompiled/functions/100445B0.c |
| 100473E0 | `SHPStruct::GetFrameBounds` | 被钩子调用 | decompiled/functions/100473E0.c |
| 10009EE0 | `TacticalClass::CoordsToClient` | 被钩子调用 | decompiled/functions/10009EE0.c |
| 10009F10 | `TacticalClass::RegisterDirtyArea` | 被钩子调用 | decompiled/functions/10009F10.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10045340 | `TechnoExt::ExtData::GetUnitType` | 被钩子调用 | decompiled/functions/10045340.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 100487F0 | `AudioController::DTOR_1` | 被钩子调用 | decompiled/functions/100487F0.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |

## 涉及的热点函数（7）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType, recovered:TechnoClass |
| 0073B470 `UnitClass::UnitClass__DrawVoxelBody` | 7 | T.misc, T.poweredby, recovered:UnitClass |
| 00423AC0 `AnimClass::AnimClass__AI` | 4 | T.misc, U.bugfixes, recovered:AnimClass |
| 00420F40 `AlphaShapeClass__DrawAll_WithMask` | 4 | T.misc, recovered:AlphaShapeClass |
| 00414BB0 `AircraftClass::AircraftClass__AI` | 2 | T.misc, U.bugfixes |
| 005F3E70 `ObjectClass::ObjectClass__AI` | 2 | T.misc, recovered:ObjectClass |
| 0073C5F0 `UnitClass::UnitClass__Draw_Body_And_Turret` | 8 | T.misc |
