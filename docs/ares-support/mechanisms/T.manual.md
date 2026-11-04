# `T.manual` 禁止手动开火/进入/卸载/守卫、不可出售

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `MassSelectable` | BuildingType | Nullable<bool> | (empty) | src/Ext/BuildingType/Body.cpp:317 | ok | — |
| `UnitsUnsellable` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:174 | ok | — |
| `NoManualEnter` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:496 | ok | — |
| `NoManualFire` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:495 | ok | src/rules/object_type.rs |
| `NoManualUnload` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:494 | ok | — |
| `NoSelfGuardArea` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:497 | ok | — |
| `Unsellable` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:565 | ok | src/rules/object_type.rs |

## 钩子（6）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 459C03 | `BuildingClass_CanBeSelectedNow_MassSelectable` | new_in_3.0 | ✓ | 00459C00 `BuildingClass::BuildingClass__CanBeSelectedNow` | decompiled/functions/10013CF0.c | src/Ext/Building/Hooks.cpp:432 | src/rules/ruleset.rs |
| 4D9EBD | `FootClass_CanBeSold_SellUnit` | new_in_3.0 | ✓ | 004D9E70 `FootClass::FootClass__CanBeSold` | decompiled/functions/10048FD0.c | src/Ext/Techno/Hooks.cpp:3105 | — |
| 51ED8E | `InfantryClass_GetActionOnObject_Tunnel` | replaces_0.A:InfantryClass_GetCursorOverObject_NoManualEnter | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/100176E0.c | src/Ext/Building/Hooks.Tunnels.cpp:312 | src/app/input/context_order.rs |
| 700536 | `TechnoClass_GetActionOnObject_NoManualFire` | replaces_0.A:TechnoClass_GetCursorOverObject_NoManualFire | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004A460.c | src/Ext/Techno/Hooks.cpp:2375 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 7008D4 | `TechnoClass_GetActionOnCell_NoManualFire` | replaces_0.A:TechnoClass_GetCursorOverCell_NoManualFire | ✓ | 00700600 `TechnoClass::TechnoClass__What_Action_OnCell` | decompiled/functions/1004A430.c | src/Ext/Techno/Hooks.cpp:2389 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/c4_plant.rs … |
| 74031A | `UnitClass_GetActionOnObject_NoManualEnter` | replaces_0.A:UnitClass_GetCursorOverObject_NoManualEnter | ✓ | 0073FD50 `UnitClass::UnitClass__What_Action_OnObject` | decompiled/functions/1004BEA0.c | src/Ext/Techno/Hooks.cpp:2403 | src/app/input/cursor.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10013C90 | `BuildingTypeClass::BuildingTypeClass__Is1x1WithUndeploy` | 被钩子调用 | decompiled/functions/10013C90.c |
