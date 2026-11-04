# `T.ic` 铁幕/力场/超时空修正与免疫

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M6 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `IronCurtainFlash` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:177 | ok | — |
| `Chronoshift.Allow` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:335 | ok | — |
| `Chronoshift.Crushable` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:337 | ok | — |
| `Chronoshift.IsVehicle` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:336 | ok | — |
| `ForceShield.Modifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:333 | ok | — |
| `IronCurtain.Modifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:331 | ok | — |
| `IronCurtain.Cap` | WarheadType | int | -1 | src/Ext/WarheadType/Body.cpp:105 | ok | — |
| `IronCurtain.Duration` | WarheadType | int | 0 | src/Ext/WarheadType/Body.cpp:104 | ok | — |
| `IronCurtain.Flash` | WarheadType | Nullable<bool> | (empty) | src/Ext/WarheadType/Body.cpp:106 | ok | — |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 701A5C | `TechnoClass_ReceiveDamage_IronCurtainFlash` | new_in_3.0 | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/10053B50.c | src/Ext/WarheadType/Hooks.cpp:245 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |
| 718275 | `TeleportLocomotionClass_MakeRoom` | new_in_3.0 | ✓ | 00718260 `TeleportLocomotionClass::TeleportLocomotionClass__Update_Position` | decompiled/functions/1004B980.c | src/Ext/Techno/Hooks.cpp:2664 | src/sim/movement/teleport_chrono.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10003BB0 | SyringeHandshake [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10003BB0.c |
| 1004F300 | TechnoExt::CanWarpTarget [translated,inferred] | 使用键 | decompiled/functions/1004F300.c |
| 10052890 | WarheadTypeExt::ExtData::applyIronCurtain [translated] | 使用键 | decompiled/functions/10052890.c |
| 10062320 | FUN_10062320 [translated,inferred] | 使用键 | decompiled/functions/10062320.c |
| 10070BA0 | PsychicDominatorStateMachine::Update [translated,inferred] | 使用键 | decompiled/functions/10070BA0.c |
| 10074A10 | SW_Protect::CanFireAt [translated] | 使用键 | decompiled/functions/10074A10.c |
| 10075920 | SW_UnitDelivery::Activate [translated,inferred] | 使用键 | decompiled/functions/10075920.c |
| 10076660 | `Helpers::Alex::getCellSpreadItems` | 被钩子调用 | decompiled/functions/10076660.c |
| 10051B70 | `WarheadTypeExt::CanAffectHouse` | 被钩子调用 | decompiled/functions/10051B70.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10062F50 | `TechnoExt::ExtData::IsOperated_Inline` | 被钩子调用 | decompiled/functions/10062F50.c |
| 10062D90 | `EMPulse::isEMPImmune` | 被钩子调用 | decompiled/functions/10062D90.c |
| 10062B40 | `EMPulse::enableEMPEffect` | 被钩子调用 | decompiled/functions/10062B40.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 100627F0 | `EMPulse::DisableEMPEffect` | 被钩子调用 | decompiled/functions/100627F0.c |
| 10044300 | `TechnoExt::Destroy` | 被钩子调用 | decompiled/functions/10044300.c |
| 10062FD0 | `EMPulse::UpdateSparkleAnim` | 被钩子调用 | decompiled/functions/10062FD0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 100708B0 | `PsyDom::Fire` | 被钩子调用 | decompiled/functions/100708B0.c |
| 1006CD50 | `NewSWType::CanFireAt` | 被钩子调用 | decompiled/functions/1006CD50.c |
| 1006CDA0 | `NewSWType::CanFireAt` | 被钩子调用 | decompiled/functions/1006CDA0.c |
