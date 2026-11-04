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
| `TogglePower` | Rules | Valueable<int>::Read | — | src/Ext/Rules/Body.cpp:236 | ok | src/rules/ini_value.rs, src/rules/object_type.rs |
| `TogglePowerAllowed` | Rules | Valueable<bool> | false | src/Ext/Rules/Body.cpp:234 | ok | — |
| `TogglePowerDelay` | Rules | Valueable<int> | 45 | src/Ext/Rules/Body.cpp:235 | ok | — |

## 钩子（0）

"源码"列的函数体已按 3.0 翻译或对照 3.0 核对（"3.0 核对"列为 ✓）；有疑问时以 3.0 反编译为准。`⚠改过` 表示 3.0 相对 0.A 改过这个钩子，源码中已是 3.0 写法。


## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 100025C0 | TogglePowerCommandClass::Execute [source_0.A,3.0✓] | 使用键 | decompiled/functions/100025C0.c |
| 1001F320 | HouseExt::ExtMap.LoadStatic [source_0.A,inferred,3.0✓] | 使用键 | decompiled/functions/1001F320.c |
| 100228A0 | HouseExt::ExtData::UpdateTogglePower [source_0.A,3.0✓] | 使用键 | decompiled/functions/100228A0.c |
| 10070230 | ChronoWarpStateMachine::Update [translated,inferred] | 使用键 | decompiled/functions/10070230.c |
| 10002920 | `MapClass::SetTogglePowerMode` | 被钩子调用 | decompiled/functions/10002920.c |
| 10020AD0 | `HouseClass::ControlledByHuman` | 被钩子调用 | decompiled/functions/10020AD0.c |
| 100219A0 | `BuildingClass::GoOnline` | 被钩子调用 | decompiled/functions/100219A0.c |
| 10021980 | `BuildingClass::GoOffline` | 被钩子调用 | decompiled/functions/10021980.c |
| 1006FFD0 | `BuildingClass::EnableTemporal` | 被钩子调用 | decompiled/functions/1006FFD0.c |
| 10057C70 | `ContainerMapBase::find` | 被钩子调用 | decompiled/functions/10057C70.c |
| 100033C0 | `CRT::_delete` | 被钩子调用 | decompiled/functions/100033C0.c |
