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
| `StartInMultiplayer.Types` | HouseType | — | — | src/Ext/HouseType/Body.cpp:329 | unscanned | — |
| `StartInMultiplayer.WithConst` | HouseType | Valueable<bool> | false | src/Ext/HouseType/Body.cpp:330 | no_use_found | — |
| `StartInMultiplayerUnitCost` | Rules | Nullable<int> | (empty) | src/Ext/Rules/Body.cpp:225 | ok | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 5D6D9A | `MPGameModeClass_CreateStartingUnits_UnitCost` | new_in_3.0 | 005D6D80 `MultiplayerGameMode__Generate_Starting_Units` | decompiled/functions/1001EE10.c | src/Recovered/MPGameModeClass.cpp:69 | src/sim/scenario_bootstrap.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1001E900 | `SWTypeExt::ExtData::IsCellEligible` | 被钩子调用 | decompiled/functions/1001E900.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
