# `B.rubble` 建筑废墟

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
示例备注（2026-11-04 核对 3.0 反编译所得）。

**表现**：设了 `Rubble.Destroyed` 的建筑被摧毁时，原地换成废墟建筑；在废墟上设 `Rubble.Intact`，友方工程师进入后恢复成原建筑。`*.Remove=yes` 只移除、不生成新建筑。

**3.0 与 0.A 的差异**（以 3.0 为准）：
- `RubbleYell`（1000F2B0）在 3.0 变成 `(建筑, remove, 新类型, 归属, 血量, 动画)` 的内联版本，返回新建筑指针或空。不再记录"类型缺失"日志，**也不检查 `Put` 是否失败**：放置失败时仍返回新建筑。
- 工程师钩子（519FAF → 100257A0）：调用 `RubbleYell(Intact…)` 后对旧建筑调用 `UnInit`。
  - 新建筑为空（只 Remove）时，跳到 0x519FB9，也就是原版"满血不修"分支，工程师不被消耗。
  - 否则先 `Limbo` 工程师，再由**新建筑** `ExitObject` 把它放出来，原来选中的重新选中，然后跳到 0x51A65D（函数尾），工程师不被消耗。
  - 0.A 放置失败时会把旧建筑放回原处并播放 `EVA_CannotDeployHere`，3.0 已删除这段。
  - 钩子后半段是 `EngineerRepairable` 的判断，属于 `B.engineer`。
- 光标（51E635 → 10025420，3.0 新增）：工程师指向同盟方的、带 `Rubble.Intact`/`.Remove` 的建筑时，动作为 GRepair（0x1D），光标为 0x5E。

**摧毁路径的时机**（`BuildingClass::DestructionEffects` 004415F0）：依次为摧毁动画和粒子 → `Health=0` → 若由攻击导致则置 `NoCrew` → **441F12 生成废墟** → `SpawnSurvivors` → `KillPassengers` → **441F2C `KickOutOfRubble`**。
- 生成废墟：旧建筑 `Limbo`，销毁全部建筑动画（slot -2）。新建筑由 `CreateObject(归属)` 生成，在旧建筑的 `Location` 处 `Put`。朝向取当前朝向（包括正在转动的中间值），换算成 8 位并四舍五入。`Put` 包在 `MapEditorMode++` 里，跳过放置检查。动画在旧建筑坐标处生成（AnimClass 构造参数：循环 1，flags 0x600）。
- `KickOutOfRubble`：遍历旧建筑地基的每个格子，把其中 `Limbo` 成功的 Foot 收集起来，再由旧建筑 `ExitObject` 逐个放出。返回 2（成功）且原来选中的重新选中，其他结果直接 `UnInit`。

**血量**：-100..-1 表示新类型 `Strength` 的百分比，至少为 1；大于 0 表示绝对值，不超过 `Strength`；0 表示满血。默认值：Destroyed 为 0（满血），Intact 为 -1（1%）。3.0 反编译中归属和血量的参数编号被 Ghidra 混淆了，以 0.A 语义为准，落地前看指令确认。

**归属**：`HouseExt::GetHouseKind(kind, allowRandom=true, 默认=旧建筑归属)`，没有传 invoker/killer/victim，所以这三项都回退为旧建筑归属。`Random` 会从 Scenario RNG 取一次随机数（影响确定性，测试要覆盖）。

**读取**：BuildingType `LoadFromINI` 1000DD40，10 个键按表中顺序读。3.0 不再修改废墟类型的 `Capturable`/`TogglePower`/`Unsellable`/`CanBeOccupied`（0.A 会改）。

**VERA20K 线索**（未逐一确认）：
- 摧毁流程：`src/sim/combat/destruction_effects.rs`；幸存者：`src/sim/crew_survival.rs`（已引用 0x441F12）；
- 工程师进入：`src/sim/movement/per_cell.rs`（0x519630）；光标：`src/app/input/context_order.rs`、`cursor.rs`（0x51E3B0）；
- 放出单位：`src/sim/production/production_spawn.rs`（0x443C60）；归属解析：`Simulation::house_kind`（`src/sim/world/house_defeat.rs`）；归属读取器：`HouseSelector`（`src/rules/ares_warhead.rs`）。
<!-- /notes -->

## 配置键（10）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Rubble.Destroyed` | BuildingType | Valueable<BuildingTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:229 | check_0A_use | — |
| `Rubble.Destroyed.Anim` | BuildingType | Valueable<AnimTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:231 | check_0A_use | — |
| `Rubble.Destroyed.Owner` | BuildingType | Valueable<OwnerHouseKind> | OwnerHouseKind::Default | src/Ext/BuildingType/Body.cpp:233 | check_0A_use | — |
| `Rubble.Destroyed.Remove` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:237 | check_0A_use | — |
| `Rubble.Destroyed.Strength` | BuildingType | Valueable<int> | 0 | src/Ext/BuildingType/Body.cpp:235 | check_0A_use | — |
| `Rubble.Intact` | BuildingType | Valueable<BuildingTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:230 | check_0A_use | — |
| `Rubble.Intact.Anim` | BuildingType | Valueable<AnimTypeClass*> | nullptr | src/Ext/BuildingType/Body.cpp:232 | check_0A_use | — |
| `Rubble.Intact.Owner` | BuildingType | Valueable<OwnerHouseKind> | OwnerHouseKind::Default | src/Ext/BuildingType/Body.cpp:234 | check_0A_use | — |
| `Rubble.Intact.Remove` | BuildingType | Valueable<bool> | false | src/Ext/BuildingType/Body.cpp:238 | check_0A_use | — |
| `Rubble.Intact.Strength` | BuildingType | Valueable<int> | -1 | src/Ext/BuildingType/Body.cpp:236 | check_0A_use | — |

## 钩子（4）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 441F12 | `BuildingClass_Destroy_RubbleYell` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017160.c | src/Ext/Building/Hooks.Trenches.cpp:86 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 441F2C | `BuildingClass_Destroy_KickOutOfRubble` | kept | 004415F0 `BuildingClass::BuildingClass__DestructionEffects` | decompiled/functions/10017130.c | src/Ext/Building/Hooks.Trenches.cpp:103 | src/rules/effect_asset_catalog.rs, src/sim/combat/destruction_effects.rs, src/sim/combat/world_receiver.rs … |
| 519FAF | `InfantryClass_UpdatePosition_EngineerRepairsFriendly` | kept ⚠改过 | 00519630 `InfantryClass::InfantryClass__PerCellProcess` | decompiled/functions/100257A0.c | src/Ext/Infantry/Hooks.cpp:16 | src/audio/events.rs, src/rules/ruleset.rs, src/sim/house_tracking.rs … |
| 51E635 | `InfantryClass_GetActionOnObject_EngineerOverFriendlyBuilding` | new_in_3.0 | 0051E3B0 `InfantryClass::InfantryClass__What_Action_OnObject` | decompiled/functions/10025420.c | src/Recovered/InfantryClass.cpp:111 | src/app/input/context_order.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000D380 | BuildingTypeExt::ExtData::CompleteInitialization [source_0.A] | 使用键 | decompiled/functions/1000D380.c |
| 10012970 | `BuildingExt::ExtData::KickOutOfRubble` | 被钩子调用 | decompiled/functions/10012970.c |
| 1000F2B0 | `BuildingExt::ExtData::RubbleYell` | 被钩子调用 | decompiled/functions/1000F2B0.c |
| 10057E20 | `Actions::SetCursor` | 被钩子调用 | decompiled/functions/10057E20.c |
