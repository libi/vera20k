# `W.damage` 相对伤害、Culling、部署状态伤害

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | W.filter | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Culling.%sBelowHealth` | WarheadType | Promotable<int>::Read | — | src/Ext/WarheadType/Body.cpp:157 | no_use_found | — |
| `Culling.%sChance` | WarheadType | Promotable<int>::Read | — | src/Ext/WarheadType/Body.cpp:158 | no_use_found | — |
| `Damage.Deployed` | WarheadType | double | 1.00 | src/Ext/WarheadType/Body.cpp:95 | no_use_found | — |
| `RelativeDamage` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:129 | ok | — |
| `RelativeDamage.Aircraft` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:131 | ok | — |
| `RelativeDamage.Buildings` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:130 | ok | — |
| `RelativeDamage.Infantry` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:132 | ok | — |
| `RelativeDamage.Terrain` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:134 | ok | — |
| `RelativeDamage.Vehicles` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:133 | ok | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 5F53E5 | `ObjectClass_ReceiveDamage_Relative` | new_in_3.0 | 005F5390 `ObjectClass::ObjectClass__ReceiveDamage` | decompiled/functions/10053A80.c | src/Recovered/ObjectClass.cpp:286 | src/sim/combat/damage/receive.rs, src/sim/combat/receiver_call.rs, src/sim/terrain_object.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10051C50 | WarheadTypeExt::ExtData::GetRelativeDamage [translated] | 使用键 | decompiled/functions/10051C50.c |
