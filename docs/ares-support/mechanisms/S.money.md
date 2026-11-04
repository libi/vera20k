# `S.money` 超武费用与持续扣费

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core | done | docs/ares-superweapon-money.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `EVA.InsufficientFunds` | SWType | ValueableIdx<VoxClass> | -1 | src/Ext/SWType/Body.cpp:570 | no_use_found | src/rules/ruleset.rs |
| `Message.InsufficientFunds` | SWType | Valueable<CSFText> | (empty) | src/Ext/SWType/Body.cpp:649 | ok | src/rules/superweapon_type.rs |
| `Money.Amount` | SWType | Valueable<int> | 0 | src/Ext/SWType/Body.cpp:583 | ok | src/rules/superweapon_type.rs |
| `Money.DrainAmount` | SWType | Valueable<int> | 0 | src/Ext/SWType/Body.cpp:584 | ok | — |
| `Money.DrainDelay` | SWType | Valueable<int> | 0 | src/Ext/SWType/Body.cpp:585 | ok | — |
| `SW.ChargeToDrainRatio` | SWType | Nullable<double> | (empty) | src/Ext/SWType/Body.cpp:610 | no_use_found | — |

## 钩子（5）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 6A932B | `StripClass_GetTip_MoneySW` | replaces_0.A:CameoClass_GetTip_MoneySW | ✓ | 006A92E0 `StripClass::StripClass__GetCameoTooltip` | decompiled/functions/10034F80.c | src/Ext/SWType/Hooks.cpp:129 | — |
| 6A99B7 | `StripClass_Draw_SuperDarken` | replaces_0.A:TabCameoListClass_Draw_SuperDarken | ✓ | 006A9540 `StripClass::StripClass__Draw` | decompiled/functions/10034EF0.c | src/Ext/SWType/Hooks.cpp:435 | src/app/presentation/sidebar_render.rs, src/app/sidebar_projection.rs, src/sim/production/production_tech.rs … |
| 6AAEDF | `SidebarClass_ProcessCameoClick_SuperWeapons` | kept ⚠改过 | ✓ | 006AAD00 `SelectClass__Action` | decompiled/functions/10034DA0.c | src/Ext/SWType/Hooks.cpp:75 | src/app/input/commands.rs, src/app/input/dispatch.rs, src/app/input/sidebar_eva.rs … |
| 6CB920 | `SuperClass_ClickFire` | kept | ✓ | 006CB920 `SuperClass::SuperClass__ClickFire` | decompiled/functions/10035130.c | src/Ext/SWType/Hooks.cpp:560 | src/sim/superweapon/fire.rs, src/sim/superweapon/mod.rs |
| 6CBD6B | `SuperClass_Update_DrainMoney` | kept | ✓ | 006CBCA0 `SuperClass::SuperClass__AI` | decompiled/functions/100358A0.c | src/Ext/SWType/Hooks.cpp:708 | src/app/match_runtime/eva_producers.rs, src/sim/superweapon/mod.rs, src/sim/world/mod.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100382D0 | SWTypeExt::TryFireAITargeted [translated] | 使用键 | decompiled/functions/100382D0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 100348D0 | `SuperClass::CanFire` | 被钩子调用 | decompiled/functions/100348D0.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 10033BD0 | `SWTypeExt::ExtData::PrintMessage` | 被钩子调用 | decompiled/functions/10033BD0.c |
| 10032110 | `SWTypeExt::ExtData::CanFireAITargeted` | 被钩子调用 | decompiled/functions/10032110.c |
| 100328A0 | `SWTypeExt::ExtData::GetNewSWType` | 被钩子调用 | decompiled/functions/100328A0.c |
| 100348F0 | `SuperClass::Launch` | 被钩子调用 | decompiled/functions/100348F0.c |
| 10032B90 | `SWTypeExt::ExtData::HasShotsLeft` | 被钩子调用 | decompiled/functions/10032B90.c |
| 10034910 | `SuperClass::Lose` | 被钩子调用 | decompiled/functions/10034910.c |
| 100348B0 | `SuperClass::GetRechargeTime` | 被钩子调用 | decompiled/functions/100348B0.c |
| 10034930 | `TimerStruct::Pause` | 被钩子调用 | decompiled/functions/10034930.c |
| 10034960 | `SuperClass::StopPreclickAnim` | 被钩子调用 | decompiled/functions/10034960.c |
| 10032690 | `SWTypeExt::ExtData::GetChargeToDrainRatio` | 被钩子调用 | decompiled/functions/10032690.c |
| 100325D0 | `SWTypeExt::Deactivate` | 被钩子调用 | decompiled/functions/100325D0.c |
| 10034860 | `HouseClass::CanTransactMoney` | 被钩子调用 | decompiled/functions/10034860.c |
| 10011E20 | `HouseClass::GiveMoney` | 被钩子调用 | decompiled/functions/10011E20.c |
| 100130C0 | `HouseClass::TakeMoney` | 被钩子调用 | decompiled/functions/100130C0.c |
| 1006D1E0 | `NewSWType::GetTargetingData` | 被钩子调用 | decompiled/functions/1006D1E0.c |
| 100385A0 | `HouseClass::Fire_SW` | 被钩子调用 | decompiled/functions/100385A0.c |
| 1006CB60 | `TargetingData::~TargetingData` | 被钩子调用 | decompiled/functions/1006CB60.c |
