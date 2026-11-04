# `S.droppod` DropPod

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（8）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DropPodMaximum` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:277 | check_0A_use | — |
| `DropPodMinimum` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:276 | check_0A_use | — |
| `DropPodTrailer` | Rules | Nullable<AnimTypeClass*> | (empty) | src/Ext/Rules/Body.cpp:274 | no_use_found | — |
| `DropPodTypes` | Rules | ValueableVector<TechnoTypeClass*> | (empty) | src/Ext/Rules/Body.cpp:275 | check_0A_use | — |
| `DropPod.Maximum` | SWType | — | — | src/Misc/SWTypes/DropPod.cpp:34 | unscanned | — |
| `DropPod.Minimum` | SWType | — | — | src/Misc/SWTypes/DropPod.cpp:33 | unscanned | — |
| `DropPod.Types` | SWType | — | — | src/Misc/SWTypes/DropPod.cpp:36 | unscanned | — |
| `DropPod.Veterancy` | SWType | — | — | src/Misc/SWTypes/DropPod.cpp:35 | unscanned | — |

## 钩子（3）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4B769B | `ScenarioClass_GenerateDropshipLoadout` | kept | 004B6C30 `Dropship__ShowSequence` | decompiled/functions/1002CCA0.c | src/Ext/Rules/Hooks.Dropships.cpp:7 | — |
| 4B93BD | `ScenarioClass_GenerateDropshipLoadout_FreeAnims` | kept | 004B6C30 `Dropship__ShowSequence` | decompiled/functions/1002CCC0.c | src/Ext/Rules/Hooks.Dropships.cpp:75 | — |
| 4B99A2 | `DropshipLoadout_WriteUnit` | kept | 004B9890 `FUN_004b9890` | decompiled/functions/1002CA30.c | src/Ext/Rules/Hooks.Dropships.cpp:14 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 10070E40 | FUN_10070e40 [source_0.A] | 使用键 | decompiled/functions/10070E40.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 1002CA20 | `Drawing::PrintUnicode` | 被钩子调用 | decompiled/functions/1002CA20.c |
| 1002CA00 | `InputManagerClass::DoSomething` | 被钩子调用 | decompiled/functions/1002CA00.c |
| 1002C9E0 | `SHPStruct::~SHPStruct` | 被钩子调用 | decompiled/functions/1002C9E0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10009590 | `MapClass::Pathfinding_Find` | 被钩子调用 | decompiled/functions/10009590.c |
| 10011DE0 | `CellClass::GetNeighbourCell` | 被钩子调用 | decompiled/functions/10011DE0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004B9890 `FUN_004b9890` | 2 | S.droppod, ext:ArmorTypes |
