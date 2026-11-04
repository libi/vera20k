# `U.eva` EVA 语音扩展

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | docs/ares-eva.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（5）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 7528E8 | `VoxClass_PlayEVASideSpecific` | kept | 00752760 `VoxClass__PlayNextQueued` | decompiled/functions/10063AB0.c | src/Misc/EVAVoices.cpp:94 | src/app/presentation/building_anim.rs, src/audio/sfx.rs, src/audio/vox.rs … |
| 752FDC | `VoxClass_LoadFromINI` | kept | 00752DB0 `VoxClass__ReadINI` | decompiled/functions/100639C0.c | src/Misc/EVAVoices.cpp:73 | src/rules/sound_ini.rs |
| 753000 | `VoxClass_CreateFromINIList` | kept | 00753000 `VoxClass__ReadEVAINI` | decompiled/functions/10063660.c | src/Misc/EVAVoices.cpp:11 | src/rules/sound_ini.rs |
| 7531CF | `VoxClass_DeleteAll` | kept | 007531A0 `VoxClass__ClearAllEntries` | decompiled/functions/100638D0.c | src/Misc/EVAVoices.cpp:59 | — |
| 753380 | `VoxClass_GetFilename` | kept | 00753380 `VoxClass__GetFilename` | decompiled/functions/10063920.c | src/Misc/EVAVoices.cpp:104 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10003940 | `CRT::free` | 被钩子调用 | decompiled/functions/10003940.c |
| 100091C0 | `INIClass::GetSection` | 被钩子调用 | decompiled/functions/100091C0.c |
| 1002A7A0 | `INIClass::GetKeyCount` | 被钩子调用 | decompiled/functions/1002A7A0.c |
| 1002A7C0 | `INIClass::GetKeyName` | 被钩子调用 | decompiled/functions/1002A7C0.c |
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |
| 10003990 | `CRT::strcmpi` | 被钩子调用 | decompiled/functions/10003990.c |
| 10063650 | `CRT::strdup` | 被钩子调用 | decompiled/functions/10063650.c |
| 10063490 | `VoxClass::Find` | 被钩子调用 | decompiled/functions/10063490.c |
| 10063630 | `VoxClass::LoadFromINI` | 被钩子调用 | decompiled/functions/10063630.c |
| 10063540 | `VoxClass::~VoxClass` | 被钩子调用 | decompiled/functions/10063540.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
| 100039A0 | `AresCRT::strCopy` | 被钩子调用 | decompiled/functions/100039A0.c |
| 10063610 | `VoxClass::GetFilename` | 被钩子调用 | decompiled/functions/10063610.c |
