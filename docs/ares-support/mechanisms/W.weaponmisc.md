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
| `ApplyDamage` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:99 | check_0A_use | — |
| `ProjectileRange` | WeaponType | Valueable<Leptons> | Leptons(100000) | src/Ext/WeaponType/Body.cpp:97 | check_0A_use | — |

## 钩子（4）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4A76ED | `DiskLaserClass_Update_Anim` | kept | 004A7340 `DiskLaserClass::DiskLaserClass__AI` | decompiled/functions/1004CFF0.c | src/Ext/Techno/Hooks.Firing.cpp:27 | — |
| 6F7511 | `TechnoClass_IsCloseEnoughToTarget_Obstacle` | kept | 006F7220 `TechnoClass::TechnoClass__InRange` | decompiled/functions/1004D120.c | src/Ext/Techno/Hooks.Firing.cpp:9 | src/rules/weapon_type.rs, src/sim/combat/in_range.rs, src/sim/combat/line_of_fire.rs … |
| 6FE31C | `TechnoClass_Fire_AllowDamage` | kept ⚠改过 | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004A370.c | src/Ext/Techno/Hooks.cpp:1520 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FE53F | `TechnoClass_Fire_CreateBullet` | kept | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1001ADE0.c | src/Ext/Bullet/Hooks.cpp:298 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10019210 | FUN_10019210 [source_0.A] | 使用键 | decompiled/functions/10019210.c |
| 100191E0 | `WeaponTypeExt::ExtData::GetProjectileRange` | 被钩子调用 | decompiled/functions/100191E0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006F7220 `TechnoClass::TechnoClass__InRange` | 3 | U.bugfixes, W.weaponmisc |
