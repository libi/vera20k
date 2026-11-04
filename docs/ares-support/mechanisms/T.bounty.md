# `T.bounty` 击杀赏金

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `GivesBounty` | HouseType | Valueable<bool> | true | src/Ext/HouseType/Body.cpp:331 | no_use_found | src/rules/native_processing.rs |
| `BountyDisplay` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:291 | ok | src/rules/ares_bounty.rs |
| `BountyEnablers` | Rules | ValueableVector<BuildingTypeClass*> | (empty) | src/Ext/Rules/Body.cpp:290 | ok | src/rules/native_processing.rs |
| `Bounty` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:499 | ok | src/rules/ares_bounty.rs |
| `Bounty.%sValue` | TechnoType | Promotable<int> | 0 | src/Ext/TechnoType/Body.cpp:498 | no_use_found | src/rules/ares_bounty.rs |
| `Bounty.Display` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:500 | ok | src/rules/ares_bounty.rs |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 702E64 | `TechnoClass_RegisterDestruction_Bounty` | new_in_3.0 | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/1004B100.c | src/Recovered/TechnoClass.cpp:540 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10043230 | TechnoExt::ExtData::GiveBounty [translated] | 使用键 | decompiled/functions/10043230.c |
| 100130C0 | `HouseClass::TakeMoney` | 被钩子调用 | decompiled/functions/100130C0.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
