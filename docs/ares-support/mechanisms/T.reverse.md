# `T.reverse` 逆向工程与可逆转

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
| `ReverseEngineersVictims` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:282 | check_0A_use | — |
| `SpyEffect.UndoReverseEngineer` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:256 | no_use_found | — |
| `CanBeReversed` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:338 | ok | — |
| `ReversedAs` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:339 | ok | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10012E00 | BuildingExt::ExtData::ReverseEngineer [source_0.A] | 使用键 | decompiled/functions/10012E00.c |
| 10022360 | FUN_10022360 [translated] | 使用键 | decompiled/functions/10022360.c |
| 10021B00 | `HouseExt::PrereqValidate` | 被钩子调用 | decompiled/functions/10021B00.c |
| 10021FF0 | `HouseExt::PrerequisitesMet` | 被钩子调用 | decompiled/functions/10021FF0.c |
