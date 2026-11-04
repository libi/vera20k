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
| `EmptyReloadAmount` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:470 | check_0A_use | — |
| `NoAmmoAmount` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:471 | ok | — |
| `NoAmmoWeapon` | TechnoType | Valueable<int> | -1 | src/Ext/TechnoType/Body.cpp:472 | ok | — |
| `ReloadAmount` | TechnoType | Valueable<int> | 1 | src/Ext/TechnoType/Body.cpp:469 | check_0A_use | — |
| `ReloadRate` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:467 | no_use_found | src/rules/ruleset.rs |
| `Ammo` | WeaponType | — | — | src/Ext/WeaponType/Body.cpp:101 | unscanned | src/rules/attach_effect.rs, src/rules/digital_display.rs, src/rules/ini_value.rs … |

## 钩子（7）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 43FE8E | `BuildingClass_Update_Reload` | kept | 0043FB20 `BuildingClass::BuildingClass__Update` | decompiled/functions/100478E0.c | src/Ext/Techno/Hooks.Ammo.cpp:12 | src/sim/combat/world_receiver.rs, src/sim/world/bridge_orchestrator.rs, src/sim/world/building_working_sound.rs … |
| 51BCB2 | `InfantryClass_Update_Reload` | kept | 0051BAB0 `InfantryClass::InfantryClass__AI` | decompiled/functions/10047980.c | src/Ext/Techno/Hooks.Ammo.cpp:36 | src/sim/mission/authority.rs, src/sim/movement/infantry_action.rs, src/sim/world/infantry_death.rs … |
| 5200D7 | `InfantryClass_UpdatePanic_DontReload` | kept | 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | decompiled/functions/10047970.c | src/Ext/Techno/Hooks.Ammo.cpp:31 | src/sim/infantry.rs |
| 6F3410 | `TechnoClass_SelectWeapon_NoAmmoWeapon` | new_in_3.0 | 006F3330 `TechnoClass::TechnoClass__SelectWeaponAgainst` | decompiled/functions/10047AE0.c | src/Recovered/TechnoClass.cpp:313 | src/sim/combat/combat_weapon.rs, src/sim/combat/gattling.rs, src/sim/combat/world_fire.rs … |
| 6FB05B | `TechnoClass_Reload_ReloadAmount` | kept | 006FB010 `TechnoClass::TechnoClass__Reload` | decompiled/functions/10047A70.c | src/Ext/Techno/Hooks.Ammo.cpp:61 | — |
| 6FCFA4 | `TechnoClass_GetROF_BuildingHack` | kept | 006FCFA0 `TechnoClass::TechnoClass__GetROF` | decompiled/functions/10047A60.c | src/Ext/Techno/Hooks.Ammo.cpp:22 | src/rules/ruleset.rs, src/sim/combat/rof.rs, src/sim/combat/world_fire.rs |
| 6FF656 | `TechnoClass_Fire_Ammo` | kept | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/100479F0.c | src/Ext/Techno/Hooks.Ammo.cpp:50 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100441F0 | `TechnoExt::DecreaseAmmo` | 被钩子调用 | decompiled/functions/100441F0.c |
| 100478C0 | `TechnoClass::StartReloading` | 被钩子调用 | decompiled/functions/100478C0.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType, recovered:TechnoClass |
| 0043FB20 `BuildingClass::BuildingClass__Update` | 6 | B.firestorm, T.ammo, T.jammer, ext:Building, recovered:BuildingClass |
| 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | 4 | T.ammo, T.poweredby, W.death, ext:Infantry |
| 0051BAB0 `InfantryClass::InfantryClass__AI` | 2 | B.firestorm, T.ammo |
| 006FCFA0 `TechnoClass::TechnoClass__GetROF` | 2 | T.ammo, recovered:TechnoClass |
