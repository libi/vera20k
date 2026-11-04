# `F.armor` [ArmorTypes] 自定义护甲与 Verses

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | done | docs/ares-custom-armor.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（1）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Verses` | WarheadType | — | — | src/Ext/WarheadType/Body.cpp:81 | unscanned | src/rules/ares_armor.rs, src/rules/warhead_type.rs, src/sim/game_entity/emp.rs |

## 钩子（8）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 489235 | `GetTotalDamage_Verses` | new_in_3.0 | ✓ | 00489180 `ComputeWarheadVersesFalloffDamage` | decompiled/functions/10054230.c | src/Ext/WarheadType/Hooks.Verses.cpp:156 | src/sim/combat/damage/estimate.rs, src/sim/combat/damage/kernel.rs, src/sim/combat/damage/mod.rs … |
| 6F36E3 | `TechnoClass_SelectWeapon_Verses` | new_in_3.0 | ✓ | 006F3330 `TechnoClass::TechnoClass__SelectWeaponAgainst` | decompiled/functions/10054360.c | src/Ext/WarheadType/Hooks.Verses.cpp:51 | src/sim/combat/combat_weapon.rs, src/sim/combat/gattling.rs, src/sim/combat/world_fire.rs … |
| 6F7D3D | `TechnoClass_CanAutoTargetObject_Verses` | replaces_0.A:Verses_fld_4 | ✓ | 006F7CA0 `TechnoClass::TechnoClass__Evaluate_Candidate` | decompiled/functions/10054270.c | src/Ext/WarheadType/Hooks.Verses.cpp:101 | src/sim/combat/greatest_threat.rs, src/sim/combat/threat_range.rs |
| 6FCB6A | `TechnoClass_CanFire_Verses` | replaces_0.A:Verses_fld_3 | ✓ | 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | decompiled/functions/100542A0.c | src/Ext/WarheadType/Hooks.Verses.cpp:87 | src/rules/weapon_type.rs, src/sim/combat/fire_error.rs, src/sim/combat/greatest_threat.rs … |
| 708AF7 | `TechnoClass_ShouldRetaliate_Verses` | replaces_0.A:Verses_fld_2 | ✓ | 007087C0 `TechnoClass::TechnoClass__ShouldRetaliate` | decompiled/functions/100543E0.c | src/Ext/WarheadType/Hooks.Verses.cpp:73 | src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs, src/sim/combat/world_receiver.rs … |
| 70CEA0 | `TechnoClass_EvalThreatRating_Verses1` | new_in_3.0 | ✓ | 0070CD10 `TechnoClass::TechnoClass__Calculate_Threat_Score` | decompiled/functions/10054310.c | src/Ext/WarheadType/Hooks.Verses.cpp:115 | src/rules/object_type.rs, src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs |
| 70CF45 | `TechnoClass_EvalThreatRating` | new_in_3.0 | ✓ | 0070CD10 `TechnoClass::TechnoClass__Calculate_Threat_Score` | decompiled/functions/100542E0.c | src/Ext/WarheadType/Hooks.Verses.cpp:115 | src/rules/object_type.rs, src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs |
| 75DDCC | `Verses_OrigParser` | kept | ✓ | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/10054410.c | src/Ext/WarheadType/Hooks.Verses.cpp:170 | src/rules/warhead_type.rs |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FC0B0 `TechnoClass::TechnoClass__GetFireError` | 7 | F.armor, T.ammo, U.bugfixes, W.ivanbomb, ext:Techno |
| 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | 5 | F.armor, U.bugfixes, ext:WarheadType |
| 006F7CA0 `TechnoClass::TechnoClass__Evaluate_Candidate` | 3 | F.armor, ext:Techno |
| 006F3330 `TechnoClass::TechnoClass__SelectWeaponAgainst` | 2 | F.armor, T.ammo |
| 007087C0 `TechnoClass::TechnoClass__ShouldRetaliate` | 2 | F.armor, ext:Techno |
