# Ares 3.0 INI 支持：开发文档

本文档规划 VERA20K 在 `--features ares` 下补全 Ares 3.0 INI 支持的工作：拆成哪些机制、按什么顺序做、
每一步怎样做才不影响已有的 Ares 功能，以及怎样验收和跟踪进度。

同目录的数据文件：

| 文件 | 内容 |
|---|---|
| `catalog.tsv` | 逐键目录：每个 Ares 键的所属机制、里程碑、类、节、类型、构造函数默认值、读取函数与源码位置、使用函数、VERA20K 线索与状态 |
| `mechanisms.tsv` | 机制表：ID、里程碑、依赖、VERA20K 现状、键数、涉及的热点函数数 |
| `hotspots.tsv` | 热点函数表：被多个机制同时钩住，或钩子数达到 5 个的 gamemd 函数，列出每个钩子和所属机制 |
| `mechanisms/<id>.md` | 每个机制一页阅读索引：键、要读的 Ares 钩子和函数（3.0 反编译与源码位置）、钩子所在的 gamemd 函数、共用的热点函数、VERA20K 中已引用这些原生函数的文件，以及人工备注区 |

表格由 `tools/ares_support/build_catalog.py` 生成，索引页由 `build_index.py` 生成，机制划分规则在 `tools/ares_support/mechanisms.py`。

本文档不重复写每个机制的逻辑：实现者（人或 AI）照索引页直接读 ares-src 工作区的代码，方法见第 6 节。
数据基于 ares-src 的 `recover/inj-layout` 分支（`d310643a`，2026-11-04）：该版本的函数体已全部按 3.0 翻译或核对过，
并能编译链接。`main` 分支（`10c1c922`）是中央合并后的旧版，保留钩子仍是 0.A 写法，不要用它。
本文档中的地址、函数名和源码路径都只是线索，落地时按 [项目契约](../../AGENTS.md) 对照原生指令确认。

## 1. 目标与范围

**目标**：Ares 3.0 读取的每个 INI 键，在 `ares` 构建中都有对应的 Rust 读取和使用。读取要符合原生实现：
文件与层、节、精确大小写的键、默认值、解析器、上下限、读取顺序、读后处理。使用指原生使用点上的行为。
做不到的部分记录为残差，不能静默省略。

**范围内**：

- 650 个 Ares ExtData 键（16 个加载函数读的 577 个，加 18 个超武类型读的 73 个），以及从还原源码扫描到的
  另外 122 个键（`[RadiationTypes]`、`[TunnelTypes]`、Side 展示、界面与战役设置等），共 772 个；
- 这些键在原生代码中的全部使用点；
- 这些键依赖的、没有键的配套钩子，例如读取器、存档、热点函数里的跳转。

**范围外（需要单独决定）**：M7 中的无条件 Bugfix 钩子、界面外壳、新触发动作。它们改变的是 retail 行为本身，
是否在 `ares` 构建中启用、启用到什么程度，由维护者逐项决定。

**不做**：加载 `Ares.dll`；导入或照抄 Ares 源码，包括还原源码；通用 mod 支持。

## 2. 现状

| | 数量 |
|---|---|
| 键（ExtData / 源码扫描） | 772（650 / 122） |
| 机制 | 111 |
| 已有 VERA20K 实现的机制（done / partial，来自 `docs/ares.md`，M0 复核） | 23 / 24 |
| VERA20K 非测试源码中出现字面量的键 | 248 |
| Ares 3.0 钩子 / 被钩的 gamemd 函数 | 1448 / 788 |
| 热点函数（被多个机制钩住） | 129 |

Ares 侧数据的已知缺口（来自 Ares 3.0 还原工作区，下称 `ares-src`；`catalog.tsv` 的 `use_status` 列）：

- `ok`（529 个）：所有使用函数都已按 3.0 翻译或核对过（`symbols/verified_30.tsv`）。
  ares-src 自己的 `report/ini_coverage.tsv` 生成得比最后几轮核对早，其中 `check_0A_use` 标签已过时，这里按核对表重新计算；
- `needs_check`（9 个）：有使用函数尚无源码，多为偏移推断的误判；
- `no_use_found`（112 个）：扫描不到使用点；
- `unscanned`（122 个）：源码扫描补上的键，还没有做使用点扫描；
- 使用点中标 `inferred` 的是按偏移推断的，有误判。

因此 `catalog.tsv` 是**任务清单**，不是规格。每个机制开工时都要先核实自己那部分数据（见第 6 节第 1 步）。

## 3. 原则：只增不改

1. **默认构建零变化。** 所有新状态、读取器和分支都放在 `#[cfg(feature = "ares")]` 之后，INI 不能开启没编译进来的功能。
2. **不配置等于没变化。** 在 `ares` 构建中，键没有设置时的行为必须和当前一致，每个机制都要有"未设置"的测试用例。
   如果 Ares 3.0 的默认值本身就会改变行为（无条件修复，或默认值与原版不同），要单独立项，PR 中写明。
3. **只扩展现有负责者。** 一个原生函数只有一份 Rust 实现。需要的逻辑已有负责者时，扩展那个负责者，
   不复制、不包装、不另起一份。缺读取器就先补读取器。
4. **热点函数先重构再接入。** 在同一个 PR 里分成两个提交：先是纯重构的扩展点（行为零变化），再接入机制。
5. **一条链一个 PR。** 按项目契约，一条玩家可见的玩法链从证据、实现到集成在一个 PR 里完成，不拆成多个 PR，
   也不叠未合并的分支。机制太大时，按契约以一种常见单位、武器或界面路径为界分成几条链，依次做。
6. **三种构建都要通过**：默认、`ares`、`ares,phobos,npatch`（见 [跨扩展兼容](../npatch.md#cross-extension-compatibility)）。
   Phobos 和 Ares 同名的键（如 `AttachEffect.*`）、同一原生位置被两个扩展改动的情况，按那里的优先级规则处理。

## 4. 三层结构

```
第 1 层  公共基础      读取器语义、扩展数据存放、存档、[ArmorTypes] 等新列表节
           │
第 2 层  热点流程      开火、伤害、范围伤害、建筑易主、建造完成、单位更新、生产判断……
           │           每个原生函数一个负责者，按 Ares 钩子的地址顺序留扩展点
           │
第 3 层  机制          挂在扩展点上，各自读自己的键、存自己的状态
```

- 第 1 层只做一次，所有机制共用。
- 第 2 层由 `hotspots.tsv` 给出。`mechanisms` 列为 1 的末端函数可以随所属机制一起改。
  列中有两个以上机制的函数，由**最先需要它的里程碑**负责加扩展点，之后的机制只接入，不再改结构。
- 第 3 层是主要工作量，大部分机制之间互相独立，可以并行开发。

## 5. 里程碑

每个里程碑列出其中的机制、键数、依赖和 VERA20K 现状。现状是 `docs/ares.md` 的初步判断，M0 复核后以
`catalog.tsv` 的 `vera_status` 为准。

### M0 基线与盘点（不改游戏代码）

1. 记录三种构建的测试基线。
2. 对 `vera_hint` 为 done/partial 的机制，逐键核对 VERA20K 的读取和使用，填写 `catalog.tsv` 的 `vera_status`：
   - `none`：未实现；
   - `read`：只有读取，没有使用；
   - `partial`：部分使用点已实现；
   - `done`：全部实现；
   - `residual`：已知未做，写明原因。
3. 对已实现的机制，标出它们已经占用的热点函数，以及在 VERA20K 里对应的负责模块。
4. 纠正 `mechanisms.py` 中划错的键，并重新生成目录。

**完成标准**：所有 done/partial 机制的键都有 `vera_status`；已占用的热点函数有清单。

### M1 公共基础

<!-- table:M1 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`F.readers`](mechanisms/F.readers.md) | Ares 读取器语义（Valueable/Nullable/Promotable/列表/none/长度上限） | 0 | — | partial |
| [`F.directives`](mechanisms/F.directives.md) | #include 与 [节]:[父节] 继承 | 0 | F.readers | done |
| [`F.csf`](mechanisms/F.csf.md) | 补充 CSF 字符串表 | 0 | — | done |
| [`F.armor`](mechanisms/F.armor.md) | [ArmorTypes] 自定义护甲与 Verses | 1 | F.readers | done |
| [`F.radtypes`](mechanisms/F.radtypes.md) | [RadiationTypes] 辐射类型 | 9 | F.readers | none |
| [`F.prereq`](mechanisms/F.prereq.md) | [GenericPrerequisites] 与 Prerequisite.Lists/Theaters/StolenTechs | 3 | F.readers | partial |
| [`F.globalcontrols`](mechanisms/F.globalcontrols.md) | [GlobalControls] 与全局开关 | 4 | F.readers | none |
| [`F.cursors`](mechanisms/F.cursors.md) | [MouseCursors] 与各类 Cursor.* 键 | 12 | F.readers | partial |
| [`F.save`](mechanisms/F.save.md) | 扩展状态存档（无键） | 0 | — | partial |
<!-- /table:M1 -->

- 读取器（`F.readers`）对照 `ares-src` 的 `src/Utilities/TemplateDef.h`、`Parser.h`、`INIParser.h`，以及 3.0 中
  未命名的读取函数（`catalog.tsv` 的 `type` 列里出现的 `FUN_…`）。需要明确的语义包括：
  - `Nullable` 的"未设置"与显式 `none`；
  - `Promotable` 的 `.Rookie/.Veteran/.Elite` 回退；
  - 列表的分隔与追加；
  - 读取的缓冲长度和大小写规则。
- 存档（`F.save`）：各机制新增状态的保存和恢复方式先定一个统一做法，后面的机制照做。

### M2 补完已有机制

这些机制在 VERA20K 中已经有负责者，缺的主要是剩余的键和使用点，风险最低。

<!-- table:M2 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`T.emp`](mechanisms/T.emp.md) | EMP（单位/弹头/建筑/超武共用状态） | 10 | F.readers | done |
| [`T.driver`](mechanisms/T.driver.md) | 驾驶员、偷车、KillDriver、Operator | 18 | T.crew | done |
| [`T.crew`](mechanisms/T.crew.md) | 乘员与幸存者 | 9 | — | done |
| [`T.factory`](mechanisms/T.factory.md) | BuiltAt、Factory.ExplicitOnly、FactoryOwners | 7 | F.prereq | done |
| [`T.cameo`](mechanisms/T.cameo.md) | PCX 图标与图标调色板 | 3 | — | partial |
| [`T.ammo`](mechanisms/T.ammo.md) | 步兵/武器弹药与装填 | 6 | — | partial |
| [`T.ifv`](mechanisms/T.ifv.md) | IFV 扩展模式与乘客炮塔 | 2 | — | partial |
| [`T.manual`](mechanisms/T.manual.md) | 禁止手动开火/进入/卸载/守卫、不可出售 | 7 | — | partial |
| [`T.convert`](mechanisms/T.convert.md) | 类型转换（Convert.*） | 4 | — | done |
| [`T.typeselect`](mechanisms/T.typeselect.md) | GroupAs 与类型选择 | 2 | — | done |
| [`T.bounty`](mechanisms/T.bounty.md) | 击杀赏金 | 6 | — | done |
| [`T.hunterseeker`](mechanisms/T.hunterseeker.md) | HunterSeeker 单位参数 | 13 | — | done |
| [`T.keepalive`](mechanisms/T.keepalive.md) | House 存活判定 KeepAlive | 1 | — | done |
| [`X.attacheffect`](mechanisms/X.attacheffect.md) | AttachEffect | 17 | F.readers | partial |
| [`H.country`](mechanisms/H.country.md) | Country 展示、载入画面、权重、ParentCountry | 15 | — | done |
| [`H.paradrop`](mechanisms/H.paradrop.md) | ParaDrop 覆盖链 Country→Side→Rules | 6 | — | done |
| [`H.powerplants`](mechanisms/H.powerplants.md) | AI.PowerPlants | 1 | — | done |
| [`H.side`](mechanisms/H.side.md) | Side 默认值与 EVA.Tag | 4 | — | done |
| [`H.sidepresent`](mechanisms/H.sidepresent.md) | Side 展示：计分、对话框背景、图形文字、侧栏文件、颜色、载入主题 | 19 | H.side | partial |
| [`S.core`](mechanisms/S.core.md) | 超武通用：类型、显示、文本、EVA、光照、动画、声音、次数 | 60 | F.readers | partial |
| [`S.money`](mechanisms/S.money.md) | 超武费用与持续扣费 | 6 | S.core | done |
| [`S.gates`](mechanisms/S.gates.md) | 超武建筑门槛与 House 限制 | 6 | S.core | done |
| [`S.ai`](mechanisms/S.ai.md) | 超武 AI 使用与目标选择 | 7 | S.core | partial |
| [`S.generic`](mechanisms/S.generic.md) | GenericWarhead 超武 | 0 | S.core | done |
| [`S.unitdelivery`](mechanisms/S.unitdelivery.md) | UnitDelivery 超武 | 3 | S.core | done |
| [`S.hunterseeker`](mechanisms/S.hunterseeker.md) | HunterSeeker 超武 | 3 | S.core,T.hunterseeker | done |
| [`S.emp`](mechanisms/S.emp.md) | EMPulse 超武 | 5 | S.core,T.emp | partial |
| [`W.filter`](mechanisms/W.filter.md) | 弹头作用对象与伤害门槛 | 8 | — | partial |
| [`U.eva`](mechanisms/U.eva.md) | EVA 语音扩展 | 0 | — | done |
| [`U.palettes`](mechanisms/U.palettes.md) | 建筑 SHP 调色板 | 0 | — | done |
<!-- /table:M2 -->

重点：`S.core` 是 M5 所有超武子类型的前提，需要在 M2 内完成。

### M3 独立末端机制

使用点集中在少数几个函数，不碰（或只轻触）热点函数，适合多人或多个代理同时做。

<!-- table:M3 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`T.cloak`](mechanisms/T.cloak.md) | 隐身阶段、条件隐身、隐身/显形音效 | 10 | — | partial |
| [`T.spotlight`](mechanisms/T.spotlight.md) | 探照灯 | 8 | — | partial |
| [`T.promotion`](mechanisms/T.promotion.md) | 晋升经验、晋升类型、闪光、能力、徽章 | 27 | — | partial |
| [`T.buildtime`](mechanisms/T.buildtime.md) | 建造时间与低电惩罚 | 7 | — | none |
| [`T.passengers`](mechanisms/T.passengers.md) | 乘客容量、允许/禁止列表、初始载荷 | 7 | — | none |
| [`T.carryall`](mechanisms/T.carryall.md) | Carryall 吊运限制 | 2 | — | none |
| [`T.crush`](mechanisms/T.crush.md) | 碾压伤害与全能碾压 | 4 | — | none |
| [`T.harvester`](mechanisms/T.harvester.md) | 采矿车扫描与精炼厂存储 | 7 | — | none |
| [`T.tibunit`](mechanisms/T.tibunit.md) | 单位与矿石：免疫、残留、溢出、转化 | 4 | — | none |
| [`T.selfheal`](mechanisms/T.selfheal.md) | 自愈 | 4 | — | none |
| [`T.damagefx`](mechanisms/T.damagefx.md) | 受损烟雾/火花粒子 | 6 | — | none |
| [`T.fall`](mechanisms/T.fall.md) | 降落伞、下落速度、坠毁 | 9 | — | partial |
| [`T.missile`](mechanisms/T.missile.md) | 自定义导弹单位 | 9 | — | none |
| [`T.dig`](mechanisms/T.dig.md) | 地下单位钻入钻出 | 5 | — | none |
| [`T.jammer`](mechanisms/T.jammer.md) | 雷达干扰、抑制/指示范围、传感器警告 | 4 | S.core | none |
| [`T.gap`](mechanisms/T.gap.md) | 裂缝产生器半径 | 2 | — | none |
| [`T.drain`](mechanisms/T.drain.md) | 能量/资金抽取（Drain） | 2 | — | none |
| [`T.cloning`](mechanisms/T.cloning.md) | 克隆设施 | 4 | — | none |
| [`T.reverse`](mechanisms/T.reverse.md) | 逆向工程与可逆转 | 4 | — | none |
| [`T.ui`](mechanisms/T.ui.md) | 敌方显示名、范围指示圈、透明度、扳手图标 | 5 | — | none |
| [`T.misc`](mechanisms/T.misc.md) | 单位杂项：炮塔速度、Gattling、部署朝向、空速、Assaulter 等 | 15 | — | none |
| [`T.dehardcode`](mechanisms/T.dehardcode.md) | 去硬编码身份（IsCow/IsDesolator/IsBomb） | 4 | — | none |
| [`B.rubble`](mechanisms/B.rubble.md) | 建筑废墟 | 10 | — | none |
| [`B.academy`](mechanisms/B.academy.md) | Academy 老兵加成与 VeteranBuildings | 7 | T.promotion | none |
| [`B.gates`](mechanisms/B.gates.md) | 门音效与可通行建筑 | 3 | — | none |
| [`B.foundation`](mechanisms/B.foundation.md) | 自定义地基与实体高度 | 7 | — | partial |
| [`B.misc`](mechanisms/B.misc.md) | 建筑杂项：建造/出售时间、停靠、可回收、消息、钱数显示 | 11 | — | none |
| [`B.engineer`](mechanisms/B.engineer.md) | 工程师：多工程师、伤害、修理、占领科技 | 6 | — | none |
| [`B.togglepower`](mechanisms/B.togglepower.md) | 手动开关电 | 3 | — | none |
| [`B.degrade`](mechanisms/B.degrade.md) | 建筑退化 | 7 | — | none |
| [`B.secretlab`](mechanisms/B.secretlab.md) | 秘密实验室 | 2 | — | none |
| [`B.fake`](mechanisms/B.fake.md) | 假建筑 | 1 | — | none |
| [`B.tunnel`](mechanisms/B.tunnel.md) | [TunnelTypes] 隧道网络 | 2 | — | none |
| [`B.ai`](mechanisms/B.ai.md) | AI 基地建造计数与内圈 | 6 | — | partial |
| [`A.anim`](mechanisms/A.anim.md) | 动画：自定义调色板、伤害延迟、步兵归属、粒子范围、武器 | 6 | — | partial |
| [`R.tiberium`](mechanisms/R.tiberium.md) | 矿石伤害/治疗/爆炸 | 14 | — | none |
| [`R.particles`](mechanisms/R.particles.md) | 粒子类型/粒子系统扩展 | 1 | — | none |
| [`R.ai`](mechanisms/R.ai.md) | AI 与玩家自动反击、团队报复 | 4 | — | none |
| [`R.misc`](mechanisms/R.misc.md) | 全局杂项：随机箱钱数、显示延迟、盘状激光动画 | 4 | — | none |
| [`H.start`](mechanisms/H.start.md) | 多人开局单位与基地车 | 3 | — | none |
<!-- /table:M3 -->

### M4 热点流程及其上的机制

先按 `hotspots.tsv` 为本组需要的热点函数逐个加扩展点（每个函数一个纯重构 PR），再接入机制。建议顺序：

1. `TechnoClass::FireAt`（006FDD50）
2. `TechnoClass::ReceiveDamage`（00701900）、`InfantryClass::ReceiveDamage`（00517FA0）
3. `Apply_area_damage`（00489280）、`BulletClass::DetonateAtCoord`（004690B0）
4. `BuildingClass::ChangeOwner`（00448260）、`OnConstructionComplete`（00445F80）、`Update`（0043FB20）、
   `Unlimbo`（00440580）、`DestructionEffects`（004415F0）、`ExitObject`（00443C60）

<!-- table:M4 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`B.spy`](mechanisms/B.spy.md) | 间谍渗透效果 | 20 | B.engineer | none |
| [`B.prism`](mechanisms/B.prism.md) | Prism 转发 | 13 | — | none |
| [`B.occupy`](mechanisms/B.occupy.md) | 驻军（UC）与掠夺 | 5 | — | none |
| [`B.trench`](mechanisms/B.trench.md) | 战壕 | 2 | B.occupy | none |
| [`B.firestorm`](mechanisms/B.firestorm.md) | 火风暴墙 | 7 | — | none |
| [`W.beams`](mechanisms/W.beams.md) | 波束、激光、电弧、磁轨炮 | 21 | — | partial |
| [`W.ivanbomb`](mechanisms/W.ivanbomb.md) | 疯狂伊文炸弹 | 14 | — | none |
| [`W.abductor`](mechanisms/W.abductor.md) | Abductor 绑架 | 6 | — | none |
| [`W.weaponmisc`](mechanisms/W.weaponmisc.md) | 武器杂项：ApplyDamage、射程 | 2 | — | none |
| [`W.damage`](mechanisms/W.damage.md) | 相对伤害、Culling、部署状态伤害 | 9 | W.filter | none |
| [`W.effects`](mechanisms/W.effects.md) | 弹头效果：离子炮、涟漪、闪光、声呐、禁武、预冲击动画、桥 | 17 | — | none |
| [`W.death`](mechanisms/W.death.md) | 死亡表现与死亡武器抑制 | 8 | — | none |
| [`W.mctemporal`](mechanisms/W.mctemporal.md) | 心控永久与超时空 | 3 | — | none |
| [`P.projectile`](mechanisms/P.projectile.md) | 抛射体：空爆、分裂、重定向、散布、受建筑/战壕阻挡 | 11 | — | none |
<!-- /table:M4 -->

### M5 超武子类型

每种 `Type=` 一个里程碑，均依赖 `S.core`。

<!-- table:M5 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`S.chrono`](mechanisms/S.chrono.md) | ChronoSphere/ChronoWarp | 10 | S.core | none |
| [`S.dominator`](mechanisms/S.dominator.md) | Dominator 心控超武 | 12 | S.core,W.mctemporal | none |
| [`S.droppod`](mechanisms/S.droppod.md) | DropPod | 8 | S.core | none |
| [`S.mutator`](mechanisms/S.mutator.md) | GeneticMutator | 4 | S.core | none |
| [`S.lightning`](mechanisms/S.lightning.md) | LightningStorm | 18 | S.core | none |
| [`S.nuke`](mechanisms/S.nuke.md) | Nuke | 4 | S.core | none |
| [`S.protect`](mechanisms/S.protect.md) | Protect（铁幕/力场） | 3 | S.core,T.ic | none |
| [`S.spyplane`](mechanisms/S.spyplane.md) | SpyPlane | 3 | S.core | none |
| [`S.sonar`](mechanisms/S.sonar.md) | SonarPulse | 1 | S.core | none |
| [`S.paradrop`](mechanisms/S.paradrop.md) | ParaDrop 超武 | 4 | S.core,H.paradrop | partial |
| [`S.firewall`](mechanisms/S.firewall.md) | Firewall 超武 | 0 | S.core,B.firestorm | none |
| [`S.battery`](mechanisms/S.battery.md) | Battery 超武（3.0 新增，供电/保持在线） | 3 | S.core | none |
<!-- /table:M5 -->

注：超武子类型还复用了原版键和 `S.core` 的键，所以表中键数偏少。`S.firewall` 没有自己的新键，
行为来自 `B.firestorm` 和 `S.core`；`S.battery` 是 3.0 新增的类型（`Battery.*`）。

### M6 状态交叉的机制

需要先在第 8 节约定共享状态的写入方，再实现。

<!-- table:M6 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`T.ic`](mechanisms/T.ic.md) | 铁幕/力场/超时空修正与免疫 | 9 | — | none |
| [`T.berserk`](mechanisms/T.berserk.md) | 狂暴与绑架免疫 | 4 | — | none |
| [`T.poweredby`](mechanisms/T.poweredby.md) | PoweredBy 供电单位 | 2 | T.emp | partial |
<!-- /table:M6 -->

### M7 目录外部分（逐项决定是否做）

<!-- table:M7 -->
| 机制 | 名称 | 键 | 依赖 | 现状 |
|---|---|---|---|---|
| [`U.shell`](mechanisms/U.shell.md) | 主菜单/战役/影片列表/RMG/遭遇战设置 | 26 | — | none |
| [`U.triggers`](mechanisms/U.triggers.md) | 新触发动作/事件（TAction/TEvent） | 0 | — | none |
| [`U.bugfixes`](mechanisms/U.bugfixes.md) | 无条件 Bugfix 钩子（无键） | 0 | — | none |
<!-- /table:M7 -->

- `U.bugfixes`：`ares-src` 中 `src/Misc/Bugfixes*.cpp` 的钩子。逐个判断：是修正崩溃或明显错误（可考虑做），
  还是改变 retail 玩法（需要维护者决定）。
- `U.triggers`：`src/Ext/TAction`、`src/Ext/TEvent` 中的新动作和事件编号，属于地图 INI 内容。
- `U.shell`：`Ares.UISettings.cpp`、`MoviesList`、`RMG`、`Campaign` 等，影响菜单和战役外壳。

## 6. 单个机制的开发流程

### 准备

需要一份本地的 ares-src 工作区（Ares 3.0 还原工作区，含 `decompiled/`、`src/`、`reference/gamemd_hooks/`、
`symbols/`、`report/`）。索引页里的路径都相对于它。让 AI 实现某个机制时，告诉它工作区路径和机制 ID 即可，例如：

> 按 `docs/ares-support/README.md` 第 6 节实现 `B.rubble`，ares-src 在 `../ares-src`。

### 读代码的优先级

ares-src 必须是 `recover/inj-layout` 或更新的版本（见文首）。

1. **还原源码**（`src/`）：主要阅读对象。函数体已按 3.0 翻译或对照 3.0 核对，改动处注有 `// 3.0 (Ares.dll 地址)`；
   核对结果记录在 `symbols/verified_30.tsv`，索引页"3.0 核对"列据此标 ✓。
2. **gamemd 原版上下文**（`reference/gamemd_hooks/<函数地址>.c`，钩子处标 `// >>> HOOK`）：钩子在原版流程中的时机，
   以及跳转地址的含义（返回的地址落在原版哪个分支）。源码只给出跳转地址，含义要看这里。
3. **Ares 3.0 反编译**（`decompiled/functions/<地址>.c`）：源码有疑问、索引页"3.0 核对"列不是 ✓，
   或 `report/body_check*.tsv` 列出该函数时，以反编译为准。

源码和注释数据库仍可能有错。发现与反编译不一致时，以反编译为准，把差异写进索引页备注区，并告知 ares-src 的维护者。

### 已知的坑

- **不要用 ares-src 的 `main` 分支。** 那里的保留钩子仍是 0.A 写法。例如 `B.rubble` 的工程师钩子：0.A 在放置失败时
  恢复原建筑；3.0 改为销毁旧建筑，并且不消耗工程师。`recover/inj-layout` 中已是 3.0 写法。
- **搜源码用 `rg` 或 `grep -a`。** `src/Ext/BuildingType/Body.cpp` 是 Latin-1 编码，macOS 自带的 `grep` 在 UTF-8 环境下
  搜不到其中的内容，也不报错（例如 `BuildingTypeExt::SwitchRubble` 的定义就在这里）。
- **还原源码可以编译，但从未在游戏中运行过**（需要 Syringe）。`report/body_check*.tsv` 列出了与 Ares.dll 静态比对仍不一致的
  函数（多为工具链差异），涉及时看反编译。
- **反编译中的虚函数调用**写作 `(**(code **)(*p + 0xd4))()`：用 `ares-src/reference/gamemd_vtables.tsv` 按类和偏移查函数名
  （例如 BuildingClass +0xD4 为 `Limbo`，+0xF8 为 `UnInit`，+0x100 为 `ExitObject_Main`）。
- **字段偏移**：Ares 扩展字段用 `symbols/extdata_fields.tsv` 查；gamemd 字段用 `reference/gamemd_structs.tsv` 查。
  3.0 把扩展指针存在 gamemd 对象的空闲字段里（如 BuildingType +0xE24、TechnoType +0x2FC），见 `symbols/ares_ext_offsets.tsv`。
- **Ghidra 会混淆参数和寄存器**（样板中归属与血量两个参数的编号被混在一起）。结果可疑时，要看原生指令确认。
- **钩子返回值是跳转地址。** 0 表示继续执行原版代码，其他值要在原版上下文中找到对应位置，才能知道走了哪个分支。
- **随机数**：用到 `ScenarioClass::Random` 的地方（如 `OwnerHouseKind::Random`）都会影响确定性，要记录取随机数的次数和顺序。

### 步骤

1. **读索引页**（`mechanisms/<id>.md`）和其中的备注区。
2. **核实数据。** 对表中每个使用函数：
   - `inferred` 的条目确认是否真属于本机制；
   - `no_use_found` / `unscanned` 的键，在 `decompiled/` 中搜字段偏移，找出读取后用在哪里；
   - 发现机制划分有误时，改 `tools/ares_support/mechanisms.py` 并重新生成。
3. **按上面的优先级读代码**，弄清每个钩子的时机、条件、对状态的修改、跳转，以及调用的辅助函数（索引页"非钩子函数"一节）。
4. **找 VERA20K 负责者。** 索引页最后一列列出了已经引用该 gamemd 函数的文件，从那里开始找；也可以按地址（如 `4415F0`）
   和原生函数名搜索。找不到负责者时，先确认原版功能本身是否已经移植。
5. **把结论写进备注区**：3.0 与 0.A 的差异、确认过的时机和分支、未决问题。只写结论和地址，不贴反编译代码。
   比较复杂的机制，可以另写 `docs/ares-<机制>.md` 给玩家和模组作者看，格式参照现有的 Ares 文档。
6. **实现。** 按下面的提交顺序。Rust 代码按 VERA20K 自己的结构写，在旁边注明原生地址（gamemd 地址和 Ares 3.0 地址），
   不逐行转写 C++。
7. **验收。** 按第 9 节验收，然后更新 `catalog.tsv` 的 `vera_status`、`docs/ares.md` 的支持列表和致谢。

### 一个 PR 内的提交顺序

一条链一个 PR（第 3 节第 5 条）。PR 内按下面顺序提交，方便审查和定位问题：

| 提交 | 内容 | 行为变化 |
|---|---|---|
| 读取 | 读取器、规则数据、按层合并 | 无（只读不用） |
| 扩展点（如需要） | 热点函数纯重构 | 无 |
| 使用点 | 本条链上的使用点 | 仅在 `ares` 构建、配置了键时 |
| 收尾 | 文档、目录状态、残差 | 无 |

开 PR 前由一个只读的审查代理过一遍（项目契约要求，规则见 `.agents/skills/_shared/review.md`）。
PR 描述按项目要求写：改了什么、为什么（原生地址）、怎么测的，约 15 行。

### 规格模板（可选）

需要先审规格再实现时（例如热点函数或 M6 的交叉机制），按下面的模板写进备注区；一般机制不必写这么全。

```
键：           Promote.EliteSound                  （精确大小写）
文件与层：     rulesmd.ini 全部规则层 / artmd.ini 单层
节：           [<TechnoType ID>]
类型/解析器：  NullableIdx<VocClass>；缓冲长度；名字查表规则；none → 显式空；未知名的处理
默认：         未设置                               （取自构造函数，不取 retail 值）
读取：         <Ares 加载函数地址>；在哪个键之后读；有无前置条件
读后处理：     无 / 继承 / 复制 / 校验
使用点：       <Ares 地址 名字>，钩在 gamemd <地址 函数名>
               时机、条件、修改的状态、RNG 调用次数与顺序、跳过或返回的原版代码
VERA20K 负责者：读取在 …，使用在 …
默认构建：     忽略该键
测试：         未设置 / none / 有效值 / 边界值
残差：         …
```

使用点写行为描述，不抄还原源码。

## 7. 热点函数规则

1. **一个函数一个负责者。** 在 PR 中写明负责者，所有机制都通过它接入。
2. **扩展点按原生顺序排列。** 同一函数里 Ares 钩子的执行顺序就是钩子地址的顺序（`hotspots.tsv` 的 `hook_list` 已按地址排序）。
   扩展点按同样的顺序排，并写明在每个扩展点：
   - 能否跳过后面的原版代码；
   - 能否提前返回；
   - 返回值或寄存器结果怎样传给后面的代码。
3. **一个钩子服务多个机制时，**把它拆成每个机制一个调用，按原钩子内部的顺序依次执行。
4. **与 Phobos/NPatch 的交叉。** 同一原生位置被多个扩展改动时，按原生优先级选定胜出方，另一方保留自己的负责者。
   做法参照 `npatch.md` 中的辐射例子。
5. **扩展点提交不带行为。** 测试前后完全一致，便于审查。

## 8. 共享状态约定

以下状态被多个机制读写，必须先定写入方，再实现读取方：

| 状态 | 写入方 | 读取方 |
|---|---|---|
| 停用（EMP / 无驾驶员 / 断电） | `T.emp`、`T.driver`、`T.poweredby` 各写自己的原因位 | 移动、开火、显示变暗（`DeactivateDim*`） |
| 伤害/火力/速度/射速倍率 | `X.attacheffect`、`T.ic`、`T.berserk` 各写自己的部分 | 伤害、开火、移动 |
| 晋升等级与经验 | `T.promotion` | `B.academy`、`B.spy`（老兵效果）、能力免疫判断 |
| 超武实例状态（充能、次数、可用性） | `S.core` | `S.money`、`S.gates`、`S.ai`、各子类型、`B.spy`（重置超武） |
| Country → Side → Rules 回退 | `H.country`、`H.side` | `H.paradrop`、`T.fall`、`T.crew` 等 |
| 建筑归属变化 | 原版易主流程 | `B.prism`、`B.spy`、`B.academy`、`T.jammer` |
| 矿石与覆盖物 | 原版矿石流程 | `R.tiberium`、`T.tibunit`、`T.harvester` |

## 9. 测试与验收

- **三种构建都通过**：默认、`ares`、`ares,phobos,npatch`（`tools/capability_tests.py`）。
- **测试命令**：经 `python -m tools.cargo_run -- test -p <crate> --lib <模块路径>::` 跑受影响模块的测试，
  用 retail `ini/` 并设 `VERA20K_REQUIRE_RETAIL_INI=1`（见项目契约 Git and validation 一节）。
- **回归测试**：每个改动一到三个，通过生产读取器读 INI。必须覆盖：
  - 键未设置（等于当前行为）；
  - 设为 `none`（适用的键）；
  - 一个有效值；
  - 一个边界值。
- **retail INI 不受影响**：retail 规则没有设置 Ares 键时，`ares` 构建的结果和改动前一致。
- **确定性**：新状态只用 `SimFixed` 和确定的 RNG 流，RNG 调用的次数和顺序写进规格。
- **存档**：新状态进入存档。不同能力组合的存档本来就不能互相加载。
- **原生执行**：只有项目契约列出的情况（数值核心、RNG、已知陷阱）才需要跑原生代码。
- **机制完成标准**：机制内所有键的 `vera_status` 都是 `done` 或 `residual`（附原因）；每个使用点在 Rust 代码旁注明原生地址；
  `docs/ares.md` 已更新。

## 10. 进度跟踪

- `catalog.tsv` 的 `vera_status` 列手工维护。重新生成目录时，脚本按 `(cls, key)` 保留已填写的值。
- 更新目录和索引页（备注区和 `vera_status` 会保留）：

  ```sh
  python3 -I tools/ares_support/build_catalog.py <ares-src 路径>
  python3 -I tools/ares_support/build_index.py <ares-src 路径>
  ```

- 机制划分有误时，改 `tools/ares_support/mechanisms.py` 的 `RULES`，再重新生成。
- 每个里程碑结束时，在本文件第 2 节更新数字。

## 11. 风险与待定事项

| 项 | 影响 | 处理 |
|---|---|---|
| 推断出的使用点有误 | 规格错误 | 第 6 节第 1 步逐个核实 |
| 用错 ares-src 版本 | 照 0.A 写法实现 | 文首注明的分支与提交；索引页"3.0 核对"列 |
| 112 + 122 个键没有已知使用点 | 工作量被低估 | 在所属机制开工前查清 |
| 还原源码未在游戏中运行 | 源码与 3.0 仍可能有差别 | 有疑问时看反编译；必要时与安装了 Ares 的游戏并排对照 |
| 机制划分按键名前缀，有错配 | 里程碑边界不准 | M0 校正；开工时以使用函数为准 |
| Phobos 同名键 | 两个扩展争用同一负责者 | 按跨扩展规则，先定胜出方 |
| 还原源码的使用条款 | 发布风险 | 只参考行为，不复制代码；引用方式由维护者决定 |
| M7 改变 retail 行为 | 与项目基线冲突 | 逐项由维护者决定 |
