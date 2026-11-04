# `B.occupy` 驻军（UC）与掠夺

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（5）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Bunker.Raidable` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:211 | check_0A_use | — |
| `CanBeOccupiedBy` | BuildingType | ValueableVector<InfantryTypeClass *> | (empty) | src/Ext/BuildingType/Body.cpp:278 | check_0A_use | — |
| `UC.DamageMultiplier` | BuildingType | Valueable<double> | 1.0 | src/Ext/BuildingType/Body.cpp:210 | check_0A_use | — |
| `UC.FatalRate` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:209 | check_0A_use | — |
| `UC.PassThrough` | BuildingType | Valueable<double> | 0.0 | src/Ext/BuildingType/Body.cpp:208 | check_0A_use | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 457D58 | `BuildingClass_CanBeOccupied_SpecificOccupiers` | kept | 00457CE0 `BuildingClass::BuildingClass__CanBeOccupiedBy` | decompiled/functions/10017060.c | src/Ext/Building/Hooks.Trenches.cpp:17 | src/sim/building_entry.rs, src/sim/combat/greatest_threat.rs, src/sim/passenger.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000D2F0 | BuildingTypeExt::ExtData::CanBeOccupiedBy [source_0.A] | 使用键 | decompiled/functions/1000D2F0.c |
| 10019AC0 | FUN_10019ac0 [source_0.A] | 使用键 | decompiled/functions/10019AC0.c |
| 1002DFB0 | FUN_1002dfb0 [source_0.A,inferred] | 使用键 | decompiled/functions/1002DFB0.c |
| 10016F40 | `ObjectClass::IsRedHP` | 被钩子调用 | decompiled/functions/10016F40.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10019D00 | `TechnoClass::UpdateThreatInCell` | 被钩子调用 | decompiled/functions/10019D00.c |
| 100136C0 | `BuildingExt::ExtData::evalRaidStatus` | 被钩子调用 | decompiled/functions/100136C0.c |
