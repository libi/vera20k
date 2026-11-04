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
| `Degrade.Amount` | BuildingType | Nullable<int> | (empty) | src/Ext/BuildingType/Body.cpp:309 | check_0A_use | — |
| `Degrade.Percentage` | BuildingType | Nullable<double> | (empty) | src/Ext/BuildingType/Body.cpp:310 | check_0A_use | — |
| `Degrades` | HouseType | Nullable<bool> | (empty) | src/Ext/HouseType/Body.cpp:326 | no_use_found | — |
| `Degrade.AmountConsumer` | Rules | Valueable<int> | 1 | src/Ext/Rules/Body.cpp:207 | no_use_found | — |
| `Degrade.AmountNormal` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:206 | no_use_found | — |
| `Degrade.Enabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:204 | check_0A_use | — |
| `Degrade.Percentage` | Rules | Nullable<double> | (empty) | src/Ext/Rules/Body.cpp:205 | check_0A_use | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4F8B08 | `HouseClass_Update_DamageDelay` | kept ⚠改过 | 004F8440 `HouseClass::HouseClass__Update` | decompiled/functions/100247A0.c | src/Ext/House/Hooks.cpp:222 | src/app/match_runtime/scenario_exit.rs, src/app/presentation/sidebar_gadgets.rs, src/sim/house_eva.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
