# `S.dominator` Dominator 心控超武

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core,W.mctemporal | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（12）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Dominator.Capture` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:76 | unscanned | — |
| `Dominator.CaptureImmuneToPsionics` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:80 | unscanned | — |
| `Dominator.CaptureMindControlled` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:78 | unscanned | — |
| `Dominator.CapturePermaMindControlled` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:79 | unscanned | — |
| `Dominator.ControlAnim` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:74 | unscanned | — |
| `Dominator.FireAtPercentage` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:75 | unscanned | — |
| `Dominator.FirstAnim` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:72 | unscanned | — |
| `Dominator.FirstAnimHeight` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:70 | unscanned | — |
| `Dominator.PermanentCapture` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:81 | unscanned | — |
| `Dominator.Ripple` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:77 | unscanned | — |
| `Dominator.SecondAnim` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:73 | unscanned | — |
| `Dominator.SecondAnimHeight` | SWType | — | — | src/Misc/SWTypes/Dominator.cpp:71 | unscanned | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

