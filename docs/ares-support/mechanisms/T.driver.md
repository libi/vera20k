# `T.driver` 驾驶员、偷车、KillDriver、Operator

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | T.crew | done | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（18）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CanBeDriven` | HouseType | Nullable<bool> | (empty) | src/Ext/HouseType/Body.cpp:330 | ok | src/rules/ares_driver.rs, src/rules/native_processing.rs |
| `DeactivateDimOperator` | Rules | Valueable<double> | 0.65 | src/Ext/Rules/Body.cpp:150 | ok | — |
| `CanBeDriven` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:305 | ok | src/rules/ares_driver.rs, src/rules/native_processing.rs |
| `CanDrive` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:304 | ok | src/rules/ares_driver.rs |
| `Operator` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:237 | unscanned | src/rules/native_processing.rs |
| `ProtectedDriver` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:302 | ok | src/rules/ares_driver.rs |
| `ProtectedDriver.MinHealth` | TechnoType | Nullable<double> | (empty) | src/Ext/TechnoType/Body.cpp:303 | ok | src/rules/ares_driver.rs |
| `VehicleThief.Allowed` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:328 | ok | src/rules/ares_driver.rs |
| `VehicleThief.BreakMindControl` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:327 | ok | src/rules/ares_driver.rs |
| `VehicleThief.EnterSound` | TechnoType | ValueableIdx<VocClass> | -1 | src/Ext/TechnoType/Body.cpp:324 | ok | src/rules/object_type.rs |
| `VehicleThief.KillPilots` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:326 | ok | src/rules/ares_driver.rs |
| `VehicleThief.LeaveSound` | TechnoType | ValueableIdx<VocClass> | -1 | src/Ext/TechnoType/Body.cpp:325 | ok | src/rules/object_type.rs |
| `VehicleThief.OneTime` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:329 | ok | src/rules/ares_driver.rs |
| `KillDriver` | WarheadType | bool | false | src/Ext/WarheadType/Body.cpp:122 | ok | src/rules/ares_warhead.rs |
| `KillDriver.Chance` | WarheadType | Valueable<double> | 1.00 | src/Ext/WarheadType/Body.cpp:124 | ok | src/rules/ares_warhead.rs |
| `KillDriver.KillBelowPercent` | WarheadType | Valueable<double> | 1.00 | src/Ext/WarheadType/Body.cpp:123 | ok | src/rules/ares_warhead.rs |
| `KillDriver.Owner` | WarheadType | Valueable<OwnerHouseKind> | OwnerHouseKind::Special | src/Ext/WarheadType/Body.cpp:125 | ok | src/rules/ares_warhead.rs |
| `KillDriver.RemoveVeterancy` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:126 | ok | src/rules/ares_warhead.rs |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 51E7BF | `InfantryClass_GetActionOnObject_CanCapture` | replaces_0.A:InfantryClass_GetCursorOverObject_CanCapture | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10049690.c | src/Ext/Techno/Hooks.cpp:956 | src/app/input/context_order.rs |
| 5202F9 | `InfantryClass_UpdateVehicleThief_Check` | kept | ✓ | 005202F0 `InfantryClass::InfantryClass__TryThiefVehicleCapture` | decompiled/functions/10049970.c | src/Ext/Techno/Hooks.cpp:916 | src/sim/world/techno_ai/mission_handlers.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002D410.c |
| 1002E060 | FUN_1002e060 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002E060.c |
| 100434C0 | TechnoExt::ExtData::CanKillDriver [translated] | 使用键 | decompiled/functions/100434C0.c |
| 10044FF0 | TechnoExt::ExtData::GetActionHijack [translated] | 使用键 | decompiled/functions/10044FF0.c |
| 100464C0 | TechnoExt::SpawnSurvivors [translated] | 使用键 | decompiled/functions/100464C0.c |
| 10046950 | TechnoExt::ExtData::TryTakeVehicle [translated] | 使用键 | decompiled/functions/10046950.c |
| 10052B40 | WarheadTypeExt::ExtData::applyKillDriver [translated] | 使用键 | decompiled/functions/10052B40.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10044C60 | `ObjectClass::GetHealthPercentage` | 被钩子调用 | decompiled/functions/10044C60.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 10009180 | `CaptureManagerClass::FreeUnit` | 被钩子调用 | decompiled/functions/10009180.c |
| 10043630 | `CaptureManagerClass::CaptureUnit` | 被钩子调用 | decompiled/functions/10043630.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100464A0 | `TagClass::ShouldReplace` | 被钩子调用 | decompiled/functions/100464A0.c |
| 10046360 | `ObjectClass::ReplaceTag` | 被钩子调用 | decompiled/functions/10046360.c |
| 10046320 | `FootClass::RemoveFirstPassenger` | 被钩子调用 | decompiled/functions/10046320.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
| 10021310 | `HouseExt::GetHouseKind` | 被钩子调用 | decompiled/functions/10021310.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020D20 | `HouseClass::FindByCountryIndex` | 被钩子调用 | decompiled/functions/10020D20.c |
| 100456D0 | `TechnoExt::ExtData::KillDriver` | 被钩子调用 | decompiled/functions/100456D0.c |
