# `T.selfheal` 自愈

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SelfHealing.%sAmount` | TechnoType | Promotable<int> | 1 | src/Ext/TechnoType/Body.cpp:482 | check_0A_use | — |
| `SelfHealing.%sMax` | TechnoType | Promotable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:483 | no_use_found | — |
| `SelfHealing.CombatDelay` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:484 | ok | — |
| `SelfHealing.Rate` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:481 | no_use_found | src/rules/phobos_shields.rs |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 702819 | `TechnoClass_ReceiveDamage_Aftermath` | new_in_3.0 | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AB50.c | src/Recovered/TechnoClass.cpp:626 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10044E80 | TechnoExt::ExtData::GetSelfHealAmount [source_0.A] | 使用键 | decompiled/functions/10044E80.c |
| 10052870 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052870.c |
| 10052B40 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052B40.c |
