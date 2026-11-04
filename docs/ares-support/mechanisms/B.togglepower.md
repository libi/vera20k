# `B.togglepower` 手动开关电

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M3 | — | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（3）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `TogglePower` | Rules | Valueable<int>::Read | — | src/Ext/Rules/Body.cpp:281 | check_0A_use | src/rules/ini_value.rs, src/rules/object_type.rs |
| `TogglePowerAllowed` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:279 | check_0A_use | — |
| `TogglePowerDelay` | Rules | Valueable<int> | 45 | src/Ext/Rules/Body.cpp:280 | check_0A_use | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1000B240 | BuildingTypeExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B240.c |
| 1000B780 | BuildingTypeExt::ExtMap.SaveStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1000B780.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred] | 使用键 | decompiled/functions/1001F320.c |
| 100228A0 | HouseExt::ExtData::UpdateTogglePower [source_0.A] | 使用键 | decompiled/functions/100228A0.c |
| 10070230 | FUN_10070230 [source_0.A,inferred] | 使用键 | decompiled/functions/10070230.c |
| 100219A0 | `BuildingClass::GoOnline` | 被钩子调用 | decompiled/functions/100219A0.c |
| 10021980 | `BuildingClass::GoOffline` | 被钩子调用 | decompiled/functions/10021980.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
