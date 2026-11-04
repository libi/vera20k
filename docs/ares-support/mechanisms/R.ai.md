# `R.ai` AI 与玩家自动反击、团队报复

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
| `AIFriendlyDistance` | Rules | Nullable<int> | (empty) | src/Ext/Rules/Body.cpp:226 | ok | — |
| `AutoRepel` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:199 | no_use_found | — |
| `PlayerAutoRepel` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:200 | no_use_found | — |
| `TeamRetaliate` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:191 | check_0A_use | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6EB432 | `TeamClass_AttackedBy_Retaliate` | kept | 006EB380 `TeamClass::TeamClass__Took_Damage` | decompiled/functions/1003A180.c | src/Ext/Team/Hooks.cpp:10 | src/sim/team_script_vm/team_ai.rs |
| 6EFB69 | `TeamClass_GatherAtFriendlyBase_Distance` | new_in_3.0 | 006EFA10 `TeamClass::TeamClass__Script_Regroup_At_Base` | decompiled/functions/1003A2A0.c | src/Recovered/TeamClass.cpp:238 | src/sim/team_script_vm/actions.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F490.c |
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
