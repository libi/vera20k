# `X.attacheffect` AttachEffect

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | F.readers | partial | docs/ares-attach-effect.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（17）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AttachEffect.Animation` | AttachEffect | Valueable<AnimTypeClass*>::Read | — | src/Misc/AttachEffect.cpp:70 | check_0A_use | src/rules/native_processing.rs |
| `AttachEffect.AnimResetOnReapply` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:71 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.ArmorMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:76 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.Cloakable` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:78 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.Cumulative` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:69 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.Damage` | AttachEffect | — | — | src/Misc/AttachEffect.cpp:81 | unscanned | — |
| `AttachEffect.DamageDelay` | AttachEffect | — | — | src/Misc/AttachEffect.cpp:82 | unscanned | — |
| `AttachEffect.Delay` | AttachEffect | Valueable<int>::Read | — | src/Misc/AttachEffect.cpp:86 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.DiscardOnEntry` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:74 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.Duration` | AttachEffect | Valueable<int>::Read | — | src/Misc/AttachEffect.cpp:68 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.FirepowerMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:75 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.ForceDecloak` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:73 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.InitialDelay` | AttachEffect | Valueable<int>::Read | — | 10058A50 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.PenetratesIronCurtain` | AttachEffect | Valueable<bool>::Read | — | 10058A50 | check_0A_use | src/rules/ares_warhead.rs |
| `AttachEffect.ROFMultiplier` | AttachEffect | Valueable<double>::Read | — | 10058A50 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.SpeedMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:77 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.TemporalHidesAnim` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:72 | no_use_found | src/rules/ares_warhead.rs |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100361B0 | FUN_100361b0 [source_0.A,inferred] | 使用键 | decompiled/functions/100361B0.c |
| 10058520 | Helpers::Alex::getCellSpreadItems [source_0.A,inferred] | 使用键 | decompiled/functions/10058520.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10036430 | `CellClass::FindTechnoNearestTo` | 被钩子调用 | decompiled/functions/10036430.c |
| 100460A0 | `TechnoExt::ExtData::RecalculateStats` | 被钩子调用 | decompiled/functions/100460A0.c |
