# `S.battery` Battery 超武（3.0 新增，供电/保持在线）

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
| `Battery.KeepOnline` | SWType | ValueableVector<BuildingTypeClass*> | (empty) | src/Misc/SWTypes/Battery.cpp:75 | ok | — |
| `Battery.Overpower` | SWType | ValueableVector<BuildingTypeClass*> | (empty) | src/Misc/SWTypes/Battery.cpp:76 | ok | — |
| `Battery.Power` | SWType | Valueable<int> | 0 | src/Misc/SWTypes/Battery.cpp:74 | ok | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1006EBD0 | SW_Battery::Activate [translated] | 使用键 | decompiled/functions/1006EBD0.c |
| 1006EC40 | SW_Battery::Deactivate [translated] | 使用键 | decompiled/functions/1006EC40.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
