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
| `SubterraneanSpeakDelay` | Rules | Valueable<double> | 0.0 | src/Ext/Rules/Body.cpp:217 | check_0A_use | — |
| `DigIn` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:520 | ok | — |
| `DigInSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:518 | no_use_found | — |
| `DigOut` | TechnoType | Nullable<AnimTypeClass*> | (empty) | src/Ext/TechnoType/Body.cpp:521 | no_use_found | — |
| `DigOutSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:519 | no_use_found | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 70DA95 | `TechnoClass_RadarTrackingUpdate_AnnounceDetected` | kept | 0070D990 `TechnoClass::TechnoClass__IdleAnimDispatch` | decompiled/functions/10016E60.c | src/Ext/Building/Hooks.Sensor.cpp:9 | src/render/radar/geometry.rs, src/render/radar/tracker.rs, src/sim/world/lifecycle.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 100443E0 | PlayDigEffects [translated] | 使用键 | decompiled/functions/100443E0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
