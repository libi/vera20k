# `B.rubble` 建筑废墟

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
示例备注（2026-11-04，对照 ares-src `d310643a` 的源码和 3.0 反编译）。

**表现**：设了 `Rubble.Destroyed` 的建筑被摧毁时，原地换成废墟建筑；在废墟上设 `Rubble.Intact`，友方工程师进入后恢复成原建筑。`*.Remove=yes` 只移除、不生成新建筑。

**代码位置**：
- 换建筑：`BuildingTypeExt::SwitchRubble`（1000F2B0）在 `src/Ext/BuildingType/Body.cpp`，用 `rg` 搜；
- 摧毁钩子：`src/Ext/Building/Hooks.Trenches.cpp`；
- 工程师进入：`src/Ext/Infantry/Hooks.cpp`。

**摧毁路径的时机**（`BuildingClass::DestructionEffects` 004415F0）：依次为摧毁动画和粒子 → `Health=0` → 若由攻击导致则置 `NoCrew` → **441F12 换成废墟** → `SpawnSurvivors` → `KillPassengers` → **441F2C 把地基上的单位踢出去**。

**和 0.A 不同、实现时容易照错的地方**：
- `SwitchRubble` 不检查 `Put` 是否失败，放置失败时仍返回新建筑。
- 工程师进入时，先销毁旧建筑（`UnInit`），再由新建筑把工程师放出来，**工程师不被消耗**（跳到 0x51A65D）。只 Remove 时跳到 0x519FB9，也就是原版"满血不修"分支。0.A 放置失败时会恢复旧建筑，3.0 没有这段。
- 光标钩子 51E635 是 3.0 新增的：工程师指向同盟方的、带 `Rubble.Intact`/`.Remove` 的建筑时，动作为 GRepair，光标为 0x5E。
- 3.0 不再修改废墟类型的 `Capturable`/`TogglePower`/`Unsellable`/`CanBeOccupied`（0.A 会改）。

**确定性**：`Owner=random` 会从 Scenario RNG 取一次随机数。没有传 invoker/killer/victim，所以这三项都回退为旧建筑的归属。

**VERA20K 线索**（未逐一确认）：
- 摧毁流程：`src/sim/combat/destruction_effects.rs`；幸存者：`src/sim/crew_survival.rs`（已引用 0x441F12）；
- 工程师进入：`src/sim/movement/per_cell.rs`（0x519630）；光标：`src/app/input/context_order.rs`、`cursor.rs`（0x51E3B0）；
- 放出单位：`src/sim/production/production_spawn.rs`（0x443C60）；
- 归属解析：`Simulation::house_kind`（`src/sim/world/house_defeat.rs`）；归属读取器：`HouseSelector`（`src/rules/ares_warhead.rs`）。
<!-- /notes -->

## 配置键（10）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Rubble.Destroyed` | BuildingType | Valueable<BuildingTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:225 | ok | — |
| `Rubble.Destroyed.Anim` | BuildingType | Valueable<AnimTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:227 | ok | — |
| `Rubble.Destroyed.Owner` | BuildingType | Valueable<OwnerHouseKind> | OwnerHouseKind::Default | src/Ext/BuildingType/Body.cpp:229 | ok | — |
| `Rubble.Destroyed.Remove` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:233 | ok | — |
| `Rubble.Destroyed.Strength` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:231 | ok | — |
| `Rubble.Intact` | BuildingType | Valueable<BuildingTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:226 | ok | — |
| `Rubble.Intact.Anim` | BuildingType | Valueable<AnimTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:228 | ok | — |
| `Rubble.Intact.Owner` | BuildingType | Valueable<OwnerHouseKind> | OwnerHouseKind::Default | src/Ext/BuildingType/Body.cpp:230 | ok | — |
| `Rubble.Intact.Remove` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:234 | ok | — |
| `Rubble.Intact.Strength` | BuildingType | Valueable<int> | -1 | src/Ext/BuildingType/Body.cpp:232 | ok | — |

## 钩子（4）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 441F12 | `BuildingClass_Destroy_RubbleYell` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017160.c | src/Ext/Building/Hooks.Trenches.cpp:98 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441F2C | `BuildingClass_Destroy_KickOutOfRubble` | kept | ✓ | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017130.c | src/Ext/Building/Hooks.Trenches.cpp:117 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 519FAF | `InfantryClass_UpdatePosition_EngineerRepairsFriendly` | kept ⚠改过 | ✓ | 00519630 `InfantryClass::InfantryClass__PerCellProcess` | decompiled/functions/100257A0.c | src/Ext/Infantry/Hooks.cpp:43 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/house_tracking.rs … |
| 51E635 | `InfantryClass_GetActionOnObject_EngineerOverFriendlyBuilding` | new_in_3.0 | ✓ | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10025420.c | src/Ext/Infantry/Hooks.cpp:23 | src/app/input/context_order.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10012970 | `BuildingExt::KickOutOfRubble` | 被钩子调用 | decompiled/functions/10012970.c |
| 1000F2B0 | `BuildingTypeExt::SwitchRubble` | 被钩子调用 | decompiled/functions/1000F2B0.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
