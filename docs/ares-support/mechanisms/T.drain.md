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
| `Drain.Amount` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:428 | check_0A_use | — |
| `Drain.Local` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:427 | check_0A_use | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 508D32 | `HouseClass_UpdatePower_LocalDrain1` | kept | 00508C30 `HouseClass::HouseClass__AI_AssessPower` | decompiled/functions/10024690.c | src/Ext/House/Hooks.cpp:410 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10023D20 | `BuildingClass::GetPowerOutput` | 被钩子调用 | decompiled/functions/10023D20.c |
