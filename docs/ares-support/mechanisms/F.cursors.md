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
| `Cursor.Spy` | BuildingType | Valueable<MouseCursorType> | MouseCursorType::Enter | src/Ext/BuildingType/Body.cpp:328 | ok | — |
| `MouseCursors` | Rules | — | — | src/Ext/Rules/Body.cpp:35 | unscanned | — |
| `Cursor` | SWType | Valueable<MouseCursorType> | MouseCursorType::Attack | src/Ext/SWType/Body.cpp:161 | ok | src/app/diagnostics/shell_capture.rs, src/app/diagnostics/shell_capture/movies.rs, src/app/diagnostics/shell_capture/movies/opening.rs … |
| `NoCursor` | SWType | Valueable<MouseCursorType> | MouseCursorType::Disallowed | src/Ext/SWType/Body.cpp:162 | ok | — |
| `Cursor.Deploy` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Deploy | src/Ext/TechnoType/Body.cpp:530 | ok | — |
| `Cursor.Enter` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Enter | src/Ext/TechnoType/Body.cpp:532 | ok | — |
| `Cursor.Move` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::Move | src/Ext/TechnoType/Body.cpp:534 | ok | — |
| `Cursor.NoDeploy` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoDeploy | src/Ext/TechnoType/Body.cpp:531 | ok | — |
| `Cursor.NoEnter` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoEnter | src/Ext/TechnoType/Body.cpp:533 | ok | — |
| `Cursor.NoMove` | TechnoType | Valueable<MouseCursorType> | MouseCursorType::NoMove | src/Ext/TechnoType/Body.cpp:535 | ok | — |
| `Cursor.Attack` | WeaponType | Valueable<MouseCursorType> | MouseCursorType::Attack | src/Ext/WeaponType/Body.cpp:104 | ok | — |
| `Cursor.AttackOutOfRange` | WeaponType | Valueable<MouseCursorType> | MouseCursorType::AttackOutOfRange | src/Ext/WeaponType/Body.cpp:105 | ok | — |

## 钩子（6）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4ABFBE | `DisplayClass_LeftMouseButtonUp_ExecPowerToggle` | kept | 004AB9B0 `DisplayClass::DisplayClass__BandBox_LeftUp` | decompiled/functions/10057ED0.c | src/Misc/Actions.cpp:137 | src/app/input/beacon.rs, src/app/input/camera.rs, src/app/input/commands.rs … |
| 51EE6B | `InfantryClass_GetActionOnObject_Saboteur` | replaces_0.A:InfantryClass_GetCursorOverObject_Saboteur | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10025FC0.c | src/Recovered/InfantryClass.cpp:192 | src/app/input/context_order.rs |
| 6929FC | `DisplayClass_ChooseAction_CanSell` | kept | 00692610 `DisplayClass::DisplayClass__DetermineAction` | decompiled/functions/10057E70.c | src/Misc/Actions.cpp:121 | src/app/input/cursor.rs, src/app/input/dispatch.rs, src/app/input/entity_pick.rs … |
| 6FFEC0 | `TechnoClass_GetActionOnObject_Cursors` | new_in_3.0 | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004C990.c | src/Recovered/TechnoClass.cpp:761 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 7000CD | `TechnoClass_GetActionOnObject_SelfDeployCursor` | new_in_3.0 | 006FFEC0 `TechnoClass::TechnoClass__What_Action_OnObject` | decompiled/functions/1004CA20.c | src/Recovered/TechnoClass.cpp:747 | src/app/input/context_order.rs, src/app/input/entity_pick.rs, src/sim/world/techno_ai/target_scan.rs |
| 700600 | `TechnoClass_GetActionOnCell_Cursors` | new_in_3.0 | 00700600 `TechnoClass::TechnoClass__What_Action_OnCell` | decompiled/functions/1004C920.c | src/Recovered/TechnoClass.cpp:783 | src/app/input/context_order.rs, src/app/input/cursor.rs, src/sim/c4_plant.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10033EB0 | SWTypeExt::ExtData::GetAction [translated] | 使用键 | decompiled/functions/10033EB0.c |
| 10044A80 | TechnoExt::GetAttackCursor [translated] | 使用键 | decompiled/functions/10044A80.c |
| 100252D0 | `BuildingTypeExt::IsSabotagable` | 被钩子调用 | decompiled/functions/100252D0.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
| 10026980 | `MapClass::IsLocationShrouded` | 被钩子调用 | decompiled/functions/10026980.c |
| 10057E40 | `Actions::Set` | 被钩子调用 | decompiled/functions/10057E40.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004AB9B0 `DisplayClass::DisplayClass__BandBox_LeftUp` | 5 | F.cursors, ext:TechnoType, recovered:DisplayClass |
| 00692610 `DisplayClass::DisplayClass__DetermineAction` | 2 | F.cursors, U.bugfixes |
