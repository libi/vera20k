# `B.secretlab` 秘密实验室

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SecretLab.GenerateOnCapture` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:205 | check_0A_use | — |
| `SecretLab.PossibleBoons` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:204 | unscanned | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10013700 | BuildingExt::ExtData::UpdateSecretLab [source_0.A] | 使用键 | decompiled/functions/10013700.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
