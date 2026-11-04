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
| `IsBomb` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:225 | ok | — |
| `IsCow` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:203 | check_0A_use | — |
| `IsDesolator` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:201 | check_0A_use | — |
| `IsDesolator.RadDependant` | TechnoType | bool | false | src/Ext/TechnoType/Body.cpp:202 | check_0A_use | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 51CE9A | `InfantryClass_Idle` | kept ⚠改过 | 0051CDB0 `InfantryClass::InfantryClass__UpdateIdleAction` | decompiled/functions/10049740.c | src/Ext/Techno/Hooks.cpp:472 | src/sim/world/techno_ai/mission_handlers.rs |
| 5F5ADD | `ObjectClass_SpawnParachuted_Animation` | replaces_0.A:Parachute_Animation | 005F5940 `ObjectClass::ObjectClass__Paradrop` | decompiled/functions/10047C60.c | src/Recovered/ObjectClass.cpp:76 | src/app/presentation/lighting.rs, src/sim/aircraft/drop_payload.rs, src/sim/game_entity.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1003DEC0 | TechnoTypeExt::ExtMap.LoadFromINI [source_0.A,inferred] | 使用键 | decompiled/functions/1003DEC0.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
