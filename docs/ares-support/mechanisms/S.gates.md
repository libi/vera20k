# `S.gates` 超武建筑门槛与 House 限制

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M2 | S.core | done | docs/ares-superweapon-building-gates.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（6）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `SW.AllowAI` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:128 | no_use_found | src/rules/superweapon_type.rs |
| `SW.AllowPlayer` | SWType | Valueable<bool> | true | src/Ext/SWType/Body.cpp:127 | check_0A_use | src/rules/superweapon_type.rs |
| `SW.AuxBuildings` | SWType | — | — | src/Ext/SWType/Body.cpp:224 | unscanned | src/rules/native_processing.rs |
| `SW.ForbiddenHouses` | SWType | — | — | src/Ext/SWType/Body.cpp:223 | unscanned | src/rules/superweapon_type.rs |
| `SW.NegBuildings` | SWType | — | — | src/Ext/SWType/Body.cpp:225 | unscanned | src/rules/native_processing.rs |
| `SW.RequiredHouses` | SWType | — | — | src/Ext/SWType/Body.cpp:222 | unscanned | src/rules/superweapon_type.rs |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100329E0 | BuildingExt::ExtData::GetSuperWeaponIndex [source_0.A] | 使用键 | decompiled/functions/100329E0.c |
| 10021480 | `HouseExt::ExtData::GetSuperWeaponStatus` | 被钩子调用 | decompiled/functions/10021480.c |
