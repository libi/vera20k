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
| `Missile` | TechnoType | Valueable<RocketStruct> | (empty) | src/Ext/TechnoType/Body.cpp:379 | check_0A_use | src/rules/mission_data.rs |
| `Missile.Custom` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:378 | check_0A_use | — |
| `Missile.EliteWarhead` | TechnoType | Valueable<WarheadTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:382 | no_use_found | — |
| `Missile.EliteWeapon` | TechnoType | Valueable<WeaponTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:387 | no_use_found | — |
| `Missile.TakeOffAnim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:383 | check_0A_use | — |
| `Missile.TrailerAnim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:384 | check_0A_use | — |
| `Missile.TrailerSeparation` | TechnoType | Valueable<int> | 3 | src/Ext/TechnoType/Body.cpp:385 | check_0A_use | — |
| `Missile.Warhead` | TechnoType | Valueable<WarheadTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:381 | no_use_found | — |
| `Missile.Weapon` | TechnoType | Valueable<WeaponTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:386 | no_use_found | — |

## 钩子（13）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6622E0 | `RocketLocomotionClass_ILocomotion_Process_CustomMissile` | kept | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/100610F0.c | src/Misc/CustomMissile.cpp:11 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 66238A | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff1` | kept | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061120.c | src/Misc/CustomMissile.cpp:25 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 662512 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff2` | kept | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061190.c | src/Misc/CustomMissile.cpp:42 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 6627E5 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTakeoff3` | kept | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061200.c | src/Misc/CustomMissile.cpp:59 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 662D85 | `RocketLocomotionClass_ILocomotion_Process_CustomMissileTrailer` | kept | 006622C0 `RocketLocomotionClass__Process` | decompiled/functions/10061270.c | src/Misc/CustomMissile.cpp:76 | src/rules/effect_asset_catalog.rs, src/rules/missile_spawn.rs, src/sim/movement/ground_pose.rs … |
| 66305A | `RocketLocomotionClass_Explode_CustomMissile` | kept | 00663030 `RocketLocomotionClass::RocketLocomotionClass__Detonate` | decompiled/functions/10060FA0.c | src/Misc/CustomMissile.cpp:97 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs |
| 663218 | `RocketLocomotionClass_Explode_CustomMissile2` | kept | 00663030 `RocketLocomotionClass::RocketLocomotionClass__Detonate` | decompiled/functions/10060FF0.c | src/Misc/CustomMissile.cpp:119 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs |
| 6632F2 | `RocketLocomotionClass_ILocomotion_MoveTo_CustomMissile` | kept | 006632E0 `RocketLocomotionClass__Move_To` | decompiled/functions/100610C0.c | src/Misc/CustomMissile.cpp:150 | src/rules/missile_spawn.rs, src/sim/movement/rocket_movement.rs, src/sim/world/fly_orders.rs … |
| 6634F6 | `RocketLocomotionClass_ILocomotion_DrawMatrix_CustomMissile` | kept | 00663470 `RocketLocomotionClass__Draw_Matrix` | decompiled/functions/10061090.c | src/Misc/CustomMissile.cpp:164 | src/app/presentation/instances/units.rs, src/render/unit_atlas.rs, src/render/unit_pose_cache.rs … |
| 6B6D60 | `SpawnManagerClass_CTOR_CustomMissile` | kept | 006B6C90 `SpawnManagerClass::SpawnManagerClass__Constructor` | decompiled/functions/10061320.c | src/Misc/CustomMissile.cpp:178 | src/sim/spawn_manager.rs, src/sim/world/world_spawn.rs |
| 6B752E | `SpawnManagerClass_Update_CustomMissileTakeoff` | kept | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/10061410.c | src/Misc/CustomMissile.cpp:222 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |
| 6B78F8 | `SpawnManagerClass_Update_CustomMissile` | kept | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/100613A0.c | src/Misc/CustomMissile.cpp:190 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |
| 6B7A72 | `SpawnManagerClass_Update_CustomMissile2` | kept | 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | decompiled/functions/100613C0.c | src/Misc/CustomMissile.cpp:202 | src/sim/combat/in_range.rs, src/sim/spawn_manager.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1003DEC0 | TechnoTypeExt::ExtMap.LoadFromINI [source_0.A,inferred] | 使用键 | decompiled/functions/1003DEC0.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 1000AC10 | `BulletClass::Detonate` | 被钩子调用 | decompiled/functions/1000AC10.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006B7230 `SpawnManagerClass::SpawnManagerClass__AI` | 5 | T.missile, ext:Techno, recovered:SpawnManagerClass |
| 006622C0 `RocketLocomotionClass__Process` | 5 | T.missile |
