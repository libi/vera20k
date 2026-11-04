# `T.drain` 能量/资金抽取（Drain）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Drain.Amount` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:432 | ok | — |
| `Drain.Local` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:431 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 508D32 | `HouseClass_UpdatePower_LocalDrain1` | kept | ✓ | 00508C30 `HouseClass::HouseClass__AI_AssessPower` | decompiled/functions/10024690.c | src/Ext/House/Hooks.cpp:429 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10023D20 | `BuildingClass::GetPowerOutput` | 被钩子调用 | decompiled/functions/10023D20.c |
