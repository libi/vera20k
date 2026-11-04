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
| `IvanBomb.AttachSound` | WeaponType | NullableIdx<VocClass> | (empty) | src/Ext/WeaponType/Body.cpp:81 | no_use_found | — |
| `IvanBomb.CanDetonateDeathBomb` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:85 | no_use_found | — |
| `IvanBomb.CanDetonateTimeBomb` | WeaponType | Nullable<bool> | (empty) | src/Ext/WeaponType/Body.cpp:84 | no_use_found | — |
| `IvanBomb.Damage` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:77 | no_use_found | — |
| `IvanBomb.DeathBomb` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:73 | no_use_found | — |
| `IvanBomb.DeathBombOnAllies` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:74 | no_use_found | — |
| `IvanBomb.Delay` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:78 | ok | — |
| `IvanBomb.DestroysBridges` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:75 | no_use_found | — |
| `IvanBomb.Detachable` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:76 | no_use_found | — |
| `IvanBomb.DetonateOnSell` | WeaponType | Valueable<bool> | true | src/Ext/WeaponType/Body.cpp:86 | no_use_found | — |
| `IvanBomb.FlickerRate` | WeaponType | Nullable<int> | (empty) | src/Ext/WeaponType/Body.cpp:79 | no_use_found | — |
| `IvanBomb.Image` | WeaponType | Nullable<SHPStruct *> | (empty) | src/Ext/WeaponType/Body.cpp:83 | no_use_found | — |
| `IvanBomb.TickingSound` | WeaponType | NullableIdx<VocClass> | (empty) | src/Ext/WeaponType/Body.cpp:80 | no_use_found | — |
| `IvanBomb.Warhead` | WeaponType | Nullable<WarheadTypeClass *> | (empty) | src/Ext/WeaponType/Body.cpp:82 | no_use_found | — |

## 钩子（10）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 438799 | `BombClass_Detonate1` | kept | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/10056770.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:92 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438843 | `BombClass_Detonate2` | kept | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/100567F0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:105 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438879 | `BombClass_Detonate3` | kept | 00438720 `BombClass::BombClass__Detonate` | decompiled/functions/10056870.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:118 | src/rules/combat_damage.rs, src/sim/bomb.rs, src/sim/combat/world_receiver.rs … |
| 438A00 | `BombClass_GetCurrentFrame` | kept | 00438A00 `BombClass::BombClass__GetClockFrame` | decompiled/functions/100568C0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:31 | src/sim/bomb.rs |
| 438E86 | `BombListClass_Plant_AllTechnos` | kept | 00438E70 `BombListClass::BombListClass__Attach` | decompiled/functions/10056A50.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:12 | src/sim/bomb.rs, src/sim/projectile.rs |
| 438FD7 | `BombListClass_Plant_AttachSound` | kept | 00438E70 `BombListClass::BombListClass__Attach` | decompiled/functions/10056AA0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:26 | src/sim/bomb.rs, src/sim/projectile.rs |
| 4393F2 | `BombClass_SDDTOR` | kept | 004393F0 `BombClass::BombClass__ScalarDeletingDestructor` | decompiled/functions/100569F0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:128 | — |
| 6F523C | `TechnoClass_DrawExtras_IvanBombImage` | kept | 006F5190 `TechnoClass::TechnoClass__DrawExtras` | decompiled/functions/10056CC0.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:74 | src/app/presentation/render/draw_plan_lowering.rs |
| 6FA4C6 | `TechnoClass_Update_ZeroOutTarget` | kept | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/10056D80.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:139 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |
| 725961 | `AnnounceInvalidPointer_BombCloak` | kept | 007258D0 `DispatchPointerExpiredCleanup` | decompiled/functions/10056750.c | src/Ext/WeaponType/Hooks.IvanBomb.cpp:149 | src/sim/combat/combat_aoe.rs, src/sim/team_script_vm.rs, src/sim/trigger_runtime.rs … |

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
| 100551F0 | `TechnoExt::DetachSpecificSpawnee` | 被钩子调用 | decompiled/functions/100551F0.c |
| 10016320 | `SpawnManagerClass::KillNodes` | 被钩子调用 | decompiled/functions/10016320.c |
| 10046380 | `SpawnManagerClass::ResetTarget` | 被钩子调用 | decompiled/functions/10046380.c |
| 10044980 | `LocomotionClass::End_Piggyback` | 被钩子调用 | decompiled/functions/10044980.c |
| 10045A00 | `SlaveManagerClass::Killed` | 被钩子调用 | decompiled/functions/10045A00.c |
| 10055080 | `TechnoClass::ClearPlanningTokens` | 被钩子调用 | decompiled/functions/10055080.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 100553B0 | `TechnoExt::FreeSpecificSlave` | 被钩子调用 | decompiled/functions/100553B0.c |
| 10044880 | `TechnoClass::EnteredOpenTopped` | 被钩子调用 | decompiled/functions/10044880.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006F9E50 `TechnoClass::TechnoClass__AI_Update` | 9 | T.emp, U.bugfixes, W.ivanbomb, ext:Techno, recovered:TechnoClass |
| 007258D0 `DispatchPointerExpiredCleanup` | 4 | W.ivanbomb, ext:Ares.Ext, recovered:AnnounceInvalidPointer |
| 006F5190 `TechnoClass::TechnoClass__DrawExtras` | 3 | T.cloak, W.ivanbomb, ext:Techno |
