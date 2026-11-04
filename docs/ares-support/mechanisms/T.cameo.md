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
| `AltCameoPCX` | TechnoType | AresPCXFile | — | src/Ext/TechnoType/Body.cpp:340 | ok | src/rules/art_data.rs |
| `CameoPalette` | TechnoType | FUN_10076160 | — | src/Ext/TechnoType/Body.cpp:264 | ok | — |
| `CameoPCX` | TechnoType | AresPCXFile | — | src/Ext/TechnoType/Body.cpp:339 | ok | src/rules/art_data.rs, src/rules/ini_value.rs |

## 钩子（37）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6A4EA5 | `SidebarClass_CTOR_InitCameosList` | kept | ✓ | 006A4E60 `SidebarClass::SidebarClass__Constructor` | decompiled/functions/1002C1C0.c | src/Ext/Rules/Hooks.CameoList.cpp:124 | — |
| 6A4FD8 | `SidebarClass_Load_InitCameosList` | kept | ✓ | 006A4F20 `SidebarClass::SidebarClass__Constructor_NoInit` | decompiled/functions/1002C1C0.c | src/Ext/Rules/Hooks.CameoList.cpp:132 | src/app/sidebar_projection.rs, src/sim/snapshot.rs, src/ui/sidebar/cameo_order.rs |
| 6A61B1 | `SidebarClass_SetFactoryForObject` | kept | ✓ | 006A6140 `SidebarClass::SidebarClass__LinkFactory` | decompiled/functions/1002C230.c | src/Ext/Rules/Hooks.CameoList.cpp:140 | src/sim/production/production_types.rs |
| 6A63B7 | `SidebarClass_AddCameo_SkipSizeCheck` | kept ⚠改过 | ✓ | 006A6300 `SidebarClass::SidebarClass__AddCameo` | decompiled/functions/1002C160.c | src/Ext/Rules/Hooks.CameoList.cpp:164 | src/ui/sidebar/cameo_order.rs |
| 6A8710 | `StripClass_AddCameo_ReplaceItAll` | replaces_0.A:TabCameoListClass_AddCameo_ReplaceItAll | ✓ | 006A8710 `StripClass::StripClass__InsertEntry` | decompiled/functions/1002C2D0.c | src/Ext/Rules/Hooks.CameoList.cpp:190 | src/ui/sidebar/cameo_order.rs |
| 6A8D1C | `StripClass_MouseMove_GetCameos1` | replaces_0.A:TabSidebarCameoClass_MouseMove_GetCameos1 | ✓ | 006A8B30 `StripClass::StripClass__AI` | decompiled/functions/1002C5C0.c | src/Ext/Rules/Hooks.CameoList.cpp:218 | src/app/presentation/sidebar_gadgets.rs, src/app/sidebar_projection.rs, src/ui/sidebar/cameo_order.rs |
| 6A8DB5 | `StripClass_MouseMove_GetCameos2` | replaces_0.A:TabSidebarCameoClass_MouseMove_GetCameos2 | ✓ | 006A8B30 `StripClass::StripClass__AI` | decompiled/functions/1002C5F0.c | src/Ext/Rules/Hooks.CameoList.cpp:234 | src/app/presentation/sidebar_gadgets.rs, src/app/sidebar_projection.rs, src/ui/sidebar/cameo_order.rs |
| 6A8F6C | `StripClass_MouseMove_GetCameos3` | replaces_0.A:TabSidebarCameoClass_MouseMove_GetCameos3 | ✓ | 006A8B30 `StripClass::StripClass__AI` | decompiled/functions/1002C620.c | src/Ext/Rules/Hooks.CameoList.cpp:252 | src/app/presentation/sidebar_gadgets.rs, src/app/sidebar_projection.rs, src/ui/sidebar/cameo_order.rs |
| 6A9304 | `StripClass_GetTip_NoLimit` | replaces_0.A:CameoClass_GetTip_NoLimit | ✓ | 006A92E0 `StripClass::StripClass__GetCameoTooltip` | decompiled/functions/1002C590.c | src/Ext/Rules/Hooks.CameoList.cpp:271 | — |
| 6A95C8 | `StripClass_Draw_Status` | replaces_0.A:TabCameoListClass_Draw_TestF10 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C490.c | src/Ext/Rules/Hooks.CameoList.cpp:307 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9747 | `StripClass_Draw_GetCameo1` | replaces_0.A:TabCameoListClass_Draw_GetCameo1 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C450.c | src/Ext/Rules/Hooks.CameoList.cpp:286 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A980A | `StripClass_Draw_TechnoType_PCX` | replaces_0.A:TabCameoListClass_Draw_GetTechnoPCX | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E430.c | src/Misc/Cameos.cpp:82 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9866 | `StripClass_Draw_Status_1` | replaces_0.A:TabCameoListClass_Draw_Test10_1 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C4C0.c | src/Ext/Rules/Hooks.CameoList.cpp:307 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9886 | `StripClass_Draw_Status_2` | replaces_0.A:TabCameoListClass_Draw_Test10_2 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C4F0.c | src/Ext/Rules/Hooks.CameoList.cpp:321 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9948 | `StripClass_Draw_SuperWeapon` | replaces_0.A:TabCameoListClass_Draw_SW | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E3F0.c | src/Misc/Cameos.cpp:39 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9952 | `StripClass_Draw_SuperWeapon_PCX` | replaces_0.A:TabCameoListClass_Draw_GetSWPCX | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E410.c | src/Misc/Cameos.cpp:72 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A99BE | `StripClass_Draw_BreakDrawLoop` | replaces_0.A:TabCameoListClass_Draw_BreakDrawLoop | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C430.c | src/Ext/Rules/Hooks.CameoList.cpp:346 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A99F3 | `StripClass_Draw_SkipSHPForPCX` | replaces_0.A:TabCameoListClass_Draw_SkipSHPForPCX | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E3E0.c | src/Misc/Cameos.cpp:100 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9A2A | `StripClass_Draw_Main` | replaces_0.A:TabCameoListClass_Draw_Main | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E390.c | src/Misc/Cameos.cpp:50 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9A43 | `StripClass_Draw_DrawPCX` | replaces_0.A:TabCameoListClass_Draw_DrawPCX | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1005E300.c | src/Misc/Cameos.cpp:108 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9B4F | `StripClass_Draw_TestFlashFrame` | replaces_0.A:TabCameoListClass_Draw_TestFlashFrame | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C550.c | src/Ext/Rules/Hooks.CameoList.cpp:355 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6A9EBA | `StripClass_Draw_Status_3` | replaces_0.A:TabCameoListClass_Draw_Test10_3 | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/1002C520.c | src/Ext/Rules/Hooks.CameoList.cpp:370 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6AA600 | `StripClass_RecheckCameos` | new_in_3.0 | ✓ | 006AA600 `StripClass::StripClass__Recalculate` | decompiled/functions/1002C660.c | src/Ext/Rules/Hooks.CameoList.cpp:42 | src/sim/production/factory_lifecycle.rs, src/sim/production/production_tech.rs, src/ui/sidebar/cameo_order.rs |
| 6AAD2F | `SelectClass_ProcessInput_LoadCameoData1` | replaces_0.A:SidebarClass_ProcessCameoClick_LoadCameoData1 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C0E0.c | src/Ext/Rules/Hooks.CameoList.cpp:386 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB0B0 | `SelectClass_ProcessInput_LoadCameo2` | replaces_0.A:SidebarClass_ProcessCameoClick_LoadCameo2 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C0B0.c | src/Ext/Rules/Hooks.CameoList.cpp:415 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB49D | `SelectClass_ProcessInput_FixOffset1` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset1 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002BF30.c | src/Ext/Rules/Hooks.CameoList.cpp:424 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB4E8 | `SelectClass_ProcessInput_FixOffset2` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset2 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002BF50.c | src/Ext/Rules/Hooks.CameoList.cpp:433 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB577 | `SelectClass_ProcessInput_FixOffset3` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset3 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002BF70.c | src/Ext/Rules/Hooks.CameoList.cpp:445 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB620 | `SelectClass_ProcessInput_FixOffset4` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset4 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002BFF0.c | src/Ext/Rules/Hooks.CameoList.cpp:472 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB741 | `SelectClass_ProcessInput_FixOffset5` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset5 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C010.c | src/Ext/Rules/Hooks.CameoList.cpp:480 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB802 | `SelectClass_ProcessInput_FixOffset6` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset6 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C030.c | src/Ext/Rules/Hooks.CameoList.cpp:489 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB825 | `SelectClass_ProcessInput_FixOffset7` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset7 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C060.c | src/Ext/Rules/Hooks.CameoList.cpp:498 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB920 | `SelectClass_ProcessInput_FixOffset8` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset8 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C080.c | src/Ext/Rules/Hooks.CameoList.cpp:507 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6AB92F | `SelectClass_ProcessInput_FixOffset9` | replaces_0.A:SidebarClass_ProcessCameoClick_FixOffset9 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/1002C0A0.c | src/Ext/Rules/Hooks.CameoList.cpp:516 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6ABBCB | `StripClass_AbandonCameosFromFactory_GetPointer1` | replaces_0.A:TabCameoListClass_AbandonCameosFromFactory_GetPointer1 | ✓ | 006ABB60 `StripClass::StripClass__AbandonFactoryProduction` | decompiled/functions/1002C2A0.c | src/Ext/Rules/Hooks.CameoList.cpp:526 | — |
| 6AC6D9 | `SidebarClass_FlashCameo` | replaces_0.A:MapClass_FlashCameo | ✓ | 006AC5F0 `SidebarClass::SidebarClass__FlashCameo` | decompiled/functions/1002C1D0.c | src/Ext/Rules/Hooks.CameoList.cpp:546 | src/app/sidebar_projection.rs |
| 712045 | `TechnoTypeClass_GetCameo` | kept | ✓ | 00712040 `TechnoTypeClass::TechnoTypeClass__GetCameo` | decompiled/functions/1005E480.c | src/Misc/Cameos.cpp:21 | src/app/presentation/sidebar_render.rs, src/render/sidebar_cameo_atlas.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002BCF0 | `FactoryClass::GetProgress` | 被钩子调用 | decompiled/functions/1002BCF0.c |
| 1002BCD0 | `FactoryClass::GetBuildTimeFrames` | 被钩子调用 | decompiled/functions/1002BCD0.c |
| 1002BC80 | `RulesExt::ClearCameos` | 被钩子调用 | decompiled/functions/1002BC80.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 1002BD50 | `ObjectTypeClass::IsBuildCat5` | 被钩子调用 | decompiled/functions/1002BD50.c |
| 1002BDB0 | `CameoDataStruct::SortsBefore` | 被钩子调用 | decompiled/functions/1002BDB0.c |
| 1002BD90 | `SidebarClass::SidebarClass__SwitchTab` | 被钩子调用 | decompiled/functions/1002BD90.c |
| 1002BF10 | `SidebarClass::SidebarClass__UpdateScrollButtons` | 被钩子调用 | decompiled/functions/1002BF10.c |
| 1002BD30 | `SidebarClass::SidebarClass__GetVisibleSlotCount` | 被钩子调用 | decompiled/functions/1002BD30.c |
| 1001DC90 | `PCX::BlitToSurface` | 被钩子调用 | decompiled/functions/1001DC90.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1003D800 | `TechnoTypeExt::ExtData::CameoIsElite` | 被钩子调用 | decompiled/functions/1003D800.c |

## 涉及的热点函数（4）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006A9540 `StripClass::StripClass__Draw` | 18 | H.country, S.core, T.cameo, T.factory, ext:Rules |
| 006AAD00 `SelectClass__Action` | 16 | S.core, T.cameo, T.factory, ext:Gadgets, ext:TechnoType |
| 006A92E0 `StripClass::StripClass__GetCameoTooltip` | 3 | S.core, T.cameo, U.bugfixes |
| 006A6300 `SidebarClass::SidebarClass__AddCameo` | 2 | T.cameo, ext:Rules |
