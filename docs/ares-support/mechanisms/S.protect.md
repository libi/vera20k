# `S.protect` Protect（铁幕/力场）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core,T.ic | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Protect.Duration` | SWType | — | — | src/Misc/SWTypes/Protect.cpp:77 | unscanned | — |
| `Protect.PlayFadeSoundTime` | SWType | — | — | src/Misc/SWTypes/Protect.cpp:79 | unscanned | — |
| `Protect.PowerOutage` | SWType | — | — | src/Misc/SWTypes/Protect.cpp:78 | unscanned | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

