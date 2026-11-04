# `B.misc` 建筑杂项：建造/出售时间、停靠、可回收、消息、钱数显示

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（11）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `BuildupTime` | BuildingType | Nullable<double> | (empty) | src/Ext/BuildingType/Body.cpp:313 | ok | src/rules/ruleset.rs |
| `DockUnloadCell` | BuildingType | Valueable<Point2D> | Point2D{ 3, 1 } | src/Ext/BuildingType/Body.cpp:94 | ok | — |
| `DockUnloadFacing` | BuildingType | Valueable<int> | 8 | src/Ext/BuildingType/Body.cpp:95 | ok | — |
| `LostEvaEvent` | BuildingType | ValueableIdx<VoxClass> | -1 | src/Ext/BuildingType/Body.cpp:301 | ok | — |
| `Message.Capture` | BuildingType | Valueable<CSFText> | (empty) | src/Ext/BuildingType/Body.cpp:302 | ok | — |
| `Message.Lost` | BuildingType | Valueable<CSFText> | (empty) | src/Ext/BuildingType/Body.cpp:303 | ok | — |
| `ProduceCashDisplay` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:318 | ok | — |
| `Returnable` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:276 | ok | — |
| `SellTime` | BuildingType | Nullable<double> | (empty) | src/Ext/BuildingType/Body.cpp:314 | ok | — |
| `SlamSound` | BuildingType | NullableIdx<VocClass> | (empty) | src/Ext/BuildingType/Body.cpp:323 | ok | — |
| `UnitSell` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:319 | ok | — |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 43CA80 | `BuildingClass_ReceivedRadioCommand_DockUnloadCell` | new_in_3.0 | ✓ | 0043C2D0 `BuildingClass::BuildingClass__Receive_Radio` | decompiled/functions/10014540.c | src/Ext/Building/Hooks.cpp:518 | src/sim/docking/building_dock.rs, src/sim/passenger.rs, src/sim/radio/receive.rs … |
| 44840B | `BuildingClass_ChangeOwnership_Tech` | kept | ✓ | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10013E40.c | src/Ext/Building/Hooks.cpp:170 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 4D9EBD | `FootClass_CanBeSold_SellUnit` | new_in_3.0 | ✓ | 004D9E70 `FootClass::FootClass__CanBeSold` | decompiled/functions/10048FD0.c | src/Ext/Techno/Hooks.cpp:3105 | — |
| 4FC731 | `HouseClass_DestroyAll_ReturnStructures` | kept | ✓ | 004FC6D0 `HouseClass::HouseClass__Blowup_All` | decompiled/functions/10024150.c | src/Ext/House/Hooks.cpp:480 | src/sim/temporal.rs, src/sim/world/house_defeat.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10044C00 | TechnoExt::GetDockUnloadFacing [translated] | 使用键 | decompiled/functions/10044C00.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 10023D60 | `BuildingClass::KillOccupants` | 被钩子调用 | decompiled/functions/10023D60.c |
| 10020F20 | `SideClass::FindIndex` | 被钩子调用 | decompiled/functions/10020F20.c |
| 10023D00 | `BuildingClass::DisableStuff` | 被钩子调用 | decompiled/functions/10023D00.c |
| 100219C0 | `RadioClass::HasAnyLink` | 被钩子调用 | decompiled/functions/100219C0.c |
