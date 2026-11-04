# `B.degrade` 建筑退化

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
| `Degrade.Amount` | BuildingType | Nullable<int> | (empty) | src/Ext/BuildingType/Body.cpp:305 | ok | — |
| `Degrade.Percentage` | BuildingType | Nullable<double> | (empty) | src/Ext/BuildingType/Body.cpp:306 | ok | — |
| `Degrades` | HouseType | Nullable<bool> | (empty) | src/Ext/HouseType/Body.cpp:329 | no_use_found | — |
| `Degrade.AmountConsumer` | Rules | Valueable<int> | 1 | src/Ext/Rules/Body.cpp:162 | no_use_found | — |
| `Degrade.AmountNormal` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:161 | no_use_found | — |
| `Degrade.Enabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:159 | ok | — |
| `Degrade.Percentage` | Rules | Nullable<double> | (empty) | src/Ext/Rules/Body.cpp:160 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4F8B08 | `HouseClass_Update_DamageDelay` | kept ⚠改过 | ✓ | 004F8440 `HouseClass::HouseClass__Update` | decompiled/functions/100247A0.c | src/Ext/House/Hooks.cpp:241 | src/app/match_runtime/scenario_exit.rs, src/app/presentation/sidebar_gadgets.rs, src/sim/house_eva.rs … |
