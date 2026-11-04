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
| `ChronoInfantryCrush` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:179 | ok | — |
| `CrushDamage.%s` | TechnoType | Promotable<int> | 0 | src/Ext/TechnoType/Body.cpp:468 | ok | — |
| `CrushDamage.Warhead` | TechnoType | Nullable<WarheadTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:469 | ok | — |
| `OmniCrusher.Aggressive` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:467 | ok | — |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 718275 | `TeleportLocomotionClass_MakeRoom` | new_in_3.0 | ✓ | 00718260 `TeleportLocomotionClass::TeleportLocomotionClass__Update_Position` | decompiled/functions/1004B980.c | src/Ext/Techno/Hooks.cpp:2664 | src/sim/movement/teleport_chrono.rs |
| 7418AA | `UnitClass_CrushCell_CrushDamage` | kept | ✓ | 007416A0 `UnitClass::UnitClass__Crush_Or_Prescatter_Cell` | decompiled/functions/1004BCE0.c | src/Ext/Techno/Hooks.cpp:2200 | src/architecture_guards.rs, src/sim/combat/object_health.rs, src/sim/combat/techno_death.rs … |
