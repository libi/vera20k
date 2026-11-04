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
| `SelfHealing.%sAmount` | TechnoType | Promotable<int> | 1 | src/Ext/TechnoType/Body.cpp:486 | ok | — |
| `SelfHealing.%sMax` | TechnoType | Promotable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:487 | ok | — |
| `SelfHealing.CombatDelay` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:488 | ok | — |
| `SelfHealing.Rate` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:485 | ok | src/rules/phobos_shields.rs |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 702819 | `TechnoClass_ReceiveDamage_Aftermath` | new_in_3.0 | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AB50.c | src/Ext/Techno/Hooks.cpp:2559 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10044E80 | TechnoExt::ExtData::GetSelfHealAmount [translated] | 使用键 | decompiled/functions/10044E80.c |
| 10052B40 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052B40.c |
