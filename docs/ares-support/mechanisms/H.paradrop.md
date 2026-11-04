# `H.paradrop` ParaDrop 覆盖链 Country→Side→Rules

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | docs/ares-country-paradrop.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（8）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `ParaDrop.Aircraft` | HouseType | ValueableIdx<AircraftTypeClass> | -1 | src/Ext/HouseType/Body.cpp:309 | no_use_found | src/rules/native_processing.rs |
| `ParaDrop.Num` | HouseType | ValueableVector<int> | — | src/Ext/HouseType/Body.cpp:316 | no_use_found | src/rules/native_processing.rs |
| `ParaDrop.Types` | HouseType | ValueableVector<TechnoTypeClass*> | — | src/Ext/HouseType/Body.cpp:311 | no_use_found | src/rules/native_processing.rs |
| `ParaDrop.Types` | HouseType | ValueableVector<TechnoTypeClass*> | — | src/Ext/HouseType/Body.cpp:311 | no_use_found | src/rules/native_processing.rs |
| `ParaDrop.Aircraft` | Side | ValueableIdx<AircraftTypeClass> | -1 | src/Ext/Side/Body.cpp:117 | check_0A_use | src/rules/native_processing.rs |
| `ParaDrop.Num` | Side | NullableVector<int> | — | src/Ext/Side/Body.cpp:124 | check_0A_use | src/rules/native_processing.rs |
| `ParaDrop.Types` | Side | NullableVector<TechnoTypeClass*> | — | src/Ext/Side/Body.cpp:119 | no_use_found | src/rules/native_processing.rs |
| `ParaDrop.Types` | Side | NullableVector<TechnoTypeClass*> | — | src/Ext/Side/Body.cpp:119 | no_use_found | src/rules/native_processing.rs |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10030570 | SWTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/10030570.c |
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 10051730 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10051730.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
