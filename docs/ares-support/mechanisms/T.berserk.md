# `T.berserk` 狂暴与绑架免疫

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M6 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `BerserkROFMultiplier` | Rules | Valueable<double> | 0.5 | src/Ext/Rules/Body.cpp:197 | check_0A_use | — |
| `Berserk.ROFMultiplier` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:452 | no_use_found | — |
| `ImmuneToAbduction` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:364 | no_use_found | — |
| `ImmuneToBerserk` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:453 | ok | src/rules/phobos_shields.rs |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6FC417 | `TechnoClass_CanFire_PsionicsImmune` | new_in_3.0 | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/10049EF0.c | src/Recovered/TechnoClass.cpp:691 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 701BFE | `TechnoClass_ReceiveDamage_Abilities` | new_in_3.0 | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004A950.c | src/Recovered/TechnoClass.cpp:561 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F490.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
