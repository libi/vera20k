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
| `InitialPayload.Nums` | TechnoType | ValueableVector<int> | — | src/Ext/TechnoType/Body.cpp:262 | ok | — |
| `InitialPayload.Types` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:261 | unscanned | — |
| `Passengers.Allowed` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:490 | unscanned | — |
| `Passengers.BySize` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:492 | ok | src/rules/object_type.rs |
| `Passengers.Disallowed` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:491 | unscanned | — |
| `MaxSize` | TunnelTypes | — | — | src/Enum/TunnelTypes.cpp:299 | unscanned | — |
| `Passengers` | TunnelTypes | — | — | src/Enum/TunnelTypes.cpp:298 | unscanned | src/app/diagnostics/shell_capture/movies/opening.rs, src/app/diagnostics/tactical_capture/map_observation.rs, src/app/input/dispatch.rs … |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 73762B | `UnitClass_ReceivedRadioCommand_BySize1` | new_in_3.0 | ✓ | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C150.c | src/Ext/Techno/Hooks.cpp:2842 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 73778F | `UnitClass_ReceivedRadioCommand_BySize2` | new_in_3.0 | ✓ | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C190.c | src/Ext/Techno/Hooks.cpp:2857 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 73782F | `UnitClass_ReceivedRadioCommand_BySize3` | new_in_3.0 | ✓ | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C1D0.c | src/Ext/Techno/Hooks.cpp:2871 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |
| 737994 | `UnitClass_ReceivedRadioCommand_BySize4` | new_in_3.0 | ✓ | 00737430 `UnitClass::UnitClass__Receive_Radio` | decompiled/functions/1004C210.c | src/Ext/Techno/Hooks.cpp:2886 | src/app/input/autoload.rs, src/sim/passenger.rs, src/sim/radio/receive.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10043C10 | TechnoExt::ExtData::CreateInitialPayload [translated] | 使用键 | decompiled/functions/10043C10.c |
| 100448E0 | (file-local)::CanHoldPassenger [translated] | 使用键 | decompiled/functions/100448E0.c |
| 10044880 | `TechnoClass::EnteredOpenTopped` | 被钩子调用 | decompiled/functions/10044880.c |
| 10043180 | `PassengersClass::AddPassenger` | 被钩子调用 | decompiled/functions/10043180.c |
| 10008FE0 | `FootClass::AbortMotion` | 被钩子调用 | decompiled/functions/10008FE0.c |
| 10019D00 | `TechnoClass::UpdateThreatInCell` | 被钩子调用 | decompiled/functions/10019D00.c |
| 10045320 | `PassengersClass::GetTotalSize` | 被钩子调用 | decompiled/functions/10045320.c |
