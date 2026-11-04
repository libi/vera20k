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
| `DiskLaserAnimEnabled` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:168 | ok | — |
| `Beam.Amplitude` | WeaponType | double | 40.0 | src/Ext/WeaponType/Body.cpp:171 | ok | — |
| `Beam.Color` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:173 | ok | — |
| `Beam.Duration` | WeaponType | int | 15 | src/Ext/WeaponType/Body.cpp:170 | ok | — |
| `Beam.IsHouseColor` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:172 | ok | src/rules/weapon_type.rs |
| `Bolt.Color1` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:188 | no_use_found | — |
| `Bolt.Color2` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:189 | no_use_found | — |
| `Bolt.Color3` | WeaponType | Nullable<ColorStruct> | (empty) | src/Ext/WeaponType/Body.cpp:190 | no_use_found | — |
| `Bolt.ParticleSystem` | WeaponType | Nullable<ParticleSystemTypeClass*> | (empty) | src/Ext/WeaponType/Body.cpp:191 | no_use_found | — |
| `IsDetachedRailgun` | WeaponType | Valueable<bool> | false | src/Ext/WeaponType/Body.cpp:168 | ok | — |
| `LaserThickness` | WeaponType | Valueable<int> | -1 | src/Ext/WeaponType/Body.cpp:193 | ok | src/rules/weapon_type.rs |
| `Wave.Color` | WeaponType | Nullable<Vector3D<int>> | (empty) | src/Ext/WeaponType/Body.cpp:180 | no_use_found | — |
| `Wave.Intensity` | WeaponType | Nullable<Vector3D<int>> | (empty) | src/Ext/WeaponType/Body.cpp:179 | no_use_found | — |
| `Wave.IsBigLaser` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:176 | ok | — |
| `Wave.IsHouseColor` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:177 | no_use_found | — |
| `Wave.IsLaser` | WeaponType | bool | false | src/Ext/WeaponType/Body.cpp:175 | ok | — |
| `Wave.ReverseAgainstAircraft` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:183 | ok | — |
| `Wave.ReverseAgainstBuildings` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:184 | ok | — |
| `Wave.ReverseAgainstInfantry` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:185 | ok | — |
| `Wave.ReverseAgainstOthers` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:186 | ok | — |
| `Wave.ReverseAgainstVehicles` | WeaponType | Valueable<bool>::Read | — | src/Ext/WeaponType/Body.cpp:182 | ok | — |

## 钩子（31）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 44ABD0 | `BuildingClass_FireLaser` | kept | ✓ | 0044ABD0 `BuildingClass::BuildingClass__FirePrismSupportBeam` | decompiled/functions/10016490.c | src/Ext/Building/Hooks.Prism.cpp:162 | src/sim/world/techno_ai/building_missions.rs |
| 4A76ED | `DiskLaserClass_Update_Anim` | kept | ✓ | 004A7340 `DiskLaserClass::DiskLaserClass__AI` | decompiled/functions/1004CFF0.c | src/Ext/Techno/Hooks.Firing.cpp:33 | — |
| 4C1F33 | `EBolt_Draw_Colors` | new_in_3.0 | ✓ | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/100561F0.c | src/Ext/WeaponType/Hooks.Bolt.cpp:63 | — |
| 4C24BE | `EBolt_Draw_Color1` | kept | ✓ | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/10056190.c | src/Ext/WeaponType/Hooks.Bolt.cpp:97 | — |
| 4C25CB | `EBolt_Draw_Color2` | kept | ✓ | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/100561B0.c | src/Ext/WeaponType/Hooks.Bolt.cpp:104 | — |
| 4C26CF | `EBolt_Draw_Color3` | removed_in_3.0 | — | 004C1F20 `EBolt__DrawRecursiveBolt` | decompiled/functions/100561D0.c | src/Ext/WeaponType/Hooks.Bolt.cpp:113 | — |
| 4C2951 | `EBolt_DTOR` | kept | ✓ | 004C2830 `EBoltMgr__UpdateAndDrawAll` | decompiled/functions/10056130.c | src/Ext/WeaponType/Hooks.Bolt.cpp:49 | src/app/presentation/electric_bolts.rs, src/sim/combat/electric_bolt.rs |
| 4C2AFF | `EBolt_Fire_Particles` | new_in_3.0 | ✓ | 004C2A60 `EBolt__Init` | decompiled/functions/100562C0.c | src/Ext/WeaponType/Hooks.Bolt.cpp:30 | src/sim/combat/electric_bolt.rs, src/sim/particles/spawn.rs |
| 6FD438 | `TechnoClass_FireLaser` | kept | ✓ | 006FD210 `TechnoClass::TechnoClass__SpawnLaser` | decompiled/functions/100563A0.c | src/Ext/WeaponType/Hooks.cpp:19 | src/sim/combat/fire_coord.rs |
| 6FD469 | `TechnoClass_FireEBolt` | kept | ✓ | 006FD460 `TechnoClass::TechnoClass__CreateElectricBolt` | decompiled/functions/10056350.c | src/Ext/WeaponType/Hooks.Bolt.cpp:11 | src/sim/combat/electric_bolt.rs, src/sim/combat/fire_coord.rs |
| 6FD64A | `TechnoClass_FireRadBeam1` | kept | ✓ | 006FD620 `TechnoClass::TechnoClass__SpawnRadBeam` | decompiled/functions/10056010.c | src/Ext/WeaponType/Hooks.Beam.cpp:6 | — |
| 6FD79C | `TechnoClass_FireRadBeam2` | kept | ✓ | 006FD620 `TechnoClass::TechnoClass__SpawnRadBeam` | decompiled/functions/10056080.c | src/Ext/WeaponType/Hooks.Beam.cpp:28 | — |
| 6FF1FB | `TechnoClass_Fire_DetachedRailgun` | new_in_3.0 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0C0.c | src/Ext/Techno/Hooks.Firing.cpp:77 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF26E | `TechnoClass_Fire_DetachedRailgun2` | new_in_3.0 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/1004D0F0.c | src/Ext/Techno/Hooks.Firing.cpp:87 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF449 | `TechnoClass_Fire_SonicWave` | new_in_3.0 | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10056EF0.c | src/Ext/WeaponType/Hooks.Wave.cpp:36 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF4DE | `TechnoClass_Fire_IsLaser` | kept | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/100563D0.c | src/Ext/WeaponType/Hooks.cpp:33 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 6FF5F5 | `TechnoClass_Fire_OtherWaves` | replaces_0.A:TechnoClass_Fire | ✓ | 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | decompiled/functions/10056E60.c | src/Ext/WeaponType/Hooks.Wave.cpp:58 | src/sim/aircraft/fire_at.rs, src/sim/combat/damage/attacker.rs, src/sim/combat/fire_coord.rs … |
| 75EE2E | `WaveClass_Draw_Green` | new_in_3.0 | ✓ | 0075EDF0 `WaveClass::WaveClass__Sonic_Pixel` | decompiled/functions/100570B0.c | src/Ext/WeaponType/Hooks.Wave.cpp:246 | src/render/wave_geometry.rs |
| 75EE57 | `WaveClass_Draw_Sonic` | kept ⚠改过 | ✓ | 0075EDF0 `WaveClass::WaveClass__Sonic_Pixel` | decompiled/functions/10057140.c | src/Ext/WeaponType/Hooks.Wave.cpp:228 | src/render/wave_geometry.rs |
| 75F38F | `WaveClass_DamageCell` | kept | ✓ | 0075F330 `WaveClass::WaveClass__DamageArea` | decompiled/functions/10056FA0.c | src/Ext/WeaponType/Hooks.Wave.cpp:380 | src/sim/world/mod.rs |
| 75FA29 | `WaveClass_Draw_Colors` | new_in_3.0 | ✓ | 0075F9F0 `WaveClass::WaveClass__DrawIt` | decompiled/functions/10057080.c | src/Ext/WeaponType/Hooks.Wave.cpp:162 | — |
| 7601C7 | `WaveClass_Draw_Magnetron` | replaces_0.A:WaveClass_Draw_Purple | ✓ | 00760190 `WaveClass::WaveClass__MagBeam_Pixel` | decompiled/functions/100570B0.c | src/Ext/WeaponType/Hooks.Wave.cpp:259 | — |
| 7601FB | `WaveClass_Draw_Magnetron2` | replaces_0.A:WaveClass_Draw_Magnetron | ✓ | 00760190 `WaveClass::WaveClass__MagBeam_Pixel` | decompiled/functions/100570D0.c | src/Ext/WeaponType/Hooks.Wave.cpp:274 | — |
| 760286 | `WaveClass_Draw_Magnetron3` | replaces_0.A:WaveClass_Draw_Magnetron2 | ✓ | 00760190 `WaveClass::WaveClass__MagBeam_Pixel` | decompiled/functions/10057110.c | src/Ext/WeaponType/Hooks.Wave.cpp:292 | — |
| 7609E3 | `WaveClass_Draw_NodLaser_Details` | new_in_3.0 | ✓ | 007609E0 `WaveClass::WaveClass__Draw_Laser` | decompiled/functions/10057120.c | src/Ext/WeaponType/Hooks.Wave.cpp:404 | — |
| 760BC2 | `WaveClass_Draw2` | kept ⚠改过 | ✓ | 007609E0 `WaveClass::WaveClass__Draw_Laser` | decompiled/functions/10057000.c | src/Ext/WeaponType/Hooks.Wave.cpp:193 | — |
| 760DE2 | `WaveClass_Draw3` | kept ⚠改过 | ✓ | 007609E0 `WaveClass::WaveClass__Draw_Laser` | decompiled/functions/10057040.c | src/Ext/WeaponType/Hooks.Wave.cpp:209 | — |
| 760F50 | `WaveClass_Update` | kept | ✓ | 00760F50 `WaveClass::WaveClass__AI` | decompiled/functions/100571A0.c | src/Ext/WeaponType/Hooks.Wave.cpp:100 | src/sim/wave.rs |
| 76110B | `WaveClass_RecalculateAffectedCells_Clear` | new_in_3.0 | ✓ | 007610F0 `WaveClass::WaveClass__UpdateCells` | decompiled/functions/10057180.c | src/Ext/WeaponType/Hooks.Wave.cpp:393 | src/sim/wave.rs, src/util/native_coord.rs |
| 762B62 | `WaveClass_Update_Beam` | new_in_3.0 | ✓ | 00762AF0 `WaveClass::WaveClass__Update_Wave` | decompiled/functions/10057290.c | src/Ext/WeaponType/Hooks.Wave.cpp:305 | src/sim/wave.rs, src/sim/world/techno_ai.rs |
| 763226 | `WaveClass_DTOR` | kept | ✓ | 00763200 `WaveClass::WaveClass__ScalarDeletingDestructor` | decompiled/functions/10056F40.c | src/Ext/WeaponType/Hooks.Wave.cpp:88 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10055210 | WeaponTypeExt::ExtData::GetBeamColor [source_0.A,3.0✓] | 使用键 | decompiled/functions/10055210.c |
| 100553F0 | FUN_100553f0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/100553F0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 10016180 | `LaserDrawClass::LaserDrawClass` | 被钩子调用 | decompiled/functions/10016180.c |
| 10016220 | `LaserDrawClass::LaserDrawClass` | 被钩子调用 | decompiled/functions/10016220.c |
| 10016280 | `RadBeam::Allocate` | 被钩子调用 | decompiled/functions/10016280.c |
| 100162A0 | `EBolt::Fire` | 被钩子调用 | decompiled/functions/100162A0.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10019A20 | `ParticleSystemClass::ParticleSystemClass` | 被钩子调用 | decompiled/functions/10019A20.c |
| 10056380 | `TechnoClass::CreateLaser` | 被钩子调用 | decompiled/functions/10056380.c |
| 10055160 | `WeaponTypeExt::CreateWave` | 被钩子调用 | decompiled/functions/10055160.c |
| 10055920 | `WeaponTypeExt::ModifyWaveColor` | 被钩子调用 | decompiled/functions/10055920.c |
| 10055270 | `WeaponTypeExt::GetWaveColorData` | 被钩子调用 | decompiled/functions/10055270.c |
| 10056E20 | `CellClass::Get3DCoords3` | 被钩子调用 | decompiled/functions/10056E20.c |
| 10056DC0 | `WaveClass::DamageArea` | 被钩子调用 | decompiled/functions/10056DC0.c |
| 10056E40 | `WaveClass::Update_Wave` | 被钩子调用 | decompiled/functions/10056E40.c |
| 10056DE0 | `WaveClass::Draw_Magnetic` | 被钩子调用 | decompiled/functions/10056DE0.c |
| 10056E00 | `WaveClass::Draw_NonMagnetic` | 被钩子调用 | decompiled/functions/10056E00.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006FDD50 `TechnoClass::TechnoClassFireAtSpawnsBullet` | 14 | B.firestorm, T.ammo, T.misc, W.beams, W.weaponmisc, ext:Bullet, ext:BulletType, ext:Techno, ext:WeaponType |
| 0075F330 `WaveClass::WaveClass__DamageArea` | 2 | U.bugfixes, W.beams |
