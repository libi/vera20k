# `T.cloak` 隐身阶段、条件隐身、隐身/显形音效

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（10）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `CloakHeight` | Rules | Nullable<int> | (empty) | src/Ext/Rules/Body.cpp:256 | check_0A_use | — |
| `DecloakSound` | Rules | NullableIdx<VocClass> | (empty) | src/Ext/Rules/Body.cpp:255 | check_0A_use | — |
| `StealthSpeakDelay` | Rules | Valueable<double> | 0.0 | src/Ext/Rules/Body.cpp:216 | check_0A_use | — |
| `CanPassiveAquire.Cloak` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:478 | check_0A_use | — |
| `Cloakable.Allowed` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:411 | check_0A_use | src/rules/native_processing.rs |
| `Cloakable.Deployed` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:410 | check_0A_use | — |
| `Cloakable.Powered` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:409 | check_0A_use | — |
| `Cloakable.Stages` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:412 | check_0A_use | — |
| `CloakSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:407 | check_0A_use | src/rules/ruleset.rs |
| `DecloakSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:408 | check_0A_use | — |

## 钩子（14）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 481D3D | `CellClass_CrateBeingCollected_Cloak1` | kept | 00481A00 `CellClass::CellClass__PickupCrate` | decompiled/functions/1005F8D0.c | src/Misc/Crates.cpp:49 | src/rules/crate_rules.rs, src/rules/effect_asset_catalog.rs, src/rules/object_type.rs … |
| 4D9920 | `FootClass_SelectAutoTarget_Cloaked` | kept | 004D9920 `FootClass::FootClass__Greatest_Threat` | decompiled/functions/100491E0.c | src/Ext/Techno/Hooks.cpp:1570 | src/sim/aircraft/hunt_mission.rs, src/sim/combat/combat_targeting.rs, src/sim/combat/greatest_threat.rs … |
| 4D99AA | `FootClass_PointerGotInvalid_Parasite` | kept | 004D9960 `FootClass::FootClass__PointerExpired` | decompiled/functions/10048280.c | src/Ext/Techno/Hooks.Cloak.cpp:113 | src/sim/world/lifecycle.rs, src/sim/world/techno_ai_cloak.rs |
| 4DBDD4 | `FootClass_IsCloakable_CloakStop` | kept | 004DBDA0 `FootClass::FootClass__IsCloakable` | decompiled/functions/10048250.c | src/Ext/Techno/Hooks.Cloak.cpp:64 | src/sim/world/techno_ai_cloak.rs |
| 62A283 | `ParasiteClass_PointerGotInvalid_Cloak` | kept | 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | decompiled/functions/100482D0.c | src/Ext/Techno/Hooks.Cloak.cpp:138 | src/sim/combat/parasite.rs |
| 6F5388 | `TechnoClass_DrawExtras_Submerged` | kept | 006F5190 `TechnoClass::TechnoClass__DrawExtras` | decompiled/functions/10048350.c | src/Ext/Techno/Hooks.Cloak.cpp:72 | src/app/presentation/render/draw_plan_lowering.rs |
| 6FB757 | `TechnoClass_UpdateCloak` | kept | 006FB740 `TechnoClass::TechnoClass__CloakingTick` | decompiled/functions/100484A0.c | src/Ext/Techno/Hooks.Cloak.cpp:9 | src/rules/object_type.rs, src/sim/cloak_disguise.rs, src/sim/world/lifecycle.rs … |
| 6FBC90 | `TechnoClass_ShouldNotBeCloaked` | kept | 006FBC90 `TechnoClass::TechnoClass__ShouldUncloak` | decompiled/functions/100483D0.c | src/Ext/Techno/Hooks.Cloak.cpp:30 | src/rules/object_type.rs, src/sim/cloak_disguise.rs, src/sim/world/techno_ai_cloak.rs |
| 6FBDC0 | `TechnoClass_ShouldBeCloaked` | kept | 006FBDC0 `TechnoClass::TechnoClass__CanAutoCloak` | decompiled/functions/100483B0.c | src/Ext/Techno/Hooks.Cloak.cpp:19 | src/rules/object_type.rs, src/sim/cloak_disguise.rs, src/sim/combat/rof.rs … |
| 7036EB | `TechnoClass_Uncloak_CloakingStages` | kept | 007036C0 `TechnoClass::TechnoClass__StartUncloaking` | decompiled/functions/10048400.c | src/Ext/Techno/Hooks.Cloak.cpp:87 | src/audio/sfx.rs, src/sim/cloak_disguise.rs, src/sim/cloak_transitions.rs … |
| 70375B | `TechnoClass_Uncloak_DecloakSound` | kept | 007036C0 `TechnoClass::TechnoClass__StartUncloaking` | decompiled/functions/10048450.c | src/Ext/Techno/Hooks.Cloak.cpp:52 | src/audio/sfx.rs, src/sim/cloak_disguise.rs, src/sim/cloak_transitions.rs … |
| 70380A | `TechnoClass_Cloak_CloakSound` | kept | 00703770 `TechnoClass::TechnoClass__StartCloaking` | decompiled/functions/10048300.c | src/Ext/Techno/Hooks.Cloak.cpp:44 | src/sim/cloak_disguise.rs, src/sim/cloak_transitions.rs, src/sim/world/lifecycle.rs … |
| 703A79 | `TechnoClass_VisualCharacter_CloakingStages` | kept | 00703860 `TechnoClass::TechnoClass_GetVisualState` | decompiled/functions/100484D0.c | src/Ext/Techno/Hooks.Cloak.cpp:98 | src/sim/cloak_disguise.rs, src/sim/world/lifecycle.rs, src/sim/world/techno_ai.rs |
| 70DA95 | `TechnoClass_RadarTrackingUpdate_AnnounceDetected` | kept | 0070D990 `TechnoClass::TechnoClass__IdleAnimDispatch` | decompiled/functions/10016E60.c | src/Ext/Building/Hooks.Sensor.cpp:9 | src/render/radar/geometry.rs, src/render/radar/tracker.rs, src/sim/world/lifecycle.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1002D410 | SideExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1002D410.c |
| 1002DDD0 | FUN_1002ddd0 [source_0.A,inferred] | 使用键 | decompiled/functions/1002DDD0.c |
| 10043A40 | TechnoExt::ExtData::CloakDisallowed [source_0.A] | 使用键 | decompiled/functions/10043A40.c |
| 10064F70 | GetSectionCached [translated,inferred] | 使用键 | decompiled/functions/10064F70.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 10013CD0 | `Game::RaiseError` | 被钩子调用 | decompiled/functions/10013CD0.c |
| 10043990 | `TechnoExt::ExtData::CloakAllowed` | 被钩子调用 | decompiled/functions/10043990.c |
| 10043450 | `TechnoExt::CanICloakByDefault` | 被钩子调用 | decompiled/functions/10043450.c |
| 10078000 | `Exception::PrepareSnapshotDirectory` | 被钩子调用 | decompiled/functions/10078000.c |

## 涉及的热点函数（3）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 006F5190 `TechnoClass::TechnoClass__DrawExtras` | 3 | T.cloak, W.ivanbomb, ext:Techno |
| 004D9960 `FootClass::FootClass__PointerExpired` | 2 | T.cloak, ext:Techno |
| 0062A260 `ParasiteClass::ParasiteClass__PointerExpired` | 2 | T.cloak, U.bugfixes |
