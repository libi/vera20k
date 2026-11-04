# `S.mutator` GeneticMutator

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Mutate.Explosion` | SWType | Nullable<bool> | — | src/Misc/SWTypes/GeneticMutator.cpp:87 | no_use_found | — |
| `Mutate.IgnoreCyborg` | SWType | Valueable<bool> | false | src/Misc/SWTypes/GeneticMutator.cpp:88 | ok | — |
| `Mutate.IgnoreNotHuman` | SWType | Valueable<bool> | false | src/Misc/SWTypes/GeneticMutator.cpp:89 | ok | — |
| `Mutate.KillNatural` | SWType | Valueable<bool> | false | src/Misc/SWTypes/GeneticMutator.cpp:90 | no_use_found | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

