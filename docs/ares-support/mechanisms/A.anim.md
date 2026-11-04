# `A.anim` 动画：自定义调色板、伤害延迟、步兵归属、粒子范围、武器

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CustomPalette` | AnimType | FUN_10076160 | — | src/Ext/AnimType/Body.cpp:24 | ok | — |
| `Damage.Delay` | AnimType | Valueable<int> | 0 | src/Ext/AnimType/Body.cpp:32 | ok | — |
| `MakeInfantryOwner` | AnimType | Valueable<OwnerHouseKind> | OwnerHouseKind::Invoker | src/Ext/AnimType/Body.cpp:21 | ok | src/rules/art_data.rs |
| `SpawnsParticle.RangeMaximum` | AnimType | Valueable<Leptons> | Leptons(0) | src/Ext/AnimType/Body.cpp:28 | ok | — |
| `SpawnsParticle.RangeMinimum` | AnimType | Valueable<Leptons> | Leptons(0) | src/Ext/AnimType/Body.cpp:27 | ok | — |
| `Weapon` | AnimType | Valueable<WeaponTypeClass*> | nullptr | src/Ext/AnimType/Body.cpp:31 | ok | src/app/diagnostics/tactical_capture/map_observation.rs, src/rules/ruleset.rs, src/sim/attach_effect.rs … |

## 钩子（3）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 424538 | `AnimClass_Update_DamageDelay` | new_in_3.0 | ✓ | 00423AC0 `AnimClass::AnimClass__AI` | decompiled/functions/1000AFE0.c | src/Ext/AnimType/Hooks.cpp:181 | src/sim/anim_class.rs, src/sim/bounce.rs, src/sim/radiation.rs |
| 425002 | `AnimClass_Expired_SpawnsParticle` | new_in_3.0 | ✓ | 00424F00 `AnimClass::AnimClass__Middle` | decompiled/functions/1000AE40.c | src/Ext/AnimType/Hooks.cpp:117 | src/rules/art_data.rs, src/sim/anim_class.rs, src/sim/combat/effects.rs … |
| 6F9E50 | `TechnoClass_Update` | kept | ✓ | 006F9E50 `TechnoClass::TechnoClass__AI_Update` | decompiled/functions/1004B320.c | src/Ext/Techno/Hooks.cpp:150 | src/sim/combat/greatest_threat.rs, src/sim/shield.rs, src/sim/world/techno_ai.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000A5F0 | AnimTypeExt::SetMakeInfOwner [source_0.A,3.0✓] | 使用键 | decompiled/functions/1000A5F0.c |
| 10060540 | CSFLoader::FindDynamic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10060540.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10021310 | `HouseExt::GetHouseKind` | 被钩子调用 | decompiled/functions/10021310.c |
| 1000AC30 | `MapClass::GetCellFloorHeight` | 被钩子调用 | decompiled/functions/1000AC30.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 1000ACB0 | `Math::cos` | 被钩子调用 | decompiled/functions/1000ACB0.c |
| 1000ACC0 | `Math::sin` | 被钩子调用 | decompiled/functions/1000ACC0.c |
| 1000ABD0 | `ParticleSystemClass::ParticleSystemClass__SpawnParticle_OfType` | 被钩子调用 | decompiled/functions/1000ABD0.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 100191E0 | `BulletTypeExt::ExtData::CreateBullet` | 被钩子调用 | decompiled/functions/100191E0.c |
| 1000AC10 | `BulletClass::Detonate` | 被钩子调用 | decompiled/functions/1000AC10.c |
| 10043650 | `TechnoExt::ConvertToType` | 被钩子调用 | decompiled/functions/10043650.c |
| 1006A6F0 | `PoweredUnitClass::Update` | 被钩子调用 | decompiled/functions/1006A6F0.c |
| 10044300 | `TechnoExt::Destroy` | 被钩子调用 | decompiled/functions/10044300.c |
| 10058E00 | `AttachEffectClass::Update` | 被钩子调用 | decompiled/functions/10058E00.c |
| 10011A60 | `CellClass::GetBuilding` | 被钩子调用 | decompiled/functions/10011A60.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 100454F0 | `TechnoExt::ExtData::IsPowered` | 被钩子调用 | decompiled/functions/100454F0.c |
| 100487D0 | `TechnoClass::Reactivate` | 被钩子调用 | decompiled/functions/100487D0.c |
| 10048690 | `TechnoClass::Deactivate` | 被钩子调用 | decompiled/functions/10048690.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 10068460 | `JammerClass::UnjamAll` | 被钩子调用 | decompiled/functions/10068460.c |
| 10068500 | `JammerClass::Update` | 被钩子调用 | decompiled/functions/10068500.c |
| 10044490 | `TechnoExt::ExtData::DisplayBounty` | 被钩子调用 | decompiled/functions/10044490.c |
