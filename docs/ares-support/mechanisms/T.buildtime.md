# `T.buildtime` 建造时间与低电惩罚

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `BuildTime.Cost` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:543 | ok | — |
| `BuildTime.LowPowerPenalty` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:544 | ok | — |
| `BuildTime.MaxLowPower` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:546 | ok | — |
| `BuildTime.MinLowPower` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:545 | ok | — |
| `BuildTime.MultipleFactory` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:547 | ok | — |
| `BuildTime.Speed` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:542 | ok | — |
| `FactoryPlant.Multiplier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:519 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6F47A0 | `TechnoClass_GetBuildTime` | new_in_3.0 | ✓ | 006F47A0 `TechnoClass::TechnoClass__GetProductionTime` | decompiled/functions/10041AB0.c | src/Ext/TechnoType/Hooks.cpp:322 | src/rules/ruleset.rs, src/sim/production/factory.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10041540 | `HouseClass::HouseClass__GetBuildTimeBonus` | 被钩子调用 | decompiled/functions/10041540.c |
| 10041520 | `HouseClass::GetPowerPercentage` | 被钩子调用 | decompiled/functions/10041520.c |
| 10041500 | `HouseClass::HouseClass__GetFactoryCount` | 被钩子调用 | decompiled/functions/10041500.c |
