# `S.unitdelivery` UnitDelivery 超武

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core | done | docs/ares-unit-delivery.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Deliver.BaseNormal` | SWType | Valueable<bool> | true | src/Misc/SWTypes/UnitDelivery.cpp:40 | ok | src/rules/native_processing.rs |
| `Deliver.Owner` | SWType | Valueable<OwnerHouseKind> | — | src/Misc/SWTypes/UnitDelivery.cpp:41 | ok | src/rules/native_processing.rs |
| `Deliver.Types` | SWType | ValueableVector<TechnoTypeClass *> | — | src/Misc/SWTypes/UnitDelivery.cpp:39 | ok | src/rules/native_processing.rs |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10075A60 | FUN_10075a60 [translated] | 使用键 | decompiled/functions/10075A60.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 10021310 | `HouseExt::GetHouseKind` | 被钩子调用 | decompiled/functions/10021310.c |
| 1000D6B0 | `BuildingTypeClass::GetFoundationWidth` | 被钩子调用 | decompiled/functions/1000D6B0.c |
| 1000D690 | `BuildingTypeClass::GetFoundationHeight` | 被钩子调用 | decompiled/functions/1000D690.c |
| 10009590 | `MapClass::Pathfinding_Find` | 被钩子调用 | decompiled/functions/10009590.c |
| 100454F0 | `TechnoExt::ExtData::IsPowered` | 被钩子调用 | decompiled/functions/100454F0.c |
| 10045440 | `TechnoExt::ExtData::IsOperated` | 被钩子调用 | decompiled/functions/10045440.c |
| 10048690 | `TechnoClass::Deactivate` | 被钩子调用 | decompiled/functions/10048690.c |
