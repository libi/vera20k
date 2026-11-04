# `W.filter` 弹头作用对象与伤害门槛

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（8）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `AffectsEnemies` | WarheadType | bool | true | src/Ext/WarheadType/Body.cpp:97 | check_0A_use | src/rules/ares_emp.rs, src/rules/ares_warhead.rs, src/rules/warhead_type.rs |
| `AffectsOwner` | WarheadType | Nullable<bool> | (empty) | src/Ext/WarheadType/Body.cpp:98 | check_0A_use | src/rules/ares_emp.rs, src/rules/ares_warhead.rs, src/rules/warhead_type.rs |
| `AllowZeroDamage` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:152 | ok | — |
| `CellSpread.MaxAffect` | WarheadType | Valueable<int> | -1 | src/Ext/WarheadType/Body.cpp:116 | no_use_found | — |
| `DamageAirThreshold` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:121 | check_0A_use | — |
| `EffectsRequireDamage` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:150 | ok | src/rules/ares_warhead.rs |
| `EffectsRequireVerses` | WarheadType | Valueable<bool> | true | src/Ext/WarheadType/Body.cpp:151 | ok | src/rules/ares_warhead.rs, src/rules/warhead_type.rs |
| `Malicious` | WarheadType | Valueable<bool> | true | src/Ext/WarheadType/Body.cpp:112 | ok | — |

## 钩子（8）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4892BE | `DamageArea_NullDamage` | new_in_3.0 | 00489280 `Apply_area_damage` | decompiled/functions/10053940.c | src/Recovered/DamageArea.cpp:18 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4893BA | `DamageArea_DamageAir` | kept ⚠改过 | 00489280 `Apply_area_damage` | decompiled/functions/100533A0.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:126 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4895B8 | `DamageArea_CellSpread1` | kept | 00489280 `Apply_area_damage` | decompiled/functions/100531F0.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:12 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4895C7 | `DamageArea_CellSpread2` | kept | 00489280 `Apply_area_damage` | decompiled/functions/10053250.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:26 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4899BE | `DamageArea_CellSpread3` | kept | 00489280 `Apply_area_damage` | decompiled/functions/10053270.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:38 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4899DA | `DamageArea_Damage_MaxAffect` | kept | 00489280 `Apply_area_damage` | decompiled/functions/10053400.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:59 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 4F94A5 | `HouseClass_BuildingUnderAttack_Malicious` | replaces_0.A:HouseClass_BuildingUnderAttack | 004F93E0 `HouseClass::HouseClass__NotifyUnderAttack` | decompiled/functions/10053980.c | src/Recovered/HouseClass.cpp:312 | src/sim/trigger_runtime.rs, src/sim/world/mod.rs, src/sim/world/techno_ai/building_retaliation.rs |
| 702819 | `TechnoClass_ReceiveDamage_Aftermath` | new_in_3.0 | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AB50.c | src/Recovered/TechnoClass.cpp:626 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10051B70 | WarheadTypeExt::CanAffectHouse [translated] | 使用键 | decompiled/functions/10051B70.c |
| 10052700 | WarheadTypeExt::ExtData::applyIronCurtain [source_0.A] | 使用键 | decompiled/functions/10052700.c |
| 10052890 | WarheadTypeExt::ExtData::applyPermaMC [source_0.A] | 使用键 | decompiled/functions/10052890.c |
| 10052B40 | WarheadTypeExt::ExtData::applyKillDriver [translated] | 使用键 | decompiled/functions/10052B40.c |
| 10052870 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052870.c |
| 10058520 | `Helpers::Alex::getCellSpreadItems` | 被钩子调用 | decompiled/functions/10058520.c |
| 100434C0 | `TechnoExt::ExtData::CanKillDriver` | 被钩子调用 | decompiled/functions/100434C0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10021310 | `HouseExt::GetHouseKind` | 被钩子调用 | decompiled/functions/10021310.c |
| 100033E0 | `HouseTypeClass::FindIndexOfName` | 被钩子调用 | decompiled/functions/100033E0.c |
| 10020D20 | `HouseClass::FindByCountryIndex` | 被钩子调用 | decompiled/functions/10020D20.c |
| 100456D0 | `TechnoExt::ExtData::KillDriver` | 被钩子调用 | decompiled/functions/100456D0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00489280 `Apply_area_damage` | 14 | R.tiberium, U.bugfixes, W.filter, ext:Rules, recovered:DamageArea |
