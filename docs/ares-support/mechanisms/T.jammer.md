# `T.jammer` 雷达干扰、抑制/指示范围、传感器警告

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DesignatorRange` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:460 | ok | src/rules/superweapon_influence.rs |
| `InhibitorRange` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:461 | ok | src/rules/superweapon_influence.rs |
| `RadarJamRadius` | TechnoType | Valueable<int> | 0 | src/Ext/TechnoType/Body.cpp:346 | ok | — |
| `SensorArray.Warn` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:419 | ok | — |

## 钩子（11）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 43FE69 | `BuildingClass_Update_SensorArray` | kept | ✓ | 0043FB20 `BuildingClass::BuildingClass__Update` | decompiled/functions/10016E30.c | src/Ext/Building/Hooks.Sensor.cpp:129 | src/sim/combat/world_receiver.rs, src/sim/world/bridge_orchestrator.rs, src/sim/world/building_working_sound.rs … |
| 4416A2 | `BuildingClass_Destroy_SensorArray` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10016CA0.c | src/Ext/Building/Hooks.Sensor.cpp:161 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 448B70 | `BuildingClass_ChangeOwnership_SensorArrayA` | kept | ✓ | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10016CA0.c | src/Ext/Building/Hooks.Sensor.cpp:142 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 448C3E | `BuildingClass_ChangeOwnership_SensorArrayB` | kept | ✓ | 00448260 `BuildingClass::BuildingClass__ChangeOwner` | decompiled/functions/10016CD0.c | src/Ext/Building/Hooks.Sensor.cpp:151 | src/sim/miner/miner_system.rs, src/sim/radio/tether.rs, src/sim/sensor_lifecycle.rs … |
| 4524A3 | `BuildingClass_DisableThings` | kept | ✓ | 00452480 `BuildingClass__ApplyOfflineEffects` | decompiled/functions/10016D00.c | src/Ext/Building/Hooks.Sensor.cpp:117 | src/sim/building_art.rs |
| 4549F8 | `BuildingClass_UpdatePowered_SensorArray` | kept | ✓ | 004549B0 `BuildingClass::BuildingClass__UpdateGapAndSpecialEffects` | decompiled/functions/10016E30.c | src/Ext/Building/Hooks.Sensor.cpp:104 | src/sim/building_art_power.rs, src/sim/capture_manager.rs, src/sim/powered_unit.rs … |
| 4556E1 | `BuildingClass_SensorArrayDeactivate` | kept | ✓ | 004556D0 `BuildingClass::BuildingClass__RemoveSensorArrayAt` | decompiled/functions/10016DB0.c | src/Ext/Building/Hooks.Sensor.cpp:69 | src/sim/sensor_lifecycle.rs, src/sim/vision/mod.rs |
| 455828 | `BuildingClass_SensorArrayActivate` | kept | ✓ | 00455820 `BuildingClass::BuildingClass__AddSensorArrayAt` | decompiled/functions/10016D60.c | src/Ext/Building/Hooks.Sensor.cpp:56 | src/sim/sensor_lifecycle.rs, src/sim/vision/mod.rs |
| 455923 | `BuildingClass_SensorArray_BuildingRedraw` | kept | ✓ | 00455820 `BuildingClass::BuildingClass__AddSensorArrayAt` | decompiled/functions/10016DF0.c | src/Ext/Building/Hooks.Sensor.cpp:88 | src/sim/sensor_lifecycle.rs, src/sim/vision/mod.rs |
| 4566F9 | `BuildingClass_GetRangeOfRadial_SensorArray` | kept | ✓ | 004566B0 `BuildingClass::BuildingClass__GetSensorRange` | decompiled/functions/10016D30.c | src/Ext/Building/Hooks.Sensor.cpp:171 | src/sim/world/gap_generator.rs |
| 70DA95 | `TechnoClass_RadarTrackingUpdate_AnnounceDetected` | kept | ✓ | 0070D990 `TechnoClass::TechnoClass__IdleAnimDispatch` | decompiled/functions/10016E60.c | src/Ext/Building/Hooks.Sensor.cpp:27 | src/render/radar/geometry.rs, src/render/radar/tracker.rs, src/sim/world/lifecycle.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10068500 | AttachEffectClass::Update [source_0.A,3.0✓] | 使用键 | decompiled/functions/10068500.c |
| 1006D1E0 | NewSWType::GetTargetingData [translated] | 使用键 | decompiled/functions/1006D1E0.c |
| 100138F0 | `BuildingExt::ExtData::UpdateSensorArray` | 被钩子调用 | decompiled/functions/100138F0.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 100683F0 | `JammerClass::Unjam` | 被钩子调用 | decompiled/functions/100683F0.c |

## 涉及的热点函数（5）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 00448260 `BuildingClass::BuildingClass__ChangeOwner` | 9 | B.academy, B.prism, B.spy, T.jammer, ext:Building |
| 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | 7 | B.spy, B.trench, T.jammer, U.bugfixes, ext:House |
| 0043FB20 `BuildingClass::BuildingClass__Update` | 6 | B.firestorm, T.ammo, T.jammer, ext:Building, ext:BuildingType |
| 004549B0 `BuildingClass::BuildingClass__UpdateGapAndSpecialEffects` | 5 | B.prism, T.jammer, ext:Building, ext:Techno |
| 004566B0 `BuildingClass::BuildingClass__GetSensorRange` | 3 | T.jammer, ext:Techno |
