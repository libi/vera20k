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
| `AIFriendlyDistance` | Rules | Nullable<int> | (empty) | src/Ext/Rules/Body.cpp:181 | ok | — |
| `AutoRepel` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:154 | no_use_found | — |
| `PlayerAutoRepel` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:155 | no_use_found | — |
| `TeamRetaliate` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:146 | ok | — |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6EB432 | `TeamClass_AttackedBy_Retaliate` | kept | ✓ | 006EB380 `TeamClass::TeamClass__Took_Damage` | decompiled/functions/1003A180.c | src/Ext/Team/Hooks.cpp:234 | src/sim/team_script_vm/team_ai.rs |
| 6EFB69 | `TeamClass_GatherAtFriendlyBase_Distance` | new_in_3.0 | ✓ | 006EFA10 `TeamClass::TeamClass__Script_Regroup_At_Base` | decompiled/functions/1003A2A0.c | src/Ext/Team/Hooks.cpp:312 | src/sim/team_script_vm/actions.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F320.c |
| 1001F490 | HouseExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F490.c |
| 10009730 | `TunnelData::Save` | 被钩子调用 | decompiled/functions/10009730.c |
