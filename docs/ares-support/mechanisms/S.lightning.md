# `S.lightning` LightningStorm

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（18）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `LightningRod.Modifier` | BuildingType | Valueable<double> | 1.0 | src/Ext/BuildingType/Body.cpp:221 | ok | — |
| `Lightning.BoltExplosion` | SWType | Nullable<AnimTypeClass*> | — | src/Misc/SWTypes/LightningStorm.cpp:88 | needs_check | — |
| `Lightning.Bolts` | SWType | NullableVector<AnimTypeClass*> | — | src/Misc/SWTypes/LightningStorm.cpp:91 | ok | — |
| `Lightning.CloudHeight` | SWType | Valueable<int> | -1 | src/Misc/SWTypes/LightningStorm.cpp:87 | ok | — |
| `Lightning.Clouds` | SWType | NullableVector<AnimTypeClass*> | — | src/Misc/SWTypes/LightningStorm.cpp:90 | ok | — |
| `Lightning.Debris` | SWType | NullableVector<AnimTypeClass*> | — | src/Misc/SWTypes/LightningStorm.cpp:92 | ok | — |
| `Lightning.DebrisMax` | SWType | Valueable<int> | 0 | src/Misc/SWTypes/LightningStorm.cpp:86 | needs_check | — |
| `Lightning.DebrisMin` | SWType | Valueable<int> | 0 | src/Misc/SWTypes/LightningStorm.cpp:85 | needs_check | — |
| `Lightning.Duration` | SWType | Nullable<int> | — | src/Misc/SWTypes/LightningStorm.cpp:77 | ok | — |
| `Lightning.HitDelay` | SWType | Nullable<int> | — | src/Misc/SWTypes/LightningStorm.cpp:79 | ok | — |
| `Lightning.IgnoreLightningRod` | SWType | Valueable<bool> | false | src/Misc/SWTypes/LightningStorm.cpp:84 | ok | — |
| `Lightning.PrintText` | SWType | Nullable<bool> | — | src/Misc/SWTypes/LightningStorm.cpp:83 | ok | — |
| `Lightning.RadarOutage` | SWType | Nullable<int> | — | src/Misc/SWTypes/LightningStorm.cpp:78 | ok | — |
| `Lightning.RadarOutageAffects` | SWType | Valueable<SuperWeaponAffectedHouse> | SuperWeaponAffectedHouse::None | src/Misc/SWTypes/LightningStorm.cpp:89 | ok | — |
| `Lightning.ScatterCount` | SWType | Valueable<int> | 1 | src/Misc/SWTypes/LightningStorm.cpp:81 | ok | — |
| `Lightning.ScatterDelay` | SWType | Nullable<int> | — | src/Misc/SWTypes/LightningStorm.cpp:80 | ok | — |
| `Lightning.Separation` | SWType | Nullable<int> | — | src/Misc/SWTypes/LightningStorm.cpp:82 | no_use_found | — |
| `Lightning.Sounds` | SWType | NullableIdxVector<VocClass> | — | src/Misc/SWTypes/LightningStorm.cpp:93 | ok | — |

## 钩子（5）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 48A59A | `MapClass_SelectDamageAnimation_LightningWarhead` | kept | ✓ | 0048A4F0 `Warhead__SelectExplosionAnim` | decompiled/functions/100377E0.c | src/Ext/SWType/Hooks.Legacy.cpp:690 | src/rules/effect_asset_catalog.rs, src/sim/combat/detonation_anim.rs, src/sim/crates/effects.rs … |
| 539EB0 | `LightningStorm_Start` | kept | ✓ | 00539EB0 `LightningStorm__Start` | decompiled/functions/10036B00.c | src/Ext/SWType/Hooks.Legacy.cpp:218 | src/app/match_runtime/sound_dispatch.rs, src/audio/events.rs, src/sim/superweapon/lightning_storm.rs … |
| 53A140 | `LightningStorm_Strike` | kept | ✓ | 0053A140 `LightningStorm__CreateCloudBolt` | decompiled/functions/10036D60.c | src/Ext/SWType/Hooks.Legacy.cpp:526 | src/sim/superweapon/lightning_storm.rs |
| 53A300 | `LightningStorm_Strike2` | kept | ✓ | 0053A300 `LightningStorm__GroundStrike` | decompiled/functions/10036F50.c | src/Ext/SWType/Hooks.Legacy.cpp:576 | src/sim/superweapon/lightning_storm.rs |
| 53A6CF | `LightningStorm_Update` | kept ⚠改过 | ✓ | 0053A6C0 `LightningStorm__Process` | decompiled/functions/10037330.c | src/Ext/SWType/Hooks.Legacy.cpp:315 | src/rules/ruleset.rs, src/sim/anim_class.rs, src/sim/scenario_session.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100226D0 | SWTypeExt::ExtData::GetSound [source_0.A,3.0✓] | 使用键 | decompiled/functions/100226D0.c |
| 1002DD40 | FUN_1002dd40 [missing,inferred,3.0✓] | 使用键 | decompiled/functions/1002DD40.c |
| 1002DFB0 | FUN_1002dfb0 [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1002DFB0.c |
| 10032ED0 | SWTypeExt::Launch [source_0.A,3.0✓] | 使用键 | decompiled/functions/10032ED0.c |
| 10055920 | WeaponTypeExt::ModifyWaveColor [translated,inferred] | 使用键 | decompiled/functions/10055920.c |
| 10060540 | CSFLoader::FindDynamic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10060540.c |
| 1006D120 | VectorClass<ChronoWarpStateMachine::ChronoWarpContainer>::GetItem [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1006D120.c |
| 10072B00 | SW_LightningStorm::Activate [translated] | 使用键 | decompiled/functions/10072B00.c |
| 100753A0 | SW_SonarPulse::Activate [translated] | 使用键 | decompiled/functions/100753A0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10021480 | `HouseExt::ExtData::GetSuperWeaponStatus` | 被钩子调用 | decompiled/functions/10021480.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 100130C0 | `HouseClass::TakeMoney` | 被钩子调用 | decompiled/functions/100130C0.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 1001BB30 | `ColorScheme::FindIndex` | 被钩子调用 | decompiled/functions/1001BB30.c |
| 100028C0 | `MessageListClass::PrintMessage` | 被钩子调用 | decompiled/functions/100028C0.c |
| 10033E90 | `VoxClass::SilenceIndex` | 被钩子调用 | decompiled/functions/10033E90.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10032D30 | `SWTypeExt::ExtData::IsHouseAffected` | 被钩子调用 | decompiled/functions/10032D30.c |
| 10036410 | `HouseClass::CreateRadarOutage` | 被钩子调用 | decompiled/functions/10036410.c |
| 10036560 | `ScenarioClass::UpdateLighting` | 被钩子调用 | decompiled/functions/10036560.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10010090 | `VocClass::PlayGlobal` | 被钩子调用 | decompiled/functions/10010090.c |
| 10012D60 | `MapClass::RedrawSidebar` | 被钩子调用 | decompiled/functions/10012D60.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10036430 | `CellClass::FindTechnoNearestTo` | 被钩子调用 | decompiled/functions/10036430.c |
| 100326B0 | `SWTypeExt::ExtData::GetDamage` | 被钩子调用 | decompiled/functions/100326B0.c |
| 10032B40 | `SWTypeExt::ExtData::GetWarhead` | 被钩子调用 | decompiled/functions/10032B40.c |
| 10032D20 | `NukeFlash::IsFadingIn` | 被钩子调用 | decompiled/functions/10032D20.c |
| 10036470 | `NukeFlash::IsFadingOut` | 被钩子调用 | decompiled/functions/10036470.c |
| 10036530 | `PsyDom::Update` | 被钩子调用 | decompiled/functions/10036530.c |
| 10036520 | `ChronoScreenEffect::Update` | 被钩子调用 | decompiled/functions/10036520.c |
| 100364C0 | `LightningStorm::Strike2` | 被钩子调用 | decompiled/functions/100364C0.c |
| 100364E0 | `LightningStorm::Strike` | 被钩子调用 | decompiled/functions/100364E0.c |
| 10032980 | `SWTypeExt::ExtData::GetRange` | 被钩子调用 | decompiled/functions/10032980.c |
| 100363D0 | `MapClass::CellExists` | 被钩子调用 | decompiled/functions/100363D0.c |
| 10032D50 | `SWTypeExt::ExtData::IsTechnoAffected` | 被钩子调用 | decompiled/functions/10032D50.c |
