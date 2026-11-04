# `T.passengers` 乘客容量、允许/禁止列表、初始载荷

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
| `MaxSize` | House | — | — | src/Ext/House/Body.cpp:951 | unscanned | — |
| `Passengers` | House | — | — | src/Ext/House/Body.cpp:950 | unscanned | src/app/diagnostics/shell_capture/movies/opening.rs, src/app/diagnostics/tactical_capture/map_observation.rs, src/app/input/dispatch.rs … |
| `InitialPayload.Nums` | TechnoType | ValueableVector<int> | — | src/Ext/TechnoType/Body.cpp:258 | check_0A_use | — |
| `InitialPayload.Types` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:257 | unscanned | — |
| `Passengers.Allowed` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:486 | unscanned | — |
| `Passengers.BySize` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:488 | ok | src/rules/object_type.rs |
| `Passengers.Disallowed` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:487 | unscanned | — |

## 钩子（4）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 73762B | `UnitClass_ReceivedRadioCommand_BySize1` | new_in_3.0 | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C150.c | src/Recovered/UnitClass.cpp:203 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 73778F | `UnitClass_ReceivedRadioCommand_BySize2` | new_in_3.0 | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C190.c | src/Recovered/UnitClass.cpp:218 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 73782F | `UnitClass_ReceivedRadioCommand_BySize3` | new_in_3.0 | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C1D0.c | src/Recovered/UnitClass.cpp:232 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 737994 | `UnitClass_ReceivedRadioCommand_BySize4` | new_in_3.0 | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C210.c | src/Recovered/UnitClass.cpp:247 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10043C10 | TechnoExt::ExtData::CreateInitialPayload [source_0.A] | 使用键 | decompiled/functions/10043C10.c |
| 100448E0 | (file-local)::CanHoldPassenger [translated] | 使用键 | decompiled/functions/100448E0.c |
| 10044880 | `TechnoClass::EnteredOpenTopped` | 被钩子调用 | decompiled/functions/10044880.c |
| 10043180 | `PassengersClass::AddPassenger` | 被钩子调用 | decompiled/functions/10043180.c |
| 10008FE0 | `FootClass::AbortMotion` | 被钩子调用 | decompiled/functions/10008FE0.c |
| 10019D00 | `TechnoClass::UpdateThreatInCell` | 被钩子调用 | decompiled/functions/10019D00.c |
| 10045320 | `PassengersClass::GetTotalSize` | 被钩子调用 | decompiled/functions/10045320.c |
