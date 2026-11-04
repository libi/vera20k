# `T.crush` 碾压伤害与全能碾压

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
| `ChronoInfantryCrush` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:224 | ok | — |
| `CrushDamage.%s` | TechnoType | Promotable<int> | 0 | src/Ext/TechnoType/Body.cpp:464 | no_use_found | — |
| `CrushDamage.Warhead` | TechnoType | Nullable<WarheadTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:465 | check_0A_use | — |
| `OmniCrusher.Aggressive` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:463 | no_use_found | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 718275 | `TeleportLocomotionClass_MakeRoom` | new_in_3.0 | 00718260 `TeleportLocomotionClass::TeleportLocomotionClass__Update_Position` | decompiled/functions/1004B980.c | src/Recovered/TeleportLocomotionClass.cpp:30 | src/sim/movement/teleport_chrono.rs |
| 7418AA | `UnitClass_CrushCell_CrushDamage` | kept | 007416A0 `UnitClass::UnitClass__Crush_Or_Prescatter_Cell` | decompiled/functions/1004BCE0.c | src/Ext/Techno/Hooks.cpp:1548 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/combat/techno_death.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
