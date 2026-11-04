# `S.hunterseeker` HunterSeeker 超武

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core,T.hunterseeker | done | docs/ares-hunter-seeker.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `HunterSeeker.Buildings` | SWType | ValueableVector<BuildingTypeClass*> | (empty) | src/Misc/SWTypes/HunterSeeker.cpp:50 | ok | src/rules/native_processing.rs |
| `HunterSeeker.RandomOnly` | SWType | Valueable<bool> | false | src/Misc/SWTypes/HunterSeeker.cpp:49 | needs_check | src/rules/native_processing.rs |
| `HunterSeeker.Type` | SWType | Valueable<UnitTypeClass*> | nullptr | src/Misc/SWTypes/HunterSeeker.cpp:48 | needs_check | src/rules/native_processing.rs |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1001BC20 | FUN_1001bc20 [missing,inferred,3.0✓] | 使用键 | decompiled/functions/1001BC20.c |
| 10042E40 | TechnoExt::ExtData::AcquireHunterSeekerTarget [translated] | 使用键 | decompiled/functions/10042E40.c |
| 10072850 | SW_HunterSeeker::Activate [translated] | 使用键 | decompiled/functions/10072850.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 1002E060 | `SideExt::ExtData::GetParaDropTypes` | 被钩子调用 | decompiled/functions/1002E060.c |
| 1002DFB0 | `SideExt::ExtData::GetParaDropNum` | 被钩子调用 | decompiled/functions/1002DFB0.c |
| 10032D50 | `SWTypeExt::ExtData::IsTechnoAffected` | 被钩子调用 | decompiled/functions/10032D50.c |
| 10032D30 | `SWTypeExt::ExtData::IsHouseAffected` | 被钩子调用 | decompiled/functions/10032D30.c |
| 1000AC70 | `Randomizer::RandomRanged` | 被钩子调用 | decompiled/functions/1000AC70.c |
| 10061B00 | `Debug::LogFlushed_Severity` | 被钩子调用 | decompiled/functions/10061B00.c |
