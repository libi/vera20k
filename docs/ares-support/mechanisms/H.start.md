# `H.start` 多人开局单位与基地车

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `StartInMultiplayer.Types` | HouseType | — | — | src/Ext/HouseType/Body.cpp:332 | unscanned | — |
| `StartInMultiplayer.WithConst` | HouseType | Valueable<bool> | false | src/Ext/HouseType/Body.cpp:333 | no_use_found | — |
| `StartInMultiplayerUnitCost` | Rules | Nullable<int> | (empty) | src/Ext/Rules/Body.cpp:180 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 5D6D9A | `MPGameModeClass_CreateStartingUnits_UnitCost` | new_in_3.0 | ✓ | 005D6D80 `MultiplayerGameMode__Generate_Starting_Units` | decompiled/functions/1001EE10.c | src/Ext/HouseType/Hooks.StartInMultiplayer.cpp:141 | src/sim/scenario_bootstrap.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001E5C0 | `MPGameModeClass_CreateStartingUnits_UnitCost::AddTypes` | 被钩子调用 | decompiled/functions/1001E5C0.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
