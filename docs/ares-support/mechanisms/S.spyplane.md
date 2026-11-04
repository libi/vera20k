# `S.spyplane` SpyPlane

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SpyPlane.Count` | SWType | Valueable<int> | 1 | src/Misc/SWTypes/SpyPlane.cpp:39 | ok | — |
| `SpyPlane.Mission` | SWType | Valueable<Mission> | Mission::SpyplaneApproach | src/Misc/SWTypes/SpyPlane.cpp:41 | ok | — |
| `SpyPlane.Type` | SWType | ValueableIdx<AircraftTypeClass> | 0 | src/Misc/SWTypes/SpyPlane.cpp:40 | needs_check | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DD40 | FUN_1002dd40 [missing,inferred,3.0✓] | 使用键 | decompiled/functions/1002DD40.c |
| 100325D0 | SWTypeExt::Deactivate [source_0.A,3.0✓] | 使用键 | decompiled/functions/100325D0.c |
| 10055920 | WeaponTypeExt::ModifyWaveColor [translated,inferred] | 使用键 | decompiled/functions/10055920.c |
| 10060540 | CSFLoader::FindDynamic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/10060540.c |
| 1006D120 | VectorClass<ChronoWarpStateMachine::ChronoWarpContainer>::GetItem [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1006D120.c |
| 10075780 | SW_SpyPlane::Activate [translated] | 使用键 | decompiled/functions/10075780.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10025FA0 | `TagClass::RaiseEvent` | 被钩子调用 | decompiled/functions/10025FA0.c |
| 10075900 | `HouseClass::SendSpyPlanes` | 被钩子调用 | decompiled/functions/10075900.c |
