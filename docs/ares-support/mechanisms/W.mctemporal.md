# `W.mctemporal` 心控永久与超时空

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `MindControl.Permanent` | WarheadType | bool | false | src/Ext/WarheadType/Body.cpp:82 | check_0A_use | — |
| `Temporal.HealthFactor` | WarheadType | Valueable<double> | 0.0 | src/Ext/WarheadType/Body.cpp:92 | ok | — |
| `Temporal.WarpAway` | WarheadType | Nullable<AnimTypeClass *> | (empty) | src/Ext/WarheadType/Body.cpp:93 | check_0A_use | — |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 71A900 | `TemporalClass_Update_WarpAway` | kept | 0071A760 `TemporalClass::TemporalClass__Update` | decompiled/functions/10053CE0.c | src/Ext/WarheadType/Hooks.cpp:70 | src/sim/temporal.rs |
| 71AFB2 | `TemporalClass_Fire_HealthFactor` | new_in_3.0 | 0071AF20 `TemporalClass::TemporalClass__InitiateWarp` | decompiled/functions/10053C20.c | src/Recovered/TemporalClass.cpp:111 | src/sim/projectile.rs, src/sim/temporal.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10055920 | WeaponTypeExt::ModifyWaveColor [translated,inferred] | 使用键 | decompiled/functions/10055920.c |
| 10060540 | CSFLoader::FindDynamic [source_0.A,inferred] | 使用键 | decompiled/functions/10060540.c |
