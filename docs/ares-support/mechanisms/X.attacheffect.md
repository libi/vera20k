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
| `AttachEffect.Animation` | AttachEffect | Valueable<AnimTypeClass*>::Read | — | src/Misc/AttachEffect.cpp:77 | ok | src/rules/native_processing.rs |
| `AttachEffect.AnimResetOnReapply` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:78 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.ArmorMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:84 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.Cloakable` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:87 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.Cumulative` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:76 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.Damage` | AttachEffect | — | — | src/Misc/AttachEffect.cpp:90 | unscanned | — |
| `AttachEffect.DamageDelay` | AttachEffect | — | — | src/Misc/AttachEffect.cpp:91 | unscanned | — |
| `AttachEffect.Delay` | AttachEffect | Valueable<int>::Read | — | src/Misc/AttachEffect.cpp:95 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.DiscardOnEntry` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:82 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.Duration` | AttachEffect | Valueable<int>::Read | — | src/Misc/AttachEffect.cpp:75 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.FirepowerMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:83 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.ForceDecloak` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:81 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.InitialDelay` | AttachEffect | Valueable<int>::Read | — | src/Misc/AttachEffect.cpp:96 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.PenetratesIronCurtain` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:80 | ok | src/rules/ares_warhead.rs |
| `AttachEffect.ROFMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:85 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.SpeedMultiplier` | AttachEffect | Valueable<double>::Read | — | src/Misc/AttachEffect.cpp:86 | no_use_found | src/rules/ares_warhead.rs |
| `AttachEffect.TemporalHidesAnim` | AttachEffect | Valueable<bool>::Read | — | src/Misc/AttachEffect.cpp:79 | no_use_found | src/rules/ares_warhead.rs |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100361B0 | LightningStorm_Update::<lambda_GetRandomCoords> [translated,inferred] | 使用键 | decompiled/functions/100361B0.c |
| 10058520 | AttachEffectTypeClass::Attach [translated,inferred] | 使用键 | decompiled/functions/10058520.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10036430 | `CellClass::FindTechnoNearestTo` | 被钩子调用 | decompiled/functions/10036430.c |
| 100586E0 | `AttachEffectClass::CreateAnim` | 被钩子调用 | decompiled/functions/100586E0.c |
| 100460A0 | `TechnoExt::ExtData::RecalculateStats` | 被钩子调用 | decompiled/functions/100460A0.c |
