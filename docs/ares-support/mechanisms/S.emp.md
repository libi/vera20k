# `S.emp` EMPulse 超武

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core,T.emp | partial | docs/ares-emp.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（5）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `EMPulse.Cannons` | SWType | ValueableVector<BuildingTypeClass*> | — | src/Misc/SWTypes/EMPulse.cpp:44 | no_use_found | — |
| `EMPulse.Linked` | SWType | Valueable<bool> | false | src/Misc/SWTypes/EMPulse.cpp:40 | needs_check | — |
| `EMPulse.PulseBall` | SWType | Nullable<AnimTypeClass*> | (empty) | src/Misc/SWTypes/EMPulse.cpp:43 | no_use_found | — |
| `EMPulse.PulseDelay` | SWType | Valueable<int> | 32 | src/Misc/SWTypes/EMPulse.cpp:42 | no_use_found | — |
| `EMPulse.TargetSelf` | SWType | Valueable<bool> | false | src/Misc/SWTypes/EMPulse.cpp:41 | ok | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001BC20 | FUN_1001bc20 [missing,inferred,3.0✓] | 使用键 | decompiled/functions/1001BC20.c |
| 100711C0 | SW_EMPulse::Activate [translated] | 使用键 | decompiled/functions/100711C0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1002E060 | `SideExt::ExtData::GetParaDropTypes` | 被钩子调用 | decompiled/functions/1002E060.c |
| 1002DFB0 | `SideExt::ExtData::GetParaDropNum` | 被钩子调用 | decompiled/functions/1002DFB0.c |
| 1006DA20 | `NewSWType::IsLaunchSiteEligible` | 被钩子调用 | decompiled/functions/1006DA20.c |
| 100191E0 | `BulletTypeExt::ExtData::CreateBullet` | 被钩子调用 | decompiled/functions/100191E0.c |
| 1000AC10 | `BulletClass::Detonate` | 被钩子调用 | decompiled/functions/1000AC10.c |
