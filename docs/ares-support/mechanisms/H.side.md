# `H.side` Side 默认值与 EVA.Tag

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | done | docs/ares-side-defaults.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DefaultDisguise` | Side | Nullable<InfantryTypeClass*> | — | src/Ext/Side/Body.cpp:111 | check_0A_use | src/rules/native_processing.rs |
| `EVA.Tag` | Side | ValueableIdx<EVAVoices> | -1 | src/Ext/Side/Body.cpp:113 | check_0A_use | src/rules/native_processing.rs |
| `MessageTextColor` | Side | ValueableIdx<ColorScheme> | -1 | src/Ext/Side/Body.cpp:130 | check_0A_use | src/rules/native_processing.rs |
| `ToolTipColor` | Side | Valueable<ColorStruct> | (empty) | src/Ext/Side/Body.cpp:128 | check_0A_use | src/rules/native_processing.rs |

## 钩子（3）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 6DE0D3 | `TActionClass_Execute_MessageColor` | kept | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/1002FA80.c | src/Ext/Side/Hooks.cpp:220 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 72F440 | `Game_InitializeToolTipColor` | kept | 0072F440 `SetSidebarTextColor` | decompiled/functions/1002F360.c | src/Ext/Side/Hooks.cpp:235 | src/render/sidebar_text.rs |
| 7534E0 | `VoxClass_SetEVAIndex` | kept | 007534E0 `VoxClass__SetSide` | decompiled/functions/1002FB00.c | src/Ext/Side/Hooks.cpp:204 | src/rules/sound_ini.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DEA0 | SideExt::ExtData::GetDisguise [source_0.A] | 使用键 | decompiled/functions/1002DEA0.c |
| 10030570 | SWTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/10030570.c |
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 10051730 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10051730.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
