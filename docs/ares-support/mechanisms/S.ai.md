# `S.ai` 超武 AI 使用与目标选择

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（7）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SW.AIRequiresHouse` | SWType | — | — | src/Ext/SWType/Body.cpp:153 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.AIRequiresTarget` | SWType | Nullable<SuperWeaponTarget> | — | src/Ext/SWType/Body.cpp:152 | no_use_found | src/rules/superweapon_targeting.rs |
| `SW.AITargeting` | SWType | — | — | src/Ext/SWType/Body.cpp:145 | unscanned | src/rules/superweapon_targeting.rs |
| `SW.AITargeting.Constraints` | SWType | Nullable<SuperWeaponAITargetingConstraints> | (empty) | src/Ext/SWType/Body.cpp:147 | ok | src/rules/superweapon_targeting.rs |
| `SW.AITargeting.Preference` | SWType | Nullable<SuperWeaponAITargetingPreference> | (empty) | src/Ext/SWType/Body.cpp:148 | ok | src/rules/superweapon_targeting.rs |
| `SW.AutoFire` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:122 | check_0A_use | src/rules/superweapon_targeting.rs |
| `SW.UseAITargeting` | SWType | Valueable<bool> | false | src/Ext/SWType/Body.cpp:144 | check_0A_use | src/rules/superweapon_targeting.rs |

## 钩子（5）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 4C78D6 | `Networking_RespondToEvent_SpecialPlace` | new_in_3.0 | 004C6CB0 `EventClass::EventClass__Execute` | decompiled/functions/10038E90.c | src/Recovered/Networking.cpp:18 | src/sim/command.rs, src/sim/radio/tether.rs, src/sim/world/world_commands.rs |
| 5098F0 | `HouseClass_Update_AI_TryFireSW` | kept | 005098F0 `HouseClass::HouseClass__AI_TryFireSW` | decompiled/functions/10038E10.c | src/Ext/SWType/Hooks.Targeting.cpp:643 | src/sim/house_state.rs, src/sim/house_strategy.rs, src/sim/superweapon/ai_fire.rs |
| 50B1D0 | `HouseClass_UpdateSuperWeaponsUnavailable` | kept | 0050B1D0 `HouseClass::HouseClass__Grant_Provided_Supers` | decompiled/functions/10039580.c | src/Ext/SWType/Hooks.Update.cpp:154 | src/sim/ai_base_building.rs |
| 6A99B7 | `StripClass_Draw_SuperDarken` | replaces_0.A:TabCameoListClass_Draw_SuperDarken | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/10034EF0.c | src/Recovered/StripClass.cpp:669 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6AAEDF | `SidebarClass_ProcessCameoClick_SuperWeapons` | kept ⚠改过 | 006AAD00 `SelectClass__Action` | decompiled/functions/10034DA0.c | src/Ext/SWType/Hooks.cpp:63 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10032110 | SWTypeExt::ExtData::CanFireAITargeted [translated] | 使用键 | decompiled/functions/10032110.c |
| 10032660 | SWTypeExt::ExtData::GetAIPreference [translated] | 使用键 | decompiled/functions/10032660.c |
| 10032ED0 | SWTypeExt::Launch [source_0.A] | 使用键 | decompiled/functions/10032ED0.c |
| 10032100 | `PsyDom::Active` | 被钩子调用 | decompiled/functions/10032100.c |
| 10021480 | `HouseExt::ExtData::GetSuperWeaponStatus` | 被钩子调用 | decompiled/functions/10021480.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 100130C0 | `HouseClass::TakeMoney` | 被钩子调用 | decompiled/functions/100130C0.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
| 10033BB0 | `VocClass::PlayIndexAtPos` | 被钩子调用 | decompiled/functions/10033BB0.c |
| 10011780 | `RadarEventClass::Create` | 被钩子调用 | decompiled/functions/10011780.c |
| 100028C0 | `MessageListClass::PrintMessage` | 被钩子调用 | decompiled/functions/100028C0.c |
| 10033E90 | `VoxClass::SilenceIndex` | 被钩子调用 | decompiled/functions/10033E90.c |
| 100348D0 | `SuperClass::CanFire` | 被钩子调用 | decompiled/functions/100348D0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 100382D0 | `SWTypeExt::TryFireAITargeted` | 被钩子调用 | decompiled/functions/100382D0.c |
| 100385A0 | `HouseClass::Fire_SW` | 被钩子调用 | decompiled/functions/100385A0.c |
| 10011E40 | `SuperClass::Grant` | 被钩子调用 | decompiled/functions/10011E40.c |
| 100113B0 | `SidebarClass::AddCameo` | 被钩子调用 | decompiled/functions/100113B0.c |
| 10011E00 | `SidebarClass::GetObjectTabIdx` | 被钩子调用 | decompiled/functions/10011E00.c |
| 10012DA0 | `SidebarClass::RepaintSidebar` | 被钩子调用 | decompiled/functions/10012DA0.c |
