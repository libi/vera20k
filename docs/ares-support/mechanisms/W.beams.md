# `W.beams` 波束、激光、电弧、磁轨炮

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M4 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（21）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `DiskLaserAnimEnabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:213 | check_0A_use | — |
| `Beam.Amplitude` | WeaponType | double | 40.0 | src/Ext/WeaponType/Body.cpp:49 | check_0A_use | — |
| `Beam.Color` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:51 | check_0A_use | — |
| `Beam.Duration` | WeaponType | int | 15 | src/Ext/WeaponType/Body.cpp:48 | check_0A_use | — |
| `Beam.IsHouseColor` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:50 | check_0A_use | src/rules/weapon_type.rs |
| `Bolt.Color1` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:66 | no_use_found | — |
| `Bolt.Color2` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:67 | no_use_found | — |
| `Bolt.Color3` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:68 | no_use_found | — |
| `Bolt.ParticleSystem` | WeaponType | Nullable<ParticleSystemTypeClass*> | (empty) | src/Ext/WeaponType/Body.cpp:69 | no_use_found | — |
| `IsDetachedRailgun` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:46 | ok | — |
| `LaserThickness` | WeaponType | Valueable<int> | -1 | src/Ext/WeaponType/Body.cpp:71 | check_0A_use | src/rules/weapon_type.rs |
| `Wave.Color` | WeaponType | Nullable<Vector3D<int>> | (empty) | src/Ext/WeaponType/Body.cpp:58 | no_use_found | — |
| `Wave.Intensity` | WeaponType | Nullable<Vector3D<int>> | (empty) | src/Ext/WeaponType/Body.cpp:57 | no_use_found | — |
| `Wave.IsBigLaser` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:54 | ok | — |
| `Wave.IsHouseColor` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:55 | no_use_found | — |
| `Wave.IsLaser` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:53 | ok | — |
| `Wave.ReverseAgainstAircraft` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:61 | check_0A_use | — |
| `Wave.ReverseAgainstBuildings` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:62 | check_0A_use | — |
| `Wave.ReverseAgainstInfantry` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:63 | check_0A_use | — |
| `Wave.ReverseAgainstOthers` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:64 | check_0A_use | — |
| `Wave.ReverseAgainstVehicles` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:60 | check_0A_use | — |

## 钩子（20）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 44ABD0 | `BuildingClass_FireLaser` | kept | 0044ABD0 `BuildingClass::BuildingClass__FirePrismSupportBeam` | decompiled/functions/10016490.c | src/Ext/Building/Hooks.Prism.cpp:137 | src/sim/world/techno_ai/building_missions.rs |
| 4A76ED | `DiskLaserClass_Update_Anim` | kept | 004A7340 `DiskLaserClass::DiskLaserClass__AI` | decompiled/functions/1004CFF0.c | src/Ext/Techno/Hooks.Firing.cpp:27 | — |
| 4C24BE | `EBolt_Draw_Color1` | kept | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/10056190.c | src/Ext/WeaponType/Hooks.Bolt.cpp:25 | — |
| 4C25CB | `EBolt_Draw_Color2` | kept | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/100561B0.c | src/Ext/WeaponType/Hooks.Bolt.cpp:32 | — |
| 4C26CF | `EBolt_Draw_Color3` | removed_in_3.0 | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/100561D0.c | src/Ext/WeaponType/Hooks.Bolt.cpp | — |
| 4C2951 | `EBolt_DTOR` | kept | 004C2830 `EBoltMgr__UpdateAndDrawAll` | decompiled/functions/10056130.c | src/Ext/WeaponType/Hooks.Bolt.cpp:15 | src/app/presentation/electric_bolts.rs, src/sim/combat/electric_bolt.rs |
| 6FD438 | `TechnoClass_FireLaser` | kept | 006FD210 `TechnoClass::TechnoClass__SpawnLaser` | decompiled/functions/100563A0.c | src/Ext/WeaponType/Hooks.cpp:8 | src/sim/combat/fire_coord.rs |
| 6FD469 | `TechnoClass_FireEBolt` | kept | 006FD460 `TechnoClass::TechnoClass__CreateElectricBolt` | decompiled/functions/10056350.c | src/Ext/WeaponType/Hooks.Bolt.cpp:4 | src/sim/combat/electric_bolt.rs, src/sim/combat/fire_coord.rs |
| 6FD64A | `TechnoClass_FireRadBeam1` | kept | 006FD620 `TechnoClass::TechnoClass__SpawnRadBeam` | decompiled/functions/10056010.c | src/Ext/WeaponType/Hooks.Beam.cpp:6 | — |
| 6FD79C | `TechnoClass_FireRadBeam2` | kept | 006FD620 `TechnoClass::TechnoClass__SpawnRadBeam` | decompiled/functions/10056080.c | src/Ext/WeaponType/Hooks.Beam.cpp:28 | — |
| 6FF1FB | `TechnoClass_Fire_DetachedRailgun` | new_in_3.0 | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0C0.c | src/Recovered/TechnoClass.cpp:816 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF26E | `TechnoClass_Fire_DetachedRailgun2` | new_in_3.0 | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0F0.c | src/Recovered/TechnoClass.cpp:826 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF4DE | `TechnoClass_Fire_IsLaser` | kept | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/100563D0.c | src/Ext/WeaponType/Hooks.cpp:22 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF5F5 | `TechnoClass_Fire_OtherWaves` | replaces_0.A:TechnoClass_Fire | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10056E60.c | src/Recovered/TechnoClass.cpp:1179 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 75EE57 | `WaveClass_Draw_Sonic` | kept ⚠改过 | 0075EDF0 `WaveClass::WaveClass__Sonic_Pixel` | decompiled/functions/10057140.c | src/Ext/WeaponType/Hooks.Wave.cpp:152 | src/render/wave_geometry.rs |
| 75F38F | `WaveClass_DamageCell` | kept | 0075F330 `WaveClass::WaveClass__DamageArea` | decompiled/functions/10056FA0.c | src/Ext/WeaponType/Hooks.Wave.cpp:172 | src/sim/world/mod.rs |
| 760BC2 | `WaveClass_Draw2` | kept ⚠改过 | 007609E0 `WaveClass::WaveClass__Draw_Laser` | decompiled/functions/10057000.c | src/Ext/WeaponType/Hooks.Wave.cpp:117 | — |
| 760DE2 | `WaveClass_Draw3` | kept ⚠改过 | 007609E0 `WaveClass::WaveClass__Draw_Laser` | decompiled/functions/10057040.c | src/Ext/WeaponType/Hooks.Wave.cpp:133 | — |
| 760F50 | `WaveClass_Update` | kept | 00760F50 `WaveClass::WaveClass__AI` | decompiled/functions/100571A0.c | src/Ext/WeaponType/Hooks.Wave.cpp:35 | src/sim/wave.rs |
| 763226 | `WaveClass_DTOR` | kept | 00763200 `WaveClass::WaveClass__ScalarDeletingDestructor` | decompiled/functions/10056F40.c | src/Ext/WeaponType/Hooks.Wave.cpp:23 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 10055210 | WeaponTypeExt::ExtData::GetBeamColor [source_0.A] | 使用键 | decompiled/functions/10055210.c |
| 100553F0 | FUN_100553f0 [source_0.A,inferred] | 使用键 | decompiled/functions/100553F0.c |
| 10016280 | `RadBeam::Allocate` | 被钩子调用 | decompiled/functions/10016280.c |
| 100162A0 | `EBolt::Fire` | 被钩子调用 | decompiled/functions/100162A0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10056380 | `TechnoClass::CreateLaser` | 被钩子调用 | decompiled/functions/10056380.c |
| 10055160 | `WeaponTypeExt::CreateWave` | 被钩子调用 | decompiled/functions/10055160.c |
| 10055920 | `WeaponTypeExt::ModifyWaveColor` | 被钩子调用 | decompiled/functions/10055920.c |
| 10056E20 | `CellClass::Get3DCoords3` | 被钩子调用 | decompiled/functions/10056E20.c |
| 10056DC0 | `WaveClass::DamageArea` | 被钩子调用 | decompiled/functions/10056DC0.c |
| 10056E40 | `WaveClass::Update_Wave` | 被钩子调用 | decompiled/functions/10056E40.c |

## 涉及的热点函数（4）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 004C1F20 `EBolt__DrawRecursiveBolt` | 4 | W.beams, recovered:EBolt |
| 007609E0 `WaveClass::WaveClass__Draw_Laser` | 3 | W.beams, recovered:WaveClass |
| 0075EDF0 `WaveClass::WaveClass__Sonic_Pixel` | 2 | W.beams, recovered:WaveClass |
| 0075F330 `WaveClass::WaveClass__DamageArea` | 2 | W.beams, recovered:WaveClass |
