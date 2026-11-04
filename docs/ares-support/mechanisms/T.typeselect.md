# `T.typeselect` GroupAs 与类型选择

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `TypeSelectUseDeploy` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:189 | ok | src/rules/native_processing.rs |
| `GroupAs` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:420 | unscanned | src/rules/native_processing.rs |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 732D47 | `TacticalClass_CollectSelectedIDs` | removed_in_3.0 | 00732D10 `TypeSelect__BuildSeedTypeIDs` | decompiled/functions/10041990.c | src/Ext/TechnoType/Hooks.cpp | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 1003DB30 | `TechnoTypeExt::ExtData::GetSelectionGroupID` | 被钩子调用 | decompiled/functions/1003DB30.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
