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
| `EnemyInsignia` | Rules | Valueable<bool> | true | src/Ext/Rules/Body.cpp:185 | check_0A_use | src/rules/insignia_type.rs |
| `VeteranFlashTimer` | Rules | Valueable<int> | 0 | src/Ext/Rules/Body.cpp:220 | ok | — |
| `EliteAbilities` | TechnoType | FUN_100405a0 | (empty) | src/Ext/TechnoType/Body.cpp:278 | ok | src/rules/ares_abilities.rs, src/rules/object_type.rs, src/rules/phobos_crit.rs |
| `EVA.ElitePromoted` | TechnoType | ValueableIdx<VoxClass> | -1 | src/Ext/TechnoType/Body.cpp:509 | check_0A_use | — |
| `EVA.VeteranPromoted` | TechnoType | ValueableIdx<VoxClass> | -1 | src/Ext/TechnoType/Body.cpp:508 | check_0A_use | — |
| `Experience.AirstrikeModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:312 | check_0A_use | — |
| `Experience.FromAirstrike` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:311 | check_0A_use | — |
| `Experience.FromPassengers` | TechnoType | Valueable<bool> | true | src/Ext/TechnoType/Body.cpp:305 | check_0A_use | — |
| `Experience.MindControlSelfModifier` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:307 | unscanned | — |
| `Experience.MindControlVictimModifier` | TechnoType | — | — | src/Ext/TechnoType/Body.cpp:308 | unscanned | — |
| `Experience.PassengerModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:306 | check_0A_use | — |
| `Experience.PromotePassengers` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:304 | check_0A_use | — |
| `Experience.SpawnModifier` | TechnoType | Valueable<double> | 1.0 | src/Ext/TechnoType/Body.cpp:310 | check_0A_use | — |
| `Experience.SpawnOwnerModifier` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:309 | check_0A_use | — |
| `Insignia.%s` | TechnoType | Promotable<SHPStruct *> | nullptr | src/Ext/TechnoType/Body.cpp:227 | no_use_found | — |
| `Insignia.ShowEnemy` | TechnoType | Nullable<bool> | (empty) | src/Ext/TechnoType/Body.cpp:313 | check_0A_use | src/rules/insignia_type.rs |
| `InsigniaFrame.%s` | TechnoType | Promotable<int>::Read | — | src/Ext/TechnoType/Body.cpp:228 | no_use_found | — |
| `Promote.EliteExperience` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:513 | ok | — |
| `Promote.EliteFlash` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:507 | ok | — |
| `Promote.EliteSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:505 | no_use_found | src/rules/object_type.rs |
| `Promote.EliteType` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:511 | ok | — |
| `Promote.IncludePassengers` | TechnoType | Valueable<bool> | false | src/Ext/TechnoType/Body.cpp:503 | ok | — |
| `Promote.VeteranExperience` | TechnoType | Valueable<double> | 0.0 | src/Ext/TechnoType/Body.cpp:512 | ok | — |
| `Promote.VeteranFlash` | TechnoType | Nullable<int> | (empty) | src/Ext/TechnoType/Body.cpp:506 | ok | — |
| `Promote.VeteranSound` | TechnoType | NullableIdx<VocClass> | (empty) | src/Ext/TechnoType/Body.cpp:504 | no_use_found | src/rules/object_type.rs |
| `Promote.VeteranType` | TechnoType | Valueable<TechnoTypeClass*> | nullptr | src/Ext/TechnoType/Body.cpp:510 | ok | — |
| `VeteranAbilities` | TechnoType | FUN_100405a0 | (empty) | src/Ext/TechnoType/Body.cpp:277 | ok | src/rules/ares_abilities.rs, src/rules/ares_emp.rs, src/rules/object_type.rs … |

## 钩子（2）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 702E9D | `TechnoClass_RegisterDestruction_Veterancy` | kept | 00702D40 `TechnoClass::TechnoClass__RecordKill` | decompiled/functions/1004EEF0.c | src/Ext/Techno/Hooks.Veterancy.cpp:12 | src/rules/object_type.rs, src/sim/combat/record_kill.rs, src/sim/combat/veterancy.rs … |
| 70A990 | `TechnoClass_DrawVeterancy` | kept | 0070A990 `DrawVeterancyPips` | decompiled/functions/1004E0C0.c | src/Ext/Techno/Hooks.Insignia.cpp:7 | src/app/presentation/render/object_extras.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002AB60 | RulesExt::ExtData::LoadBeforeTypeData [loader] | 使用键 | decompiled/functions/1002AB60.c |
| 1003DEC0 | TechnoTypeExt::ExtMap.LoadFromINI [source_0.A,inferred] | 使用键 | decompiled/functions/1003DEC0.c |
| 10046AF0 | TechnoExt::UpdateVeterancy [translated] | 使用键 | decompiled/functions/10046AF0.c |
| 10062D90 | FUN_10062d90 [translated] | 使用键 | decompiled/functions/10062D90.c |
| 1002B510 | `WeaponTypeClass::FindOrAllocate` | 被钩子调用 | decompiled/functions/1002B510.c |
| 1000ACD0 | `CRT::strcmp` | 被钩子调用 | decompiled/functions/1000ACD0.c |
| 1000D3E0 | `AnimTypeClass::Find` | 被钩子调用 | decompiled/functions/1000D3E0.c |
| 100095D0 | `VocClass::PlayAt` | 被钩子调用 | decompiled/functions/100095D0.c |
| 10013CB0 | `VoxClass::PlayIndex` | 被钩子调用 | decompiled/functions/10013CB0.c |
