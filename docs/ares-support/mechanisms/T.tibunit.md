# `T.tibunit` 单位与矿石：免疫、残留、溢出、转化

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `TiberiumProof` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:402 | ok | — |
| `TiberiumRemains` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:403 | ok | — |
| `TiberiumSpill` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:404 | ok | — |
| `TiberiumTransmogrify` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:405 | ok | — |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 441B30 | `BuildingClass_Destroy_Refinery` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10023DA0.c | src/Ext/House/Hooks.cpp:355 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 4D85E4 | `FootClass_UpdatePosition_TiberiumDamage` | kept | ✓ | 004D85D0 `FootClass::FootClass__PerCellProcess` | decompiled/functions/10049280.c | src/Ext/Techno/Hooks.cpp:1545 | src/sim/movement/per_cell.rs, src/sim/world/techno_ai_cloak.rs |
| 702200 | `TechnoClass_ReceiveDamage_SpillTiberium` | kept | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AEC0.c | src/Ext/Techno/Hooks.cpp:1592 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |
| 702216 | `TechnoClass_ReceiveDamage_TiberiumHeal` | kept | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004B050.c | src/Ext/Techno/Hooks.cpp:1521 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1000AC50 | `MapClass::GetRandomCoordsNear` | 被钩子调用 | decompiled/functions/1000AC50.c |
| 10023D40 | `CellClass::IncreaseTiberium` | 被钩子调用 | decompiled/functions/10023D40.c |
| 100453B0 | `TechnoTypeClass::HasAbility` | 被钩子调用 | decompiled/functions/100453B0.c |
| 100149C0 | `CellClass::GetContainedTiberiumIndex` | 被钩子调用 | decompiled/functions/100149C0.c |
| 100508B0 | `TiberiumExt::ExtData::GetWarhead` | 被钩子调用 | decompiled/functions/100508B0.c |
| 100507A0 | `TiberiumExt::ExtData::GetDamage` | 被钩子调用 | decompiled/functions/100507A0.c |
| 100468B0 | `TechnoExt::SpawnVisceroid` | 被钩子调用 | decompiled/functions/100468B0.c |
| 10011DE0 | `CellClass::GetNeighbourCell` | 被钩子调用 | decompiled/functions/10011DE0.c |
