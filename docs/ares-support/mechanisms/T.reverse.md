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
| `ReverseEngineersVictims` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:278 | ok | — |
| `SpyEffect.UndoReverseEngineer` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:252 | ok | — |
| `CanBeReversed` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:342 | ok | — |
| `ReversedAs` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:343 | ok | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10012E00 | BuildingExt::ExtData::ReverseEngineer [translated] | 使用键 | decompiled/functions/10012E00.c |
| 10022360 | FUN_10022360 [translated] | 使用键 | decompiled/functions/10022360.c |
| 10021B00 | `HouseExt::PrereqValidate` | 被钩子调用 | decompiled/functions/10021B00.c |
| 10021FF0 | `HouseExt::ExtData::RequirementsMet` | 被钩子调用 | decompiled/functions/10021FF0.c |
