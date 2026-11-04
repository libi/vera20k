# `F.directives` #include 与 [节]:[父节] 继承

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | done | docs/ini-directives.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 474200 | `CCINIClass_ReadCCFile1` | kept | 004741F0 `CCINIClass::CCINIClass__ReadCCFile` | decompiled/functions/100645D0.c | src/Misc/Includes.cpp:8 | src/rules/ini_directives.rs, src/rules/ini_parser.rs |
| 474314 | `CCINIClass_ReadCCFile2` | kept | 004741F0 `CCINIClass::CCINIClass__ReadCCFile` | decompiled/functions/10064660.c | src/Misc/Includes.cpp:20 | src/rules/ini_directives.rs, src/rules/ini_parser.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10063650 | `CRT::strdup` | 被钩子调用 | decompiled/functions/10063650.c |
| 10003940 | `CRT::free` | 被钩子调用 | decompiled/functions/10003940.c |
| 1002A7A0 | `INIClass::GetKeyCount` | 被钩子调用 | decompiled/functions/1002A7A0.c |
| 1002A7C0 | `INIClass::GetKeyName` | 被钩子调用 | decompiled/functions/1002A7C0.c |
| 10005800 | `INIClass::ReadString` | 被钩子调用 | decompiled/functions/10005800.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 10003860 | `CCINIClass::ReadCCFile` | 被钩子调用 | decompiled/functions/10003860.c |
