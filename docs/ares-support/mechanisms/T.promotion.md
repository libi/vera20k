# `T.promotion` 晋升经验、晋升类型、闪光、能力、徽章

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（27）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `EnemyInsignia` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:140 | ok | src/rules/insignia_type.rs |
| `VeteranFlashTimer` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:175 | ok | — |
| `EliteAbilities` | TechnoType | FUN_100405a0 | (empty) | src/Ext/TechnoType/Body.cpp:282 | ok | src/rules/ares_abilities.rs, src/rules/object_type.rs, src/rules/phobos_crit.rs |
| `EVA.ElitePromoted` | TechnoType | ValueableIdx<VoxClass> | -1 | src/Ext/TechnoType/Body.cpp:513 | ok | — |
| `EVA.VeteranPromoted` | TechnoType | ValueableIdx<VoxClass> | -1 | src/Ext/TechnoType/Body.cpp:512 | ok | — |
| `Experience.AirstrikeModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:316 | ok | — |
| `Experience.FromAirstrike` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:315 | ok | — |
| `Experience.FromPassengers` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:309 | ok | — |
| `Experience.MindControlSelfModifier` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:311 | unscanned | — |
| `Experience.MindControlVictimModifier` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:312 | unscanned | — |
| `Experience.PassengerModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:310 | ok | — |
| `Experience.PromotePassengers` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:308 | ok | — |
| `Experience.SpawnModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:314 | ok | — |
| `Experience.SpawnOwnerModifier` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:313 | ok | — |
| `Insignia.%s` | TechnoType | Promotable<SHPStruct *> | nullptr | src/Ext/TechnoType/Body.cpp:231 | ok | — |
| `Insignia.ShowEnemy` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:317 | ok | src/rules/insignia_type.rs |
| `InsigniaFrame.%s` | TechnoType | Promotable<int>::Read | — | src/Ext/TechnoType/Body.cpp:232 | ok | — |
| `Promote.EliteExperience` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:517 | ok | — |
| `Promote.EliteFlash` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:511 | ok | — |
| `Promote.EliteSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:509 | ok | src/rules/object_type.rs |
| `Promote.EliteType` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:515 | ok | — |
| `Promote.IncludePassengers` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:507 | ok | — |
| `Promote.VeteranExperience` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:516 | ok | — |
| `Promote.VeteranFlash` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:510 | ok | — |
| `Promote.VeteranSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:508 | ok | src/rules/object_type.rs |
| `Promote.VeteranType` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:514 | ok | — |
| `VeteranAbilities` | TechnoType | FUN_100405a0 | (empty) | src/Ext/TechnoType/Body.cpp:281 | ok | src/rules/ares_abilities.rs, src/rules/ares_emp.rs, src/rules/object_type.rs … |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 702E9D | `TechnoClass_RegisterDestruction_Veterancy` | kept | ✓ | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/1004EEF0.c | src/Ext/Techno/Hooks.Veterancy.cpp:18 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |
| 70A990 | `TechnoClass_DrawVeterancy` | kept | ✓ | 0070A990 `DrawVeterancyPips` | decompiled/functions/1004E0C0.c | src/Ext/Techno/Hooks.Insignia.cpp:7 | src/app/presentation/render/object_extras.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10046AF0 | TechnoExt::UpdateVeterancy [translated] | 使用键 | decompiled/functions/10046AF0.c |
| 10062D90 | FUN_10062d90 [translated] | 使用键 | decompiled/functions/10062D90.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
| 1003DC50 | `TechnoTypeExt::ExtData::GetWeaponSlots` | 被钩子调用 | decompiled/functions/1003DC50.c |
