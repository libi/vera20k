# `S.lightning` LightningStorm

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M5 | S.core | none | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（18）

| 键 | 类 | 类型 | 默认 | 读取位置 | 使用点状态 | VERA20K 字面量 |
|---|---|---|---|---|---|---|
| `LightningRod.Modifier` | BuildingType | Valueable<double> | 1.0 | src/Ext/BuildingType/Body.cpp:225 | check_0A_use | — |
| `Lightning.BoltExplosion` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:80 | unscanned | — |
| `Lightning.Bolts` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:83 | unscanned | — |
| `Lightning.CloudHeight` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:79 | unscanned | — |
| `Lightning.Clouds` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:82 | unscanned | — |
| `Lightning.Debris` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:84 | unscanned | — |
| `Lightning.DebrisMax` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:78 | unscanned | — |
| `Lightning.DebrisMin` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:77 | unscanned | — |
| `Lightning.Duration` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:69 | unscanned | — |
| `Lightning.HitDelay` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:71 | unscanned | — |
| `Lightning.IgnoreLightningRod` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:76 | unscanned | — |
| `Lightning.PrintText` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:75 | unscanned | — |
| `Lightning.RadarOutage` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:70 | unscanned | — |
| `Lightning.RadarOutageAffects` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:81 | unscanned | — |
| `Lightning.ScatterCount` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:73 | unscanned | — |
| `Lightning.ScatterDelay` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:72 | unscanned | — |
| `Lightning.Separation` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:74 | unscanned | — |
| `Lightning.Sounds` | SWType | — | — | src/Misc/SWTypes/LightningStorm.cpp:85 | unscanned | — |

## 钩子（0）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 1002DFB0 | FUN_1002dfb0 [source_0.A,inferred] | 使用键 | decompiled/functions/1002DFB0.c |
