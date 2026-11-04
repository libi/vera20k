# `T.missile` 自定义导弹单位

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Missile` | TechnoType | Valueable<RocketStruct> | (empty) | src/Ext/TechnoType/Body.cpp:383 | ok | src/rules/mission_data.rs |
| `Missile.Custom` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:382 | ok | — |
| `Missile.EliteWarhead` | TechnoType | Valueable<WarheadTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:386 | ok | — |
| `Missile.EliteWeapon` | TechnoType | Valueable<WeaponTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:391 | ok | — |
| `Missile.TakeOffAnim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:387 | ok | — |
| `Missile.TrailerAnim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:388 | ok | — |
| `Missile.TrailerSeparation` | TechnoType | Valueable<int> | 3 | src/Ext/TechnoType/Body.cpp:389 | ok | — |
| `Missile.Warhead` | TechnoType | Valueable<WarheadTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:385 | ok | — |
| `Missile.Weapon` | TechnoType | Valueable<WeaponTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:390 | ok | — |

## 钩子（14）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6622E0 | `RocketLocomotionClass_ILocomotion_Process_CustomMissile` | kept | ✓ | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/100610F0.c | src/Misc/CustomMissile.cpp:12 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 66238A | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff1` | kept | ✓ | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061120.c | src/Misc/CustomMissile.cpp:26 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 662512 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff2` | kept | ✓ | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061190.c | src/Misc/CustomMissile.cpp:43 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 6627E5 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff3` | kept | ✓ | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061200.c | src/Misc/CustomMissile.cpp:60 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 662D85 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTrailer` | kept | ✓ | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061270.c | src/Misc/CustomMissile.cpp:77 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 66305A | `RocketLocomotionClass_Explode_CustomMissile` | kept | ✓ | 00663030 `RocketLocomotionClass::RocketLocomotionClass__Detonate` | decompiled/functions/10060FA0.c | src/Misc/CustomMissile.cpp:98 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs |
| 663218 | `RocketLocomotionClass_Explode_CustomMissile2` | kept | ✓ | 00663030 `RocketLocomotionClass::RocketLocomotionClass__Detonate` | decompiled/functions/10060FF0.c | src/Misc/CustomMissile.cpp:120 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs |
| 6632F2 | `RocketLocomotionClass_ILocomotion_MoveTo_CustomMissile` | kept | ✓ | 006632E0 `RocketLocomotionClass__Move_To` | decompiled/functions/100610C0.c | src/Misc/CustomMissile.cpp:151 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs, src/sim/world/fly_orders.rs … |
| 6634F6 | `RocketLocomotionClass_ILocomotion_DrawMatrix_CustomMissile` | kept | ✓ | 00663470 `RocketLocomotionClass__Draw_Matrix` | decompiled/functions/10061090.c | src/Misc/CustomMissile.cpp:165 | src/app/presentation/instances/units.rs, src/render/unit_atlas.rs, src/render/unit_pose_cache.rs … |
| 6B6D60 | `SpawnManagerClass_CTOR_CustomMissile` | kept | ✓ | 006B6C90 `SpawnManagerClass::SpawnManagerClass__Constructor` | decompiled/functions/10061320.c | src/Misc/CustomMissile.cpp:179 | src/sim/spawn_manager.rs, src/sim/world/world_spawn.rs |
| 6B72F9 | `SpawnManagerClass_Update_Buildings` | new_in_3.0 | ✓ | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/10061350.c | src/Misc/CustomMissile.cpp:196 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |
| 6B752E | `SpawnManagerClass_Update_CustomMissileTakeoff` | kept | ✓ | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/10061410.c | src/Misc/CustomMissile.cpp:243 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |
| 6B78F8 | `SpawnManagerClass_Update_CustomMissile` | kept | ✓ | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/100613A0.c | src/Misc/CustomMissile.cpp:211 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |
| 6B7A72 | `SpawnManagerClass_Update_CustomMissile2` | kept | ✓ | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/100613C0.c | src/Misc/CustomMissile.cpp:223 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000AC10 | `BulletClass::Detonate` | 被钩子调用 | decompiled/functions/1000AC10.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | 5 | T.missile, ext:Techno |
| 006622C0 `RocketLocomotionClass__Process` | 5 | T.missile |
