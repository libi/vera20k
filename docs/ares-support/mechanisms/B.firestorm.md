# `B.firestorm` 火风暴墙

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Firestorm.Wall` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:89 | ok | — |
| `DamageToFirestormDamageCoefficient` | Rules | Valueable<double> | 0.0 | src/Ext/Rules/Body.cpp:243 | no_use_found | — |
| `FirestormActiveAnim` | Rules | Valueable<AnimTypeClass*> | nullptr | src/Ext/Rules/Body.cpp:238 | ok | — |
| `FirestormAirAnim` | Rules | Valueable<AnimTypeClass*> | nullptr | src/Ext/Rules/Body.cpp:241 | no_use_found | — |
| `FirestormGroundAnim` | Rules | Valueable<AnimTypeClass*> | nullptr | src/Ext/Rules/Body.cpp:240 | no_use_found | — |
| `FirestormIdleAnim` | Rules | Valueable<AnimTypeClass*> | nullptr | src/Ext/Rules/Body.cpp:239 | ok | — |
| `FirestormWarhead` | Rules | Nullable<WarheadTypeClass*> | (empty) | src/Ext/Rules/Body.cpp:242 | ok | — |

## 钩子（18）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 43EFB3 | `BuildingClass_GetStaticImageFrame` | kept | ✓ | 0043EF90 `BuildingClass::BuildingClass__GetCurrentFrame` | decompiled/functions/100149E0.c | src/Ext/Building/Hooks.Firewall.cpp:91 | src/app/presentation/instances/shp.rs, src/render/sprite_atlas.rs, src/sim/building_art.rs … |
| 440378 | `BuildingClass_Update_FirestormWall` | kept | ✓ | 0043FB20 `BuildingClass::BuildingClass__Update` | decompiled/functions/10014B40.c | src/Ext/Building/Hooks.Firewall.cpp:157 | src/sim/combat/world_receiver.rs, src/sim/world/bridge_orchestrator.rs, src/sim/world/building_working_sound.rs … |
| 440D01 | `BuildingClass_Put_FirestormWall` | kept | ✓ | 00440580 `BuildingClass::BuildingClass__Unlimbo` | decompiled/functions/10014A70.c | src/Ext/Building/Hooks.Firewall.cpp:133 | src/sim/base_plan.rs, src/sim/world/lifecycle.rs |
| 4423E7 | `BuildingClass_ReceiveDamage_FSW` | kept | ✓ | 00442230 `BuildingClass::BuildingClass__ReceiveDamage` | decompiled/functions/10014A90.c | src/Ext/Building/Hooks.Firewall.cpp:107 | src/audio/events.rs, src/rules/ruleset.rs, src/rules/warhead_type.rs … |
| 445355 | `BuildingClass_KickOutUnit_Firewall` | kept | ✓ | 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | decompiled/functions/10014A50.c | src/Ext/Building/Hooks.Firewall.cpp:59 | src/sim/ai_base_building.rs, src/sim/base_plan.rs, src/sim/game_entity.rs … |
| 445DF4 | `BuildingClass_Remove_FirestormWall` | kept | ✓ | 00445880 `BuildingClass::BuildingClass__Limbo` | decompiled/functions/10014A70.c | src/Ext/Building/Hooks.Firewall.cpp:145 | src/sim/base_plan.rs, src/sim/sensor_lifecycle.rs, src/sim/world/lifecycle.rs |
| 449518 | `BuildingClass_IsSellable_FirestormWall` | new_in_3.0 | ✓ | 004494C0 `BuildingClass::BuildingClass__CanSell` | decompiled/functions/10014A20.c | src/Ext/Building/Hooks.Firewall.cpp:266 | src/rules/buildup_asset_catalog.rs, src/sim/production/production_sell.rs |
| 483D94 | `CellClass_UpdatePassability` | replaces_0.A:CellClass_Setup_Slave | ✓ | 00483C80 `CellClass::CellClass__RecalcZoneType` | decompiled/functions/10014B70.c | src/Ext/Building/Hooks.Firewall.cpp:177 | src/map/resolved_terrain.rs |
| 4DA53E | `FootClass_Update` | kept | ✓ | 004DA530 `FootClass::FootClass__AI` | decompiled/functions/10014BA0.c | src/Ext/Building/Hooks.Firewall.cpp:279 | src/sim/aircraft/mod.rs |
| 4FB257 | `HouseClass_UnitFromFactory_Firewall` | kept | ✓ | 004FB0E0 `HouseClass::HouseClass__Place_Production` | decompiled/functions/10014D50.c | src/Ext/Building/Hooks.Firewall.cpp:46 | src/rules/ruleset.rs, src/sim/production/production_types.rs, src/sim/production/wall_placement.rs … |
| 51BD4C | `InfantryClass_Update` | kept ⚠改过 | ✓ | 0051BAB0 `InfantryClass::InfantryClass__AI` | decompiled/functions/10014DB0.c | src/Ext/Building/Hooks.Firewall.cpp:194 | src/sim/mission/authority.rs, src/sim/movement/infantry_action.rs, src/sim/world/infantry_death.rs … |
| 51C4C8 | `InfantryClass_IsCellOccupied` | kept ⚠改过 | ✓ | 0051BF90 `InfantryClass::InfantryClass__Can_Enter_Cell` | decompiled/functions/10014D70.c | src/Ext/Building/Hooks.Firewall.cpp:219 | src/sim/aircraft/drop_payload.rs, src/sim/movement/foot_path.rs, src/sim/movement/infantry_action.rs … |
| 5880A0 | `MapClass_FindFirstFirestorm` | kept | ✓ | 005880A0 `MapClass::MapClass__FindFirstFirestorm` | decompiled/functions/10014DF0.c | src/Ext/Building/Hooks.Firewall.cpp:17 | — |
| 6D5455 | `sub_6D5030` | kept ⚠改过 | ✓ | 006D5030 `Tactical::BuildingPlacement_OverlayRenderer` | decompiled/functions/10015140.c | src/Ext/Building/Hooks.Firewall.cpp:73 | src/app/presentation/render/draw_passes.rs, src/render/selection_overlay.rs |
| 6D5A5C | `sub_6D59D0` | kept ⚠改过 | ✓ | 006D59D0 `FirestormWall_PlacementShadow` | decompiled/functions/10015170.c | src/Ext/Building/Hooks.Firewall.cpp:82 | src/app/presentation/placement_preview.rs |
| 6F64CB | `TechnoClass_DrawHealthBar_FirestormWall` | kept ⚠改过 | ✓ | 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | decompiled/functions/100150D0.c | src/Ext/Building/Hooks.Firewall.cpp:272 | src/app/presentation/render/object_extras.rs, src/app/presentation/ui_overlays.rs |
| 6FF008 | `TechnoClass_Fire_FSW` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10024C00.c | src/Ext/House/Hooks.Firewall.cpp:13 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 73F7B0 | `UnitClass_IsCellOccupied` | kept ⚠改过 | ✓ | 0073F0A0 `UnitClass::UnitClass__Can_Enter_Cell` | decompiled/functions/10015100.c | src/Ext/Building/Hooks.Firewall.cpp:244 | src/sim/combat/combat_weapon.rs, src/sim/movement/foot_path.rs, src/sim/movement/fresh_oracle_seam.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000D770 | BuildingTypeExt::ExtData::IsLinkable [translated] | 使用键 | decompiled/functions/1000D770.c |
| 100113D0 | BuildingExt::buildLines [source_0.A,3.0✓] | 使用键 | decompiled/functions/100113D0.c |
| 10011DB0 | BuildingExt::GetImageFrameIndex [source_0.A,3.0✓] | 使用键 | decompiled/functions/10011DB0.c |
| 10011F30 | BuildingExt::ExtData::ImmolateVictim [source_0.A,3.0✓] | 使用键 | decompiled/functions/10011F30.c |
| 10012770 | BuildingExt::IsActiveFirestormWall [translated] | 使用键 | decompiled/functions/10012770.c |
| 10013280 | BuildingExt::ExtData::UpdateFirewall [translated] | 使用键 | decompiled/functions/10013280.c |
| 10013630 | BuildingExt::ExtData::UpdateFirewallLinks [source_0.A,3.0✓] | 使用键 | decompiled/functions/10013630.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10011670 | `CellClass::CanThisExistHere` | 被钩子调用 | decompiled/functions/10011670.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10012CE0 | `Randomizer::Random` | 被钩子调用 | decompiled/functions/10012CE0.c |
| 10011DE0 | `CellClass::GetNeighbourCell` | 被钩子调用 | decompiled/functions/10011DE0.c |
| 100130A0 | `CellClass::Setup` | 被钩子调用 | decompiled/functions/100130A0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10003430 | `HouseClass::FindSuperWeapon` | 被钩子调用 | decompiled/functions/10003430.c |
| 100149C0 | `CellClass::GetContainedTiberiumIndex` | 被钩子调用 | decompiled/functions/100149C0.c |
| 10050820 | `TiberiumExt::ExtData::GetHealDelay` | 被钩子调用 | decompiled/functions/10050820.c |
| 10050840 | `TiberiumExt::ExtData::GetHealStep` | 被钩子调用 | decompiled/functions/10050840.c |
| 10019FE0 | `MapClass::FindFirstFirestorm` | 被钩子调用 | decompiled/functions/10019FE0.c |

## 涉及的热点函数（14）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, W.beams, W.weaponmisc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType |
| 0043FB20 `BuildingClass::BuildingClass__Update` | 6 | B.firestorm, T.ammo, T.jammer, ext:Building, ext:BuildingType |
| 00443C60 `BuildingClass::BuildingClass__ExitObject_Main` | 11 | B.firestorm, T.factory, ext:Building, ext:TechnoType |
| 00440580 `BuildingClass::BuildingClass__Unlimbo` | 5 | B.firestorm, B.gates, T.spotlight, ext:Building |
| 00442230 `BuildingClass::BuildingClass__ReceiveDamage` | 4 | B.firestorm, B.prism, ext:Building, ext:WarheadType |
| 00445880 `BuildingClass::BuildingClass__Limbo` | 3 | B.academy, B.firestorm, ext:Building |
| 006F64A0 `TechnoClass::TechnoClass__DrawHealthBar` | 3 | B.firestorm, U.bugfixes, ext:Techno |
| 0073F0A0 `UnitClass::UnitClass__Can_Enter_Cell` | 3 | B.firestorm, U.bugfixes, ext:Building |
| 004DA530 `FootClass::FootClass__AI` | 4 | B.firestorm, ext:Techno |
| 006D5030 `Tactical::BuildingPlacement_OverlayRenderer` | 3 | B.firestorm, B.foundation |
| 004494C0 `BuildingClass::BuildingClass__CanSell` | 2 | B.firestorm, B.trench |
| 004FB0E0 `HouseClass::HouseClass__Place_Production` | 2 | B.firestorm, ext:BuildingType |
| 0051BAB0 `InfantryClass::InfantryClass__AI` | 2 | B.firestorm, T.ammo |
| 0051BF90 `InfantryClass::InfantryClass__Can_Enter_Cell` | 2 | B.firestorm, ext:Infantry |
