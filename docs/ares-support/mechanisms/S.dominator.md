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
| `Dominator.Capture` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:77 | ok | — |
| `Dominator.CaptureImmuneToPsionics` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:81 | no_use_found | — |
| `Dominator.CaptureMindControlled` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:79 | no_use_found | — |
| `Dominator.CapturePermaMindControlled` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:80 | no_use_found | — |
| `Dominator.ControlAnim` | SWType | Nullable<AnimTypeClass*> | — | src/Misc/SWTypes/Dominator.cpp:75 | no_use_found | — |
| `Dominator.FireAtPercentage` | SWType | Nullable<int> | — | src/Misc/SWTypes/Dominator.cpp:76 | ok | — |
| `Dominator.FirstAnim` | SWType | Nullable<AnimTypeClass*> | — | src/Misc/SWTypes/Dominator.cpp:73 | ok | — |
| `Dominator.FirstAnimHeight` | SWType | Valueable<int> | 0 | src/Misc/SWTypes/Dominator.cpp:71 | ok | — |
| `Dominator.PermanentCapture` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:82 | needs_check | — |
| `Dominator.Ripple` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Dominator.cpp:78 | needs_check | — |
| `Dominator.SecondAnim` | SWType | Nullable<AnimTypeClass*> | — | src/Misc/SWTypes/Dominator.cpp:74 | ok | — |
| `Dominator.SecondAnimHeight` | SWType | Valueable<int> | 0 | src/Misc/SWTypes/Dominator.cpp:72 | ok | — |

## 钩子（1）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 53B080 | `PsyDom_Fire` | kept | ✓ | 0053B080 `PsychicDominator__MindControlArea` | decompiled/functions/10037840.c | src/Ext/SWType/Hooks.Legacy.cpp:30 | src/rules/effect_asset_catalog.rs, src/rules/mind_control_rules.rs, src/rules/ruleset.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001BC20 | FUN_1001bc20 [missing,inferred,3.0✓] | 使用键 | decompiled/functions/1001BC20.c |
| 10070BA0 | PsychicDominatorStateMachine::Update [translated] | 使用键 | decompiled/functions/10070BA0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1002E060 | `SideExt::ExtData::GetParaDropTypes` | 被钩子调用 | decompiled/functions/1002E060.c |
| 1002DFB0 | `SideExt::ExtData::GetParaDropNum` | 被钩子调用 | decompiled/functions/1002DFB0.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 10035FC0 | `IonBlastClass::IonBlastClass` | 被钩子调用 | decompiled/functions/10035FC0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 100326B0 | `SWTypeExt::ExtData::GetDamage` | 被钩子调用 | decompiled/functions/100326B0.c |
| 10032B40 | `SWTypeExt::ExtData::GetWarhead` | 被钩子调用 | decompiled/functions/10032B40.c |
| 10032980 | `SWTypeExt::ExtData::GetRange` | 被钩子调用 | decompiled/functions/10032980.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 100708B0 | `PsyDom::Fire` | 被钩子调用 | decompiled/functions/100708B0.c |
