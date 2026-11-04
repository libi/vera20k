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
| `DefaultDisguise` | Side | Nullable<InfantryTypeClass*> | — | src/Ext/Side/Body.cpp:116 | ok | src/rules/native_processing.rs |
| `EVA.Tag` | Side | ValueableIdx<EVAVoices> | -1 | src/Ext/Side/Body.cpp:118 | ok | src/rules/native_processing.rs |
| `MessageTextColor` | Side | ValueableIdx<ColorScheme> | -1 | src/Ext/Side/Body.cpp:135 | ok | src/rules/native_processing.rs |
| `ToolTipColor` | Side | Valueable<ColorStruct> | (empty) | src/Ext/Side/Body.cpp:133 | ok | src/rules/native_processing.rs |

## 钩子（3）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6DE0D3 | `TActionClass_Execute_MessageColor` | kept | ✓ | 006DD8B0 `TActionClass::TActionClass__Execute` | decompiled/functions/1002FA80.c | src/Ext/Side/Hooks.cpp:394 | src/map/playfield.rs, src/map/resolved_terrain/cell_recalc.rs, src/sim/trigger_runtime.rs … |
| 72F440 | `Game_InitializeToolTipColor` | kept | ✓ | 0072F440 `SetSidebarTextColor` | decompiled/functions/1002F360.c | src/Ext/Side/Hooks.cpp:409 | src/render/sidebar_text.rs |
| 7534E0 | `VoxClass_SetEVAIndex` | kept | ✓ | 007534E0 `VoxClass__SetSide` | decompiled/functions/1002FB00.c | src/Ext/Side/Hooks.cpp:378 | src/rules/sound_ini.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DEA0 | SideExt::ExtData::GetDisguise [source_0.A,3.0✓] | 使用键 | decompiled/functions/1002DEA0.c |
| 10030570 | SWTypeExt::ExtMap.SaveStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10030570.c |
| 10055AE0 | FUN_10055ae0 [translated,inferred] | 使用键 | decompiled/functions/10055AE0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10076390 | `CustomPalette::Save` | 被钩子调用 | decompiled/functions/10076390.c |
| 10076370 | `AresPCXFile::Save` | 被钩子调用 | decompiled/functions/10076370.c |
| 10055AC0 | `BombListClass::Plant` | 被钩子调用 | decompiled/functions/10055AC0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
