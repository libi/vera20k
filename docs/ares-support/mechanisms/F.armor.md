# `F.armor` [ArmorTypes] 自定义护甲与 Verses

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | F.readers | done | docs/ares-custom-armor.md |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（1）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `Verses` | WarheadType | — | — | src/Ext/WarheadType/Body.cpp:65 | unscanned | src/rules/ares_armor.rs, src/rules/warhead_type.rs, src/sim/game_entity/emp.rs |

## 钩子（1）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 75DDCC | `Verses_OrigParser` | kept | 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | decompiled/functions/10054410.c | src/Ext/WarheadType/Hooks.Verses.cpp:33 | src/rules/warhead_type.rs |

## 涉及的热点函数（1）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0075D3A0 `WarheadTypeClass::WarheadTypeClass__ReadINI` | 5 | F.armor, U.bugfixes, ext:WarheadType |
