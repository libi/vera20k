# `T.dehardcode` 去硬编码身份（IsCow/IsDesolator/IsBomb）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `IsBomb` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:229 | ok | — |
| `IsCow` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:207 | ok | — |
| `IsDesolator` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:205 | ok | — |
| `IsDesolator.RadDependant` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:206 | ok | — |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 51CE9A | `InfantryClass_Idle` | kept ⚠改过 | ✓ | 0051CDB0 `InfantryClass::InfantryClass__UpdateIdleAction` | decompiled/functions/10049740.c | src/Ext/Techno/Hooks.cpp:750 | src/sim/world/techno_ai/mission_handlers.rs |
| 5F5ADD | `ObjectClass_SpawnParachuted_Animation` | replaces_0.A:Parachute_Animation | ✓ | 005F5940 `ObjectClass::ObjectClass__Paradrop` | decompiled/functions/10047C60.c | src/Ext/Techno/Hooks.Art.cpp:52 | src/app/presentation/lighting.rs, src/sim/aircraft/drop_payload.rs, src/sim/game_entity.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001BBC0 | `HouseTypeExt::ExtData::GetParachuteAnim` | 被钩子调用 | decompiled/functions/1001BBC0.c |
