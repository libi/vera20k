---
layout: default
title: RA2、YR 与 Ares 的 INI 兼容手册
lang: zh-CN
---

# RA2、YR 与 Ares 的 INI 兼容手册

本手册供 MOD 制作者和地图作者查阅，说明 VERA Mod Engine 对各版 INI 的读取规则、旧标签的映射方式、导入时的诊断，以及写回原格式的条件。RA2、YR 和 Ares 的兼容预设共用单位、弹体、武器、弹头和地图触发系统。每个预设分别规定文件来源、读值顺序、默认行为和指令编号。

**文档状态为设计稿。** 仓库已提供零售文件索引、原版读取位置索引和 Ares 手册索引。下文的包清单、直接 INI 补丁、导入报告和回写要求属于开发规格。标签的运行兼容性按预设和场景逐项验收。[可组合玩法总设计](composable-gameplay-design.md) · [可组合玩法与 INI 兼容实施路线图](composable-gameplay-roadmap.md)

**查阅入口**　[选择预设](#预设与兼容状态) · [文件与标签分类](#文件与标签分类) · [射程与护甲示例](#配置示例) · [标签族对照](#标签族对照) · [导入与导出](#导入与导出) · [查询来源索引](#附录来源索引)

## 预设与兼容状态

| 目标预设 | 输入及原版对照 | 使用场景 |
|---|---|---|
| `Base=ra2` | RA2 的 `rules.ini`、`art.ini`、`ai.ini` 和原版地图，行为以 `game.exe` 为准 | 维护 RA2 单位、任务与旧标签。制作组也可以在自己的 MOD 中单独为某个弹头挂减速 AE |
| `Base=yr` | YR 的 `rulesmd.ini`、`artmd.ini`、`aimd.ini` 和原版地图，行为以 `gamemd.exe` 为准 | 维护 YR 遭遇战、战役及其模式和地图覆盖 |
| `Base=ares@3.0`、`Base=ares@3.0p1` | Ares 预设引用 YR 的定义。每个版本对照 YR 1.001 加相应的 Ares | 导入旧 Ares MOD，同时纳入版本规定的键、地图指令与无需新键即可生效的改动 |

`Base=` 是拟议的包清单字段。导入旧 MOD 时，版本由作者确认。自定义 MOD 可以保留旧武器节的 `Damage`、`Range`、`Warhead`，再用 `Projectile=` 引用采用新运动定义的弹体。导入既有 Ares 工程时，应按原工程所用版本选择完整预设。[Ares 官方说明](https://ares-developers.github.io/Ares-docs/#introduction)

### 兼容状态

状态按“预设版本、文件、节、键或指令、运行条件”记录。同名标签在不同节可以有不同结果。某个标签在零售文件里出现过，只能说明该处存在这种写法。

| 导入报告中的状态 | 判定条件 |
|---|---|
| 已在场景验证 | 已确认读取函数、默认值和游戏内的使用位置，并在指定版本和模式的正常游戏路径验证了结果 |
| 仅可读取或保留 | 可以识别记录或原样导出。报告列明已确认的读取行为和仍待验证的游戏效果 |
| 当前条件忽略或不可达 | 已知该版本或模式会跳过该条记录，报告注明条件及依据 |
| 已知差异 | 对照场景已经显示结果不同，报告指出触发条件以及受影响的单位、地图或任务目标 |
| 待验证 | 版本、节类型、指令含义、实现或所需素材仍有缺项，报告保留原文位置及待查事项 |

## 文件与标签分类

先按文件用途和节类型检索标签，再查负责执行该行为的系统。地图内嵌 Rules 可以覆盖场景规则。地图压缩块、客户端设置和规则标签分别由各自的读取路径处理。

| 旧文件、节或标签 | 对应的引擎数据 | 兼容要求 |
|---|---|---|
| Rules 类型表、`[Sides]`、国家、参战方（House）、单位和建筑节 | 类型 ID、旧序号、阵营、单位原型和默认 Flag | 保留按名查找、按引用分配、仅查询已存在类型的区别。移除 `Infantry` Flag 只改变相关过滤结果，乘员和占格仍由相应能力管理 |
| `[General]`、难度、科技树、生产、电力、采矿、资金、建造上限 | 难度参数、经济参数和生产条件 | 全局参数可能同时影响多个 House。资金、生产队列、同类建筑或单位数量都读取本局实际状态，转属后按新归属重新判定 |
| 武器、弹体、弹头、护甲、超武、升级和附加效果（AE） | 武器循环、运动、接触、伤害响应、施放条件与持续效果 | 分别记录载入时的单位换算、发射时取样的属性、受击时执行的过滤。一次伤害或支出只结算一次 |
| `Locomotor`、`SpeedType`、`MovementZone`、地表通行表、桥梁、地底移动 | 运动器、寻路、地形查询、占格、落地或引爆动作 | 旧 GUID 映射到具名运动方案。每步只有负责移动的系统提交坐标，Flag 只影响引用它的条件 |
| ART 的图像、序列、炮塔、动画、粒子和弹道 | 资源绑定、画面、音效、参与模拟的动画或粒子 | 动画的 `Damage` 可造成实际伤害。字段应注明按对象类型 ID 或图像名查找，以及缺少视觉资源时的处理方式 |
| 战区 INI、`TileSetNNNN`、覆盖物、矿物、装饰和地形对象 | 地块定义、场景地形、覆盖层及显示资源 | 保留地块编号、子地块、通行属性和 MIX 内的资源引用。`AllowBurrowing` 会影响地底移动 |
| AI、TaskForce、ScriptType、TeamType、`AITriggerTypes` | 征募条件、小队编成、脚本指令和 AI 生产决策 | 保留引用顺序、数量、权重、随机数、难度及模式限制。AI 触发和地图触发各自保存运行状态 |
| 地图对象、House 占位符、`Triggers/Events/Actions/Tags/CellTags`、局部及全局变量 | 地图对象实例、触发器、标签绑定、剧本变量与小队脚本 | 保留原 ID、参数类型、触发顺序和重复方式。FA2 的 Tag 关联地图触发器，可挂卸的玩法 Flag 参与规则筛选，分别存储 |
| `IsoMapPack5`、`OverlayPack`、`OverlayDataPack`、`PreviewPack` 等 | 地图编解码、地形、覆盖层或预览图 | 按文件格式保留容量限制、地块索引和块顺序，原样回写未编辑的块 |
| `MPModes`、模式 INI、PKT、`Battle/Mission/MapSel/CoopCamp`、地图头、RMG | 模式目录、任务选择、场景加载、随机地图生成器 | 分别确认文件存在、模式可选和实际读取。生成参数应能追溯到相应地图 |
| Sound、EVA、Theme、UI、教程、快捷键、WDT 和本机选项 | 媒体目录、客户端偏好、教程和旧版界面 | 剧本按编号引用的项目须保留编号。若编号或设置会影响模拟命令的解释，该部分计入联机规则摘要 |
| Ares 的 include、继承、集合追加、预注册、跨文件字段、数字指令和未新增标签的改动 | Ares 语法、类型目录及相应玩法系统 | 记录语法生效阶段、目标版本对原有逻辑的替换范围，以及原逻辑继续执行的位置 |

RA2 和 YR 分别规定文件集合及优先级。YR 一局依次读取 `RULESMD.INI`、存在时的 `LANGRULE.INI`、所选模式和地图，战役及特定模式还会额外读取。AI 使用根文件和地图覆盖，ART、Sound、EVA、Theme 各有固定来源。`MPSiege(MD).ini` 可能只有注释，模式目录决定该模式是否可选。`RA2(MD).ini` 等本机运行设置另行登记读取位置，注明影响本机、房间设置或游戏模拟。零售根档案中的 31 个 INI 未覆盖这部分。 [规则层及来源](../ini-roots.md) · [模式选择](../../src/skirmish_modes.rs#L312) · [地图格式](../map-format.md)

## 字段字典与编译顺序

### 字段身份与读值

一条字典记录由产品及修订、INI 方言、文件用途、节类型、精确键名或有界编号形式、读取阶段共同确定。例如武器节的 `ROF` 是整数开火间隔，`[Easy]` 等难度节的 `ROF` 是倍率。另一种常见情形是多个键共同决定一个结果，弹体的实际飞行可能同时依赖武器 `Range`、`Speed` 和弹体 `ROT`、`Floater`。

将 `Weapon1`、`Weapon2` 归并为 `Weapon<N>` 前，应按实际循环和引用位置确定编号起点、上限、是否允许缺号和越界处理。同一键有多个读取位置时，字典须记录各次读取的顺序和覆盖关系。新增参数有独立 ID、类型及单位，原拼法仍归属于所选兼容预设。

`Strength`、`Sight`、`Cost` 分别绑定单位生命值、视野与生产价格，同类型实例共享载入后的定义。`Primary` 需要先解析武器引用，`Verses` 按护甲顺序绑定数组位置，`OpenTopped` 涉及乘员的登车、开火、离车与清理。地图动作编译为带参数的指令。需要响应事件的玩法组合使用触发器、过滤器和效果器，Flag 可加入过滤条件。首版作者接口为直接 INI、织网者模板和字段字典，后续 Wasm 沿用同一套带类型的命令。 [生命值与价格](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/object_type.rs#L4438) · [视野](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/object_type.rs#L4664)

| 字典栏目 | 必须记录的内容 |
|---|---|
| 身份和依据 | 产品、修订、文件与读取阶段、节的识别条件、原文拼法及位置、可达的读取函数、游戏内使用处、已验证的平台或模式 |
| 读值规则 | 原版读取函数、单位、精度、舍入及范围。分别说明整节缺席、缺键、物理空值、解析失败、显式哨兵、重复和继承，区分构造默认值与后续重读时的回退值 |
| 游戏行为 | 产生的字段、引用、策略或命令，后处理及生效时点，负责更新状态的系统，同帧可见性、存档和随机数影响 |
| 作者接口 | 模板名和可调参数、直接 INI 写法、无效值诊断、展开来源、允许原样回写或能从新值合成的目标版本 |
| 验证记录 | 各产品分别登记已观察到的拼法、读取位置、使用位置和已验证场景。记录已知差异及可区分两种解释的最小用例 |

### 读取、覆盖与运行

1. **选择预设和版本。** 包清单指定 RA2、YR、相应版本的 Ares 或制作组的自定义预设。选用 Ares 时，先合并其引用的 YR 定义及该版本改动，再按版本规定的顺序处理规则、模式和地图，保留原来的多次读取。缺少明确版本的旧 MOD 由导入器列出判断依据，供作者确认。现有 VERA 的 Ares 开关和读取函数可用作实现对照。 [Ares 开关与读法](../../src/rules/ares_switch.rs#L225)

2. **保存源文件，按原方言查值。** 编辑工程保留字节、编码、节与键的顺序、重复项、注释、空行、include 边界和块位置。运行时按各版查值。YR 的物理枚举顺序与 CRC 索引次序可能不同，同名项以实际查值方法为准。Ares 的 `[#include]`、`[Child]:[Base]`、`+=值` 在相应文档和阶段处理。Rules 和地图是不同次读取。`$Inherits=` 的来源另见文件指令说明。 [INI 索引](../../src/rules/ini_parser.rs#L268) · [Ares 语法](../../src/rules/ini_ares_syntax.rs) · [文件指令](../ini-directives.md)

3. **按原顺序建类型和解析引用。** 每一轮区分表内成员、引用时新建的类型、仅查询已存在的类型、延迟解析及缺失引用的回退。按读取函数规定使用当前值或默认值，保留空节与缺键可能引起的写入。旧数字编号登记为 `(序号, 稳定 ID)`，供回放、地图和导出使用。新格式自动发现具名定义，可省去手写注册表。 [规则处理所有者](../../src/rules/process_owner.rs) · [各阶段及 Ares 武器预注册](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/native_processing.rs#L6705)

4. **应用本层覆盖并计算派生值。** 先执行本层原有读取，再执行本层 `[Patch 完整名]`。下一层从该状态继续。若原版后处理再次覆盖同一字段，记录之前的值、来源和重算原因。新补丁只重算当前预设声明的依赖。更换完整运动方案时连同其速度计算规则一并更换。最后一层完成后冻结定义、校验引用并计算规则摘要。额外的战役重读单独登记，导入报告应列出实际顺序。 [原生后处理](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/native_processing.rs#L10118) · [新格式覆盖规则](composable-gameplay-design.md)

5. **将结果交给对应的游戏系统。** 静态字段、Flag 集合及过滤条件由同类型实例共享。持续效果和剧本编译为有范围限制的事件订阅和命令。导航、占格、武器循环和资金仍由原系统成批更新，派生查询索引需声明失效条件。存档记录活跃效果、剩余时间、旧脚本状态及编号对应表的版本。联机比较有效规则、与顺序相关的表和协议版本。外观及原文注释按其实际用途处理。 [模拟和回放设计](composable-gameplay-design.md)

### 织网者与来源查询

同一份已核准的字典记录用于生成中文字段说明、织网者类别与端口、模板参数及旧格式所需的注册表。选用“零售常速抛射”模板时使用原版的速度计算规则，也可展开模板修改某一部件。编辑器应支持从旧 INI 行查到生效值、单位、覆盖层、使用处及可导出的版本，也能从部件反查源行。模板实例有固定 ID，调整图形节点的显示顺序应保持编译结果一致。 [织网者作者手册](https://github.com/ra2diy/INIWeaver-1.0/blob/dc490bdf60cf058c9ae95f5380a032a997df9267/docs/Info.md#L49-L86) · [模块库](https://github.com/ra2diy/INI-Weaver-Module-Library)

## 配置示例

### 武器射程与弹体运动

一张地图将已注册武器 `105mm` 的射程改为 6 格，最小射程定为 1 格。以下两个示例使用相同的输入值。

```ini
[105mm]
Range=6
MinimumRange=1
```

对应的拟议补丁明确标记了 YR 武器和距离单位。引用名 `yr.Weapon.105mm` 必须能解析到本局目录中的既有武器。

```ini
[Patch yr.Weapon.105mm]
Range.Max=6cell
Range.Min=1cell
```

| 版本和标签 | 原版读取位置 | 解释与验收条件 |
|---|---|---|
| RA2 武器 `Range`、`MinimumRange`（`weapon-range`） | `game.exe` 武器构造函数 `0x00732EA0` 将两项置零。目录从 `0x00652386` 调武器读取函数 `0x00733220`，读取位置 `0x00733448`、`0x00733462` 调用 `0x00469AE0`。`0x006523EC` 随后调用后处理 `0x007339C0` | 数值 `-1` 保留传入的当前值，其余数值按格乘 256 转为整数。示例对应 1,536 和 256 lepton。每次发射的速度及碰撞判定仍需在 RA2 发射路径核实。 |
| YR 武器 `Range`、`MinimumRange` | `gamemd.exe` 的 `0x00772331`、`0x0077234B` 调用 `0x00474620`。VERA 已有逐轮计算的武器速度后处理和实际开火时的速度计算 | 在具有弹体且 `ROT=0` 的相应路径，`Range`、重力和 `Floater` 参与速度计算，开火还可能按水平距离求速。分别验证近、远目标和下一层只改射程的情况。显式 `-1` 应保留上一轮射程。 [范围读者](../../src/rules/ini_value.rs#L1423) · [速度后处理](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/native_processing.rs#L10118) · [实际发射](../../src/sim/projectile/launch.rs#L222) |
| RA2 武器和难度节的 `ROF`（`rof`） | 武器 `0x0073342E` 调整数读取函数 `0x0050D7F0`，难度 `0x0064DBE8` 调浮点读取函数 `0x0050E3A0`，默认值为 `1.0`。两款游戏的根规则均含这两类输入 | 武器保存基础开火间隔，难度保存倍率。分别覆盖 `[105mm]` 和 `[Easy]`，核对各自修改的字段及实战开火间隔。 [VERA 武器读者](../../src/rules/weapon_type.rs#L479) · [难度读者](../../src/rules/ruleset.rs#L390) |

新运动方案允许将飞行时间和顶高分别设为 `Pace=FixedTicks(60)`、`Height=PeakAboveHigherEnd(192lepton)`。要保证 60 个逻辑帧后抵达预定落点，还须规定何时记录目标位置、如何处理目标移动、提前碰撞及抵达。相同的 `Motion` 可以供跳跃步兵使用，末端改由单位提交占格。零售炮仍可选用原运动方式。旧 `Arcing=yes` 不能独立表达上述两个参数，向旧版导出时应定位具体超出表达范围的字段。 [运动与接触的完整示例](composable-gameplay-design.md)

### Ares 自定义护甲与弹头响应

旧 Ares MOD 定义名为 `ceramic` 的护甲，以 `heavy` 为基础，令弹头 `AP` 对它造成 60% 伤害，并关闭受击还击。

```ini
[ArmorTypes]
ceramic=heavy

[AP]
Versus.ceramic=60%
Versus.ceramic.Retaliate=no
```

导入时先建立 `ceramic` 并登记从 `heavy` 继承默认响应的关系，再分别写入 `AP × ceramic` 的伤害倍率和还击许可。受击结算读取倍率，受击还击判定读取许可。强制开火和被动索敌各有自己的读取及回退规则。旧 `Verses` 的 11 个位置须按目标产品的护甲顺序解释。此例可以写回三个 Ares 原键。若 MOD 另行规定“`Insulated` Flag 挂载后的 30 帧内改写响应”，新规则订阅 Flag 变化。导出旧版本时，报告应列出此条件的来源、目标版本和表达差异，同时保留可导出的护甲部分。 [Ares 护甲定义](https://ares-developers.github.io/Ares-docs/new/additionalarmortypesandverses.html) · [护甲继承与读值](../../src/rules/ares_armor.rs#L149) · [伤害使用处](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/sim/combat/combat_aoe.rs#L3870) · [索敌使用处](../../src/sim/combat/greatest_threat.rs#L2476)

## 标签族对照

以下十一项与前述 `weapon-range`、`rof` 共十三个案例。索引中的 `designCase` 指向相应的旧文件或手册声明。表中的验收条件用于实施后的对照。

| 案例与原标签 | 映射规则及适用范围 | 场景验收条件 |
|---|---|---|
| `ordered-identity` · `[InfantryTypes]`、`[WeaponTypes]` | 原版类型表建立身份、数字序号与默认分类。武器和弹体也可能在首次引用时建立。Ares `[WeaponTypes]` 会在类型读取前预建武器。新定义自动注册，导出时按目标版本还原相应表项 | 使用两种引用顺序及一件仅在 Ares 表中出现的武器，核对分配顺序、查找结果、旧编号和重新导入后的 ID。 [目录分配点](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/native_processing.rs#L6705) · [Ares 的武器表](https://ares-developers.github.io/Ares-docs/new/weapons/weapontypes.html) |
| `stock-motion` · 单位 `Locomotor/SpeedType/MovementZone`、弹体运动标签 | GUID 选择运动器，并与导航区域、地表速度、占格及运动状态转换配合。弹体和单位可以引用同一运动定义，分别处理接触和抵达 | 在同一地形核对可通行性、桥面或坡上的高度、受阻与抵达时刻，再单独替换运动参数复测。 [运动规格与现有所有者](composable-gameplay-design.md) |
| `open-topped` · `[BFRT] OpenTopped` | YR 在 `0x007143C5` 调布尔读取函数，`0x007143CA` 写入结果。载具允许乘员开火时仍需有座位、合法目标、武器及发射位置。RA2 根规则样本未见该键，其支持情况单独核实 | 验证乘员登车、对合法目标开火、下车及载具被毁后的清理。单独切换用于筛选的 `Transport` Flag 时，座位数应保持原值。 [读者](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/object_type.rs#L5249) · [乘员生命周期](../../src/sim/passenger.rs) |
| `armor-response` · `Verses`、`Versus.<armor>.*` | YR 已进入弹头节、却读不到 `Verses` 时可把 11 项重置为 100%。须分别处理整节缺席、节存在但缺键、物理空值与显式列表。Ares 在此基础上支持自定义护甲、倍率与三种许可 | 在模式层和地图层分别省去整节或只省键，核对伤害、还击和被动索敌。对不完整列表，分别记录目标版本行为。 [YR 缺键路径](../../src/rules/warhead_type.rs#L767) · [Ares 实现范围](../ares-custom-armor.md) |
| `prerequisites` · `Prerequisite`、`[GenericPrerequisites]`、`Prerequisite.Lists/List0/List1` | 列表内全部满足，列表之间任选一组。`Lists` 决定额外列表数，空的第 0 组也可能成立。战区、负向条件、逆向工程、窃取科技、Override 等有既定顺序。玩家的实际持有与 AI 规划中的建筑须分别查询 | 组合空的基础组、一个额外组和一个负向条件，再转移一座可替代前置的建筑。核对双方侧栏、实际生产和 AI 计划。 [原键定义](https://ares-developers.github.io/Ares-docs/new/prerequisites.html) · [组与门控顺序](../ares-prerequisites.md) · [生产判断](../../src/sim/production/can_build.rs#L200) |
| `sw-money` · `Money.Amount/DrainAmount/DrainDelay` | 负的 `Money.Amount` 表示发射支出，正值表示发放资金。区分预检、准许施放、余额结算及周期性充放电。已接入路径使用同一份 House 资金状态，各超武类型分别登记单段、双段和自动发射行为 | 对同一 `GenericWarhead` 超武测试余额不足、目标拒绝和成功施放，只有成功才结算一次。对充放电类另测暂停、恢复及 3.0p1 差异。 [手册](https://ares-developers.github.io/Ares-docs/new/superweapons/cost.html) · [钱包](../../src/sim/superweapon/money.rs#L46) · [当前接入范围](../ares-superweapon-money.md) |
| `attach-effect` · Techno 或 Warhead 的 `AttachEffect.Duration` 等 | 对象自带与命中挂载分别触发相应的 `Behavior`，保存首个来源 House、叠加或刷新策略、暂停、移除与属性合并。Ares 预设保留永久时长 `-1` 和同帧受击与挂载顺序 | 验证零伤命中刷新非叠加效果、进出载具、存读档与销毁，核对剩余时间、属性和清理结果。仅见于早期构想的键按目标版本核实。 [Ares 读取及生命周期](../ares-attach-effect.md) |
| `ai-enablement` · `[AITriggerTypes]`、`[AITriggerTypesEnable]` | 前者每条有 18 个位置字段，比较数据用 64 个十六进制字符表示 32 字节。后者按 ID 控制启用，战役与遭遇战对同一 `id=no` 的解释有别 | 同一 ID 写 `no` 时，核对战役禁用、遭遇战里列出的项仍可进入启用集合。另测无法解析的条件对所属条目的影响。 [记录解析](../../src/rules/team_ai_ini.rs#L583) · [按模式启用](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/rules/team_ai_ini.rs#L660) · [AI 小队选择](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/sim/ai_team_creation.rs#L455) |
| `scenario-records` · `[Events]/[Actions]/[Tags]`、ScriptType 数字行 | 每个事件有类型、参数种类和值，种类 2 再取类型名，YR 读入时反序排列。动作按计数读取，每条 8 个字段。地图动作和小队脚本各有编号空间。剧本保存触发实例、变量、一次性或重复状态及激活顺序 | 由一个触发修改变量，另一个在规定时机读取。加入编号相同的地图动作和小队脚本，存读档后继续。RA2 同形数据按 RA2 分派表核对。 [事件](../../src/map/events.rs#L81) · [动作](../../src/map/actions.rs#L210) · [地图装配](../../src/map/map_file.rs#L408) |
| `animation-damage` · ART `Damage`、Ares `Damage.Delay` | 动画伤害由模拟结算，画帧和调色板由显示端使用。有些字段按对象类型 ID 读取，另一些从 `Rules.Image` 查 ART，字典须注明引用方式 | 对同一动画只改伤害周期，在两端使用不同显示条件，核对扣血帧和读档后的剩余时间。更改 `Image` 时另查按图像名绑定的字段。 [ART 读取](../../src/rules/art_data.rs#L999) · [名称绑定](../../src/rules/art_data.rs#L2228) · [模拟使用处](https://github.com/EncoreAlpha/vera20k/blob/f9e5717eb719e5a6c67ed3774b39a8d9da8d1b8d/src/sim/anim_class.rs#L2829) |
| `tile-burrowing` · 战区 `[TileSetNNNN] AllowBurrowing` | 地块属性参与地下通行判断，当前 YR 读取函数的默认值为真。保留地块及子地块与当前位置之间的对应关系 | 让同一地下移动单位依次进入许可不同的两块地形，核对对应动作。更换地图后重新载入战区属性。 [战区读值](../../src/map/theater.rs#L635) · [地下运动消费者](../../src/sim/movement/burrowing.rs#L393) |

## Ares 版本差异

Ares 3.0 手册的 918 处显式字段声明中，`SuperWeapon` 占 245 处，`TechnoType` 165 处，`BuildingType` 96 处，`Warhead` 64 处。一个功能可能跨越多类标签，例如超武发射需要同时核对选取目标、AI 自动施放、侧栏和资金。KratosPP 的来源、宿主、叠层及弹体设计可供部件接口参考。Ares 兼容性以对应发布版本的资料、读取函数及游戏行为为准。 [Ares 逐项索引](ini-preset-wrapping/ares-index.json) · [KratosPP 与各预设的边界](composable-gameplay-design.md)

**节、注册表与参数化键。** `[WeaponTypes]` 先于类型读取注册武器。`[ArmorTypes]` 的左值是作者命名的护甲。`Versus.<name>`、`Prerequisite.List<N>` 在索引中可能只有具体示例，字典收录通式前须补充适用对象、大小写、编号边界与查找失败的行为。`CanPassiveAquire.Guard` 保留历史拼写，中文名称可用于检索。 [Ares 预注册](https://ares-developers.github.io/Ares-docs/new/weapons/weapontypes.html) · [实际拼写读点](../../src/rules/ares_techno_type.rs#L1084)

**事件、动作和小队脚本。** 手册列出 42 项指令。地图事件编号为 62–88，地图动作为 146–149，另有恢复的 Firestorm 动作 92、93。小队脚本 53–55 扩展已有编号，65–70 为新增编号。每项记录参数、目标选择、动作返回值、脚本推进和存档。同号的不同记录族分别登记。 [事件](https://ares-developers.github.io/Ares-docs/new/triggerevents.html) · [动作](https://ares-developers.github.io/Ares-docs/new/triggeractions.html) · [脚本](https://ares-developers.github.io/Ares-docs/new/scripts.html) · [Firestorm 墙](https://ares-developers.github.io/Ares-docs/restored/firestormwall.html)

**未新增标签的行为修订。** 版本包另行记录产品、激活条件、原生调用位置、替代行为和原逻辑恢复位置。每项可关联零个或多个键，涵盖无新键的修复、旧键语义变化、p1 修正和恢复的 TS 行为。手册声明、可达分派和已确认使用的字段分别登记。`AttachEffect.Damage` 等早期设想须核对目标版本的实际读取函数。 [Ares 已知读取范围](../ares-attach-effect.md) · [3.0p1 修改项](https://launchpad.net/ares/3.0/3.0p1)

**嵌套 include 的待核差异。** Ares 手册记载 `[#include]` 按深度优先读取。VERA 对 Ares 函数 `10064660` 的记录则指出各层共用条目游标。可用三个文件区分两种读法，根依次列 A、B，A 用新键列 C，B、C 为同一属性赋不同值。`A,C,B` 和 `A,B,C` 将产生不同的最终值。取得对应 Ares 版本的实际结果后，记录版本、原文、读序和最终值。 [官方 include 说明](https://ares-developers.github.io/Ares-docs/new/misc/include.html) · [当前实现及残差](../ini-directives.md#precedence-and-boundaries)

## 导入与导出

| 操作 | 工具的预期输出 | 验收条件 |
|---|---|---|
| 打开旧 MOD 或第三方地图 | 选择产品和版本，逐项标注文件、行、推断的节类型、覆盖层、兼容状态。遇到未知地图指令时定位原记录和受影响的任务目标 | 作者可分别定位确定生效、本模式跳过、旧版忽略、仍待验证的项目 |
| 原样导回同一版本 | 工作副本保留原文和压缩块，包括旧版忽略项、未知节、注释、重复节及先后顺序 | 未编辑的输入和输出逐字节一致，游戏原始资产由用户的安装提供 |
| 修改可以回写的旧标签 | 只修改对应源字段，或按作者选择写入 MOD 或地图覆盖层。标出受影响的派生值、引用编号和剧本 | 重新导入后，规则定义、编号和相应玩法快照符合预期。未编辑的地图块仍能原样导出 |
| 导出到其他产品、方言或较早版本 | 按字段或记录标记“可直接表达”“可用等价组合表达”“仅可保留为 fork 数据”“待核”。报告含文件位置、受影响行为及可导出的其余部分 | 独立弧高或动态 Flag 条件超出目标格式时，报告应定位单位、地图及具体字段。仅在作者明确选择近似导出时采用近似值，并列出失去的参数 |

现有 `IniFile` 为游戏提供查值接口，地图导出器已有特定的输出范围。为满足逐字节回写，编辑工程须保留原文和压缩块，并维护修改位置。进入对局的是一份编译后的游戏定义。无效值先按旧版本的读取规则处理，无法还原时保留原文并在报告中指出位置。是否允许开局取决于该局所需的玩法及所选版本能否执行。 [INI 查值实现](../../src/rules/ini_parser.rs) · [地图导出的现有范围](../../src/map/map_export.rs) · [地图存储约束](../map-format.md)

迁移同一项能力时，可用相同规则和玩家命令分别运行现有实现与新实现，离线比较有效规则和逐帧结果。正式对局中每项能力只有一个执行入口。以 Ares AE 为例，待 `Behavior` 覆盖自带效果、命中挂载、刷新、暂停、存档与清理后，统一切换该版标签的运行入口，移除旧实例的创建和更新。导入器继续识别 `AttachEffect.*`，仍依赖旧实现的能力会列入报告。 [现有 AE 状态与清理](../ares-attach-effect.md) · [原设计的迁移条件](composable-gameplay-design.md)

## 分版本验收

每个版本按实际读取函数和游戏内使用位置建立台账。除三份来源索引外，还需核对原版间接或专用读取函数、VERA 现有读取逻辑、目标 Ares 版本的 hook 和发布说明、地图与小队脚本分派表，并收录第三方 MOD 暴露的新写法。字符串搜索未命中时还要核查拼接键和间接分派。文件选择、节名、注册次序、位置字段、数字指令、无新标签的修订、忽略项与不可达分支分别记录。经核实可以共享读法的产品，仍各自保存调用位置及生效条件。

| 阶段 | 验收范围 | 通过条件 |
|---|---|---|
| YR 遭遇战 | 一门零售武器，从模式或地图覆盖、类型注册、射程、飞行到命中、伤害和存档。附原文说明、向旧版导出的结果和可替换的运动模板 | 同图同输入对照读值、派生速度、首次开火与命中帧。未编辑回写须逐字节相同，只改一项参数时可定位受影响的行为 |
| YR 战役 | 一关能正常结束的任务，覆盖任务选择、地图头、对象放置、脚本号、变量、触发器、AI 小队、胜负和下一关入口 | 直接开局和中途读档均能按同一条件过关，包括地图覆盖和额外规则读取。随后逐关、逐模式扩大验收 |
| RA2 | 用 `rules.ini`、`art.ini`、`ai.ini` 与 RA2 地图，对照 `game.exe` 的文件顺序、目录、同名键和指令分派。验证同一武器与一关任务 | 分列可与 YR 共用的项、只有 RA2 证据的项及待核项。两版分别对照规则序号、地图和战役结果 |
| Ares | 从旧语法和类型注册起，逐项覆盖护甲、前置、AE、超武、AI 和地图指令。每项记录手册依据、版本改动、可重现的场景和导出结果 | 用 YR 1.001 加相应版本验证 3.0 及 3.0p1，再用纯 YR 检查版本差异。另用 RA2 自定义 MOD 单独加载已接入的减速 AE，复核命中、到期和读档 |
| 其余文件与第三方导入 | 补齐 ART、战区、地图块、随机地图、媒体、UI 和设置，逐项核实待定节类型、手册以外的旧入口及实际 MOD 差异 | 随每版发布已支持、可保留、确认忽略或不可达、待验证及已知差异清单，列明触发条件和实际影响。核心玩法另在 20,000 单位、30 个 House 下检查锁步及耗时 |

`asset ini-roots`、`ini-expand`、`ini-get` 和 `ini-type` 可查询配置来源与生效值。`ini-get` 的读值依据限于其支持的 YR Rules 和地图组合或固定 ART。`KeyReads` 表示哪些 `(节, 键)` 成功读到，默认值、读取顺序、来源层及游戏内使用位置仍要分别核实。场景验收采用该版本可达的原生指令和对应零售数据，涉及数值、随机数或可见差异时补相应运行记录。逐键、逐关等价与大规模性能均须在实现后验证。 [INI 查询工具](../../tools/ini_lookup.md) · [规则来源与工具范围](../ini-roots.md) · [各预设的兼容范围](composable-gameplay-roadmap.md#各预设怎样扩大兼容范围)

## 附录来源索引

以下索引取自 RA2、YR 零售档案、附加地图包、Ares 官方发布手册，以及与 VERA `f9e5717eb` 的源码对照。每份索引保留观察范围及样本位置，供核实具体标签。

| 来源 | 覆盖范围 | 可查内容 |
|---|---|---|
| RA2 零售档案 | 31 个 INI、1 个地图目录 PKT、127 张具名地图，共 159 份 | 键所在文件、节和物理行 |
| YR 零售档案 | 31 个 INI、1 个 PKT、184 张具名地图，共 216 份。`rulesmd.ini` 和 `soundmd.ini` 使用 `expandmd01.mix` 的覆盖版本 | 同名键在两款产品中的分布，以及模式和地图覆盖 |
| 安装目录中的附加地图包 | 40 个 MMX、13 个 YRO，各取一张地图和一份 PKT，共 106 份 | 附加地图使用的标签、剧本记录及来源包 |
| RA2 `game.exe`、YR `gamemd.exe` | 静态反查整数、布尔、浮点、字符串、射程、速度六类读取函数。RA2 有 1,852 个带字面键名的直接读取位置、1,696 种拼法，YR 分别为 2,049 个和 1,883 种。两者合计 2,000 种不同拼法 | 原版尝试按何种类型读取某键，以及哪些读取位置还需查证 |
| Ares 3.0 官方手册 | 203 页含显式字段声明，共 918 处。按原作用域及键名去重，计 827 对。另收 42 项事件、动作或小队脚本号 | 旧写法的适用范围、手册页和行号 |

零售及附加包共 481 份来源，含 364 张地图。按文件用途、候选节类型、区分大小写的左值或记录形式归类，共 4,129 项，其中 4,048 项是对应上下文中的字面左值。数字指令按记录族与编号归类，共 182 项，其中地图动作 92 项、地图事件 46 项、小队脚本 44 项。`[InfantryTypes]` 的多行成员归为同一种表项，一行 `[Actions]` 却可能含多条动作。零售未配置的开关、固定键和作者命名的实例均需要读取函数补证。 [零售逐项索引](ini-preset-wrapping/retail-index.json) · [Ares 逐项索引](ini-preset-wrapping/ares-index.json) · [零售未启用键的现有调查](../dormant-stock-rules.md)

### 零售文件索引

`files` 给出产品来源、逻辑文件名、用途、归档路径及物理赋值行数。`keys` 的四个样本列依次对应 RA2、YR、MMX、YRO。非空记录依次给出出现次数、从 0 开始的文件序号、首次出现的物理节名和从 1 开始的行号。`rules / Body.Weapon / ROF` 可定位武器行，`rules / Easy / ROF` 可定位难度行。

`<entry>` 表示注册表成员，`<id>` 为作者指定的记录 ID，`<chunk>` 为压缩段，`<number>` 为数值槽位。`key.ini` 只登记不透明条目的数量和位置。`reviewRoute` 指向待核的实现模块，`designCase` 对应本文的案例。空值表示该列样本中未观察到，或尚无对应案例。`Body.*` 的节类型从注册表和引用推断，同名多类型保留所有候选。当前分类中有 392 项须先确认上下文、967 项须先定位读取函数，其余分类也须核实语义。

### 原版读取位置索引

每项列出原拼法、各产品的读取函数类别、每种读法的调用点数量和首个地址，最后两列记录该拼法是否实际出现在该产品归档样本中的字面左值。静态扫描只收录可执行段中直接调用六种已定位函数，且在调用前 40 字节的无分支指令内可确定第二参数为数据段文字地址的情形。键名限以字母或下划线开头的短 ASCII 标识。寄存器传名、格式拼接、表驱动、专用读取函数和指令分派需另查。结果也可能包含设置、工具及未执行的分支。 [原生读点索引](ini-preset-wrapping/native-reader-index.json) · [YR 读函数目录](../../src/rules/ini_value.rs#L15)

RA2 有 347 种、YR 有 388 种原版读取位置中的拼法未在各自归档样本的字面左值中出现。`BuildOffAlly` 在两个产品中各有两个布尔读取位置，YR 可追到 `[MultiplayerDialogSettings]` 和借用盟友建造范围的放置判断。`Assaulter` 同时见于两个产品的文件与读取函数，但判断零售步兵是否具有突击能力，还需核对其配置值和游戏内的使用位置。 [场景设置读者](../../src/sim/game_options.rs#L306) · [建造放置判断](../../src/sim/production/production_placement.rs#L848) · [突击步兵的可达链](../dormant-stock-rules.md)

### Ares 手册索引

`fields` 保存原作用域、键名、类型说明、页序号、RST 行号和网页锚点。`SW.AITargeting` 有 18 处声明，须按超武类型选择对应页。`pages` 中的 `.html` 对应发布树下 `_sources/同路径.txt`，行号从 1 开始。`opcodes` 单列编号、记录族及新增、恢复或扩展性质。3.0 手册依据发布树 `84147a4f`。3.0p1 单独记录行为差异，例如整数的 `$` 写法和充放电超武。 [发布手册源码](https://github.com/Ares-Developers/Ares-docs/tree/84147a4f224d106f3b1ab503a4a07bf8fed3d6ab) · [Ares 3.0p1 发布说明](https://launchpad.net/ares/3.0/3.0p1)

### 在本地查询标签

在仓库根目录运行以下 Python 示例，可列出 RA2 规则样本中 `ROF` 的节类型、文件名、首个节名、行号及出现次数。移除 `domain` 筛选条件即可同时检索地图和模式。

```python
import json
from pathlib import Path

folder = Path("docs/zh-CN/ini-preset-wrapping")
index = json.loads((folder / "retail-index.json").read_text(encoding="utf-8"))
for row in index["keys"]:
    item = dict(zip(index["keyColumns"], row))
    if item["domain"] == "rules" and item["keyOrShape"] == "ROF" and item["ra2Archive"]:
        count, file_id, section, line = item["ra2Archive"]
        print(item["context"], index["files"][file_id][1], section, line, count)
```

### 样本选择与计数方法

每个逻辑文件按 MIX 文件名目录和已识别的归档取样，若补丁覆盖同名文件则记录实际归档。逐物理行剥离 CR、分号注释和 NUL 后的内容，仅统计有节名、非空左值及非空右值的赋值，保留重复行与长压缩行。依据各自的注册表和引用为节分类，再按记录结构提取具备完整参数的数字指令。附加包以 RA2 或 YR 名称表辅助识别。产品列表示文件来源，例如 `tmcj4f.ini` 位于 RA2 档案，YR 的特定模式也会选取它。用户设置、未具名归档项、未随盘发货的 MOD 及仅能在二进制中发现的可读键需另行取证。Ares 侧共枚举发布树中 404 份 RST，保留显式声明，剔除纯交叉引用和具体示例。以散文描述的节、无新键的修订和数字操作码另行核对。 [场景规则读取顺序](../../src/rules/mod.rs) · [Ares 手册](https://ares-developers.github.io/Ares-docs/)
