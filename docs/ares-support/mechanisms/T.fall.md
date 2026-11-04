# `T.fall` 降落伞、下落速度、坠毁

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（9）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Parachute.Anim` | HouseType | Valueable<AnimTypeClass*> | nullptr | src/Ext/HouseType/Body.cpp:307 | no_use_found | src/rules/native_processing.rs |
| `Parachute.Anim` | Side | Nullable<AnimTypeClass*> | — | src/Ext/Side/Body.cpp:115 | ok | src/rules/native_processing.rs |
| `Crashable` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:391 | check_0A_use | src/asset_tools/verb_ini.rs, src/rules/native_processing.rs, src/rules/object_type.rs |
| `CrashSpin` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:393 | check_0A_use | — |
| `FallRate.NoParachute` | TechnoType | Valueable<int> | 1 | src/Ext/TechnoType/Body.cpp:524 | no_use_found | — |
| `FallRate.NoParachuteMax` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:526 | no_use_found | — |
| `FallRate.Parachute` | TechnoType | Valueable<int> | 1 | src/Ext/TechnoType/Body.cpp:523 | no_use_found | — |
| `FallRate.ParachuteMax` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:525 | no_use_found | — |
| `Parachute.Anim` | TechnoType | Valueable<AnimTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:229 | ok | src/rules/native_processing.rs |

## 钩子（3）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 41668B | `AircraftClass_ReceiveDamage` | kept | 004165C0 `AircraftClass::AircraftClass__ReceiveDamage` | decompiled/functions/10048C50.c | src/Ext/Techno/Hooks.cpp:38 | src/sim/combat/receiver_call.rs, src/sim/world/crash.rs, src/sim/world/rocket_flight.rs |
| 4DECAE | `FootClass_Crash_Spin` | kept ⚠改过 | 004DEBB0 `FootClass::FootClass__Crash` | decompiled/functions/10049050.c | src/Ext/Techno/Hooks.cpp:58 | src/sim/aircraft/idle_entry.rs, src/sim/movement/jumpjet_flight.rs, src/sim/world/crash.rs |
| 5F5ADD | `ObjectClass_SpawnParachuted_Animation` | replaces_0.A:Parachute_Animation | 005F5940 `ObjectClass::ObjectClass__Paradrop` | decompiled/functions/10047C60.c | src/Recovered/ObjectClass.cpp:76 | src/app/presentation/lighting.rs, src/sim/aircraft/drop_payload.rs, src/sim/game_entity.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 100464C0 | `TechnoExt::SpawnSurvivors` | 被钩子调用 | decompiled/functions/100464C0.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 10051730 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10051730.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
