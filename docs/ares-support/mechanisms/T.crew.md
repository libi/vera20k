# `T.crew` 乘员与幸存者

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Crew` | Side | — | — | src/Ext/Side/Body.cpp:110 | unscanned | src/rules/native_processing.rs, src/util/native_random.rs |
| `Engineer` | Side | Nullable<InfantryTypeClass*> | — | src/Ext/Side/Body.cpp:112 | ok | src/app/input/context_order.rs, src/app/input/cursor.rs, src/app/input/tooltips.rs … |
| `SurvivorDivisor` | Side | Nullable<int> | — | src/Ext/Side/Body.cpp:134 | ok | src/rules/native_processing.rs |
| `Technician` | Side | Nullable<InfantryTypeClass*> | — | src/Ext/Side/Body.cpp:114 | ok | src/rules/native_processing.rs, src/rules/rules_type_references.rs |
| `Crew.EngineerChance` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:428 | ok | src/rules/ares_crew.rs |
| `Crew.TechnicianChance` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:427 | ok | src/rules/ares_crew.rs |
| `Survivor.%sPassengerChance` | TechnoType | Promotable<int> | -1 | src/Ext/TechnoType/Body.cpp:153 | ok | — |
| `Survivor.%sPilotChance` | TechnoType | Promotable<int> | -1 | src/Ext/TechnoType/Body.cpp:152 | ok | — |
| `Survivor.Pilots` | TechnoType | int | -1 | src/Ext/TechnoType/Body.cpp:150 | ok | src/rules/ares_crew.rs |

## 钩子（5）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 44EB10 | `BuildingClass_GetCrew` | kept | ✓ | 0044EB10 `BuildingClass::BuildingClass__Crew_Type` | decompiled/functions/1004C650.c | src/Ext/Techno/Hooks.Crew.cpp:29 | src/rules/ruleset.rs, src/sim/crew_survival.rs |
| 451330 | `BuildingClass_GetCrewCount` | kept | ✓ | 00451330 `BuildingClass::BuildingClass__How_Many_Survivors` | decompiled/functions/1004C6E0.c | src/Ext/Techno/Hooks.Crew.cpp:29 | src/sim/crew_survival.rs |
| 6F3950 | `TechnoClass_GetCrewCount` | kept | ✓ | 006F3950 `TechnoClass::TechnoClass__GetCrewCount` | decompiled/functions/1004C870.c | src/Ext/Techno/Hooks.Crew.cpp:9 | — |
| 702E9D | `TechnoClass_RegisterDestruction_Veterancy` | kept | ✓ | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/1004EEF0.c | src/Ext/Techno/Hooks.Veterancy.cpp:18 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |
| 707D20 | `TechnoClass_GetCrew` | kept | ✓ | 00707D20 `TechnoClass::TechnoClass__GetCrew` | decompiled/functions/1004C780.c | src/Ext/Techno/Hooks.Crew.cpp:9 | src/rules/ruleset.rs, src/sim/crew_survival.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DEE0 | SideExt::ExtData::GetEngineer [source_0.A,3.0✓] | 使用键 | decompiled/functions/1002DEE0.c |
| 1002E130 | SideExt::ExtData::GetSurvivorDivisor [source_0.A,3.0✓] | 使用键 | decompiled/functions/1002E130.c |
| 1002E170 | SideExt::ExtData::GetTechnician [source_0.A,3.0✓] | 使用键 | decompiled/functions/1002E170.c |
| 10044FF0 | TechnoExt::ExtData::GetActionHijack [translated] | 使用键 | decompiled/functions/10044FF0.c |
| 1004F300 | TechnoExt::CanWarpTarget [translated,inferred] | 使用键 | decompiled/functions/1004F300.c |
| 10016F20 | `TechnoClass::IsMindControlled` | 被钩子调用 | decompiled/functions/10016F20.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100212B0 | `HouseExt::ExtData::GetEngineer` | 被钩子调用 | decompiled/functions/100212B0.c |
| 100214D0 | `HouseExt::ExtData::GetSurvivorDivisor` | 被钩子调用 | decompiled/functions/100214D0.c |
| 10021520 | `HouseExt::ExtData::GetTechnician` | 被钩子调用 | decompiled/functions/10021520.c |
| 10021230 | `HouseExt::ExtData::GetCrew` | 被钩子调用 | decompiled/functions/10021230.c |
| 10046AF0 | `TechnoExt::UpdateVeterancy` | 被钩子调用 | decompiled/functions/10046AF0.c |
