# `T.cloning` 克隆设施

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
| `CloningFacility` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:284 | check_0A_use | — |
| `Cloneable` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:355 | check_0A_use | — |
| `ClonedAs` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:358 | check_0A_use | — |
| `ClonedAt` | TechnoType | ValueableVector<BuildingTypeClass *> | — | src/Ext/TechnoType/Body.cpp:357 | check_0A_use | — |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 687C16 | `INIClass_ReadScenario_ValidateThings` | kept ⚠改过 | 00686B20 `CCINIClass::InitializeScenarioFromINI` | decompiled/functions/100677B0.c | src/Misc/Invalidators.cpp:65 | src/app/loading/fresh_scenario.rs, src/app/loading/init.rs, src/map/authored_overlay.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100127C0 | BuildingExt::ExtData::KickOutClones [source_0.A] | 使用键 | decompiled/functions/100127C0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10061A30 | `RadarClass::First_Time` | 被钩子调用 | decompiled/functions/10061A30.c |
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
