# `T.convert` 类型转换（Convert.*）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Convert.Deploy` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:553 | ok | — |
| `Convert.Land` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:555 | ok | src/rules/native_processing.rs |
| `Convert.Script` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:556 | ok | src/rules/native_processing.rs |
| `Convert.Water` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:554 | ok | src/rules/native_processing.rs |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10039C30 | (file-local)::ExecuteScriptAction [translated] | 使用键 | decompiled/functions/10039C30.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020D20 | `HouseClass::FindByCountryIndex` | 被钩子调用 | decompiled/functions/10020D20.c |
| 100434C0 | `TechnoExt::ExtData::CanKillDriver` | 被钩子调用 | decompiled/functions/100434C0.c |
| 100456D0 | `TechnoExt::ExtData::KillDriver` | 被钩子调用 | decompiled/functions/100456D0.c |
| 10043650 | `TechnoExt::ConvertToType` | 被钩子调用 | decompiled/functions/10043650.c |
