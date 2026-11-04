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
| `DamageAirConsiderBridges` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:166 | ok | — |
| `BridgeAbsoluteDestroyer` | WarheadType | Nullable<bool> | (empty) | src/Ext/WarheadType/Body.cpp:130 | ok | — |
| `DisableWeapons.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:153 | ok | — |
| `Flash.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:154 | ok | — |
| `IonCannon` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:156 | ok | — |
| `IonCannon.Beam` | WarheadType | Nullable<AnimTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:162 | ok | — |
| `IonCannon.Blast` | WarheadType | Nullable<AnimTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:161 | ok | — |
| `IonCannon.Damage` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:164 | ok | — |
| `IonCannon.Ripple` | WarheadType | Nullable<int>::Read | — | src/Ext/WarheadType/Body.cpp:160 | no_use_found | — |
| `IonCannon.Rock` | WarheadType | Valueable<bool> | true | src/Ext/WarheadType/Body.cpp:157 | ok | — |
| `IonCannon.Warhead` | WarheadType | Nullable<WarheadTypeClass*> | (empty) | src/Ext/WarheadType/Body.cpp:163 | ok | — |
| `NukeFlash.Duration` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:118 | ok | — |
| `PreImpactAnim` | WarheadType | ValueableIdx<AnimTypeClass> | -1 | src/Ext/WarheadType/Body.cpp:119 | ok | — |
| `PreImpactAnim.Moves` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:120 | ok | — |
| `PreventScatter` | WarheadType | Valueable<bool> | false | src/Ext/WarheadType/Body.cpp:129 | ok | — |
| `Ripple.Radius` | WarheadType | Nullable<int> | (empty) | src/Ext/WarheadType/Body.cpp:159 | ok | — |
| `Sonar.Duration` | WarheadType | Valueable<int> | 0 | src/Ext/WarheadType/Body.cpp:152 | ok | — |

## 钩子（8）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 467E59 | `BulletClass_Update_NukeBall` | kept ⚠改过 | ✓ | 004666E0 `BulletClass::BulletClass__AI` | decompiled/functions/10036960.c | src/Ext/SWType/Hooks.Legacy.cpp:816 | src/sim/combat/projectile_delivery.rs, src/sim/projectile.rs, src/sim/world/lifecycle.rs … |
| 4893BA | `DamageArea_DamageAir` | kept ⚠改过 | ✓ | 00489280 `Apply_area_damage` | decompiled/functions/100533A0.c | src/Ext/WarheadType/Hooks.CellSpread.cpp:135 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 489E9F | `DamageArea_BridgeAbsoluteDestroyer` | new_in_3.0 | ✓ | 00489280 `Apply_area_damage` | decompiled/functions/10053880.c | src/Ext/WarheadType/Hooks.cpp:289 | src/rules/warhead_type.rs, src/sim/anim_class.rs, src/sim/c4_plant.rs … |
| 53CB91 | `IonBlastClass_DTOR` | new_in_3.0 | ✓ | 0053CB90 `IonBlastClass__Destructor` | decompiled/functions/10053EF0.c | src/Ext/WarheadType/Hooks.Ion.cpp:11 | — |
| 53CBF5 | `IonBlastClass_Update_Duration` | kept | ✓ | 0053CBE0 `IonBlastClass::IonBlastClass__Update` | decompiled/functions/100541B0.c | src/Ext/WarheadType/Hooks.Ion.cpp:28 | src/sim/world/ion_blast.rs |
| 53CC0D | `IonBlastClass_Update_DTOR` | kept | ✓ | 0053CBE0 `IonBlastClass::IonBlastClass__Update` | decompiled/functions/10054190.c | src/Ext/WarheadType/Hooks.Ion.cpp:19 | src/sim/world/ion_blast.rs |
| 53CC63 | `IonBlastClass_Update_Beam` | new_in_3.0 | ✓ | 0053CBE0 `IonBlastClass::IonBlastClass__Update` | decompiled/functions/10053F10.c | src/Ext/WarheadType/Hooks.Ion.cpp:56 | src/sim/world/ion_blast.rs |
| 702819 | `TechnoClass_ReceiveDamage_Aftermath` | new_in_3.0 | ✓ | 00701900 `TechnoClass::TechnoClass__ReceiveDamage` | decompiled/functions/1004AB50.c | src/Ext/Techno/Hooks.cpp:2559 | src/rules/object_type.rs, src/rules/phobos_shields.rs, src/sim/combat/damage/gates.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10051AE0 | WarheadTypeExt::ExtData::applyRipples [translated] | 使用键 | decompiled/functions/10051AE0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 10032370 | `SWTypeExt::ChangeLighting` | 被钩子调用 | decompiled/functions/10032370.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 10052B40 | `WarheadTypeExt::ExtData::applyKillDriver` | 被钩子调用 | decompiled/functions/10052B40.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 10035FC0 | `IonBlastClass::IonBlastClass` | 被钩子调用 | decompiled/functions/10035FC0.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
