# `W.ivanbomb` 疯狂伊文炸弹

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（14）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `IvanBomb.AttachSound` | WeaponType | NullableIdx<VocClass> | (empty) | src/Ext/WeaponType/Body.cpp:203 | no_use_found | — |
| `IvanBomb.CanDetonateDeathBomb` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:207 | no_use_found | — |
| `IvanBomb.CanDetonateTimeBomb` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:206 | no_use_found | — |
| `IvanBomb.Damage` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:199 | no_use_found | — |
| `IvanBomb.DeathBomb` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:195 | no_use_found | — |
| `IvanBomb.DeathBombOnAllies` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:196 | no_use_found | — |
| `IvanBomb.Delay` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:200 | ok | — |
| `IvanBomb.DestroysBridges` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:197 | no_use_found | — |
| `IvanBomb.Detachable` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:198 | no_use_found | — |
| `IvanBomb.DetonateOnSell` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:208 | no_use_found | — |
| `IvanBomb.FlickerRate` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:201 | no_use_found | — |
| `IvanBomb.Image` | WeaponType | Nullable<SHPStruct *> | (empty) | src/Ext/WeaponType/Body.cpp:205 | no_use_found | — |
| `IvanBomb.TickingSound` | WeaponType | NullableIdx<VocClass> | (empty) | src/Ext/WeaponType/Body.cpp:202 | no_use_found | — |
| `IvanBomb.Warhead` | WeaponType | Nullable<WarheadTypeClass *> | (empty) | src/Ext/WeaponType/Body.cpp:204 | no_use_found | — |

## 钩子（20）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 438799 | `BombClass_Detonate1` | kept | ✓ | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/10056770.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:158 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438843 | `BombClass_Detonate2` | kept | ✓ | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/100567F0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:171 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438879 | `BombClass_Detonate3` | kept | ✓ | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/10056870.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:184 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438A00 | `BombClass_GetCurrentFrame` | kept | ✓ | 00438A00 `BombClass::BombClass__GetClockFrame` | decompiled/functions/100568C0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:61 | src/sim/bomb.rs |
| 438E86 | `BombListClass_Plant_AllTechnos` | kept | ✓ | 00438E70 `BombListClass::BombListClass__Attach` | decompiled/functions/10056A50.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:37 | src/sim/bomb.rs, src/sim/projectile.rs |
| 438FD7 | `BombListClass_Plant_AttachSound` | kept | ✓ | 00438E70 `BombListClass::BombListClass__Attach` | decompiled/functions/10056AA0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:51 | src/sim/bomb.rs, src/sim/projectile.rs |
| 4393F2 | `BombClass_SDDTOR` | kept | ✓ | 004393F0 `BombClass::BombClass__ScalarDeletingDestructor` | decompiled/functions/100569F0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:194 | — |
| 4471D5 | `BuildingClass_Sell_DetonateNoBuildup` | kept | ✓ | 00447110 `BuildingClass::BuildingClass__Sell_Back` | decompiled/functions/10056AD0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:314 | src/rules/buildup_asset_catalog.rs, src/rules/ruleset.rs, src/sim/production/production_sell.rs … |
| 44A1FF | `BuildingClass_Mi_Selling_DetonatePostBuildup` | kept | ✓ | 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | decompiled/functions/10056AB0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:324 | src/sim/game_entity/construction_stage.rs, src/sim/production/production_sell.rs, src/sim/slave_manager.rs … |
| 46934D | `BulletClass_DetonateAt_IvanBombs` | replaces_0.A:IvanBombs_Spread | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/10056AF0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:217 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 4D9F7B | `FootClass_Sell_Detonate` | kept | ✓ | 004D9F70 `FootClass::FootClass__OnSold` | decompiled/functions/10056AD0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:312 | src/app/match_runtime/eva_producers.rs, src/sim/docking/building_dock.rs, src/sim/production/production_sell.rs |
| 51E488 | `InfantryClass_GetActionOnObject2` | replaces_0.A:InfantryClass_GetCursorOverObject2 | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10056C00.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:142 | src/app/input/context_order.rs |
| 51F1D8 | `InfantryClass_ActionOnObject_IvanBombs` | new_in_3.0 | ✓ | 0051F190 `InfantryClass::InfantryClass__ClickedAction_Object` | decompiled/functions/10056BF0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:289 | src/app/input/context_order.rs, src/sim/world/move_cell_input.rs |
| 6F523C | `TechnoClass_DrawExtras_IvanBombImage` | kept | ✓ | 006F5190 `TechnoClass::TechnoClass__DrawExtras` | decompiled/functions/10056CC0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:103 | src/app/presentation/render/draw_plan_lowering.rs |
| 6FA4C6 | `TechnoClass_Update_ZeroOutTarget` | kept | ✓ | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/10056D80.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:205 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 6FCBAD | `TechnoClass_CanFire_IvanBomb` | replaces_0.A:TechnoClass_GetObjectActivityState_IvanBomb | ✓ | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/10056C60.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:122 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 6FFEC0 | `TechnoClass_GetActionOnObject_IvanBombsA` | new_in_3.0 | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/10056D30.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:261 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 6FFF9E | `TechnoClass_GetActionOnObject_IvanBombsB` | new_in_3.0 | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/10056D70.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:280 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 725961 | `AnnounceInvalidPointer_BombCloak` | kept | ✓ | 007258D0 `DispatchPointerExpiredCleanup` | decompiled/functions/10056750.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:306 | src/sim/combat/combat_aoe.rs, src/sim/team_script_vm.rs, src/sim/trigger_runtime.rs … |
| 7388EB | `UnitClass_ActionOnObject_IvanBombs` | new_in_3.0 | ✓ | 00738890 `UnitClass::UnitClass__ClickedAction_Object` | decompiled/functions/10056DA0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:297 | src/app/input/context_order.rs, src/sim/world/move_cell_input.rs, src/sim/world/world_orders.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10054A80 | WeaponTypeExt::ExtData::conductAbduction [translated] | 使用键 | decompiled/functions/10054A80.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
| 100097B0 | `TechnoClass::SetFocus` | 被钩子调用 | decompiled/functions/100097B0.c |
| 10009790 | `TechnoClass::SetCurrentWeaponStage` | 被钩子调用 | decompiled/functions/10009790.c |
| 100092F0 | `TeamClass::LiberateMember` | 被钩子调用 | decompiled/functions/100092F0.c |
| 10009180 | `CaptureManagerClass::FreeUnit` | 被钩子调用 | decompiled/functions/10009180.c |
| 100162E0 | `CaptureManagerClass::FreeAll` | 被钩子调用 | decompiled/functions/100162E0.c |
| 10044360 | `TemporalClass::Detach` | 被钩子调用 | decompiled/functions/10044360.c |
| 100551F0 | `HouseClass::FindByCountryName` | 被钩子调用 | decompiled/functions/100551F0.c |
| 10044380 | `TechnoExt::DetachSpecificSpawnee` | 被钩子调用 | decompiled/functions/10044380.c |
| 100038E0 | `SpawnManagerClass::KillNodes` | 被钩子调用 | decompiled/functions/100038E0.c |
| 10016320 | `SpawnManagerClass::KillNodes` | 被钩子调用 | decompiled/functions/10016320.c |
| 10046380 | `SpawnManagerClass::ResetTarget` | 被钩子调用 | decompiled/functions/10046380.c |
| 10044980 | `TechnoExt::FreeSpecificSlave` | 被钩子调用 | decompiled/functions/10044980.c |
| 10045A00 | `SlaveManagerClass::Killed` | 被钩子调用 | decompiled/functions/10045A00.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10055080 | `TechnoClass::ClearPlanningTokens` | 被钩子调用 | decompiled/functions/10055080.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 10044880 | `TechnoClass::EnteredOpenTopped` | 被钩子调用 | decompiled/functions/10044880.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 10055A80 | `WeaponTypeExt::OnSell` | 被钩子调用 | decompiled/functions/10055A80.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10055AE0 | `WeaponTypeExt::ExtData::PlantBomb` | 被钩子调用 | decompiled/functions/10055AE0.c |
| 100433A0 | `TechnoExt::CanDetonateAttachedBomb` | 被钩子调用 | decompiled/functions/100433A0.c |

## 涉及的热点函数（9）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | 13 | B.spy, T.emp, T.poweredby, W.ivanbomb, ext:Building, ext:Infantry, ext:Techno |
| 00449C30 `BuildingClass::BuildingClass__Mission_Selling` | 7 | T.emp, U.bugfixes, W.ivanbomb, ext:Building, ext:Techno |
| 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | 7 | U.bugfixes, W.death, W.ivanbomb, ext:Bullet, ext:WarheadType |
| 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | 7 | F.armor, T.ammo, U.bugfixes, W.ivanbomb, ext:Techno |
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno |
| 007258D0 `DispatchPointerExpiredCleanup` | 4 | U.bugfixes, W.ivanbomb, ext:Ares.Ext, ext:ParticleSystem |
| 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | 7 | B.spy, W.ivanbomb, ext:Techno |
| 006F5190 `TechnoClass::TechnoClass__DrawExtras` | 3 | T.cloak, W.ivanbomb, ext:Techno |
| 00447110 `BuildingClass::BuildingClass__Sell_Back` | 2 | B.prism, W.ivanbomb |
