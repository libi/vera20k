# `F.cursors` [MouseCursors] 与各类 Cursor.* 键

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（12）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Cursor.Spy` | BuildingType | Valueable<MouseCursorType> | MouseCursorType::Enter | src/Ext/BuildingType/Body.cpp:324 | ok | — |
| `MouseCursors` | Rules | — | — | src/Ext/Rules/Body.cpp:40 | unscanned | — |
| `Cursor` | SWType | Valueable<MouseCursorType> | MouseCursorType::Attack | src/Ext/SWType/Body.cpp:613 | ok | src/app/diagnostics/shell_capture.rs, src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/opening.rs … |
| `NoCursor` | SWType | Valueable<MouseCursorType> | MouseCursorType::Disallowed | src/Ext/SWType/Body.cpp:614 | ok | — |
| `Cursor.Deploy` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Deploy | src/Ext/TechnoType/Body.cpp:534 | ok | — |
| `Cursor.Enter` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Enter | src/Ext/TechnoType/Body.cpp:536 | ok | — |
| `Cursor.Move` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Move | src/Ext/TechnoType/Body.cpp:538 | ok | — |
| `Cursor.NoDeploy` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoDeploy | src/Ext/TechnoType/Body.cpp:535 | ok | — |
| `Cursor.NoEnter` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoEnter | src/Ext/TechnoType/Body.cpp:537 | ok | — |
| `Cursor.NoMove` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoMove | src/Ext/TechnoType/Body.cpp:539 | ok | — |
| `Cursor.Attack` | WeaponType | Valueable<MouseCursorType> | MouseCursorType::Attack | src/Ext/WeaponType/Body.cpp:226 | ok | — |
| `Cursor.AttackOutOfRange` | WeaponType | Valueable<MouseCursorType> | MouseCursorType::AttackOutOfRange | src/Ext/WeaponType/Body.cpp:227 | ok | — |

## 钩子（10）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 4AB35A | `DisplayClass_SetAction_CustomCursor` | new_in_3.0 | ✓ | 004AAE90 `DisplayClass::DisplayClass__SetCursorFromAction` | decompiled/functions/10057F10.c | src/Misc/Actions.cpp:262 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/app/input/radar.rs … |
| 4ABFBE | `DisplayClass_LeftMouseButtonUp_ExecPowerToggle` | kept | ✓ | 004AB9B0 `DisplayClass::DisplayClass__BandBox_LeftUp` | decompiled/functions/10057ED0.c | src/Misc/Actions.cpp:403 | src/app/input/beacon.rs, src/app/input/camera.rs, src/app/input/commands.rs … |
| 4D7524 | `FootClass_ActionOnObject_Allow` | replaces_0.A:Actions_AllowForFootClass | ✓ | 004D74E0 `FootClass::FootClass__ClickedAction_Object` | decompiled/functions/10058390.c | src/Misc/Actions.cpp:344 | src/app/input/autoload.rs, src/app/input/commands.rs, src/app/input/context_order.rs … |
| 51EE6B | `InfantryClass_GetActionOnObject_Saboteur` | replaces_0.A:InfantryClass_GetCursorOverObject_Saboteur | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10025FC0.c | src/Ext/Infantry/Hooks.Saboteur.cpp:45 | src/app/input/context_order.rs |
| 5BDDC0 | `MouseClass_Update_Reset` | replaces_0.A:Actions_Reset | ✓ | 005BDDC0 `MouseClass::MouseClass__Action` | decompiled/functions/100583C0.c | src/Misc/Actions.cpp:261 | — |
| 653CA6 | `RadarClass_GetMouseAction_AllowMinimap` | replaces_0.A:Actions_AllowMinimap | ✓ | 006539D0 `RadarClass_RTacticalClass__Action` | decompiled/functions/100583F0.c | src/Misc/Actions.cpp:366 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/radar.rs … |
| 6929FC | `DisplayClass_ChooseAction_CanSell` | kept | ✓ | 00692610 `DisplayClass::DisplayClass__DetermineAction` | decompiled/functions/10057E70.c | src/Misc/Actions.cpp:387 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/entity_pick.rs … |
| 6FFEC0 | `TechnoClass_GetActionOnObject_Cursors` | new_in_3.0 | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004C990.c | src/Ext/Techno/Hooks.Cursor.cpp:107 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 7000CD | `TechnoClass_GetActionOnObject_SelfDeployCursor` | new_in_3.0 | ✓ | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004CA20.c | src/Ext/Techno/Hooks.Cursor.cpp:71 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 700600 | `TechnoClass_GetActionOnCell_Cursors` | new_in_3.0 | ✓ | 00700600 `TechnoClass::TechnoClass__What_Action_OnCell` | decompiled/functions/1004C920.c | src/Ext/Techno/Hooks.Cursor.cpp:129 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/c4_plant.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10033EB0 | SWTypeExt::ExtData::GetAction [translated] | 使用键 | decompiled/functions/10033EB0.c |
| 10044A80 | TechnoExt::GetAttackCursor [translated] | 使用键 | decompiled/functions/10044A80.c |
| 100252D0 | `InfantryExt::GetInfiltrateAction` | 被钩子调用 | decompiled/functions/100252D0.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
| 10026980 | `MapClass::IsLocationShrouded` | 被钩子调用 | decompiled/functions/10026980.c |
| 1006CD50 | `NewSWType::CanFireAt` | 被钩子调用 | decompiled/functions/1006CD50.c |
| 1006CDA0 | `NewSWType::CanFireAt` | 被钩子调用 | decompiled/functions/1006CDA0.c |
| 10057E40 | `Actions::Set` | 被钩子调用 | decompiled/functions/10057E40.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10007580 | `MouseCursorTypes::GetCursor` | 被钩子调用 | decompiled/functions/10007580.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004AB9B0 `DisplayClass::DisplayClass__BandBox_LeftUp` | 5 | F.cursors, S.core, ext:TechnoType |
| 005BDDC0 `MouseClass::MouseClass__Action` | 3 | F.cursors, ext:CursorTypes |
| 004D74E0 `FootClass::FootClass__ClickedAction_Object` | 2 | F.cursors, T.poweredby |
| 006539D0 `RadarClass_RTacticalClass__Action` | 2 | F.cursors, S.core |
| 00692610 `DisplayClass::DisplayClass__DetermineAction` | 2 | F.cursors, U.bugfixes |
