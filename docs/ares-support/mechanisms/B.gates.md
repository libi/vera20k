# `B.gates` 门音效与可通行建筑

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `GateDownSound` | BuildingType | NullableIdx<VocClass> | (empty) | src/Ext/BuildingType/Body.cpp:283 | ok | — |
| `GateUpSound` | BuildingType | NullableIdx<VocClass> | (empty) | src/Ext/BuildingType/Body.cpp:284 | ok | — |
| `IsPassable` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:286 | ok | — |

## 钩子（8）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 440709 | `BuildingClass_Put` | kept | ✓ | 00440580 `BuildingClass::BuildingClass__Unlimbo` | decompiled/functions/100156F0.c | src/Ext/Building/Hooks.Gates.cpp:8 | src/sim/base_plan.rs, src/sim/world/lifecycle.rs |
| 44E550 | `BuildingClass_Mi_Open_GateDown` | kept | ✓ | 0044E440 `BuildingClass::BuildingClass__Mission_Open` | decompiled/functions/10015670.c | src/Ext/Building/Hooks.Gates.cpp:77 | src/sim/gate_runtime.rs |
| 44E61E | `BuildingClass_Mi_Open_GateUp` | kept | ✓ | 0044E440 `BuildingClass::BuildingClass__Mission_Open` | decompiled/functions/100156B0.c | src/Ext/Building/Hooks.Gates.cpp:86 | src/sim/gate_runtime.rs |
| 47C8AB | `CellClass_CanThisExistHere_GateOnWall` | kept | ✓ | 0047C620 `CellClass::CellClass__Is_Clear_To_Build` | decompiled/functions/10015780.c | src/Ext/Building/Hooks.Gates.cpp:23 | src/map/overlay_mark.rs, src/sim/build_site.rs, src/sim/world/lifecycle.rs … |
| 480534 | `CellClass_AttachesToNeighbourOverlay` | kept | ✓ | 00480510 `CellClass::CellClass__IsWallConnectableInDirection` | decompiled/functions/10015740.c | src/Ext/Building/Hooks.Gates.cpp:16 | src/sim/cell_kernel.rs, src/sim/combat/line_of_fire.rs, src/sim/world/bounce_terrain.rs |
| 51BD4C | `InfantryClass_Update` | kept ⚠改过 | ✓ | 0051BAB0 `InfantryClass::InfantryClass__AI` | decompiled/functions/10014DB0.c | src/Ext/Building/Hooks.Firewall.cpp:194 | src/sim/mission/authority.rs, src/sim/movement/infantry_action.rs, src/sim/world/infantry_death.rs … |
| 51C4C8 | `InfantryClass_IsCellOccupied` | kept ⚠改过 | ✓ | 0051BF90 `InfantryClass::InfantryClass__Can_Enter_Cell` | decompiled/functions/10014D70.c | src/Ext/Building/Hooks.Firewall.cpp:219 | src/sim/aircraft/drop_payload.rs, src/sim/movement/foot_path.rs, src/sim/movement/infantry_action.rs … |
| 73F7B0 | `UnitClass_IsCellOccupied` | kept ⚠改过 | ✓ | 0073F0A0 `UnitClass::UnitClass__Can_Enter_Cell` | decompiled/functions/10015100.c | src/Ext/Building/Hooks.Firewall.cpp:244 | src/sim/combat/combat_weapon.rs, src/sim/movement/foot_path.rs, src/sim/movement/fresh_oracle_seam.rs … |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00440580 `BuildingClass::BuildingClass__Unlimbo` | 5 | B.firestorm, B.gates, T.spotlight, ext:Building |
