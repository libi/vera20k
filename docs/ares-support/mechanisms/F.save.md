# `F.save` 扩展状态存档（无键）

> 自动生成（`tools/ares_support/build_index.py`），只有备注区可以手改。路径相对 ares-src 工作区。

| 里程碑 | 依赖 | VERA20K 现状 | 已有文档 |
|---|---|---|---|
| M1 | — | partial | — |

## 备注
<!-- notes -->
（尚无人工备注。实现时把 3.0 与 0.A 的差异、确认过的行为和未决问题记在这里。）
<!-- /notes -->

## 配置键（0）

无键（基础设施或无条件行为）。

## 钩子（11）

先读 3.0 反编译（`decompiled`），再读还原源码（`src`）；状态为 `kept` 的是 0.A 源码，函数体可能已被 3.0 修改。

| gamemd 地址 | 钩子 | 状态 | 所在 gamemd 函数（上下文） | 3.0 反编译 | 源码 | VERA20K 已引用该函数 |
|---|---|---|---|---|---|---|
| 483BF1 | `CellClass_Load_Crates` | kept | 004839F0 `CellClass__Load` | decompiled/functions/1006ADE0.c | src/Misc/Savegame.cpp:90 | src/sim/snapshot.rs, src/sim/world/substrate.rs |
| 559F31 | `LoadOptionsClass_GetFileInfo` | kept ⚠改过 | 00559ED0 `LoadOptionsClass__GetFileEntry` | decompiled/functions/1006AE70.c | src/Misc/Savegame.cpp:66 | src/app/persistence/mod.rs, src/sim/snapshot.rs |
| 67CEFE | `Game_Save_FixLog` | kept | 0067CEF0 `Save_Game_To_File` | decompiled/functions/1006ADF0.c | src/Misc/Savegame.cpp:79 | src/sim/snapshot.rs |
| 67D04E | `Game_Save_SavegameInformation` | kept | 0067CEF0 `Save_Game_To_File` | decompiled/functions/1006AE10.c | src/Misc/Savegame.cpp:54 | src/sim/snapshot.rs |
| 67D300 | `SaveGame_Start` | kept | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1006AEF0.c | src/Misc/Savegame.cpp:8 | — |
| 67E42E | `SaveGame` | kept | 0067D300 `Save_Game_Content_To_Stream` | decompiled/functions/1006AED0.c | src/Misc/Savegame.cpp:8 | — |
| 67E730 | `LoadGame_Start` | kept | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1006AE60.c | src/Misc/Savegame.cpp:39 | — |
| 67F7C8 | `LoadGame_End` | kept | 0067E730 `Load_Game_Content_From_Stream` | decompiled/functions/1006AE50.c | src/Misc/Savegame.cpp:45 | — |
| 6CF240 | `SwizzleManagerClass_Swizzle` | kept | 006CF240 `SwizzleManagerClass__Swizzle` | decompiled/functions/1006C5B0.c | src/Misc/Swizzle.cpp:73 | — |
| 6CF2C0 | `SwizzleManagerClass_Here_I_Am` | kept | 006CF2C0 `SwizzleManagerClass__Here_I_Am` | decompiled/functions/1006C540.c | src/Misc/Swizzle.cpp:62 | — |
| 6CF350 | `SwizzleManagerClass_ConvertNodes` | kept | 006CF350 `SwizzleManagerClass__ConvertNodes` | decompiled/functions/1006C440.c | src/Misc/Swizzle.cpp:54 | — |

## 非钩子函数

| Ares 地址 | 函数 | 来源 | 3.0 反编译 |
|---|---|---|---|
| 10061AC0 | `Debug::Log` | 被钩子调用 | decompiled/functions/10061AC0.c |
| 10005080 | `Ares::LoadGameData` | 被钩子调用 | decompiled/functions/10005080.c |
| 10003780 | `Ares::LoadGame` | 被钩子调用 | decompiled/functions/10003780.c |
| 100053F0 | `Ares::SaveGameData` | 被钩子调用 | decompiled/functions/100053F0.c |
| 100038A0 | `Ares::SaveGame` | 被钩子调用 | decompiled/functions/100038A0.c |

## 涉及的热点函数（2）

与其他机制共用，改动前先看 README 第 7 节；`hotspots.tsv` 有完整钩子列表。

| gamemd 函数 | 钩子数 | 共用机制 |
|---|---|---|
| 0067D300 `Save_Game_Content_To_Stream` | 4 | F.save, H.sidepresent |
| 0067E730 `Load_Game_Content_From_Stream` | 4 | F.save, H.sidepresent |
