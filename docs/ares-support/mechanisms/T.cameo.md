# `T.cameo` PCX 图标与图标调色板

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | partial | docs/ares-pcx-cameos.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AltCameoPCX` | TechnoType | AresPCXFile | — | src/Ext/TechnoType/Body.cpp:336 | no_use_found | src/rules/art_data.rs |
| `CameoPalette` | TechnoType | FUN_10076160 | — | src/Ext/TechnoType/Body.cpp:260 | no_use_found | — |
| `CameoPCX` | TechnoType | AresPCXFile | — | src/Ext/TechnoType/Body.cpp:335 | no_use_found | src/rules/art_data.rs, src/rules/ini_value.rs |

## 钩子（5）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6A4EA5 | `SidebarClass_CTOR_InitCameosList` | kept | 006A4E60 `SidebarClass::SidebarClass__Constructor` | decompiled/functions/1002C1C0.c | src/Ext/Rules/Hooks.CameoList.cpp:41 | — |
| 6A4FD8 | `SidebarClass_Load_InitCameosList` | kept | 006A4F20 `SidebarClass::SidebarClass__Constructor_NoInit` | decompiled/functions/1002C1C0.c | src/Ext/Rules/Hooks.CameoList.cpp:49 | src/app/sidebar_projection.rs, src/sim/snapshot.rs, src/ui/sidebar/cameo_order.rs |
| 6A61B1 | `SidebarClass_SetFactoryForObject` | kept | 006A6140 `SidebarClass::SidebarClass__LinkFactory` | decompiled/functions/1002C230.c | src/Ext/Rules/Hooks.CameoList.cpp:57 | src/sim/production/production_types.rs |
| 6A63B7 | `SidebarClass_AddCameo_SkipSizeCheck` | kept ⚠改过 | 006A6300 `SidebarClass::SidebarClass__AddCameo` | decompiled/functions/1002C160.c | src/Ext/Rules/Hooks.CameoList.cpp:81 | src/ui/sidebar/cameo_order.rs |
| 712045 | `TechnoTypeClass_GetCameo` | kept | 00712040 `TechnoTypeClass::TechnoTypeClass__GetCameo` | decompiled/functions/1005E480.c | src/Misc/Cameos.cpp:8 | src/app/presentation/sidebar_render.rs, src/render/sidebar_cameo_atlas.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002BC80 | `RulesExt::ClearCameos` | 被钩子调用 | decompiled/functions/1002BC80.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 1003D800 | `TechnoTypeExt::ExtData::CameoIsElite` | 被钩子调用 | decompiled/functions/1003D800.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006A6300 `SidebarClass::SidebarClass__AddCameo` | 2 | T.cameo, recovered:SidebarClass |
