# `S.nuke` Nuke

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（4）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Nuke.Payload` | SWType | Valueable<WeaponTypeClass*> | nullptr | src/Misc/SWTypes/Nuke.cpp:85 | ok | — |
| `Nuke.PsiWarning` | SWType | Valueable<AnimTypeClass*> | nullptr | src/Misc/SWTypes/Nuke.cpp:87 | ok | — |
| `Nuke.SiloLaunch` | SWType | Valueable<bool> | false | src/Misc/SWTypes/Nuke.cpp:88 | ok | — |
| `Nuke.TakeOff` | SWType | Nullable<AnimTypeClass*> | — | src/Misc/SWTypes/Nuke.cpp:86 | ok | — |

## 钩子（2）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。

| gamemd 地址 | 钩子 | 状态 | 3.0 核对 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|---|
| 44CB4C | `BuildingClass_Mi_Missile_NukeTakeOff` | new_in_3.0 | ✓ | 0044C980 `BuildingClass::BuildingClass__Mission_Missile` | decompiled/functions/10036660.c | src/Ext/SWType/Hooks.Legacy.cpp:737 | src/rules/object_type.rs, src/sim/superweapon/nuke.rs, src/sim/world/techno_ai/building_missile.rs |
| 46B371 | `BulletClass_NukeMaker` | kept | ✓ | 0046B310 `BulletClass::BulletClass__NukeMaker` | decompiled/functions/10036880.c | src/Ext/SWType/Hooks.Legacy.cpp:768 | src/sim/combat/world_receiver.rs, src/sim/projectile.rs, src/sim/superweapon/nuke.rs |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10001B60 | `YRMemory::AllocateChecked` | 被钩子调用 | decompiled/functions/10001B60.c |
| 1000AAE0 | `AnimClass::AnimClass` | 被钩子调用 | decompiled/functions/1000AAE0.c |
| 100326B0 | `SWTypeExt::ExtData::GetDamage` | 被钩子调用 | decompiled/functions/100326B0.c |
| 10032B40 | `SWTypeExt::ExtData::GetWarhead` | 被钩子调用 | decompiled/functions/10032B40.c |
| 10061AC0 | `Debug::LogFlushed` | 被钩子调用 | decompiled/functions/10061AC0.c |
