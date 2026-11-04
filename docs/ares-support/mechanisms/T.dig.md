# `T.dig` 地下单位钻入钻出

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（5）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SubterraneanSpeakDelay` | Rules | Valueable<double> | 0.0 | src/Ext/Rules/Body.cpp:172 | ok | — |
| `DigIn` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:524 | ok | — |
| `DigInSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:522 | ok | — |
| `DigOut` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:525 | ok | — |
| `DigOutSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:523 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 70DA95 | `TechnoClass_RadarTrackingUpdate_AnnounceDetected` | kept | ✓ | 0070D990 `TechnoClass::TechnoClass__IdleAnimDispatch` | decompiled/functions/10016E60.c | src/Ext/Building/Hooks.Sensor.cpp:27 | src/render/radar/geometry.rs, src/render/radar/tracker.rs, src/sim/world/lifecycle.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100443E0 | PlayDigEffects [translated] | 使用键 | decompiled/functions/100443E0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
