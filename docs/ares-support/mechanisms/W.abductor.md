# `W.abductor` Abductor 绑架

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Abductor` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:89 | ok | — |
| `Abductor.AbductBelowPercent` | WeaponType | Valueable<double> | 1 | src/Ext/WeaponType/Body.cpp:92 | no_use_found | — |
| `Abductor.Anim` | WeaponType | Valueable<AnimTypeClass *> | nullptr | src/Ext/WeaponType/Body.cpp:94 | no_use_found | — |
| `Abductor.ChangeOwner` | WeaponType | Valueable <bool> | false | src/Ext/WeaponType/Body.cpp:90 | no_use_found | — |
| `Abductor.MaxHealth` | WeaponType | Valueable<int> | 0 | src/Ext/WeaponType/Body.cpp:93 | no_use_found | — |
| `Abductor.Temporal` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:91 | check_0A_use | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 46920B | `BulletClass_Detonate` | kept ⚠改过 | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/100536E0.c | src/Ext/WarheadType/Hooks.cpp:21 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 71AAAC | `TemporalClass_Update_Abductor` | new_in_3.0 | 0071A760 `TemporalClass::TemporalClass__Update` | decompiled/functions/100564F0.c | src/Recovered/TemporalClass.cpp:134 | src/sim/temporal.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10054A80 | WeaponTypeExt::ExtData::conductAbduction [translated] | 使用键 | decompiled/functions/10054A80.c |
| 10051AE0 | `WarheadTypeExt::ExtData::applyEMP` | 被钩子调用 | decompiled/functions/10051AE0.c |
| 10053660 | `ObjectClass::DistanceFrom` | 被钩子调用 | decompiled/functions/10053660.c |
| 10052890 | `WarheadTypeExt::ExtData::applyPermaMC` | 被钩子调用 | decompiled/functions/10052890.c |
| 10052870 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052870.c |
| 10052B40 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052B40.c |
| 10052700 | `WarheadTypeExt::ExtData::applyIronCurtain` | 被钩子调用 | decompiled/functions/10052700.c |
| 10052C90 | `WarheadTypeExt::ExtData::applyRipples` | 被钩子调用 | decompiled/functions/10052C90.c |
| 10052CD0 | `WarheadTypeExt::applyOccupantDamage` | 被钩子调用 | decompiled/functions/10052CD0.c |
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
