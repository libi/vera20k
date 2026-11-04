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

## 钩子（9）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 474200 | `CCINIClass_ReadCCFile1` | kept | ✓ | 004741F0 `CCINIClass::CCINIClass__ReadCCFile` | decompiled/functions/100645D0.c | src/Misc/Includes.cpp:9 | src/rules/ini_directives.rs, src/rules/ini_parser.rs |
| 474314 | `CCINIClass_ReadCCFile2` | kept | ✓ | 004741F0 `CCINIClass::CCINIClass__ReadCCFile` | decompiled/functions/10064660.c | src/Misc/Includes.cpp:21 | src/rules/ini_directives.rs, src/rules/ini_parser.rs |
| 525C28 | `INIClass_Parse_IniSectionIncludes_CopySection1` | replaces_0.A:IniSectionIncludes_CopySection2 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/10065550.c | src/Misc/IniSectionIncludes.cpp:171 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 525CA5 | `INIClass_Parse_IniSectionIncludes_PreProcess1` | new_in_3.0 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/100655B0.c | src/Misc/IniSectionIncludes.cpp:137 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 525D23 | `INIClass_Parse_IteratorChar2` | replaces_0.A:IteratorChar_Process_Method2 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/100652F0.c | src/Misc/IniIteratorChar.cpp:42 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 525DDB | `INIClass_Parse_IniSectionIncludes_PreProcess2` | new_in_3.0 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/100655E0.c | src/Misc/IniSectionIncludes.cpp:155 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 525E44 | `INIClass_Parse_IniSectionIncludes_CopySection2` | new_in_3.0 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/10065580.c | src/Misc/IniSectionIncludes.cpp:187 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 5260A2 | `INIClass_Parse_IteratorChar1` | replaces_0.A:IteratorChar_Process_Method1 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/10065280.c | src/Misc/IniIteratorChar.cpp:19 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |
| 5260D9 | `INIClass_Parse_Override` | new_in_3.0 | ✓ | 00525A60 `INIClass::INIClass__LoadFromStraw` | decompiled/functions/10065610.c | src/Misc/IniSectionIncludes.cpp:204 | src/rules/ini_parser.rs, src/rules/jumpjet_params.rs, src/rules/mod.rs |

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
| 100039A0 | `AresCRT::strCopy` | 被钩子调用 | decompiled/functions/100039A0.c |
| 10065380 | `IniSectionIncludes::CopySection` | 被钩子调用 | decompiled/functions/10065380.c |
| 10065470 | `IniSectionIncludes::PreProcess` | 被钩子调用 | decompiled/functions/10065470.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00525A60 `INIClass::INIClass__LoadFromStraw` | 7 | F.directives |
