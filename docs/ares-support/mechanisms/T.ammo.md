# `T.ammo` 步兵/武器弹药与装填

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `EmptyReloadAmount` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:474 | ok | — |
| `NoAmmoAmount` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:475 | ok | — |
| `NoAmmoWeapon` | TechnoType | Valueable<int> | -1 | src/Ext/TechnoType/Body.cpp:476 | ok | — |
| `ReloadAmount` | TechnoType | Valueable<int> | 1 | src/Ext/TechnoType/Body.cpp:473 | ok | — |
| `ReloadRate` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:471 | ok | src/rules/ruleset.rs |
| `Ammo` | WeaponType | — | — | src/Ext/WeaponType/Body.cpp:223 | unscanned | src/rules/attach_effect.rs, src/rules/digital_display.rs, src/rules/ini_value.rs … |

## 钩子（11）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 43FE8E | `BuildingClass_Update_Reload` | kept | ✓ | 0043FB20 `BuildingClass::BuildingClass__Update` | decompiled/functions/100478E0.c | src/Ext/Techno/Hooks.Ammo.cpp:21 | src/sim/combat/world_receiver.rs, src/sim/world/bridge_orchestrator.rs, src/sim/world/building_working_sound.rs … |
| 51BCB2 | `InfantryClass_Update_Reload` | kept | ✓ | 0051BAB0 `InfantryClass::InfantryClass__AI` | decompiled/functions/10047980.c | src/Ext/Techno/Hooks.Ammo.cpp:59 | src/sim/mission/authority.rs, src/sim/movement/infantry_action.rs, src/sim/world/infantry_death.rs … |
| 51DF8C | `InfantryClass_Fire_Ammo` | replaces_0.A:InfantryClass_Fire_RearmTimer | ✓ | 0051DF60 `InfantryClass::InfantryClass__Fire_At` | decompiled/functions/10047910.c | src/Ext/Techno/Hooks.Ammo.cpp:113 | src/sim/combat/world_fire.rs, src/sim/infantry.rs |
| 5200D7 | `InfantryClass_UpdatePanic_DontReload` | kept | ✓ | 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | decompiled/functions/10047970.c | src/Ext/Techno/Hooks.Ammo.cpp:54 | src/sim/infantry.rs |
| 6F3410 | `TechnoClass_SelectWeapon_NoAmmoWeapon` | new_in_3.0 | ✓ | 006F3330 `TechnoClass::TechnoClass__SelectWeaponAgainst` | decompiled/functions/10047AE0.c | src/Ext/Techno/Hooks.Ammo.cpp:166 | src/sim/combat/combat_weapon.rs, src/sim/combat/gattling.rs, src/sim/combat/world_fire.rs … |
| 6FB05B | `TechnoClass_Reload_ReloadAmount` | kept | ✓ | 006FB010 `TechnoClass::TechnoClass__Reload` | decompiled/functions/10047A70.c | src/Ext/Techno/Hooks.Ammo.cpp:145 | — |
| 6FCA0D | `TechnoClass_CanFire_Ammo` | replaces_0.A:TechnoClass_GetFireError_Ammo | ✓ | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/100479B0.c | src/Ext/Techno/Hooks.Ammo.cpp:74 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 6FCFA4 | `TechnoClass_GetROF_BuildingHack` | kept | ✓ | 006FCFA0 `TechnoClass::TechnoClass__GetROF` | decompiled/functions/10047A60.c | src/Ext/Techno/Hooks.Ammo.cpp:31 | src/rules/ruleset.rs, src/sim/combat/rof.rs, src/sim/combat/world_fire.rs |
| 6FD0BF | `TechnoClass_GetROF_AttachEffect` | new_in_3.0 | ✓ | 006FCFA0 `TechnoClass::TechnoClass__GetROF` | decompiled/functions/10047A40.c | src/Ext/Techno/Hooks.Ammo.cpp:44 | src/rules/ruleset.rs, src/sim/combat/rof.rs, src/sim/combat/world_fire.rs |
| 6FF656 | `TechnoClass_Fire_Ammo` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/100479F0.c | src/Ext/Techno/Hooks.Ammo.cpp:89 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 7413FF | `UnitClass_Fire_Ammo` | new_in_3.0 | ✓ | 00741340 `UnitClass::UnitClass__Fire_At` | decompiled/functions/10047B30.c | src/Ext/Techno/Hooks.Ammo.cpp:109 | src/sim/cloak_disguise.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 100478C0 | `TechnoClass::StartReloading` | 被钩子调用 | decompiled/functions/100478C0.c |
| 100441F0 | `TechnoExt::DecreaseAmmo` | 被钩子调用 | decompiled/functions/100441F0.c |

## 涉及的热点函数（6）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, W.beams, W.weaponmisc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType |
| 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | 7 | F.armor, T.ammo, U.bugfixes, W.ivanbomb, ext:Techno |
| 0043FB20 `BuildingClass::BuildingClass__Update` | 6 | B.firestorm, T.ammo, T.jammer, ext:Building, ext:BuildingType |
| 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | 4 | T.ammo, T.poweredby, W.death, ext:Infantry |
| 0051BAB0 `InfantryClass::InfantryClass__AI` | 2 | B.firestorm, T.ammo |
| 006F3330 `TechnoClass::TechnoClass__SelectWeaponAgainst` | 2 | F.armor, T.ammo |
