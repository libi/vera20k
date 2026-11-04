# `P.projectile` 抛射体：空爆、分裂、重定向、散布、受建筑/战壕阻挡

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（11）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AirburstSpread` | BulletType | Valueable<double> | 1.5 | src/Ext/BulletType/Body.cpp:33 | no_use_found | src/rules/phobos_airburst.rs |
| `AnimLength` | BulletType | Valueable<int> | 0 | src/Ext/BulletType/Body.cpp:42 | ok | — |
| `AroundTarget` | BulletType | Nullable<bool> | (empty) | src/Ext/BulletType/Body.cpp:37 | ok | src/rules/phobos_airburst.rs |
| `AttachedSystem` | BulletType | Valueable<ParticleSystemTypeClass*> | nullptr | src/Ext/BulletType/Body.cpp:45 | no_use_found | src/rules/native_processing.rs |
| `BallisticScatter.Max` | BulletType | Nullable<Leptons> | (empty) | src/Ext/BulletType/Body.cpp:40 | ok | — |
| `BallisticScatter.Min` | BulletType | Nullable<Leptons> | (empty) | src/Ext/BulletType/Body.cpp:39 | ok | — |
| `Parachuted` | BulletType | Valueable<bool> | false | src/Ext/BulletType/Body.cpp:27 | ok | — |
| `RetargetAccuracy` | BulletType | Valueable<double> | 0.0 | src/Ext/BulletType/Body.cpp:34 | ok | src/rules/phobos_airburst.rs |
| `RetargetSelf` | BulletType | Valueable<bool> | true | src/Ext/BulletType/Body.cpp:36 | no_use_found | src/rules/phobos_airburst.rs |
| `Splits` | BulletType | Valueable<bool> | false | src/Ext/BulletType/Body.cpp:35 | ok | src/rules/phobos_airburst.rs |
| `SubjectToBuildings` | BulletType | Valueable<bool> | false | src/Ext/BulletType/Body.cpp:25 | ok | — |

## 钩子（6）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 468000 | `BulletClass_GetAnimFrame` | kept | ✓ | 00468000 `BulletClass::BulletClass__GetAnimFrame` | decompiled/functions/1001A840.c | src/Ext/Bullet/Hooks.cpp:352 | src/sim/projectile.rs, src/util/direction_tables/dragon.rs |
| 46867F | `BulletClass_SetMovement_Parachute` | kept | ✓ | 00468670 `BulletClass::BulletClass__Fire` | decompiled/functions/1001AAB0.c | src/Ext/Bullet/Hooks.cpp:61 | src/sim/combat/phobos_airburst.rs, src/sim/combat/projectile_delivery.rs, src/sim/projectile.rs |
| 468BE2 | `BulletClass_ShouldDetonate_Obstacle` | kept | ✓ | 00468BB0 `BulletClass::BulletClass__ShouldDetonateAtCurrentLocation` | decompiled/functions/1001AB10.c | src/Ext/Bullet/Hooks.cpp:35 | src/sim/projectile.rs, src/sim/world/projectile_collision.rs |
| 469EBA | `BulletClass_DetonateAt_Splits` | kept | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/1001A090.c | src/Ext/Bullet/Hooks.cpp:144 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 6FE709 | `TechnoClass_Fire_BallisticScatter1` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10019930.c | src/Ext/BulletType/Hooks.cpp:5 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FE7FE | `TechnoClass_Fire_BallisticScatter2` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10019980.c | src/Ext/BulletType/Hooks.cpp:20 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100193A0 | BulletTypeExt::ExtData::HasSplitBehavior [translated] | 使用键 | decompiled/functions/100193A0.c |
| 10057C60 | thunk_FUN_10057be0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10057C60.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1001A010 | `CellRangeEnumerator::next` | 被钩子调用 | decompiled/functions/1001A010.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
| 100191E0 | `BulletTypeExt::ExtData::CreateBullet` | 被钩子调用 | decompiled/functions/100191E0.c |
| 1000ACC0 | `Math::sin` | 被钩子调用 | decompiled/functions/1000ACC0.c |
| 1000ACB0 | `Math::cos` | 被钩子调用 | decompiled/functions/1000ACB0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 1006E540 | `AresTrajectoryHelper::GetObstacle` | 被钩子调用 | decompiled/functions/1006E540.c |
