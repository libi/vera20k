# `T.gap` 裂缝产生器半径

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（2）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `GapRadiusInCells` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:568 | ok | src/rules/object_type.rs |
| `SuperGapRadiusInCells` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:569 | ok | src/rules/object_type.rs |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 41BE80 | `ObjectClass_DrawRadialIndicator` | new_in_3.0 | ✓ | 0041BE80 `ObjectClass::ObjectClass__DrawRadialIndicator` | decompiled/functions/10049A70.c | src/Ext/Techno/Hooks.cpp:3161 | — |
| 44E2B0 | `BuildingClass_Mi_Unload_LargeGap` | new_in_3.0 | ✓ | 0044D880 `BuildingClass::BuildingClass__Mission_Unload` | decompiled/functions/1004D1B0.c | src/Ext/Techno/Hooks.Gap.cpp:163 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/transport_unload.rs … |
| 454BDC | `BuildingClass_UpdatePowered_LargeGap` | new_in_3.0 | ✓ | 004549B0 `BuildingClass::BuildingClass__UpdateGapAndSpecialEffects` | decompiled/functions/1004D250.c | src/Ext/Techno/Hooks.Gap.cpp:188 | src/sim/building_art_power.rs, src/sim/capture_manager.rs, src/sim/powered_unit.rs … |
| 6FB1B5 | `TechnoClass_CreateGap_LargeGap` | new_in_3.0 | ✓ | 006FB470 `TechnoClass::TechnoClass__RemoveCloakShroud` | decompiled/functions/1004D330.c | src/Ext/Techno/Hooks.Gap.cpp:213 | src/sim/game_entity.rs, src/sim/vision/mod.rs, src/sim/vision/shroud_knowledge.rs … |
