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
| `SecretLab.GenerateOnCapture` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:200 | ok | — |
| `SecretLab.PossibleBoons` | BuildingType | — | — | src/Ext/BuildingType/Body.cpp:199 | unscanned | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10013700 | BuildingExt::ExtData::UpdateSecretLab [source_0.A,3.0✓] | 使用键 | decompiled/functions/10013700.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
