# `S.core` 超武通用：类型、显示、文本、EVA、光照、动画、声音、次数

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | F.readers | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（60）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SuperWeapons` | BuildingType | ValueableIdxVector<SuperWeaponTypeClass> | (empty) | src/Ext/BuildingType/Body.cpp:303 | check_0A_use | src/app/frontend/cncnet_spawn.rs, src/render/sidebar_cameo_atlas.rs, src/rules/native_processing.rs … |
| `Action` | SWType | — | — | src/Ext/SWType/Body.cpp:78 | unscanned | src/app/diagnostics/shell_capture/loading_runtime.rs, src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/abort_return.rs … |
| `EVA.Activated` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:115 | no_use_found | src/rules/superweapon_type.rs |
| `EVA.Detected` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:116 | no_use_found | src/rules/superweapon_type.rs |
| `EVA.Impatient` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:117 | no_use_found | — |
| `EVA.Ready` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:114 | no_use_found | src/rules/superweapon_type.rs |
| `EVA.SelectTarget` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:119 | no_use_found | — |
| `Light.Ambient` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:183 | check_0A_use | — |
| `Light.Blue` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:186 | check_0A_use | — |
| `Light.Enabled` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:182 | check_0A_use | — |
| `Light.Green` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:185 | check_0A_use | — |
| `Light.Red` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:184 | check_0A_use | — |
| `Message.Abort` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:196 | no_use_found | — |
| `Message.Activate` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:195 | check_0A_use | — |
| `Message.CannotFire` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:198 | check_0A_use | src/rules/superweapon_type.rs |
| `Message.Color` | SWType | ValueableIdx<ColorScheme>::Read | — | src/Ext/SWType/Body.cpp:190 | check_0A_use | src/rules/native_processing.rs |
| `Message.Detected` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:192 | no_use_found | — |
| `Message.FirerColor` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:189 | check_0A_use | src/rules/superweapon_type.rs |
| `Message.Launch` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:194 | check_0A_use | — |
| `Message.Ready` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:193 | no_use_found | — |
| `SidebarPalette` | SWType | FUN_10076160 | — | src/Ext/SWType/Body.cpp:247 | no_use_found | — |
| `SidebarPCX` | SWType | AresPCXFile | — | src/Ext/SWType/Body.cpp:249 | no_use_found | src/rules/superweapon_type.rs |
| `SW.ActivationSound` | SWType | NullableIdx<VocClass> | — | src/Ext/SWType/Body.cpp:136 | check_0A_use | src/rules/ruleset.rs |
| `SW.AffectsHouse` | SWType | — | — | src/Ext/SWType/Body.cpp:142 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.AffectsTarget` | SWType | — | — | src/Misc/SWTypes/ChronoSphere.cpp:82 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.AlwaysGranted` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:143 | ok | — |
| `SW.Animation` | SWType | Nullable<AnimTypeClass *> | — | src/Ext/SWType/Body.cpp:138 | no_use_found | src/rules/native_processing.rs |
| `SW.AnimationHeight` | SWType | Valueable<int> | 0 | src/Ext/SWType/Body.cpp:139 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.AnimationVisibility` | SWType | — | — | src/Ext/SWType/Body.cpp:141 | unscanned | src/rules/superweapon_type.rs |
| `SW.AnyDesignator` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:212 | no_use_found | src/rules/native_processing.rs |
| `SW.AnyInhibitor` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:216 | no_use_found | src/rules/native_processing.rs |
| `SW.CreateRadarEvent` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:124 | check_0A_use | — |
| `SW.Damage` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:165 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.Deferment` | SWType | Nullable<int> | — | src/Ext/SWType/Body.cpp:157 | check_0A_use | src/rules/native_processing.rs |
| `SW.Designators` | SWType | ValueableVector<TechnoTypeClass*> | (empty) | src/Ext/SWType/Body.cpp:211 | no_use_found | src/rules/native_processing.rs |
| `SW.FireIntoShroud` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:121 | ok | src/rules/superweapon_targeting.rs |
| `SW.Group` | SWType | Valueable<int> | 0 | src/Ext/SWType/Body.cpp:227 | ok | — |
| `SW.Inhibitors` | SWType | ValueableVector<TechnoTypeClass*> | (empty) | src/Ext/SWType/Body.cpp:215 | no_use_found | src/rules/native_processing.rs |
| `SW.InitialReady` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:229 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.ManualFire` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:123 | check_0A_use | src/rules/superweapon_targeting.rs |
| `SW.MaxCount` | SWType | Valueable<int> | -1 | src/Ext/SWType/Body.cpp:155 | check_0A_use | src/rules/native_processing.rs |
| `SW.PostDependent` | SWType | — | — | src/Ext/SWType/Body.cpp:219 | unscanned | — |
| `SW.Range` | SWType | — | — | src/Ext/SWType/Body.cpp:167 | unscanned | — |
| `SW.RangeMaximum` | SWType | Valueable<double> | -1.0 | src/Ext/SWType/Body.cpp:208 | no_use_found | src/rules/superweapon_targeting.rs |
| `SW.RangeMinimum` | SWType | Valueable<double> | -1.0 | src/Ext/SWType/Body.cpp:207 | no_use_found | src/rules/superweapon_targeting.rs |
| `SW.RequiresHouse` | SWType | — | — | src/Ext/SWType/Body.cpp:151 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.RequiresTarget` | SWType | — | — | src/Ext/SWType/Body.cpp:150 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.Shots` | SWType | Valueable<int> | -1 | src/Ext/SWType/Body.cpp:228 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.ShowCameo` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:125 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.Sound` | SWType | NullableIdx<VocClass> | — | src/Ext/SWType/Body.cpp:135 | no_use_found | src/rules/ruleset.rs |
| `SW.TimerVisibility` | SWType | — | — | src/Ext/SWType/Body.cpp:129 | unscanned | — |
| `SW.Unstoppable` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:126 | check_0A_use | — |
| `SW.VirtualCharge` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:230 | ok | src/rules/superweapon_type.rs |
| `SW.Warhead` | SWType | Nullable<WarheadTypeClass *> | — | src/Ext/SWType/Body.cpp:164 | check_0A_use | src/rules/native_processing.rs |
| `Text.Active` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:204 | check_0A_use | — |
| `Text.Charging` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:203 | check_0A_use | — |
| `Text.Hold` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:202 | check_0A_use | — |
| `Text.Preparing` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:200 | check_0A_use | — |
| `Text.Ready` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:201 | check_0A_use | — |
| `Type` | SWType | — | — | src/Ext/SWType/Body.cpp:82 | unscanned | src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/lan/live.rs, src/app/diagnostics/shell_capture/movies/lan/live/battle.rs … |

## 钩子（55）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 43BE50 | `BuildingClass_DTOR_HasAnySW` | kept ⚠改过 | 0043BCF0 `BuildingClass::BuildingClass__Destructor` | decompiled/functions/10034980.c | src/Ext/SWType/Hooks.cpp:251 | — |
| 4463F0 | `BuildingClass_Place_SuperWeaponAnimsA` | kept | 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | decompiled/functions/10034A60.c | src/Ext/SWType/Hooks.cpp:117 | src/app/match_runtime/eva_producers.rs, src/sim/building_art_storage.rs, src/sim/building_art_super.rs … |
| 44656D | `BuildingClass_Place_SuperWeaponAnimsB` | kept | 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | decompiled/functions/10034AE0.c | src/Ext/SWType/Hooks.cpp:130 | src/app/match_runtime/eva_producers.rs, src/sim/building_art_storage.rs, src/sim/building_art_super.rs … |
| 4468F4 | `BuildingClass_Place_AnnounceSW` | kept | 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | decompiled/functions/100349D0.c | src/Ext/SWType/Hooks.cpp:160 | src/app/match_runtime/eva_producers.rs, src/sim/building_art_storage.rs, src/sim/building_art_super.rs … |
| 446AAF | `BuildingClass_Place_SkipFreeUnits` | kept | 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | decompiled/functions/10036860.c | src/Ext/SWType/Hooks.Legacy.cpp:838 | src/app/match_runtime/eva_producers.rs, src/sim/building_art_storage.rs, src/sim/building_art_super.rs … |
| 449716 | `BuildingClass_Mi_Guard_HasFirstSW` | kept ⚠改过 | 004496B0 `BuildingClass::BuildingClass__Mission_Guard` | decompiled/functions/100349B0.c | src/Ext/SWType/Hooks.cpp:257 | src/rules/mission_data.rs, src/sim/world/techno_ai/building_missions.rs |
| 44CCE7 | `BuildingClass_Mi_Missile_GenericSW` | kept | 0044C980 `BuildingClass::BuildingClass__Mission_Missile` | decompiled/functions/10036570.c | src/Ext/SWType/Hooks.Legacy.cpp:879 | src/rules/object_type.rs, src/sim/superweapon/nuke.rs, src/sim/world/techno_ai/building_missile.rs |
| 44CE46 | `BuildingClass_Mi_Missile_Pulsball` | kept | 0044C980 `BuildingClass::BuildingClass__Mission_Missile` | decompiled/functions/10036790.c | src/Ext/SWType/Hooks.Legacy.cpp:854 | src/rules/object_type.rs, src/sim/superweapon/nuke.rs, src/sim/world/techno_ai/building_missile.rs |
| 450F9E | `BuildingClass_ProcessAnims_SuperWeaponsA` | kept | 004509D0 `BuildingClass::BuildingClass__UpdateAnimation` | decompiled/functions/10034AF0.c | src/Ext/SWType/Hooks.cpp:135 | src/sim/building_art_admission.rs, src/sim/building_art_super.rs, src/sim/building_construction.rs … |
| 451132 | `BuildingClass_ProcessAnims_SuperWeaponsB` | kept | 004509D0 `BuildingClass::BuildingClass__UpdateAnimation` | decompiled/functions/10034B60.c | src/Ext/SWType/Hooks.cpp:154 | src/sim/building_art_admission.rs, src/sim/building_art_super.rs, src/sim/building_construction.rs … |
| 457630 | `BuildingClass_SWAvailable` | kept | 00457630 `BuildingClass::BuildingClass__GetSuperWeaponIndex1` | decompiled/functions/10034BB0.c | src/Ext/SWType/Hooks.cpp:237 | src/sim/superweapon/mod.rs |
| 457690 | `BuildingClass_SW2Available` | kept | 00457690 `BuildingClass::BuildingClass__GetSuperWeaponIndex2` | decompiled/functions/10034B70.c | src/Ext/SWType/Hooks.cpp:244 | src/sim/superweapon/mod.rs, src/sim/world/lifecycle.rs |
| 467E59 | `BulletClass_Update_NukeBall` | kept ⚠改过 | 004666E0 `BulletClass::BulletClass__AI` | decompiled/functions/10036960.c | src/Ext/SWType/Hooks.Legacy.cpp:749 | src/sim/combat/projectile_delivery.rs, src/sim/projectile.rs, src/sim/world/lifecycle.rs … |
| 46B371 | `BulletClass_NukeMaker` | kept | 0046B310 `BulletClass::BulletClass__NukeMaker` | decompiled/functions/10036880.c | src/Ext/SWType/Hooks.Legacy.cpp:700 | src/sim/combat/world_receiver.rs, src/sim/projectile.rs, src/sim/superweapon/nuke.rs |
| 46B423 | `BulletClass_NukeMaker_PropagateSW` | kept | 0046B310 `BulletClass::BulletClass__NukeMaker` | decompiled/functions/10036930.c | src/Ext/SWType/Hooks.Legacy.cpp:737 | src/sim/combat/world_receiver.rs, src/sim/projectile.rs, src/sim/superweapon/nuke.rs |
| 48A59A | `MapClass_SelectDamageAnimation_LightningWarhead` | kept | 0048A4F0 `Warhead__SelectExplosionAnim` | decompiled/functions/100377E0.c | src/Ext/SWType/Hooks.Legacy.cpp:675 | src/rules/effect_asset_catalog.rs, src/sim/combat/detonation_anim.rs, src/sim/crates/effects.rs … |
| 4C78D6 | `Networking_RespondToEvent_SpecialPlace` | new_in_3.0 | 004C6CB0 `EventClass::EventClass__Execute` | decompiled/functions/10038E90.c | src/Recovered/Networking.cpp:18 | src/sim/command.rs, src/sim/radio/tether.rs, src/sim/world/world_commands.rs |
| 4F9004 | `HouseClass_Update_TrySWFire` | kept | 004F8440 `HouseClass::HouseClass__Update` | decompiled/functions/10034CD0.c | src/Ext/SWType/Hooks.cpp:318 | src/app/match_runtime/scenario_exit.rs, src/app/presentation/sidebar_gadgets.rs, src/sim/house_eva.rs … |
| 4FAE72 | `HouseClass_SWFire_PreDependent` | kept | 004FAE50 `HouseClass::HouseClass__Fire_SW` | decompiled/functions/10034C90.c | src/Ext/SWType/Hooks.cpp:262 | src/rules/superweapon_type.rs, src/sim/superweapon/ai_fire.rs, src/sim/superweapon/chronosphere.rs … |
| 4FAF2A | `HouseClass_SWDefendAgainst_Aborted` | kept | 004FAF00 `HouseClass::HouseClass__Super_Defense_Alert` | decompiled/functions/10034C70.c | src/Ext/SWType/Hooks.cpp:334 | src/sim/superweapon/fire.rs |
| 5098F0 | `HouseClass_Update_AI_TryFireSW` | kept | 005098F0 `HouseClass::HouseClass__AI_TryFireSW` | decompiled/functions/10038E10.c | src/Ext/SWType/Hooks.Targeting.cpp:643 | src/sim/house_state.rs, src/sim/house_strategy.rs, src/sim/superweapon/ai_fire.rs |
| 50AF10 | `HouseClass_UpdateSuperWeaponsOwned` | kept | 0050AF10 `HouseClass::HouseClass__Update_Owned_Supers` | decompiled/functions/10039450.c | src/Ext/SWType/Hooks.Update.cpp:101 | src/app/match_runtime/super_selection.rs, src/sim/world/mod.rs |
| 50B1D0 | `HouseClass_UpdateSuperWeaponsUnavailable` | kept | 0050B1D0 `HouseClass::HouseClass__Grant_Provided_Supers` | decompiled/functions/10039580.c | src/Ext/SWType/Hooks.Update.cpp:154 | src/sim/ai_base_building.rs |
| 50CFAA | `HouseClass_PickOffensiveSWTarget` | kept | 0050CBF0 `HouseClass::HouseClass__AI_FindBestRallyTarget` | decompiled/functions/10034C50.c | src/Ext/SWType/Hooks.cpp:212 | src/rules/ruleset.rs, src/sim/superweapon/ai_fire.rs |
| 539EB0 | `LightningStorm_Start` | kept | 00539EB0 `LightningStorm__Start` | decompiled/functions/10036B00.c | src/Ext/SWType/Hooks.Legacy.cpp:209 | src/app/match_runtime/sound_dispatch.rs, src/audio/events.rs, src/sim/superweapon/lightning_storm.rs … |
| 53A140 | `LightningStorm_Strike` | kept | 0053A140 `LightningStorm__CreateCloudBolt` | decompiled/functions/10036D60.c | src/Ext/SWType/Hooks.Legacy.cpp:516 | src/sim/superweapon/lightning_storm.rs |
| 53A300 | `LightningStorm_Strike2` | kept | 0053A300 `LightningStorm__GroundStrike` | decompiled/functions/10036F50.c | src/Ext/SWType/Hooks.Legacy.cpp:564 | src/sim/superweapon/lightning_storm.rs |
| 53A6CF | `LightningStorm_Update` | kept ⚠改过 | 0053A6C0 `LightningStorm__Process` | decompiled/functions/10037330.c | src/Ext/SWType/Hooks.Legacy.cpp:306 | src/rules/ruleset.rs, src/sim/anim_class.rs, src/sim/scenario_session.rs … |
| 53AF40 | `PsyDom_Update` | kept | 0053AF40 `PsychicDominator__Process` | decompiled/functions/10037B40.c | src/Ext/SWType/Hooks.Legacy.cpp:204 | src/rules/ruleset.rs, src/sim/anim_class.rs, src/sim/superweapon/psychic_dominator.rs |
| 53B080 | `PsyDom_Fire` | kept | 0053B080 `PsychicDominator__MindControlArea` | decompiled/functions/10037840.c | src/Ext/SWType/Hooks.Legacy.cpp:21 | src/rules/effect_asset_catalog.rs, src/rules/mind_control_rules.rs, src/rules/ruleset.rs … |
| 53C280 | `ScenarioClass_UpdateLighting` | kept | 0053C280 `ScenarioClass__UpdateLighting` | decompiled/functions/10037B50.c | src/Ext/SWType/Hooks.Legacy.cpp:162 | src/app/presentation/lighting.rs, src/map/lighting.rs, src/sim/light_sources.rs … |
| 555E50 | `LightConvertClass_CTOR_Lighting` | kept | 00555DA0 `LightConvertClass::LightConvertClass__Constructor` | decompiled/functions/10036A80.c | src/Ext/SWType/Hooks.Legacy.cpp:180 | src/sim/superweapon/nuke.rs |
| 653B3A | `RadarClass_GetMouseAction_CustomSWAction` | kept | 006539D0 `RadarClass_RTacticalClass__Action` | decompiled/functions/10034D00.c | src/Ext/SWType/Hooks.cpp:12 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/radar.rs … |
| 6AAEDF | `SidebarClass_ProcessCameoClick_SuperWeapons` | kept ⚠改过 | 006AAD00 `SelectClass__Action` | decompiled/functions/10034DA0.c | src/Ext/SWType/Hooks.cpp:63 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AAF9D | `SidebarClass_ProcessCameoClick_SelectTarget` | kept | 006AAD00 `SelectClass__Action` | decompiled/functions/10034D60.c | src/Ext/SWType/Hooks.cpp:101 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6CB4D0 | `SuperClass_SetOnHold` | kept | 006CB4D0 `SuperClass::SuperClass__Suspend` | decompiled/functions/10035750.c | src/Ext/SWType/Hooks.cpp:476 | src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CB70C | `SuperClass_Grant_InitialReady` | new_in_3.0 | 006CB560 `SuperClass::SuperClass__Grant` | decompiled/functions/100354E0.c | src/Recovered/SuperClass.cpp:85 | src/app/match_runtime/eva_producers.rs |
| 6CB7B0 | `SuperClass_Lose` | kept | 006CB7B0 `SuperClass::SuperClass__Deactivate` | decompiled/functions/10035610.c | src/Ext/SWType/Hooks.cpp:351 | src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CB920 | `SuperClass_ClickFire` | kept | 006CB920 `SuperClass::SuperClass__ClickFire` | decompiled/functions/10035130.c | src/Ext/SWType/Hooks.cpp:371 | src/sim/superweapon/fire.rs, src/sim/superweapon/mod.rs |
| 6CBCDE | `SuperClass_Update_Animation` | kept | 006CBCA0 `SuperClass::SuperClass__AI` | decompiled/functions/10035870.c | src/Ext/SWType/Hooks.cpp:546 | src/app/match_runtime/eva_producers.rs, src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CBD6B | `SuperClass_Update_DrainMoney` | kept | 006CBCA0 `SuperClass::SuperClass__AI` | decompiled/functions/100358A0.c | src/Ext/SWType/Hooks.cpp:513 | src/app/match_runtime/eva_producers.rs, src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CBD86 | `SuperClass_Progress_Charged` | kept | 006CBCA0 `SuperClass::SuperClass__AI` | decompiled/functions/10035730.c | src/Ext/SWType/Hooks.cpp:343 | src/app/match_runtime/eva_producers.rs, src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CBDD7 | `SuperClass_AnnounceReady` | kept | 006CBCA0 `SuperClass::SuperClass__AI` | decompiled/functions/100350D0.c | src/Ext/SWType/Hooks.cpp:182 | src/app/match_runtime/eva_producers.rs, src/sim/superweapon/mod.rs, src/sim/world/mod.rs |
| 6CC0EA | `SuperClass_AnnounceQuantity` | kept | 006CC080 `SuperClass::SuperClass__Forced_Charge` | decompiled/functions/10035070.c | src/Ext/SWType/Hooks.cpp:197 | — |
| 6CC2B0 | `SuperClass_NameReadiness` | kept | 006CC2B0 `SuperClass::SuperClass__NameReadiness` | decompiled/functions/100356A0.c | src/Ext/SWType/Hooks.cpp:278 | src/app/presentation/sidebar_build.rs |
| 6CC390 | `SuperClass_Launch` | kept | 006CC390 `SuperClass::SuperClass__Launch` | decompiled/functions/100355C0.c | src/Ext/SWType/Hooks.cpp:224 | src/app/match_runtime/sound_dispatch.rs, src/audio/events.rs, src/rules/object_type.rs … |
| 6CE6F6 | `SuperWeaponTypeClass_CTOR` | kept | 006CE5B0 `SuperWeaponTypeClass::SuperWeaponTypeClass__Constructor` | decompiled/functions/10034550.c | src/Ext/SWType/Body.cpp:969 | src/rules/superweapon_type.rs |
| 6CE800 | `SuperWeaponTypeClass_SaveLoad_Prefix` | kept | 006CE8D0 `SuperWeaponTypeClass__Save` | decompiled/functions/10034720.c | src/Ext/SWType/Body.cpp:985 | — |
| 6CE8BE | `SuperWeaponTypeClass_Load_Suffix` | kept | 006CE800 `SuperWeaponTypeClass__Load` | decompiled/functions/100345E0.c | src/Ext/SWType/Body.cpp:996 | — |
| 6CE8EA | `SuperWeaponTypeClass_Save_Suffix` | kept | 006CE8D0 `SuperWeaponTypeClass__Save` | decompiled/functions/10034760.c | src/Ext/SWType/Body.cpp:1002 | — |
| 6CEE43 | `SuperWeaponTypeClass_LoadFromINI` | kept | 006CEA20 `SuperWeaponTypeClass::SuperWeaponTypeClass__ReadINI` | decompiled/functions/100345B0.c | src/Ext/SWType/Body.cpp:1008 | src/rules/superweapon_type.rs |
| 6CEEB0 | `SuperWeaponTypeClass_FindFirstOfAction` | kept | 006CEEB0 `SuperWeaponTypeClass__FindFirstOfAction` | decompiled/functions/100359A0.c | src/Ext/SWType/Hooks.cpp:556 | src/app/input/radar.rs, src/sim/superweapon/mod.rs |
| 6CEFE0 | `SuperWeaponTypeClass_SDDTOR` | kept | 006CEFE0 `SuperWeaponTypeClass::SuperWeaponTypeClass__ScalarDeletingDestructor` | decompiled/functions/100346E0.c | src/Ext/SWType/Body.cpp:977 | — |
| 7187DA | `TeleportLocomotionClass_Unwarp_PreventSelfCrush` | kept | 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | decompiled/functions/10037BC0.c | src/Ext/SWType/Hooks.Legacy.cpp:810 | src/sim/movement/teleport_chrono.rs |
| 7188F2 | `TeleportLocomotionClass_Unwarp_SinkJumpJets` | kept | 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | decompiled/functions/10037BE0.c | src/Ext/SWType/Hooks.Legacy.cpp:817 | src/sim/movement/teleport_chrono.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000D700 | BuildingTypeExt::ExtData::GetSuperWeaponCount [source_0.A] | 使用键 | decompiled/functions/1000D700.c |
| 100326B0 | SWTypeExt::ExtData::GetDamage [source_0.A] | 使用键 | decompiled/functions/100326B0.c |
| 10032700 | SWTypeExt::GetLightingColor [source_0.A] | 使用键 | decompiled/functions/10032700.c |
| 10032B40 | SWTypeExt::ExtData::GetWarhead [source_0.A] | 使用键 | decompiled/functions/10032B40.c |
| 10032B90 | SWTypeExt::ExtData::GetChargeToDrainRatio [source_0.A] | 使用键 | decompiled/functions/10032B90.c |
| 10033BD0 | SWTypeExt::ExtData::PrintMessage [source_0.A] | 使用键 | decompiled/functions/10033BD0.c |
| 10033EB0 | SWTypeExt::ExtData::GetAction [translated] | 使用键 | decompiled/functions/10033EB0.c |
| 10038F10 | FUN_10038f10 [translated] | 使用键 | decompiled/functions/10038F10.c |
| 10039EF0 | (file-local)::ScriptIronCurtain [translated] | 使用键 | decompiled/functions/10039EF0.c |
| 1006FB60 | FUN_1006fb60 [source_0.A] | 使用键 | decompiled/functions/1006FB60.c |
| 100707C0 | FUN_100707c0 [source_0.A] | 使用键 | decompiled/functions/100707C0.c |
| 10070BA0 | FUN_10070ba0 [source_0.A] | 使用键 | decompiled/functions/10070BA0.c |
| 100711C0 | FUN_100711c0 [source_0.A] | 使用键 | decompiled/functions/100711C0.c |
| 10072850 | FUN_10072850 [source_0.A] | 使用键 | decompiled/functions/10072850.c |
| 10072B00 | FUN_10072b00 [source_0.A] | 使用键 | decompiled/functions/10072B00.c |
| 10075920 | FUN_10075920 [source_0.A] | 使用键 | decompiled/functions/10075920.c |
| 10075DE0 | FUN_10075de0 [translated] | 使用键 | decompiled/functions/10075DE0.c |
| 10032D20 | `NukeFlash::IsFadingIn` | 被钩子调用 | decompiled/functions/10032D20.c |
| 10021480 | `HouseExt::ExtData::GetSuperWeaponStatus` | 被钩子调用 | decompiled/functions/10021480.c |
| 100028C0 | `MessageListClass::PrintMessage` | 被钩子调用 | decompiled/functions/100028C0.c |
| 10026980 | `MapClass::IsLocationShrouded` | 被钩子调用 | decompiled/functions/10026980.c |
| 10057E40 | `Actions::Set` | 被钩子调用 | decompiled/functions/10057E40.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1000D610 | `BuildingExt::ExtData::GetFirstSuperWeapon` | 被钩子调用 | decompiled/functions/1000D610.c |
| 10034890 | `BuildingClass::FirstActiveSWIdx` | 被钩子调用 | decompiled/functions/10034890.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 10016300 | `ObjectClass::IsGreenHP` | 被钩子调用 | decompiled/functions/10016300.c |
| 10016340 | `BuildingClass::BuildingClass__SetAnimSlotImage` | 被钩子调用 | decompiled/functions/10016340.c |
| 100329E0 | `BuildingExt::ExtData::GetSuperWeaponIndex` | 被钩子调用 | decompiled/functions/100329E0.c |
| 10034840 | `HouseClass::AI_TryFireSW` | 被钩子调用 | decompiled/functions/10034840.c |
| 100348D0 | `SuperClass::CanFire` | 被钩子调用 | decompiled/functions/100348D0.c |
| 10032110 | `SWTypeExt::ExtData::CanFireAITargeted` | 被钩子调用 | decompiled/functions/10032110.c |
| 100328A0 | `SWTypeExt::ExtData::GetNewSWType` | 被钩子调用 | decompiled/functions/100328A0.c |
| 100348F0 | `SuperClass::Launch` | 被钩子调用 | decompiled/functions/100348F0.c |
| 10034910 | `SuperClass::Lose` | 被钩子调用 | decompiled/functions/10034910.c |
| 100348B0 | `SuperClass::GetRechargeTime` | 被钩子调用 | decompiled/functions/100348B0.c |
| 10034960 | `SuperClass::StopPreclickAnim` | 被钩子调用 | decompiled/functions/10034960.c |
| 100325D0 | `SWTypeExt::Deactivate` | 被钩子调用 | decompiled/functions/100325D0.c |
| 10022710 | `HouseExt::ExtData::RecordSuperWeaponGranted` | 被钩子调用 | decompiled/functions/10022710.c |
| 10032020 | `SWTypeExt::Activate` | 被钩子调用 | decompiled/functions/10032020.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
| 100130C0 | `HouseClass::TakeMoney` | 被钩子调用 | decompiled/functions/100130C0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 10032370 | `SWTypeExt::ChangeLighting` | 被钩子调用 | decompiled/functions/10032370.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 10036540 | `LightConvertClass::UpdateColors` | 被钩子调用 | decompiled/functions/10036540.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10032D30 | `SWTypeExt::ExtData::IsHouseAffected` | 被钩子调用 | decompiled/functions/10032D30.c |
| 10036410 | `HouseClass::CreateRadarOutage` | 被钩子调用 | decompiled/functions/10036410.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 10010090 | `VocClass::PlayGlobal` | 被钩子调用 | decompiled/functions/10010090.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10036430 | `CellClass::FindTechnoNearestTo` | 被钩子调用 | decompiled/functions/10036430.c |
| 10036470 | `NukeFlash::IsFadingOut` | 被钩子调用 | decompiled/functions/10036470.c |
| 10036530 | `PsyDom::Update` | 被钩子调用 | decompiled/functions/10036530.c |
| 10036520 | `ChronoScreenEffect::Update` | 被钩子调用 | decompiled/functions/10036520.c |
| 100364C0 | `LightningStorm::Strike2` | 被钩子调用 | decompiled/functions/100364C0.c |
| 100364E0 | `LightningStorm::Strike` | 被钩子调用 | decompiled/functions/100364E0.c |
| 10032980 | `SWTypeExt::ExtData::GetRange` | 被钩子调用 | decompiled/functions/10032980.c |
| 100363D0 | `LightningStorm::Start` | 被钩子调用 | decompiled/functions/100363D0.c |
| 10035FC0 | `IonBlastClass::IonBlastClass` | 被钩子调用 | decompiled/functions/10035FC0.c |
| 10035D90 | `Helpers::Alex::for_each_in_rect_or_spread` | 被钩子调用 | decompiled/functions/10035D90.c |
| 10035FE0 | `SWTypeExt::ExtData::IsTechnoAffected` | 被钩子调用 | decompiled/functions/10035FE0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10033E50 | `ScenarioClass::RecalcLighting` | 被钩子调用 | decompiled/functions/10033E50.c |
| 10036500 | `CellClass::CellClass__HasBridgeOverlay` | 被钩子调用 | decompiled/functions/10036500.c |
| 100382D0 | `SWTypeExt::TryFireAITargeted` | 被钩子调用 | decompiled/functions/100382D0.c |
| 100385A0 | `HouseClass::Fire_SW` | 被钩子调用 | decompiled/functions/100385A0.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 10039270 | `SuperClass::SetOnHold` | 被钩子调用 | decompiled/functions/10039270.c |
| 10011E00 | `SidebarClass::GetObjectTabIdx` | 被钩子调用 | decompiled/functions/10011E00.c |
| 10012DA0 | `SidebarClass::RepaintSidebar` | 被钩子调用 | decompiled/functions/10012DA0.c |
| 10011E40 | `SuperClass::Grant` | 被钩子调用 | decompiled/functions/10011E40.c |
| 100113B0 | `SidebarClass::AddCameo` | 被钩子调用 | decompiled/functions/100113B0.c |
| 10039ED0 | `TeamClass::TeamClass__Fetch_A_Leader` | 被钩子调用 | decompiled/functions/10039ED0.c |
| 100323C0 | `SWTypeExt::ClearChronoAnim` | 被钩子调用 | decompiled/functions/100323C0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 100708B0 | `PsyDom::Fire` | 被钩子调用 | decompiled/functions/100708B0.c |
| 1000AC10 | `BulletClass::Detonate` | 被钩子调用 | decompiled/functions/1000AC10.c |

## 涉及的热点函数（9）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00445F80 `BuildingClass::BuildingClass__OnConstructionComplete` | 9 | B.academy, S.core, ext:Building, ext:BuildingType, ext:Techno, recovered:BuildingClass |
| 006AAD00 `SelectClass__Action` | 16 | S.core, recovered:SelectClass, recovered:SidebarClass |
| 004F8440 `HouseClass::HouseClass__Update` | 7 | S.core, ext:House, recovered:HouseClass |
| 0044C980 `BuildingClass::BuildingClass__Mission_Missile` | 5 | S.core, ext:Building, recovered:BuildingClass |
| 004666E0 `BulletClass::BulletClass__AI` | 3 | S.core, ext:Bullet, recovered:BulletClass |
| 007187A0 `TeleportLocomotionClass::TeleportLocomotionClass__PostWarpValidation` | 3 | S.core, U.bugfixes |
| 0043BCF0 `BuildingClass::BuildingClass__Destructor` | 2 | S.core, ext:Building |
| 0048A4F0 `Warhead__SelectExplosionAnim` | 2 | S.core, U.bugfixes |
| 006539D0 `RadarClass_RTacticalClass__Action` | 2 | S.core, recovered:RadarClass |
