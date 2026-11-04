# `W.weaponmisc` 武器杂项：ApplyDamage、射程

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `ApplyDamage` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:221 | ok | — |
| `ProjectileRange` | WeaponType | Valueable<Leptons> | Leptons(100000) | src/Ext/WeaponType/Body.cpp:219 | ok | — |

## 钩子（7）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4A76ED | `DiskLaserClass_Update_Anim` | kept | ✓ | 004A7340 `DiskLaserClass::DiskLaserClass__AI` | decompiled/functions/1004CFF0.c | src/Ext/Techno/Hooks.Firing.cpp:33 | — |
| 6F7511 | `TechnoClass_IsCloseEnoughToTarget_Obstacle` | kept | ✓ | 006F7220 `TechnoClass::TechnoClass__InRange` | decompiled/functions/1004D120.c | src/Ext/Techno/Hooks.Firing.cpp:15 | src/rules/weapon_type.rs, src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs … |
| 6FE31C | `TechnoClass_Fire_AllowDamage` | kept ⚠改过 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004A370.c | src/Ext/Techno/Hooks.cpp:2094 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FE53F | `TechnoClass_Fire_CreateBullet` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1001ADE0.c | src/Ext/Bullet/Hooks.cpp:329 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF1FB | `TechnoClass_Fire_DetachedRailgun` | new_in_3.0 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0C0.c | src/Ext/Techno/Hooks.Firing.cpp:77 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF26E | `TechnoClass_Fire_DetachedRailgun2` | new_in_3.0 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0F0.c | src/Ext/Techno/Hooks.Firing.cpp:87 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 70CBB0 | `TechnoClass_DealParticleDamage_AmbientDamage` | new_in_3.0 | ✓ | 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | decompiled/functions/1004D080.c | src/Ext/Techno/Hooks.Firing.cpp:59 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10019210 | BulletTypeExt::ExtData::CreateBullet(AbstractClass*,TechnoClass*,WeaponTypeClass*) [translated] | 使用键 | decompiled/functions/10019210.c |
| 100191E0 | `BulletTypeExt::ExtData::CreateBullet` | 被钩子调用 | decompiled/functions/100191E0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 1006E1F0 | `AresTrajectoryHelper::FindFirstImpenetrableObstacle` | 被钩子调用 | decompiled/functions/1006E1F0.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, W.beams, W.weaponmisc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType |
| 006F7220 `TechnoClass::TechnoClass__InRange` | 3 | U.bugfixes, W.weaponmisc |
| 0070C690 `TechnoClass::TechnoClass__ApplyRailgunLineDamage` | 3 | U.bugfixes, W.weaponmisc |
