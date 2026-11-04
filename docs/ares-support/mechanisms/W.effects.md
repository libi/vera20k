# `W.effects` 弹头效果：离子炮、涟漪、闪光、声呐、禁武、预冲击动画、桥

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（17）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DamageAirConsiderBridges` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:211 | check_0A_use | — |
| `BridgeAbsoluteDestroyer` | WarheadType | Nullable<bool> | (empty) | src/Ext/WarheadType/Body.cpp:114 | ok | — |
| `DisableWeapons.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:137 | ok | — |
| `Flash.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:138 | ok | — |
| `IonCannon` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:140 | check_0A_use | — |
| `IonCannon.Beam` | WarheadType | Nullable<AnimTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:146 | no_use_found | — |
| `IonCannon.Blast` | WarheadType | Nullable<AnimTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:145 | no_use_found | — |
| `IonCannon.Damage` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:148 | no_use_found | — |
| `IonCannon.Ripple` | WarheadType | Nullable<int>::Read | — | src/Ext/WarheadType/Body.cpp:144 | check_0A_use | — |
| `IonCannon.Rock` | WarheadType | Valueable<bool> | true | src/Ext/WarheadType/Body.cpp:141 | no_use_found | — |
| `IonCannon.Warhead` | WarheadType | Nullable<WarheadTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:147 | no_use_found | — |
| `NukeFlash.Duration` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:102 | check_0A_use | — |
| `PreImpactAnim` | WarheadType | ValueableIdx<AnimTypeClass> | -1 | src/Ext/WarheadType/Body.cpp:103 | check_0A_use | — |
| `PreImpactAnim.Moves` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:104 | no_use_found | — |
| `PreventScatter` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:113 | no_use_found | — |
| `Ripple.Radius` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:143 | check_0A_use | — |
| `Sonar.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:136 | ok | — |

## 钩子（6）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 467E59 | `BulletClass_Update_NukeBall` | kept ⚠改过 | 004666E0 `BulletClass::BulletClass__AI` | decompiled/functions/10036960.c | src/Ext/SWType/Hooks.Legacy.cpp:749 | src/sim/combat/projectile_delivery.rs, src/sim/projectile.rs, src/sim/world/lifecycle.rs … |
| 4893BA | `DamageArea_DamageAir` | kept ⚠改过 | 00489280 `Apply_area_damage` | decompiled/functions/100533A0.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:126 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 489E9F | `DamageArea_BridgeAbsoluteDestroyer` | new_in_3.0 | 00489280 `Apply_area_damage` | decompiled/functions/10053880.c | src/Recovered/DamageArea.cpp:41 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 53CBF5 | `IonBlastClass_Update_Duration` | kept | 0053CBE0 `IonBlastClass::IonBlastClass__Update` | decompiled/functions/100541B0.c | src/Ext/WarheadType/Hooks.Ion.cpp:15 | src/sim/world/ion_blast.rs |
| 53CC0D | `IonBlastClass_Update_DTOR` | kept | 0053CBE0 `IonBlastClass::IonBlastClass__Update` | decompiled/functions/10054190.c | src/Ext/WarheadType/Hooks.Ion.cpp:6 | src/sim/world/ion_blast.rs |
| 702819 | `TechnoClass_ReceiveDamage_Aftermath` | new_in_3.0 | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AB50.c | src/Recovered/TechnoClass.cpp:626 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 10051AE0 | WarheadTypeExt::ExtData::applyEMP [source_0.A] | 使用键 | decompiled/functions/10051AE0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 10032370 | `SWTypeExt::ChangeLighting` | 被钩子调用 | decompiled/functions/10032370.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 10052870 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052870.c |
| 10052B40 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052B40.c |
| 10035FC0 | `IonBlastClass::IonBlastClass` | 被钩子调用 | decompiled/functions/10035FC0.c |
| 10051730 | `EMPulse::CreateEMPulse` | 被钩子调用 | decompiled/functions/10051730.c |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0053CBE0 `IonBlastClass::IonBlastClass__Update` | 3 | W.effects, recovered:IonBlastClass |
