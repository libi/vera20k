# `F.globalcontrols` [GlobalControls] 与全局开关

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Graphics.Advanced` | Ares.GlobalControls | — | — | src/Ares.GlobalControls.cpp:26 | unscanned | — |
| `AllowBypassBuildLimit` | Rules | — | — | src/Ext/Rules/Body.cpp:102 | unscanned | src/rules/native_processing.rs |
| `AllowParallelAIQueues` | Rules | — | — | src/Ext/Rules/Body.cpp:100 | unscanned | src/rules/phobos_parallel_ai_queues.rs |
| `DebugKeysEnabled` | Rules | — | — | src/Ext/Rules/Body.cpp:114 | unscanned | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

