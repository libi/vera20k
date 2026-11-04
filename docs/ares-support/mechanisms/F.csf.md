# `F.csf` 补充 CSF 字符串表

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | — | done | tools/rules_oracle/ares_csf.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（7）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6BD886 | `CSF_LoadExtraFiles` | kept | 006BB9A0 `WinMain` | decompiled/functions/10060DF0.c | src/Misc/CSFLoader.cpp:167 | src/app/persistence/options_profile.rs, src/util/config.rs |
| 7346D0 | `CSF_LoadBaseFile` | kept | 007346A0 `CSF_LoadHeaderAndLanguage` | decompiled/functions/10060DD0.c | src/Misc/CSFLoader.cpp:76 | — |
| 734823 | `CSF_AllocateMemory` | kept | 007346A0 `CSF_LoadHeaderAndLanguage` | decompiled/functions/10060D60.c | src/Misc/CSFLoader.cpp:85 | — |
| 734A5F | `CSF_AddOrOverrideLabel` | kept | 00734990 `CSF_ParseLabelStringChunks` | decompiled/functions/10060C80.c | src/Misc/CSFLoader.cpp:100 | — |
| 734A97 | `CSF_SetIndex` | kept | 00734990 `CSF_ParseLabelStringChunks` | decompiled/functions/10060EE0.c | src/Misc/CSFLoader.cpp:154 | — |
| 734E83 | `CSF_LoadString_1` | kept | 00734E60 `StringTable__LoadString` | decompiled/functions/10060E80.c | src/Misc/CSFLoader.cpp:179 | src/app/presentation/outcome_banner.rs, src/app/presentation/super_timers.rs, src/map/ranking.rs |
| 734EC2 | `CSF_LoadString_2` | kept | 00734E60 `StringTable__LoadString` | decompiled/functions/10060EC0.c | src/Misc/CSFLoader.cpp:193 | src/app/presentation/outcome_banner.rs, src/app/presentation/super_timers.rs, src/map/ranking.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10060C60 | `CRT::bsearch` | 被钩子调用 | decompiled/functions/10060C60.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 100608C0 | `CSFLoader::LoadAdditionalCSF` | 被钩子调用 | decompiled/functions/100608C0.c |
| 100608A0 | `StringTable::GetLanguage` | 被钩子调用 | decompiled/functions/100608A0.c |
| 100607C0 | `CSFLoader::GetDynamicString` | 被钩子调用 | decompiled/functions/100607C0.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006BB9A0 `WinMain` | 4 | F.csf, U.bugfixes, recovered:LoadRA2MD |
| 00734990 `CSF_ParseLabelStringChunks` | 3 | F.csf, recovered:StringTable |
