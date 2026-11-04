# `T.harvester` 采矿车扫描与精炼厂存储

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Message.SilosNeeded` | Rules | Valueable<CSFText> | — | src/Ext/Rules/Body.cpp:202 | no_use_found | — |
| `Harvester.KickDelay` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:559 | ok | — |
| `Harvester.LongScan` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:555 | ok | — |
| `Harvester.ScanCorrection` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:557 | ok | — |
| `Harvester.ShortScan` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:556 | ok | — |
| `Harvester.TooFarDistance` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:558 | ok | — |
| `Refinery.UseStorage` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:404 | check_0A_use | — |

## 钩子（12）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6AF748 | `SlaveManagerClass_UpdateSlaves_SlaveScan` | new_in_3.0 | 006AF6C0 `SlaveManagerClass::SlaveManagerClass__AI_Update` | decompiled/functions/1004D560.c | src/Recovered/SlaveManagerClass.cpp:82 | src/sim/slave_manager.rs |
| 6AFDFC | `SlaveManagerClass_UpdateMiner_LongScan` | new_in_3.0 | 006B02C0 `FUN_006b02c0` | decompiled/functions/1004D470.c | src/Recovered/SlaveManagerClass.cpp:53 | — |
| 6B006D | `SlaveManagerClass_UpdateMiner_ShortScan` | new_in_3.0 | 006B0260 `FUN_006b0260` | decompiled/functions/1004D510.c | src/Recovered/SlaveManagerClass.cpp:18 | — |
| 6B01A3 | `SlaveManagerClass_UpdateMiner_ScanCorrection` | new_in_3.0 | 006AFD60 `SlaveManagerClass::SlaveManagerClass__Update_State` | decompiled/functions/1004D4C0.c | src/Recovered/SlaveManagerClass.cpp:68 | src/sim/slave_manager.rs |
| 6B1065 | `SlaveManagerClass_ShouldWakeUp_ShortScan` | new_in_3.0 | 006B1020 `SlaveManagerClass::SlaveManagerClass__ShouldRecallSlaves` | decompiled/functions/1004D3F0.c | src/Recovered/SlaveManagerClass.cpp:34 | src/sim/slave_manager.rs |
| 709B4E | `TechnoClass_DrawPipscale_SkipSkipTiberium` | kept | 00709A90 `DrawPipScalePips` | decompiled/functions/100148B0.c | src/Ext/Building/Hooks.cpp:71 | src/app/presentation/render/object_extras.rs, src/app/presentation/ui_overlays.rs |
| 73E851 | `UnitClass_Mi_Harvest_LongScan` | new_in_3.0 | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D640.c | src/Recovered/UnitClass.cpp:524 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73E9F1 | `UnitClass_Mi_Harvest_ShortScan` | new_in_3.0 | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D680.c | src/Recovered/UnitClass.cpp:536 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73EC0E | `UnitClass_Mi_Harvest_TooFarDistance1` | new_in_3.0 | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D6D0.c | src/Recovered/UnitClass.cpp:550 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73EE40 | `UnitClass_Mi_Harvest_TooFarDistance2` | new_in_3.0 | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D710.c | src/Recovered/UnitClass.cpp:561 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 74081F | `UnitClass_Mi_Guard_KickFrameDelay` | new_in_3.0 | 00740810 `UnitClass::UnitClass__Mission_Guard` | decompiled/functions/1004D5F0.c | src/Recovered/UnitClass.cpp:574 | src/sim/house_state.rs, src/sim/miner/miner_system.rs, src/sim/world/techno_ai/mission_handlers.rs |
| 74410D | `UnitClass_Mi_AreaGuard_KickFrameDelay` | new_in_3.0 | 00744100 `UnitClass::UnitClass__Mission_AreaGuard` | decompiled/functions/1004D5A0.c | src/Recovered/UnitClass.cpp:591 | src/sim/world/techno_ai/mission_handlers.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 10044230 | TechnoExt::ExtData::DepositTiberium [source_0.A] | 使用键 | decompiled/functions/10044230.c |
| 100461B0 | TechnoExt::ExtData::RefineTiberium [source_0.A] | 使用键 | decompiled/functions/100461B0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10045390 | `HouseClass::GiveTiberium` | 被钩子调用 | decompiled/functions/10045390.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
