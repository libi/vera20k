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
| `Message.SilosNeeded` | Rules | Valueable<CSFText> | — | src/Ext/Rules/Body.cpp:157 | no_use_found | — |
| `Harvester.KickDelay` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:563 | ok | — |
| `Harvester.LongScan` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:559 | ok | — |
| `Harvester.ScanCorrection` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:561 | ok | — |
| `Harvester.ShortScan` | TechnoType | Nullable<Leptons> | (empty) | src/Ext/TechnoType/Body.cpp:560 | ok | — |
| `Harvester.TooFarDistance` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:562 | ok | — |
| `Refinery.UseStorage` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:408 | ok | — |

## 钩子（12）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6AF748 | `SlaveManagerClass_UpdateSlaves_SlaveScan` | new_in_3.0 | ✓ | 006AF6C0 `SlaveManagerClass::SlaveManagerClass__AI_Update` | decompiled/functions/1004D560.c | src/Ext/Techno/Hooks.Harvester.cpp:142 | src/sim/slave_manager.rs |
| 6AFDFC | `SlaveManagerClass_UpdateMiner_LongScan` | new_in_3.0 | ✓ | 006B02C0 `FUN_006b02c0` | decompiled/functions/1004D470.c | src/Ext/Techno/Hooks.Harvester.cpp:113 | — |
| 6B006D | `SlaveManagerClass_UpdateMiner_ShortScan` | new_in_3.0 | ✓ | 006B0260 `FUN_006b0260` | decompiled/functions/1004D510.c | src/Ext/Techno/Hooks.Harvester.cpp:78 | — |
| 6B01A3 | `SlaveManagerClass_UpdateMiner_ScanCorrection` | new_in_3.0 | ✓ | 006AFD60 `SlaveManagerClass::SlaveManagerClass__Update_State` | decompiled/functions/1004D4C0.c | src/Ext/Techno/Hooks.Harvester.cpp:128 | src/sim/slave_manager.rs |
| 6B1065 | `SlaveManagerClass_ShouldWakeUp_ShortScan` | new_in_3.0 | ✓ | 006B1020 `SlaveManagerClass::SlaveManagerClass__ShouldRecallSlaves` | decompiled/functions/1004D3F0.c | src/Ext/Techno/Hooks.Harvester.cpp:94 | src/sim/slave_manager.rs |
| 709B4E | `TechnoClass_DrawPipscale_SkipSkipTiberium` | kept | ✓ | 00709A90 `DrawPipScalePips` | decompiled/functions/100148B0.c | src/Ext/Building/Hooks.cpp:95 | src/app/presentation/render/object_extras.rs, src/app/presentation/ui_overlays.rs |
| 73E851 | `UnitClass_Mi_Harvest_LongScan` | new_in_3.0 | ✓ | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D640.c | src/Ext/Techno/Hooks.Harvester.cpp:29 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73E9F1 | `UnitClass_Mi_Harvest_ShortScan` | new_in_3.0 | ✓ | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D680.c | src/Ext/Techno/Hooks.Harvester.cpp:41 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73EC0E | `UnitClass_Mi_Harvest_TooFarDistance1` | new_in_3.0 | ✓ | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D6D0.c | src/Ext/Techno/Hooks.Harvester.cpp:55 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 73EE40 | `UnitClass_Mi_Harvest_TooFarDistance2` | new_in_3.0 | ✓ | 0073E5E0 `UnitClass::UnitClass__Mission_Harvest` | decompiled/functions/1004D710.c | src/Ext/Techno/Hooks.Harvester.cpp:66 | src/sim/game_entity.rs, src/sim/house_state.rs, src/sim/miner/miner_system.rs … |
| 74081F | `UnitClass_Mi_Guard_KickFrameDelay` | new_in_3.0 | ✓ | 00740810 `UnitClass::UnitClass__Mission_Guard` | decompiled/functions/1004D5F0.c | src/Ext/Techno/Hooks.Harvester.cpp:156 | src/sim/house_state.rs, src/sim/miner/miner_system.rs, src/sim/world/techno_ai/mission_handlers.rs |
| 74410D | `UnitClass_Mi_AreaGuard_KickFrameDelay` | new_in_3.0 | ✓ | 00744100 `UnitClass::UnitClass__Mission_AreaGuard` | decompiled/functions/1004D5A0.c | src/Ext/Techno/Hooks.Harvester.cpp:173 | src/sim/world/techno_ai/mission_handlers.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10044230 | TechnoExt::ExtData::DepositTiberium [translated] | 使用键 | decompiled/functions/10044230.c |
| 100461B0 | TechnoExt::ExtData::RefineTiberium [translated] | 使用键 | decompiled/functions/100461B0.c |
| 10045390 | `HouseClass::GiveTiberium` | 被钩子调用 | decompiled/functions/10045390.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
