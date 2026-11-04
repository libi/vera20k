# `W.death` 死亡表现与死亡武器抑制

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（8）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `InfantryElectrocuted` | Rules | Valueable<AnimTypeClass*>::Read | — | src/Ext/Rules/Body.cpp:208 | ok | — |
| `DeathWeapon.Suppress` | WarheadType | ValueableVector<TechnoTypeClass*> | — | src/Ext/WarheadType/Body.cpp:143 | ok | — |
| `DeathWeapon.SuppressInfantry` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:142 | ok | — |
| `DeathWeapon.SuppressVehicles` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:141 | ok | — |
| `DieSound.Override` | WarheadType | NullableIdx<VocClass> | (empty) | src/Ext/WarheadType/Body.cpp:170 | ok | — |
| `InfDeathAnim` | WarheadType | Valueable<AnimTypeClass*> | nullptr | src/Ext/WarheadType/Body.cpp:116 | ok | — |
| `UnitLost.Suppress` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:139 | ok | — |
| `VoiceDie.Override` | WarheadType | NullableIdx<VocClass> | (empty) | src/Ext/WarheadType/Body.cpp:171 | ok | — |

## 钩子（16）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 469C4E | `BulletClass_DetonateAt_DamageAnimSelected` | kept | ✓ | 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | decompiled/functions/10025C90.c | src/Ext/Infantry/Hooks.MakeInf.cpp:171 | src/rules/warhead_type.rs, src/sim/combat/techno_death.rs, src/sim/combat/world_receiver.rs … |
| 5183DE | `InfantryClass_ReceiveDamage_InfantryVirus1` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025EF0.c | src/Ext/Infantry/Hooks.MakeInf.cpp:69 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 51849A | `InfantryClass_ReceiveDamage_DeathAnim` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025D60.c | src/Ext/Infantry/Hooks.MakeInf.cpp:47 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 5185C8 | `InfantryClass_ReceiveDamage_InfDeath` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025DE0.c | src/Ext/Infantry/Hooks.MakeInf.cpp:13 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 518698 | `InfantryClass_ReceiveDamage_Anims` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025D20.c | src/Ext/Infantry/Hooks.MakeInf.cpp:93 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 518744 | `InfantryClass_ReceiveDamage_ElectricDeath` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/1002C930.c | src/Ext/Rules/Hooks.cpp:36 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 51887B | `InfantryClass_ReceiveDamage_InfantryVirus2` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025F50.c | src/Ext/Infantry/Hooks.MakeInf.cpp:115 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 518A96 | `InfantryClass_ReceiveDamage_InfantryMutate` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025EB0.c | src/Ext/Infantry/Hooks.MakeInf.cpp:136 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 518CB3 | `InfantryClass_ReceiveDamage_Doggie` | kept | ✓ | 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | decompiled/functions/10025AD0.c | src/Ext/Infantry/Hooks.Doggie.cpp:33 | src/rules/warhead_type.rs, src/sim/infantry.rs, src/sim/world/infantry_death.rs |
| 51ABD7 | `InfantryClass_SetDestination_Doggie` | kept | ✓ | 0051AA40 `InfantryClass::InfantryClass__Set_Destination` | decompiled/functions/10025B00.c | src/Ext/Infantry/Hooks.Doggie.cpp:45 | src/sim/mission/concrete_effects.rs, src/sim/movement/foot_path.rs, src/sim/movement/infantry_scatter.rs … |
| 51F628 | `InfantryClass_Guard_Doggie` | kept | ✓ | 0051F620 `InfantryClass::InfantryClass__Mission_Guard` | decompiled/functions/10025960.c | src/Ext/Infantry/Hooks.Doggie.cpp:7 | src/rules/object_type.rs, src/sim/world/techno_ai/mission_handlers.rs |
| 5200C1 | `InfantryClass_UpdatePanic_Doggie` | kept | ✓ | 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | decompiled/functions/10025B50.c | src/Ext/Infantry/Hooks.Doggie.cpp:56 | src/sim/infantry.rs |
| 6E232E | `ActionClass_PlayAnimAt` | kept | ✓ | 006E2290 `TActionClass::TActionClass__PlayAnimAtWP` | decompiled/functions/10025C20.c | src/Ext/Infantry/Hooks.MakeInf.cpp:153 | — |
| 702185 | `TechnoClass_ReceiveDamage_OverrideVoiceDie` | new_in_3.0 | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004ADA0.c | src/Ext/Techno/Hooks.cpp:1734 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |
| 7021F5 | `TechnoClass_ReceiveDamage_OverrideDieSound` | new_in_3.0 | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AD60.c | src/Ext/Techno/Hooks.cpp:1756 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |
| 702669 | `TechnoClass_ReceiveDamage_SuppressDeathWeapon` | kept | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/10053BA0.c | src/Ext/WarheadType/Hooks.cpp:198 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DDD0 | FUN_1002ddd0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002DDD0.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002E060.c |
| 10064F70 | GetSectionCached [translated,inferred] | 使用键 | decompiled/functions/10064F70.c |
| 10025930 | `FacingStruct::in_motion` | 被钩子调用 | decompiled/functions/10025930.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10025910 | `FootClass::MoveToTiberium` | 被钩子调用 | decompiled/functions/10025910.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 1000A5F0 | `AnimTypeExt::SetMakeInfOwner` | 被钩子调用 | decompiled/functions/1000A5F0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10053680 | `TechnoClass::FireDeathWeapon` | 被钩子调用 | decompiled/functions/10053680.c |
| 1005F510 | `CRC32::Calculate` | 被钩子调用 | decompiled/functions/1005F510.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004690B0 `BulletClass::BulletClass__DetonateAtCoord` | 7 | U.bugfixes, W.death, W.ivanbomb, ext:Bullet, ext:WarheadType |
| 00517FA0 `InfantryClass::InfantryClass__ReceiveDamage` | 16 | W.death, ext:Infantry, ext:Rules, ext:WarheadType |
| 005200B0 `InfantryClass::InfantryClass__Fear_Decay_Handler` | 4 | T.ammo, T.poweredby, W.death, ext:Infantry |
