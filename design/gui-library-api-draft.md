# wgpu-gui 库 API 草案（保留式节点树）

范围：覆盖 VERA 当前前端菜单（主菜单、单人、战役、电影与制作人员、选图、遭遇战、选项、各类分数页、模组页、WOL/LAN/多人设置、暂停菜单、提示与消息）的全部用法。
不包含：游戏内 HUD（侧边栏、field_layout、added_parts、match_layout）、gadget 系统（仅游戏内使用）。

证据格式：`路径:行号` 指向源码；`[调研]` 表示来自子代理报告，我未逐行复核。

---

## 0. 设计原则

1. **保留式树**：控件是持久节点，有句柄、属性、子节点。菜单结构相对静态，状态要跨帧保留（按下、选中、滚动位置、编辑缓冲）。
2. **逻辑坐标 + 统一缩放**：内部用逻辑空间 800x600（DLU 换算后），窗口按 `min(w/800, h/600)` 统一缩放，非整数。`ui/shell/geom.rs:186-261`。
3. **库不读游戏文件**：字符串、图片、字体、布局数据都由游戏侧通过 trait 或数据结构注入。`ui/` 不能依赖 `render/` 和 `assets/`（`architecture_guards.rs:27-36` LAYER_RULES）。
4. **事件是数据，动作是枚举**：库返回事件队列，控件 ID 为 `u16`，游戏侧把它映射到动作枚举。`ui/shell/controller.rs:182`，`app/shell_main_menu.rs:1587-1620`。
5. **绘制顺序由树遍历决定**：不依赖深度缓冲。深度常量在源码中注释自相矛盾，`[调研]` 未核实透明通道的深度测试。

---

## 1. 核心类型

```rust
pub struct Gui { /* 节点存储、对话框栈、焦点/捕获状态、动画时钟 */ }

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeId { index: u32, generation: u32 }   // 代际句柄，防止悬空引用

pub struct Rect { pub x: i32, pub y: i32, pub w: i32, pub h: i32 }  // 逻辑单位
```

依据：控件通过 `u16` 控件 ID 与应用层通信（`ui/shell/controller.rs:182`），对话框 ID 为硬编码数值（`0xE2`、`0x100`、`0x120`、`0x121`，`app/shell_main_menu.rs:2378-2393`）。

---

## 2. 视口与缩放

```rust
pub struct Viewport {
    pub window: (u32, u32),
    pub mode: ScaleMode,          // Fit | Retail
    pub align: Align,             // Center | RightAnchored
}
impl Gui {
    pub fn set_viewport(&mut self, v: Viewport);
    pub fn logical_to_window(&self, r: Rect) -> RectPx;   // 点击坐标反变换
    pub fn window_to_logical(&self, p: (f32, f32)) -> (f32, f32);
}
```

依据：
- `ui/shell/geom.rs:198-208` ShellScale；`:226-240` 居中或右对齐；`:256-261` 统一因子；`:245-253` 800x600 与 640x480 为 RETAIL 恒等。
- `app/frontend/shell_layout.rs:26-37`, `:100-102` 其他窗口尺寸缩放。
- 留白使用黑色清屏 `render/shell_surface_present.rs:269`。
- 原版 DLU 换算：`ui/shell/geom.rs:40-61`（DLU_BASE_X=6, DLU_BASE_Y=13）。

---

## 3. 布局（锚点）

```rust
pub enum Anchor {
    Dlu(DluRect),                 // 固定 DLU 矩形，换算到逻辑单位
    Logical(Rect),                // 固定逻辑矩形（如 MAP_TEXT、版本号等计算位置）
    SnapStack { first_y_dlu: i32, step_dlu: i32 },   // 0xE2 的 125/152/179/206/233
    BottomRow { y_dlu: i32 },     // 0xE2 的退出按钮 y=330
    RightAnchor { inset: i32 },   // ui/shell/descriptor.rs 的 RightAnchor
    RightAnchorRuntimeAdjust,     // 标题文字，运行时根据宽度调整
    Inset { parent: NodeId, inset: i32 },   // HEADING/MONITOR/MAP_TEXT 的 +1w/+1h 与 inset 37 等
    Stack { prev: NodeId, gap_dlu: i32 },    // 单人界面 122/149/176 之类的堆叠
}
```

依据：
- `ui/shell/descriptor.rs:38-60`（OwnerDrawButtonSnap、OwnerDrawButtonBottomRow、RightAnchor、RightAnchorRuntimeAdjust）。
- `ui/shell/descriptor.rs:78-130`（HEADING_ANCHOR、LOW_HEADING_ANCHOR、MAP_TEXT_ANCHOR inset 14、MONITOR_ANCHOR inset 37）。
- `ui/main_menu_shell/layout.rs:147-212`（0xE2 全部控件的 DLU 坐标）。
- `ui/single_player_shell/layout.rs:10-43`（122/149/176 堆叠）。
- `ui/shell/menu_page.rs:109`、`:212`（compute_layout）。
- 注意：菜单布局中**没有 `Hold` 锚点**（`[调研]` 确认）。RFC 中的 `Hold`（九宫格）只用于游戏内 HUD。库应同时支持两者，见第 11 节。

计算性 rect（版本号、提示行）：`ui/main_menu_shell/layout.rs:89-121`，应作为 `Anchor::Logical` 由游戏侧计算后传入。

---

## 4. 节点种类

```rust
pub enum NodeKind {
    Panel(PanelStyle),          // 背景面板
    Button(ButtonStyle),
    Static(StaticContent),      // 文字、图片、动画图片、图标
    Checkbox { checked: bool },
    Radio  { group: u16, selected: bool },  // ControlKind 中有 Radio，报告未见使用，待确认
    Combo(ComboSpec),
    List(ListSpec),
    Trackbar(TrackbarSpec),
    Edit(EditSpec),
    ScrollBar(ScrollSpec),
    Custom(CustomPaintId),      // 游戏侧绘制回调
    Video(VideoSpec),
    MessageBox(ModalKind),
    Tooltip(TipSpec),
    ChatLog(ChatLogSpec),
}
```

`ControlKind` 已有对应枚举：`ui/shell/descriptor.rs:22-32`（Button、Static、Checkbox、Radio、Combo、Listbox、Trackbar、Edit、ScrollBar）。库的 `NodeKind` 应以它为基础扩展。

### 4.1 Button

```rust
pub enum ButtonStyle {
    OwnerDraw { caption: CsfKey, tooltip: Option<CsfKey>, frames: ButtonFrames, snap: Anchor },
    Modal { frames: ModalFrames },           // MNBTTN 三帧，disabled > pressed > up
    Image { frames: AnimFrames, ms_per_frame: u32, voice_after_ms: Option<u32>, hit: HitShape },
}
pub struct ButtonFrames { pub default: u16, pub hover: Option<u16>, pub pressed: u16 }
```

依据：
- 通用按钮规格：`ui/shell/menu_page.rs:27-42`（id、dlu_top、caption 键、tooltip 键、result）。
- SDBTNANM 三帧（默认 2、悬停 3、按下 4）：`render/main_menu_shell_chrome.rs:190-192`。
- 帧选择：`render/shell_paint.rs:557-595`（按下优先；悬停闪烁仅当 `hover_flash` 开启，0xE2 与 0x100 关闭，`:80`；`flash_phase` 1 秒交替，`ui/shell/button.rs:141`）。
- 模态按钮 MNBTTN 三帧：`render/shell_paint.rs:725-733`，`skirmish_shell_chrome.rs:186-188`。
- 按下时文字偏移 dx=2、dy=5，平时 dy=1：`app/frontend/shell_pass.rs:25-33`。
- 按下时图片下沉：`render/shell_paint.rs:600-640`（`art_sink_y`，当前策略为 0）。禁用时保持全彩，仅 type-0 应用黑色混合（`:596-599`）。
- 战役阵营图标：自定义图片按钮，悬停播放逐帧动画 100 ms/帧，500 ms 后排队语音，按下后释放才选中（`campaign_shell/mod.rs:12-15`、`:41`、`:220-260`、`:278-280`、`:408-458`）。命中用矩形（`:154`）。
- 按钮捕获：拖出按钮后仍保持捕获，但按下态不保留（`ui/shell/button.rs:1-31`）。
- 禁用时仍悬停：`ui/shell/controller.rs:162-227`（禁用抑制按下，悬停保留）。

### 4.2 Static（文字、图片、动画）

```rust
pub enum StaticContent {
    Text { key: CsfKey, flags: TextFlags, color: TextColor, reveal: Option<RevealSpec> },
    Picture { art: ArtRef },
    AnimatedPicture { art: ArtRef, frames: AnimFrames },   // 监视器 0x71C、艺术区 0x72B
}
pub struct RevealSpec { pub kind: RevealKind, pub step_ms: u32, pub step: u32, pub range: u32 }
```

依据：
- 文字标志位 `render/shell_text.rs:17-34`（H_CENTER 0x01、H_RIGHT 0x02、V_CENTER 0x04、PANEL_FACE 0x08；BUTTON_CAPTION 0x0D、PANEL_TITLE 0x09）。
- 标题打字机效果：`ui/shell/static_reveal.rs:13-40`（标题 30 ms/步 1/范围 8；状态行 15 ms/步 3/范围 16）。
- 逐单位颜色：`render/shell_text_reveal.rs:22-48`。
- 警告监视器 SDWRNANM 91 帧：`ui/shell/warning_monitor.rs:6`、`:28`、`:118`。
- 战役分数的艺术区 0x72B：`app/frontend/campaign_score_shell_render.rs`（100 ms 动画）`[调研]`。
- 静态图片（旗帜等）：`ui/skirmish_shell/state.rs:166-170`（0x6DA–0x6E1）。

### 4.3 Checkbox

依据：`ui/wol_options.rs:88`、`:96`（9 个复选框，固定 DLU 矩形 `:133`）；`ui/skirmish_shell/layout.rs:187-195`、`:932-948`（ShortGame、McvRepacks、CratesAppear、SuperWeapons、BuildOffAlly）；选项页 pitch 15、宽 160（`app/frontend/...` options/shell.rs:258-300 `[调研]`）。

### 4.4 Combo（下拉）

```rust
pub struct ComboSpec { pub items: ItemSource, pub selected: usize, pub popup: PopupSpec, pub disabled_when: Option<Condition> }
```

依据：
- 通用下拉：`ui/shell/combobox.rs:20`（Scroll 变体）、`:136`、`:186`。
- 遭遇战每行的 AI、Side、Color、Start、Team 下拉：`ui/skirmish_shell/state.rs:126-133`；FFA 时 Team 禁用：`state/tests.rs:1663-1709`。
- 战役/WOL 的国家下拉：`ui/wol_lobby/player_card.rs:59`、`ui/wol_my_info.rs:167`、`:252`。
- 翻译语言下拉：`ui/wol_options.rs:219`、`:303`、`:208`。
- 选项页列表（Zoom、Display、Sidebar、HealthBars、Language 等）：`ui/main_menu_dialogs/options/shell.rs` `[调研]`，Zoom 列表无滚动（`:980`）。

### 4.5 List（列表）

```rust
pub struct ListSpec { pub row_height: u32, pub rows: RowSource, pub select: SelectMode, pub double_click: bool, pub scroll: ScrollPolicy }
pub enum ScrollPolicy { Scrollbar, Wheel { rows_per_notch: u32 }, AutoFollowTail }
```

依据：
- 行高 19、箭头 22、滚动条 20：`ui/shell/list.rs:15-20`。
- 双击：`ui/shell/list.rs:510`（`is_double_click`，使用主机双击时间与矩形，`ui/shell/saved_file_input.rs:28/44`，Windows `GetDoubleClickTime`）。调用方：`app/shell_lan.rs:1116`、`:1143`；`app/shell_skirmish.rs:924`；`app/shell_movies_credits.rs:159`；`app/shell_wol/buddy.rs:547`；`app/shell_wol/lobby.rs:4690`；`ui/shell/saved_file_input.rs:135`。
- 电影列表：`ui/movies_credits_shell/movie_list.rs:18-72`、`:144-175`（按进度决定可见行），双击窗口 `:191`。
- 模组列表：`ui/mods_shell.rs:76-88`，`app/shell_mods.rs:150`（滚轮每格 3 行）。
- 滚轮路由：`app/handler.rs:1916-1990`；无滚轮的列表（存档、LAN、选图）仅能用滚动条（`app/shell_skirmish.rs:1893-1899`，`app/shell_saved_games.rs:323`，`app/shell_load_saved_game.rs:114-121`）。
- LAN 聊天自动跟随到底部，除非用户手动滚动：`ui/lan_shell.rs:325`、`:341`、`:387-394`。

### 4.6 Trackbar（滑条）

```rust
pub struct TrackbarSpec { pub range: (i32, i32), pub value: i32, pub mapping: Mapping, pub thumb_w: u32, pub plaque_reserve: u32 }
pub enum Mapping { Linear, Nonlinear(&'static [i32]) }
```

依据：
- 滑条常量 `ui/shell/trackbar.rs:12-14`（PLAQUE_RESERVE 50、THUMB_W 12）。
- 非线性速度/信用：`ui/skirmish_shell/state/trackbars.rs:92-117`（GameSpeed 0x529、Credits 0x511）。
- 单位数 0x50C：`ui/skirmish_shell/layout.rs:202-206`、`:869`。
- 战役难度 0..2：`ui/campaign_shell/mod.rs:34-37`、`:298-305`，数值静态文字 0x670 `:321-322`。
- 房间人数滑条（拖动）：`ui/room_limits.rs:53`、`:65`、`:88`、`:95`；多人设置滑条：`ui/multiplayer_setup.rs:645`、`:671`、`:683`。
- 选项页滑条：轨道 150x13，pitch 37（`[调研]`），预览音效 `ScorePreview`/`SoundPreview`/`VoicePreview`（`ui/main_menu_dialogs/options.rs:942-1000`）。

### 4.7 Edit（单行文本输入）

```rust
pub struct EditSpec {
    pub max: TextLimit,              // Bytes(n) | Utf16Units(n) | ApplyChatRule
    pub admits: fn(char) -> bool,    // 字符过滤
    pub ime: ImeMode,                // None | Chat（仅聊天/选择框）
    pub caret: bool,
    pub tab_order: Option<u16>,
}
```

依据与实例：
- 遭遇战玩家名：19 字节，`ui/skirmish_shell/state/player_name.rs:22-24`，`insert_text` `:140-160`（ACP 往返，丢弃控制字符）；键处理 `app/shell_skirmish.rs:1540-1575`；光标边距 5 px。
- 存档/种子描述：79 个 UTF-16 单位，`ui/skirmish_shell/state/saved_seed_browser.rs:9`、`:133-155`；键处理 `ui/shell/saved_file_input.rs:48-98`（Tab 与 Escape 忽略；Enter 执行动作；方向键/Home/End 编辑）。
- LAN 名称与聊天：`ui/lan_shell.rs:695-745`；聊天上限 120 字节，宽字符时截断，最少 40（`:766-775`）；键路径 `app/shell_lan.rs:915-955`。
- WOL 账户字段：`ui/wol_account.rs:700-718`、`FieldKind::admits` `:136`、`limit` `:127-134`；名称字符 alnum、_、-（`:62`）。
- WOL 聊天上限 2048 字节：`app/shell_wol/live_service.rs:62`、`:589`。
- 多人设置/房间聊天：`app/input/multiplayer_setup.rs:228-254`；Backspace/Delete/Home/End/Enter 映射 `:242-248`。
- 存档文件名字符：`app/frontend/skirmish_session/saved_setups.rs:43`。
- 选图搜索框：`app/handler.rs:1340` → `app/shell_skirmish.rs:1245`。
- IME 仅用于聊天和选择框；玩家名、登录、好友、存档描述只接受普通按键文本（`app/input/ime.rs` 头注释）。`[调研]`

### 4.8 ScrollBar

```rust
pub struct ScrollSpec { pub repeat_delay_ms: u32, pub repeat_interval_ms: u32, pub track_jump: bool }
```

依据：`ui/shell/list.rs:72-135`（按下 500 ms 后以 25 ms 间隔重复，轨道按下跳到指针，滑块拖动跟随指针）。

### 4.9 Custom（游戏侧绘制）

依据：
- 遭遇战地图预览 0x468：`ui/skirmish_shell/layout.rs:800-860` 区域；`app/frontend/preview_zoom.rs:28-76`（只保存按下/释放状态，无绘制动画）。
- 国旗、颜色样本（combo swatch）、选项滑块的 plaque 与 thumb：`[调研]`。
- 结论：WOL 与 LAN 未发现逐像素自定义绘制（`[调研]`）。

### 4.10 Video（全屏视频）

```rust
pub struct VideoSpec { pub file: MovieRef, pub fit: FitMode /* RetailTopLeftCrop | Contain */, pub filter: Filter /* Nearest | Linear */, pub subtitle_band: Option<Rect> }
```

依据：
- 解码与上传：`render/bink_movie.rs`（step `~:98-135`，帧数上限 4 追赶，`~:111`）。
- 全屏四边形 MOVIE_DEPTH：`app/frontend/fullscreen_movie.rs:153-168`；主菜单 `main_menu_shell_render.rs:344-355`。
- 过滤：主菜单 Nearest（`main_menu_shell_render.rs:402`），电影与制作人员 Linear（`app/shell_movies_credits.rs:288`）。
- 字幕条裁剪：`fullscreen_movie.rs:180-230`。
- 适配：`fullscreen_movie.rs:70-102`（`min(w/mw, h/mh)` 取整；RETAIL 为左上裁剪居中）。
- 主菜单背景电影 ra2ts_s/l.bik：`ui/main_menu_shell/layout.rs:22-35`、`:220-240`。

### 4.11 MessageBox（模态）

```rust
pub enum ModalKind { BodyOk /*0xCE*/, Confirm /*0x120*/, ThreeButton /*0x121*/, Wait { timeout_ms: Option<u32>, auto_show: bool } }
```

依据：
- `ui/shell/modal.rs:33-51`（BodyOk 0xCE、Confirm 0x120、ThreeButton 0x121）；控件 ID `:20-28`（0x05B0、0x05AE、0x0002、0x05AF、0x05C0）；尺寸 451x326 `:218-219`。
- 等待框（定时、自动显示）：`ui/wol_shell.rs:560`、`:578`（`poll()`）。
- 消息框（WOL）：`ui/wol_shell.rs:512`、`:518`、`:539`。
- 退出确认：`ui/main_menu_dialogs.rs:35-43`、`:86-93`。
- Esc 菜单三按钮（选项 / 取消 / 退出游戏）：`ui/main_menu_dialogs.rs:95-103`；`app/shell_menu.rs`、`ui/shell/...` `[调研]`。
- 暂停菜单状态机：`ui/pause_menu.rs:68`（Closed、Menu、Surrender、AbortConfirm、Sound、Keyboard），放弃确认 `:197`。
- 默认键：Enter（IDOK）与 Escape（IDCANCEL）都返回结果 1（`app/handler.rs:76-98`，Alt+Enter 除外 `:89`）；路由 `app/shell_main_menu.rs:2399`。
- 一次只能有一个 shell 消息框（`app/state` 的 Option，`raise_shell_message_box` `app/shell_main_menu.rs:2358`）。

### 4.12 Tooltip 与状态行

```rust
pub struct TipSpec { pub delay_ms: u32 /* 默认 1000 */, pub source: TipSource /* 节点 tooltip 键 */ }
```

依据：
- `ui/tooltips.rs:48`（TipRegion）、`:66`（TooltipService）、`:86`（delay，`TOOLTIP_DELAY_MS`）。
- 测试：1000 ms 静止后显示（`:281`）；动态文字刷新（`:311`）；零延迟模式（`:366`）。
- 主菜单状态行 0x695：显示悬停按钮的 CSF tooltip（`ui/main_menu_shell/state.rs:124-135`、`:80-88`）。
- 标题 0x694、状态行 0x695 等通用页面元素：`ui/shell/menu_page.rs:121-123`、`:159`。

### 4.13 ChatLog 与消息行

依据：
- 聊天行模型：`ui/wol_chat.rs:34`、`:101`；过滤器枚举 `:81`、`:21`、`:72`。
- 消息列表与揭示动画：`ui/messages.rs:225`（MessageList）、`:92`（MessageReveal）、`:117`（reveal_count）；通知槽 `:57`、`:69`。
- LAN 聊天行：`ui/lan_shell.rs:106`。
- 战役/协作分数页的聊天编辑 0x540 与聊天日志 0x53F：`ui/...coop_score_shell.rs:24-34`、`:481-485`。

---

## 5. 事件模型

```rust
pub enum InputEvent {
    PointerDown { pos: (f32, f32), button: MouseButton },   // 菜单只处理左键
    PointerUp   { pos: (f32, f32), button: MouseButton },
    PointerMove { pos: (f32, f32) },
    Wheel { lines: f32 },                                    // PixelDelta/30，限制 ±3
    Key { key: ShellKey, repeat: bool },                     // Enter | Escape | Tab | 文本键
    Text(String),                                            // 普通字符
    Ime(ImeEvent),                                           // Preedit | Commit | Disabled，仅 ImeMode::Chat
}

pub enum UiEvent {
    Activated { node: NodeId, control_id: u16 },    // 按下与释放命中同一控件
    Hovered { node: NodeId, entered: bool },
    Changed { node: NodeId, value: ControlValue },  // 复选框、滑条、下拉、编辑
    ListSelected { node: NodeId, row: usize },
    ListDoubleClicked { node: NodeId, row: usize },
    Dismissed { node: NodeId },                     // 点击对话框外或 Esc
    TextChanged { node: NodeId },
}
impl Gui {
    pub fn handle(&mut self, e: InputEvent) -> Option<UiEvent>;
    pub fn drain_events(&mut self) -> impl Iterator<Item = UiEvent>;
}
```

依据与规则：
- 键盘分发入口：`app/handler.rs:1195`；鼠标左键 `app/handler.rs:1730`、`:1792`（中键和右键在菜单被丢弃）；光标 `:1627`；滚轮 `:1916`；IME `:1999` → `app/input/ime.rs:171-191`。
- 按下记录被按控件；只有在同一控件上释放才触发；拖出则取消：`ui/shell/controller.rs:173`（on_pointer_down）、`:182`（on_pointer_up）。禁用控件按下被抑制，释放不触发，悬停保留（`:162-227`）。
- 命中测试：按输入顺序取第一个命中，无 z-order、无最小面积规则（`ui/shell/controller.rs:234` hit_any）。游戏内 gadget 的命中测试（`ui/gadget/tick.rs:83`）不用于菜单。
- 只有栈顶对话框接收指针和键盘事件（`ui/shell/controller.rs:173`、`:182`、`:203` 使用 `stack.last_mut`）。
- `ShellKey` 只有 Tab、Enter、Escape（`ui/shell/controller.rs:28`）。控制器只消费 Enter 和 Escape，且仅在键盘路由非空时（`:216`、`:156`）。Tab 不被控制器消费，只在文本输入中有效。
- 主菜单对话框键（Escape 关闭、忽略打开时按住的键）：`app/handler.rs:52-110`、`:64-72`。
- 按键重复：Esc 菜单 `app/shell_menu.rs:32`、`app/handler.rs:61`；消息框 Enter/Esc 在重复时仍转发（`:81-82`）。
- 模态时的点击吞噬：`app/handler.rs:1860`，主菜单按钮（`:1903`）被阻断。
- 网络等待页键盘直接路由到分发，绕过键路径（`app/handler.rs:1291-1297`）。
- 输入层优先级：`app/input/precedence.rs:236-247`（shell 错误卡 → 胜利等待 → 过渡 → 页面），页面层 `:158-178`。
- 双击：软件合成，见 4.5。
- 菜单未使用：焦点环、方向键导航（仅文本输入内使用方向键，`app/shell_lan.rs:939-940`）、助记键（代码中无）、手柄（未找到）`[调研]`。

---

## 6. 对话框栈

```rust
impl Gui {
    pub fn push_dialog(&mut self, id: DialogId, spec: DialogSpec) -> NodeId;
    pub fn pop_dialog(&mut self, id: DialogId) -> Option<UiEvent>;
    pub fn top_dialog(&self) -> Option<DialogId>;
    pub fn is_blocking(&self) -> bool;    // 栈非空时阻断底层点击
}
pub struct DialogSpec { pub id: DialogId, pub root: NodeSpec, pub slide: Option<SlideSpec>, pub esc_closes: bool }
```

依据：
- LIFO 栈：`ui/shell/controller.rs:66-72`（字段）、`:77`（push）、`:86`（pop）。
- 底层对话框仍保留自己的按下与悬停状态用于绘制（`pressed_in` `:129`、`hovered_in` `:134`）。
- 遭遇战的覆盖顺序：验证框 → 选图提示 → 存档种子浏览器（吞噬所有键，`app/handler.rs:1371-1387`）→ 游戏选项框（`:1390-1404`）。验证框和选图提示消费键（`:1339-1368`）。
- 对话框 ID 列表：`ui/shell/descriptor.rs:1-30`（0x100、0x102、0x6B、0xBC、0xBD、0xCE、0x120 等）。

---

## 7. 滑入/滑出与动画

```rust
pub struct SlideSpec { pub groups: &'static [SlideGroup], pub tick_ms: u32 /*30*/, pub tail_ticks: u32 /*6*/, pub ramp_steps: u32 /*6*/ }
pub trait Clock { fn now_ms(&self) -> u64; }   // 由游戏侧注入
impl Gui { pub fn update(&mut self, dt_ms: u32) -> Vec<UiEvent>; }
```

依据：
- 滑入：30 ms 一帧，不跳帧（`ui/shell/slide.rs:29`、`:797-807`）；尾部 6 帧、渐变 6 步（`:32-34`）；总帧数 `rows+3+6`（`:519-521`）；帧选择 `:968-995`；分组表 `:42-70`；起始 tick `:529-535`；按钮绘制 `:538-572`。
- 适用对话框：`ui/shell/slide.rs:152-215`、`:430-447`（0xE2、0x100、0x101、0x129、0x102、0x94、0xD5、0x108、0x103、0xBC7 等）。
- 面板：SDTP 进出、SDWRNTMP（`slide.rs:576`）；合成 `render/shell_paint.rs` `~:270-348`、`:355-365`。
- 静态文字揭示：见 4.2。
- 启动画面最少显示 6000 ms（`app/frontend/startup_splash.rs:80`）。
- 退出过渡：`app/frontend/shell_transition.rs:463`、`:507`；战役语音等待 3000 ms（`:422`）。
- 退出级联上限 48000 ms（`app/frontend/quit_cascade.rs:25`）。
- 电影帧时钟（追赶上限 4 帧）：`render/bink_movie.rs ~:111`。

---

## 8. 文字布局

```rust
pub struct TextLayout { pub flags: TextFlags, pub squeeze: SqueezePolicy, pub max_lines: Option<u32> }
pub enum SqueezePolicy { None, FitBox, PanelFace /*单行，宽 110*/ }
```

依据（`render/shell_text.rs`、`render/bit_font.rs`）：
- 标志位：`:17-34`。
- 挤压：`:103-111`，`BitFont::fit_box` 在 `render/bit_font.rs:277-300` 选择第一个不拆词且满足行数的步骤，否则取最紧的并截断；PANEL_FACE 强制单行，宽 `RIGHT_PANEL_FACE_W=110`（`ui/shell/geom.rs:61`）。
- 垂直居中：`(rect_h - measured_h) / 2`，无条件，超高时为负并被裁剪（`:123-125`）。
- 水平：居中或右对齐，RTL 右对齐，超宽行为 0（`:132-146`）。
- 最大高度：第一行总是绘制，之后行在 `line_index * cell_height >= rect.h` 时停止（`:190-200`）。
- 颜色：启用 `#FFFF00`，禁用 `(0x48, 0, 0)`（`render/shell_paint.rs:27-34`）。
- 裁剪：每段文字使用窗口像素的 `ScissorRect`（`render/shell_text.rs:48-72`），在 `app/frontend/shell_pass.rs:223-232` 设置。

---

## 9. 绘制后端（trait）

```rust
pub trait Backend {
    fn sprite(&mut self, atlas: AtlasEntry, dst: RectPx, uv: UvRect, tint: [f32; 3], alpha: f32, clip: Option<ScissorRect>);
    fn glyphs(&mut self, run: &GlyphRun, origin: (f32, f32), clip: Option<ScissorRect>);
    fn fill(&mut self, dst: RectPx, rgba: [f32; 4]);     // 1 像素白贴图 + 着色
    fn video(&mut self, frame: VideoFrameRef, dst: RectPx, filter: Filter, clip: Option<ScissorRect>);
    fn present_scaled(&mut self, src: SurfaceRef, zoom: ZoomGeometry);
    fn cursor(&mut self, atlas: AtlasEntry, pos: (f32, f32), hotspot: (f32, f32), scale: f32);
    fn clear(&mut self, rgba: [f32; 4]);
}
```

依据：
- 统一图元 `SpriteInstance`：`render/batch.rs:49-90`。
- 图集四边形变体：`render/shell_paint.rs` 的 `push_entry_sized` `:109-127`、`push_entry_native` `:130`、`push_entry_scaled` `:145`、`push_entry_grown` `:168`、`push_entry_rect` `:303`、`push_entry_crop` `:189`、`:205`、`push_entry_window_crop` `:222`、`push_clipped_top` `:324`；`push_skirmish_entry` `:847`；模态背景用 UV 裁剪而非挤压（`:770-812`）。
- 纯色与框：`app/frontend/status_screen.rs:372-402`；径向渐变光晕 `:282-296`，悬停 1.35 倍 `:515-517`。
- 文字：`render/bit_font.rs:401` `build_text` 每字形一个四边形；GAME.FNT 原生路径（`:355-365`）与 text_engine 路径分开；缺字形颜色异或 `0x5555`（`:1018`）。
- 管线：`draw_with_buffer_passthrough`（`render/batch.rs:2334`）、`draw_chrome`（`:2348-2368`）、平滑管线 `fs_shell_smooth`（仅在线菜单开启，`skirmish_shell_render.rs:932`、`main_menu_shell_render.rs:946`、`wol_welcome_render.rs:1183`）；采样 Nearest（`render/batch.rs:1844`、`:2274`）。
- 清屏：`app/frontend/shell_pass.rs:130`、`:205`。
- 绘制顺序（按提交顺序）：`app/frontend/main_menu_shell_render.rs ~:808-900`：背景 → 电影 → 面板 → 按钮 → 标签 → 模态图 → 模态文字 → 光标。
- 缩放 blit：`render/scaled_copy.rs`（整数倍块拷贝，分数倍混合），`render/upscale_pass.rs:158-185`、`:296-350`。
- 表面呈现：RGB565 码本 `render/shell_surface_present.rs:221-270`；淡出 32 行 `:20-36`，用于制作人员（`app/frontend/movies_credits_render.rs:630-634`）。
- 软件光标：`app/frontend/shell_pass.rs:52-95`。

---

## 10. 外部资源接口（游戏侧实现）

```rust
pub trait TextSource {
    fn lookup(&self, key: &str) -> Option<&str>;     // CSF 键，大写；缺失回退英文
    fn fill(&self, key: &str, args: &[(&str, &str)]) -> String;   // {name} 替换
    fn count(&self, key: &str, n: i64) -> String;     // CLDR 复数形式 zero/one/two/few/many/other
}
pub trait ArtSource {
    fn resolve(&mut self, name: &str, palette: &str, frame: u16) -> Option<AtlasEntry>;   // SHP + PAL
}
pub trait FontSource {
    fn native(&self) -> Option<&GameFnt>;             // GAME.FNT
    fn fallback(&self) -> &Fonts;                    // 打包的 Noto Sans
}
```

依据：
- 字符串：`i18n/mod.rs:864-873`（`label`）、`:876`（`tr`，缺失返回 `MISSING:'key'`，`:921-923`）、`:886`/`:928`（`fill`）、`:893-909`（`tr_count`，`plural_rules` `:263-270`）；语言选择 `:628-652`；回退 `:819-862`；伪语言 qps-ploc。
- 开发模式显示键名：`i18n/mod.rs:317-345`。
- 美术：SHP/PAL 名称写死在 Rust 中，例如 `render/skirmish_shell_chrome.rs:24-29`（PUDLGBGN.SHP、DIALOGN.PAL 等）、`render/main_menu_shell_chrome.rs:124-155`、`:275-300`（SDBTNANM、SHELL2.PAL 等）。
- 归档：`render/skirmish_shell_chrome.rs:51-58`、`:582`；`assets/asset_manager.rs:441`、`:465`、`:1770`、`:1797`。
- 字体：GAME.FNT `render/bit_font.rs:5-9`、`:37-54`；打包字体 `render/text_engine/fonts.rs:57-87`，`:80` NotoSans-VF.ttf；网页版 `web_fonts.rs`。
- `render/shell_ui_style_assets.rs` 的 `ShellUiAssetResolver`/`ShellUiAssetPair`（定义 `:42-78`）有生产调用方：`app/frontend/main_menu_shell_render.rs:443-447`、`app/frontend/startup_splash.rs:337-350`（已核实）。
- `[VERA]` INI 只用于语言（`i18n/mod.rs:628-643`）、`YROriginalStyle`（`render/shell_ui_style_assets.rs:1-6`）、`Zoom`（`ui/main_menu_dialogs/options.rs:256`）。菜单布局不读 INI。

---

## 11. 布局数据的输入方式

v1 不读文件。游戏侧构造 `DialogSpec`，这基本是现有 `ControlDescriptor` / `DialogDescriptor`（`ui/shell/descriptor.rs`）的泛化。

```rust
pub struct NodeSpec { pub kind: NodeKind, pub anchor: Anchor, pub name: Option<&'static str>, pub control_id: u16, pub children: Vec<NodeSpec>, pub enabled: bool }
```

- 菜单布局是 Rust 常量（`ui/main_menu_shell/layout.rs:147-212` 等），库只需接受这种数据。
- RFC（`docs/rfcs` 分支）的 `veraui.ini` `Hold` 九宫格属于游戏内 HUD，库中以 `Anchor::Hold` 另行支持，不影响菜单。

---

## 12. 覆盖矩阵

| 界面 | 对话框 ID | 主要节点与功能 | 覆盖状态 |
|---|---|---|---|
| 主菜单 | 0xE2 | OwnerDraw 按钮、Snap/BottomRow 锚点、标题打字机、监视器、版本号、状态行、电影背景、滑入 | 完整 `[调研]` |
| 单人 | 0x100 | OwnerDraw 按钮，堆叠，禁用，返回 | 完整 |
| 战役选择 | 0x94 | 图片按钮（动画、语音）、标签、滑条、返回 | 完整 |
| 电影与制作人员 | 0x101 | OwnerDraw、返回；制作人员文字卷动 | 完整（卷动部分待核实） |
| 电影列表 | 0x129 | List（进度门控、双击）、按钮、提示 | 完整 |
| 遭遇战 | 0x102 | 按钮、列表行（下拉、复选框、滑条）、玩家名编辑、地图预览（自定义）、选项框 | 大部分；子对话框采样 |
| 选图 | 0x6B | 列表、搜索编辑、提示模态 | 采样 |
| 随机地图设置 | — | 下拉、滑条、高级区、滚动条 | 采样 |
| 种子浏览器 | — | 列表、名称编辑、滚动条 | 采样 |
| 启动器选项 | 0xD5 | 列、列表、滑条（预览音效）、复选框、按钮、启用规则 | 完整 `[调研]`（列出页面控件） |
| 遭遇战分数 | 0x108 | 文字、按钮、分页、表格 | 完整 `[调研]` |
| 战役分数 | 0x103 | 文字揭示、艺术区动画、按钮 | 完整 `[调研]` |
| 战役结束 | 0xBC7 | 文字、监视器、按钮 | 完整 `[调研]` |
| 协作分数 | 0xBC6 | 文字、列表、聊天编辑、地图预览、按钮 | 完整 `[调研]` |
| 模组页 | 0x7E02 | 列表、详情、按钮 | 完整 |
| Esc 菜单 | — | 三按钮模态、暂停菜单状态机 | 完整 |
| WOL 欢迎、大厅、聊天、选项 | — | 按钮、弹窗、列表、编辑、复选框、下拉 | 大厅、选项完整；登录、账户、好友、我的信息、验证、快速对战、个人页**未逐控件读取** |
| LAN | — | 列表、聊天、编辑、布局 `lan_shell.rs:783-830` | 未读完整布局块 |
| 多人设置 | — | 滚动行、下拉、滑条、拖放、弹窗、聊天 | 布局块 `multiplayer_setup.rs:1295-1354` 未读 |
| 房间人数 | — | 滑条（拖动）、开关图标 | 完整 |
| 提示、消息 | — | Tooltip、消息列表揭示、编辑行 | 完整 |
| 启动画面、退出级联 | — | 全屏图、最少时间、等待上限 | 完整 |

---

## 13. 库不负责的内容

- CSF 解析、语言选择、插件目录 → 游戏侧 `i18n`。
- SHP/PAL/MIX 解析与资源管理 → 游戏侧 `assets`。
- 游戏规则、对局状态、ini 布局、侧边栏和 HUD（`field_layout`、`added_parts`、`match_layout`）→ 游戏侧。
- 多人权限、模组 ABI（RFC）→ 游戏侧。

## 14. 库不需要的功能（菜单未使用）

焦点环、方向键导航、助记键、手柄、z-order 命中测试、深度缓冲、非聊天框的 IME、逐像素自定义绘制（WOL/LAN 未发现）。

---

## 15. 待确认事项

1. 上表标记“采样”或“未读”的界面需要补充逐控件调研，特别是 WOL 登录/账户/好友/验证/快速对战，LAN 布局块，多人设置布局块 1295-1354。
2. `ControlKind::Radio` 是否真的用于菜单（报告未发现使用）。
3. `ShellUiAssetResolver` 是否有生产调用方。
4. 绘制深度测试是否开启（影响能否用深度而非树序）。
5. RFC 的 `Hold` 锚点与菜单的 Snap/DLU 锚点是否都放进库（建议都放）。
6. 库的 `ControlId` 是否保持 `u16`（与现有代码一致）。

---

## 16. 第二轮补充（已核实的结论）

### 16.1 待确认事项的结论
| 问题 | 结论 | 依据 |
|---|---|---|
| `ControlKind::Radio` 是否使用 | 生产代码中未构造、未匹配，仅声明 | `ui/shell/descriptor.rs:26` |
| `ShellUiAssetResolver` 是否有调用方 | 有：主菜单电影、启动画面 | `app/frontend/main_menu_shell_render.rs:443-447`，`app/frontend/startup_splash.rs:337-350` |
| 菜单绘制管线是否做深度测试 | 不做：passthrough 管线 `depth_write_enabled: false`、`depth_compare: Always`；主批次管线才是 LessEqual | `render/batch.rs:1020-1054`，`:1062-1063`，`:1004-1007` |
| 菜单是否处理非左键 | 不处理。右键、中键只用于游戏内 | `ui/` 中无 MouseButton；`app/input/dispatch.rs:198,206,379,964-980` |
| 菜单布局是否有九宫格 `Hold` 锚点 | 没有。`Hold` 只是滑条的按下类型 | `ui/shell/descriptor.rs:38-`（AnchorRule 无 Hold），`ui/shell/trackbar.rs:38` |

### 16.2 LAN 大厅（对话框 0xBB）
- 控件 ID：CREATE 0x6CC、JOIN 0x6CD、OBSERVE 0x6CE（隐藏，指针不可达，只能由 `activate()` 返回 observe=true）、BACK 0x5C0、NAME 0x541（编辑）、CHAT 0x540（编辑）、GAMES 0x549（列表）、PLAYERS 0x53E（列表）、CHAT_LOG 0x53F（换行列表）、OBSERVER_IMAGE 0x75F、OBSERVER_TOGGLE 0x760（复选框）。依据：`ui/lan_shell.rs:16-29`。
- 布局：`LanLobbyLayout::build` `ui/lan_shell.rs:803-829`；按钮 `dlu_rect(425,122,108,23)`，`menu_page.rs:136-142`；NAME `rect(136,17,134,19)`；CHAT `rect(12,524,609,19)`；GAMES `rect(323,73,300,292)`；PLAYERS `rect(11,73,301,292)`；CHAT_LOG `rect(11,369,612,154)`；OBSERVER_TOGGLE `rect(576,16,16,17)`。
- 命中顺序：页面按钮 → NAME → CHAT → GAMES → PLAYERS → CHAT_LOG → OBSERVER_IMAGE → OBSERVER_TOGGLE（`ui/lan_shell.rs:831-851`）。
- 列表：行高 19、滚动条宽 20、箭头 22，可见行数 `(h-2)/19`（`ui/shell/list.rs:15-20`）。GAMES 首项为 TXT_LOBBY 占位行（`ui/lan_shell.rs:467`），PLAYERS 本地玩家永远排第一（`app/shell_lan.rs:226-228`）。选中项按名称跨刷新保持（`ui/lan_shell.rs:545-558`）。
- 聊天上限：`apply_chat_length_limit` `ui/lan_shell.rs:766-780`，全部单元 ≤0xFF 时 120，否则 `max(40, 首个宽字符-1)`；名称 16 个 ACP 字节，逗号被过滤（`:685-693`）。
- 键：创建框打开时 Enter 确认、Esc 取消；模态打开时 Enter/Esc 确认；Tab 在 NAME 与 CHAT 之间切换（`ui/lan_shell.rs:724-731`）；CHAT 中 Enter 发送，NAME 中 Enter 无操作（`:733-735`）（`app/shell_lan.rs:915-958`）。
- 动作枚举 `LanLobbyAction`：Create{observe}、Join{game_id,observe}、Back、NameChanged、SendChat、ObserverChanged、SelectGame（`ui/lan_shell.rs:193-211`）。
- 创建框（VERA 专用）：以 0xCE 模态打开，`app/shell_lan.rs:566-572`；房间人数滑条 2..NETWORK_PLAYER_SLOT_COUNT，`ui/room_limits.rs:17-31`；观众复选框；布局 `ui/shell/modal.rs:706-725`。
- 残留：玩家行点击不会让列表获得焦点（`app/shell_lan.rs:1140-1142`）。

### 16.3 WOL 登录 0x10F（清洁安装的 Legacy 策略）
- NICKNAME 组合编辑 0x0628：字节限制 9，只接受 U+0100 以下（`ui/wol_login.rs:42,707`）；每输入一个单元都会查找存储并覆盖密码和记住选项（`:514-534`）。Tab 切换到 PASSWORD（`:741-750`，Legacy 忽略 Shift）；Enter 在可登录时登录（`:751`）；Esc 被消费但无动作（`app/shell_wol/login.rs:291-294`）。
- 下拉列表：槽位 1..31（`ui/wol_login.rs:46-48,317-327`）。
- PASSWORD 0x072C：掩码，8 字节（`:43,715`），无字符集过滤。
- REMEMBER 0x072D：复选框，只在按下 18x18 图标时切换，不是 Tab 停止点（`app/shell_wol/login.rs:409-418`）。
- LOGIN 0x0539：`dlu_top 121`，启用条件 `login_enabled`（`:416`），释放即触发（`app/shell_wol/login.rs:343-349`）。
- FORGET 0x06EB：`dlu_top 150`，仅 Legacy 且有昵称时可用（`:418`）。
- MANAGE 0x06EC：`dlu_top 177`，应用层无操作（`app/shell_wol/login.rs:147-149`）。
- BACK 0x0686：`dlu_top 257`，Esc 不关闭页面。

### 16.4 WOL 登录 Online 策略（VERA 页面）
- PLAY 0x7FF3（合成 ID）、CREATE_ACCOUNT 0x7FF0、FORGOT_PASSWORD 0x7FF1、DELETE_ACCOUNT 0x7FF2（`ui/wol_login.rs:37-40,97-122`）。
- 名称 0x7FE0：24 字节，字符集 `[A-Za-z0-9_-]`（`ui/wol_account.rs:25,141`）；密码 0x7FE1：64 字节，可打印 ASCII；保存的密码显示为 8 个 `*` 占位（`:32,385-387`）。
- 记住 FORM_REMEMBER（合成 ID 3），取消记住会清除已保存的名称（`:653-664`）。
- Tab 向前循环，Shift+Tab 反向（`:725-732`）；Enter 相当于 PLAY。
- 合成 ID 0x7FE0–0x7FF3 不是 gamemd 的控件 ID。FORM_CANCEL 的值 2 与 BACK 的结果值 2 重叠。

### 16.5 WOL 账户弹窗（`WolAccountForm`）
- 布局：`ui/shell/modal.rs:577-606`；窗口 451x326 居中于 POPUP_AREA；字段行 `top=40+9*(4-n)`，步长 18；OK `child(33,128,83,15)`（ID 1），Cancel `child(184,128,83,15)`（ID 2）。
- 按种类的字段：注册（名称、邮箱 254 字节、密码、确认）、验证（6 位验证码）、忘记（名称）、重置（验证码、新密码、确认）、删除（名称、密码）、已删除（无字段）（`ui/wol_account.rs:431-487`）。
- OK 启用条件：不忙且请求有效（`:601-607`）；动作 `Send(AccountRequest)` 或 `Close`（`:670,692`）。
- 结果流转：注册 → 验证；忘记 → 重置；验证或重置 → 登录（`app/shell_wol/login.rs:170-187`，`ui/wol_login.rs:658-665`）；删除 → 已删除（`ui/wol_login.rs:626-631`）。
- 弹窗优先接收所有键和指针事件（`app/shell_wol/login.rs:271-290,445-480`）。Esc 等同 Cancel（`:270-274`）。

### 16.6 WOL 其余对话框
- **我的信息 0x114**（`ui/wol_my_info.rs`，`app/shell_wol/my_info.rs`）：按钮 Play Buddy 0x687（启用条件 slot≠-1）、Clan 0x688、Tournament 0x7A3、Forget Account 0x6EB、Manage Account 0x6EC、Back 0x686；组合 Nickname 0x628、Server 0x629、Locale 0x62A、Country 0x6A1、Color 0x6A2；复选框 AutoLogin 0x6FF、MatchSameRes 0x773。**注意**：Clan、Tournament、Manage 的 `Browser` 动作在应用层被忽略（`app/shell_wol/my_info.rs:294`）。忘记账户直接删除行且静默重置选中，无确认（`ui/wol_my_info.rs:454-474`）。无键盘处理。
- **好友列表 0x116**（`ui/wol_buddy.rs`，`app/shell_wol/buddy.rs`）：按钮 Create Game 0x62B、Join 0x62D（初始禁用）、Page 0x5C2、Add 0x702、Remove 0x703（无确认直接删除）、Back 0x686；列表 0x689 为自绘，行为名称、类型、密码、人数、状态、延迟；双击先 Join 否则 Page（`ui/wol_buddy.rs:503-511`）；Refresh 0x704 在 3000 ms 计时后才启用（`:532-533`）。添加弹窗 0x115：名称编辑 0x540，24 字节（与原版 9 不同）；Enter 触发 Add，Esc 返回，Tab 无操作（`app/shell_wol/buddy.rs:613-649`）。
- **连接验证 0x130**（`ui/wol_verify.rs`，`ui/shell/modal.rs:880-899`）：8 个节点复选框 0x769、0x761、0x764、0x762、0x765、0x768、0x766、0x767，每个 `33,56+14*i`，234x14 DLU；标题 0x5B2；超过 8 个节点时出现滚动列表（`:72-92`）；VERA 房间在 1000 ms 后才显示（`:16-18,63-67`）；不匹配框 0xD0 覆盖在对话框上。无键盘入口，仅指针，OK 为按下后释放（`app/shell_wol/...` `wol.rs:800-815`）。
- **快速对战 0x117**（`ui/wol_quick_match.rs`，`app/shell_wol/quick_match.rs`）：标题 0x694、说明 0xFFFF、记录列表 0x53F（只读）、Cancel 0x5C0（`dlu_top 257`）；记录每 2000 ms 追加一个点；无键盘处理。
- **个人问题 0x112 / 0x122 / 0xFE**（`ui/wol_persona.rs`，`app/shell_wol/persona.rs`）：Yes 1、No 2、Don't Ask 0x6F9；地区选择列表 0x3F0，OK 初始禁用，选中后启用；Enter/Esc 无处理（`ui/wol_persona.rs:131-132`）。
- **页面 0xCF 与 0x11E**（`ui/wol_page.rs`，`app/shell_wol/page.rs`）：0xCF 包括名称编辑 0x5C1（24 字节）、文本编辑 0x540、记录 0x5B5、发送 0x5C2、取消 ID 2；Tab 在两个编辑框之间切换；Enter 点击发送；Esc 无操作（`app/shell_wol/page.rs:653-654`）。0x11E 包括回复编辑 0x540（120 字节）、记录 0x53F、回复对象静态 0x718、发送 0x5C2、返回 0x686。

### 16.7 对草案的修改要点
1. 覆盖矩阵中 WOL 登录、账户、好友、我的信息、验证、快速对战、个人页、LAN 从“未读完整”改为“完整”。
2. 事件模型需要支持两种配对：按钮“释放触发”（大部分）和复选框、组合项“按下触发”（`app/shell_wol/login.rs:376-384,409-418`）。这是 LAN 和登录页的实际差异。
3. 库需要一个“子状态”概念：登录页的账户弹窗、我的信息的下拉、验证的不匹配框都是同一对话框内的子状态，而不是独立对话框。
4. 光标闪烁（2 秒或 1 秒）、滚动到光标、按钮悬停的独立交互对象、计时器（3000 ms 刷新、2000 ms 点动画、1000 ms 验证延迟）都需要由游戏侧按帧驱动，不能写在节点模型里。
5. 合成 ID（0x7FE0–0x7FF3、FORM_*）只在 VERA 中使用。库不能依赖“控件 ID 唯一”，要用节点句柄，而不是控件 ID 作为主键。
6. 库需要一条“忽略动作”的路径：Clan、Tournament、Manage 在应用层无效，但模型仍产生事件。

---

## 17. 多人设置（对话框 BC/BD/C9，`ui/multiplayer_setup.rs`）

**页面规格**（DLU 顶部）
- 局域网主机：Start 0x588（147）、选图 0x5AA（176）、返回 0x2（346），`HOST_PAGE` `ui/multiplayer_setup.rs:389-415`。
- 局域网客人：接受 0x59F（149），`GUEST_PAGE` `:416-433`。
- WOL 主机/客人：`WOL_HOST_PAGE` `:442-469`（Start 156、选图 183、Page 0x5C2 210、返回 257）；`WOL_GUEST_PAGE` `:470-490`。
- VERA 房间：`MODERN_HOST_PAGE` `:501-514`（游戏选项 0x7F02 占据选图位置）；`MODERN_GUEST_PAGE` `:515-521`，只有游戏选项，没有接受按钮；`MODERN_WOL_*` `:530-560`（新增大厅 0x7F16）。

**控件 ID**（`:277-329`）
- 通用：Start、Accept、Back、选图、聊天 0x540、聊天记录 0x53F、速度 0x529、信用 0x402、单位数 0x54F、难度 0x732、复选框 0x54E/0x3E8/0x54D/0x69A/0x69D、地图预览 0x468、表情 0x3ED、Page 0x5C2。
- VERA 专用：上一页/下一页 0x7F00/0x7F01（`:27-28`）、游戏选项 0x7F02、游戏选项确认 0x7F03、地图提议 0x7F04/0x7F05、队伍数 0x7F0D、队伍出生点 0x7F0E、平衡 0x7F0F、房间人数 0x7F10、队伍颜色 0x7F11、大厅 0x7F16（`:29-47`），以及禁止/桥梁 0x7F06–0x7F0B、地形 0x7F12、超级武器 0x7F14。
- 动态 ID：表头加入 0x7140+行、添加 AI 0x7180+行（`:57-58,257-263`）；玩家 0x411+行；国家 0x510..；颜色 0x522..；出生点 0x6A3..；队伍 0x76D+行；第 8 行以后使用 0x7000+列*0x40+行（`:50,579-580`）。

**行数**：`VISIBLE_SETUP_ROWS` 8（`:22`），VERA `MODERN_SETUP_ROWS` 48（`:25`）；每列可见数：国家 7、颜色/出生点/队伍 9、玩家 3（`:598-604`）。

**布局**（局域网，`:1393-1447`，DLU 经 child_window/scale 换算）
- 5 列组合框：x/w (49,100)、(201,78)、(294,27)、(334,25)、(374,25)；行距 16，起始 y=22，高 24。
- 选项行 y `[252,280,307,335]`（BC/BD）或 `[254,281,309,336]`（C9）；滑条 x404 w129 h21；难度 (180,215,266x22)。
- 复选框 x71 宽 151 高 17（WOL 为 x72）。
- 标志 (240, 36+26r, 49x21)；接受图标 (42, y, 28x21)。
- 聊天编辑 (12, chat_y+1, w-2, 19)，聊天记录 (11,369,w,160)；chat_y 为 533/535。

**VERA 房间布局**（`move_options_into_popup` `:1553-1648`，名单 `:1716-1761`）
- 名单区域顶 40、底 551（`:77-78`）；行高 20、表头 23、分组间距 14、框边距 (5,3)（`:79-87`）；列内边距 5（`:91`）。
- 单元格偏移（800 像素坐标）：玩家 (18,260)、阵营 (286,156)、颜色 (450,57)、出生点 (515,57)、队伍 (580,57)；添加 AI (286,91)，加入位于右端（`:98-102`）。
- 队伍栏：行高 25；标签 (0,3,90,18)；滑条 (92,4,151,17)；按钮 97x19，间距 12；控件缩放为 78%（`:107-119,1910-1925`）。
- 名单底部 = 队伍栏顶 − 17（`:1651-1668`）。
- 名单分页：`roster_page` `:191-228`（表头固定，每页最多 48 行）；滚动停止点 `:239-254`。
- 聊天记录从名单顶部到 529（`:1635`）；聊天宽度 190..380，按 3/10 分配（`:72-73,1562-1564`）。
- 表情（房间内）位于 chat_x+chat_w-26，宽 34（在线）或 5（离线）（`:1637-1642`）。
- 地图预览 0x468 锚定，96x69（`:1526-1531`）。
- 游戏选项弹窗 451x326；确认 (276,292,124,24)（`:1604-1607`）。

**弹出列表与滚动**
- `popup_geometry` `:1221-1256`：锁定时不弹出；仅当组合框可见、启用，且是列表所属行时才弹出（`:1226-1232`）。玩家列和添加 AI 列上限为 0（无滚动条，`:1235-1241`）。名单列表宽度至少为最宽文字加滚动条（`:1249-1254`），最小宽度为最宽文字 × 0.975 + 每侧 6 px，色块 40 px（`:103-106,628-643`）。
- 名单滚动：`roster_scroll` `:1942-1953`；滚轮 `:1161-1195`，弹出列表打开时由列表接收滚轮。

**启用条件**（`:865-901`）
- 始终启用：聊天、聊天记录、游戏选项确认、地图提议按钮、表情（WOL）（`:866-870`）。
- 锁定（`view.locked`）时其余全部禁用（`:873-875`）。
- 开始、选图：仅主机（`:879`）；返回、游戏选项：始终（`:877`）；大厅：`lobby_enabled`（`:878`）。
- 上一页：`first_row > 0`；下一页：`first_row + page_len < row_count`（`:884-885`）。
- 接受：`!host && accept_enabled`（`:886`）；`accept_enabled` 由 `view.rs:86-96` 定义。
- 复选框：仅主机；0x54E 在 `short_game_locked` 时也禁用（`:887-888`）。
- 加入：`header_button` 且 `join_enabled`（`:889-891`）。
- 接受高亮（闪烁）：渲染传入 `!host && start_requested`（`render :66-71`），`accept_highlight` `:1214-1220`，仅绘制 ACCEPT（`render :748`）。VERA 房间没有接受按钮，所以这个闪烁在房间中永远不绘制。

**拖放**
- `row_drag` 在按下时设置，前提是 `view.drag_rows` 包含该组合框所在行（`app/input/multiplayer_setup.rs:442-446`）。移动 6 px 后开始拖动（`ROW_DRAG_START` 774，`:1080-1095`）；拖动时关闭列表（`:1088-1089`）并清除悬停（`app/input/multiplayer_setup.rs:347-349`）。
- `DragRow {row, group, may_watch}` `:753-759`；`may_watch` 仅对客人为真（`:756-758`）。
- `drop_group` `:848-857`：队伍和玩家可接收；观众仅在 `may_watch` 时可接收；无队伍和封闭座位永不接收；必须与行所在分组不同；锁定时拒绝。
- `drop_row` `:1098-1101` 返回 `(row, 页眉行)`。处理函数按分组而不是按页行重新找页眉（`app/shell_network_setup/actions.rs:590-598`）。如果两个页眉共享一个分组，可能不匹配（已记录为风险）。

**动作**（`app/shell_network_setup/actions.rs`）
- Start → `request_start`（`:74`）；Accept → `accept()` `:683-700`（置为就绪，排队 `Ready(true)`）；Back `:76-111`；选图 `:113-125`（仅主机）；RosterPage `:137-146`；OpenBox `:147+`。
- Join `:554-575`：Team(n) 设置队伍；Players 使用默认队伍；Spectators 设为观众；NoTeam 和 ClosedSeats 直接退出。
- DropRow `:590-626`：人类玩家 → `service::move_player(index, team, observer)`；AI → `set_network_row_choices` + `PublishOptions`。
- 开始过程中（`held_while_starting` `:14-35`）：DropRow、Join、Select、Trackbar、Toggle、TeamStarts、TeamColors、Balance、Lobby、选图均被拒绝；Back 在提交后被拒绝。

**键**（`app/input/multiplayer_setup.rs:194-268`）
- 验证框、0xCF 页面、通知：接收所有键（`:197-213`）。
- 地图提议：Enter/小键盘 Enter → `MapOffer{download:true}`（未在下载时）；Esc → false；打开后 750 ms 内忽略（`:224-233`，`MapOffer` `:966-975`）。
- 打开的弹窗：Esc、Enter、小键盘 Enter → `OpenBox(None)`（`:234-239`）。
- 聊天有焦点：Backspace、Delete、方向键、Home、End → `LanEditKey`；Enter → 发送非空聊天（`:240-252`）；其他文字插入，但 Esc 和 Tab 不插入（`:254-259`）。
- 聊天发送规则：Enter 追加 CRLF，超过 2 个单元才发送（`ui/lan_shell.rs:264-273`）。

**点击**（`app/input/multiplayer_setup.rs:369-540`）
- 弹窗打开时：Pick → `SetupAction::Select`（`:416-431`）。
- 名单格按下：选项超过 1 个时打开列表（`roster_cell_opens` `:156-158`，`:442-450`）；局域网格使用 `press_face`（`:452-459`）。
- 复选框：点击图标区 → Toggle（`:462-471`）；滑条：按下 → Trackbar（`:472-482`）。
- 聊天记录滚动条（`:483-495`）。
- 按钮按下/释放 → 动作（`:496-531`）：开始、接受、返回、选图、表情、Page、上一页/下一页 → RosterPage（`:518-519`）、游戏选项 → `OpenBox(Some GameOptions)`（`:520`）、队伍出生点、队伍颜色、平衡、大厅、地图提议、加入（`:528`）。
- 释放被拖动的行 → `drop_row` → `DropRow`（`:504-509`）。

**文字限制**
- 聊天：全部单元 ≤0xFF 时为 120 个 UTF-16 单位；否则上限为 `max(首个宽字符-1, 40)`（`ui/lan_shell.rs:766-778`）；此路径无字符过滤（`insert_unit_filtered` 以 `0, false, &[]` 调用，`ui/lan_shell.rs:252-262`）。

**发现的问题**
- `ui/multiplayer_setup.rs:1316-1319`：聊天列的一段文档注释位于 `roster_bottom` 上方，内容与字段不符。
- 接受闪烁在 VERA 房间中是死代码（见上）。
- 拖放的页眉查找按分组而非页行，可能在两个页眉共享分组时出错。

**覆盖说明**：测试模块（`:2010-2716`）、`tooltip_key` 函数体（`:1958-2009`）、`flash_phase` 函数、聊天和 WOL 发送路径的内部未读。

---

## 18. 遭遇战子对话框（选图、随机地图、种子浏览器）

### 18.1 选图 0x6B（`ui/skirmish_shell/state/choose_map.rs`，`layout.rs`）

| 控件 | 种类 | 矩形 | 启用条件 | 动作 / 结果 |
|---|---|---|---|---|
| 使用地图 0x6C5 | 按钮 | `dlu(425,122,108,23)` 吸附，`layout.rs:1030,1049` | 始终（`choose_map.rs:857`） | `ChooseMapModalButton::UseMap0x6c5`（`layout.rs:323-325`）；先检查 AI 弹出，再离开对话框（`app/shell_skirmish.rs:1036`） |
| 取消 0x5C0 | 按钮 | `back_rect`，`layout.rs:1056` | 始终 | `Cancel0x5c0`（`:324`）；`shell_skirmish.rs:1051` |
| 创建随机地图 0x583 | 按钮 | `dlu(425,149,108,23)`，`layout.rs:1031,1057` | 构建的地图列表中 `random_maps_allowed`（`choose_map.rs:516,840-849`） | `CreateRandomMap0x583`（`layout.rs:326`）；`shell_skirmish.rs:1054` 隐藏选择器 |
| 收藏（VERA） | 按钮 | `dlu(425,176,108,23)`，`layout.rs:1033,1064` | 无 `row_labels` 且高亮行可收藏（`choose_map.rs:850-856`） | `Favorite`（`layout.rs:328`）；`toggle_favorite`（`choose_map.rs:594-609`）；`shell_skirmish.rs:1058-1069` |
| 模式列表（游戏类型，0x6EB） | 列表 | `child_window(77,78,130,211)`，向左加宽，`layout.rs:1041` | — | 按行选中，重建地图列表（`choose_map.rs:734-780`）；无键盘分支（`:4-6`） |
| 地图列表 0x553 | 列表 | `child_window(225,78,130,211)`，向右加宽，顶部偏移 20（`MAP_SEARCH_LIST_SHIFT`），`layout.rs:1035,1042-1047,1010-1011` | — | 按行高亮（`choose_map.rs:771-778`）；双击忽略（`:700-725`） |
| 搜索框（VERA） | 编辑 | `map_search`，`layout.rs:1048` | — | 32 个字符上限 `SEARCH_MAX_CHARS`（`choose_map.rs:178,572`）；不区分大小写的子串匹配 `display_name`（`:413-419`）；丢弃控制字符（`:588`）；可打印字符和 Backspace（`shell_skirmish.rs:1245-1295`，IME 矩形 `:1245-1255`） |
| 两个列表的滚动条 | 滚动条 | `ShellListGeometry`（`choose_map.rs:426-442`） | — | 按下、捕获、500 ms 重复（`choose_map.rs:744-753,799-815`） |
| 使用地图确认框 | 模态提示 | `quit_confirm_layout` 的确认/取消（`shell_skirmish.rs:1132-1147`） | — | 种类：EjectAi、Download、Downloading、DownloadFailed、SaveFailed、Downloaded、RandomMapFailed（`choose_map.rs:72-95`）；回答 `:124-135`；Enter 确认，EjectAi 除外（`shell_skirmish.rs:1315-1327`） |
| 标题 0x694 / 状态 0x695 / 预览 0x468 / 作者 | 静态 | 标题 `dlu(425,1,108,10)`（`layout.rs:1071`）；状态 `:1080`；预览 `dlu(428,23,96,69)`（`:1081-1086`）；作者 `:1087` | — | 仅悬停帮助（`hit_test.rs:248-265`） |

- **预设列表**：此对话框没有预设。
- **模式列表内容**：游戏类型目录行，之后是 VERA 行“收藏”（`FAVORITES_ROW = i32::MIN+1`）和“在线地图”（`ONLINE_MAPS_ROW = i32::MIN`）（`choose_map.rs:156-160,372-388`）。官方专用房间省略“在线地图”（`:381-384`）。合作战役使用自定义行标签（`:206-208`）。
- **不寻常行为**：禁用按钮会吞掉按下但不触发（`:861-867`）；点击不改变目录条目的游戏类型行，不重建列表（`:755-769`）；下载进行中只有取消可用（`:119-122`）；Ares 的使用地图 AI 弹出检查读取超过地图玩家上限的 AI 行（`shell_skirmish.rs:1102-1128`）。

### 18.2 创建随机地图 0x105（`ui/skirmish_shell/state/random_map_setup.rs`，`layout.rs`，`app/shell_random_map.rs`）

控件 ID 是 `layout.rs:490-523` 中的枚举名，布局常量见 `layout.rs:107-153`，函数见 `layout.rs:1096-1206`。

| 控件 | 种类 | 矩形 | 启用条件 | 动作 / 结果 |
|---|---|---|---|---|
| 地图类型 0x405 | 下拉 | 第 0 行：x179 y41 w150 h103（`layout.rs:107-117,1116-1124`） | 未生成时启用（`random_map_setup.rs:521-527`）；高级视图隐藏（`:665-668`） | 5 项：大陆 1、团队大陆 2、内陆 3、山地 4、群岛 0（`:114-120`） |
| 时间 0x3EA | 下拉 | 第 1 行，h101（`layout.rs:119`） | 同上 | 4 项：早晨、下午、黄昏、夜晚（`:133-138`） |
| 地形 0x407 | 下拉 | 第 2 行 | 同上 | 4 项：温带 0、雪地 1、城市 2、沙漠 3（`:145-150`） |
| 大小 0x406 | 下拉 | 第 3 行 | 同上 | 8 项 0–7，小到巨大（`:163-172`）；同时设置宽和高（`:559-563`） |
| 资源 0x408 | 下拉 | 第 4 行 | 同上 | 4 项：低、中、高、极高（`:174-179`） |
| 玩家数 0x3EB | 滑条 | 第 5 行，h13，`child_window`（`layout.rs:1117`） | 未生成时启用 | 范围 2 至 `MAX_PLAYERS`，步长 1（`random_map_setup.rs:219`）；网络模式上限 `NETWORK_PLAYER_SLOT_COUNT`（`:494-497`） |
| 随机化 0x621 | 按钮 | `dlu(74,257,83,15)`（`layout.rs:121-123,1147`） | 始终 | `randomize_options`（`random_map_setup.rs:790-823`） |
| 生成 0x620 | 按钮 | `dlu(246,257,83,15)`（`layout.rs:125,1153`） | 始终 | 重新掷骰后 `begin_generate`（`app/shell_random_map.rs` 约 2120 行） |
| 确定 0x6C5 | 按钮 | 右列 y122（`layout.rs:134,1172`） | 仅已生成（`random_map_setup.rs:530`） | 未生成则先生成，再接受（`shell_random_map.rs` 约 2107 行） |
| 读取 0x6C2 | 按钮 | y149（`layout.rs:135,1173`） | `saved_seed_buttons_enabled`（`:531`） | 打开种子浏览器 |
| 保存 0x6C3 | 按钮 | y176（`layout.rs:136`） | 仅已生成（`:530`） | 打开种子浏览器 |
| 删除 0x6C4 | 按钮 | y203（`layout.rs:137`） | 同读取 | 打开种子浏览器 |
| 取消 0x5C0 | 按钮 | `back_rect`（`layout.rs:1176`） | 生成期间不可用（`random_map_setup.rs:521-527`） | 关闭设置 |
| 高级切换 | 按钮 | 右列 y230（`layout.rs:140,1177`） | 始终 | 依次切换：基本、高级、附加、基本（`random_map_setup.rs:863-885`） |
| 导出地图 | 按钮 | 右列 y257（`layout.rs:142,1178`） | 已生成且未导出（`:536`） | 写出地图，显示路径或失败 |
| 公平出生开关 | 按钮 | 右列 y284（`layout.rs:145`） | 始终 | 切换 `fair_starts`（`random_map_setup.rs:854-858`） |
| 预设循环 | 按钮 | 右列 y311（`layout.rs:147`） | 始终 | `cycle_preset`（`:693-742`） |
| 高级滑条（7 行） | 滑条 | y 41、65、90、114、138、163、187（`layout.rs:150`）；x179 w150 h13（`:1189-1196`） | 仅地图类型 3 或 4 时有平台大小（`:534`） | 范围 0–100（`:406`）；按下和拖动立即生效（`:924-968`） |
| 高级滚动条 | 滚动条 | 第一个滑条右边缘 +4（`layout.rs:399-408`） | 仅高级视图 | 2 页（`layout.rs:379-380`；`random_map_setup.rs:889-907`） |
| 附加项 0–6（`ExtrasRow`） | 类按钮 | 与高级滑条相同的七个窗口 | `extras_row()` 有值时（`:535`） | 见下方附加项表 |
| 城市区域 5000（仅 Ares） | 复选框 | `checkbox_dlu_rect(177,192,150,14)`（`layout.rs:604-607,1204`） | 仅 Ares 启用时（`layout.rs:1253-1262`） | `toggle_urban_areas`（`random_map_setup.rs:831-833`）；仅基本视图（`layout.rs:1315-1320`） |
| 种子字段 | 静态 | `dlu(279,287,50,12)`（`layout.rs:127,1159`） | — | 枚举中无对应控件变体，不参与命中测试 |
| 进度文字 / 进度条 | 静态 | `layout.rs:1202-1203` | — | 显示进度、导出结果或附加项说明 |

**高级字段**（`layout.rs:333-357,469-485`；字段映射 `random_map_setup.rs:409-425`）
- 第 1 页（行 0–6）：水域（GUI:Water）、丘陵、建筑（在城市开启前读为 0，`:912-920`）、植被、矿田（GUI:TiberiumFields）、平台大小、可达性（GUI:Accessability）。
- 第 2 页（行 7–12）：科技建筑、残骸、损伤、平民、动物、中立防御者（VERA:RandomMapWild）。

**附加项视图**（`state/random_map_extras.rs`）
- 16 项，每页 6 项加一个翻页行（`:67-84,16,86-93`）；按页数取模循环（`:246-251`）；短页的填充行隐藏（`:200-205`）。
- 前四项：分享代码（复制）、使用代码（粘贴）、上一张地图（返回）、我的预设（保存）（`:68-71,210-221`）。之后：矿田、城镇、基地房间、海路、公平油井、中央宝石、道路、氛围、战争损伤、乡村、地标、城镇围栏（`:72-83`）。
- 选择值会切换并返回 `Regenerate`（`:241-245`）；上一张地图后退并重新生成（`:237-239`）。
- 取值：矿田和城镇 0 为原版，否则为数字；氛围有五个命名值；其余为关/开、少/多、许多、小/大（`:146-175`）。

**预设列表**（`state/random_map_setup.rs:249-322,695-698`）
循环顺序：1v1 决斗、30 人混战、城市战争、群岛、团队战、然后是我的预设 0–4 槽。字段为（玩家数、地图类型、大小、资源、地形、高级[13]）：
- 决斗：2、3、1、1、无，`[10,25,0,40,60,50,80,0…]`
- 混战：30、3、7、2、无，`[5,20,0,30,70,50,90,0…]`
- 城市战争：4、3、2、1、地形 2，`[5,10,100,20,50,60,90,0…]`
- 群岛：4、0、2、2、地形 0，`[60,20,0,50,60,50,50,0…]`
- 团队战：8、2、4、2、无，`[35,25,0,40,60,50,70,0…]`
- 预设还会根据建筑值设置 `fair_starts` 和 `urban_enabled`（`:723-724`）。

**不寻常行为**
- 生成期间所有控件都无效，包括取消（`random_map_setup.rs:521-527`，测试 `:1429-1443`）。
- 生成期间按 Esc 无效果（`app/shell_skirmish.rs:1296-1300` 注释）。
- 任何选项编辑都会清除已生成、预览、预设和公平状态（`random_map_setup.rs:682-688`）。确定、读取和附加项视图都依赖这一点。
- 高级视图隐藏组合框行（`random_map_setup.rs:665-668`；`layout.rs:1289-1321`）。
- 附加项的选择按下和确定按下都会生成，但不使用生成按钮的随机掷骰（`app/shell_random_map.rs` 附加项释放分支）。
- Ares：群岛排在最前，沙漠在雪地之后（`random_map_setup.rs:121-160`）；随机化从这些列表中抽取（`:802-816`）。

### 18.3 种子浏览器（补充，来自第二轮）
- 按钮：Action 0x40F/0x6C7/0x6C8（同一按钮，按模式改变标签和行为）、返回 0x686（三种模式共用）（`ui/skirmish_shell/layout.rs:1454-1457`）。
- 列表：0x525/0x527/0x528（`layout.rs:166-169,1458-1459`），其中 166-169 的 ID 与 AI 颜色共用映射，待核实。
- 名称编辑 0x526：仅保存模式（`layout.rs:1460-1461`）。
- 命中测试顺序：名称编辑 → 列表 → 动作 → 返回（`layout.rs:1744-1762`）。
- 文字规则：长度上限、控制字符过滤，字节模式下拒绝 ≥0x100 的单元（`ui/skirmish_shell/state/saved_seed_browser.rs:145-160`）；IME 组合路径 `:253`。
- 提示：`SavedSeedPrompt` `:38`，`SavedSeedPromptPurpose` `:23`，`SavedSeedOutcome` `:12`。
- 未核实：`compute_saved_seed_layout` 的 DLU 公式、启用条件、按键绑定、滚动按钮 ID。

### 18.4 对草案的修改要点
1. 选图和随机地图都依赖“**父控件状态变化后清除派生状态**”（任何选项编辑清除已生成标志）。库的数据绑定层需要支持“依赖失效”，否则每个界面都要手写清除逻辑。
2. “生成期间全部冻结”是一个模式，不是单个控件的启用条件。库需要有“模态锁定”或“忙碌”状态，统一禁用子树。
3. 下拉列表的选项值和显示标签分离（例如地图类型 1..5 与显示顺序不一致）。节点的 `items` 需要 `(value, label)` 对，而不是只有标签。
4. 高级视图和基本视图共用同一位置的控件，隐藏其中一组。库需要支持“视图切换”，把一组节点作为一个页面隐藏/显示。
5. 随机地图的按键：生成期间 Esc 无效果，其他时候 Esc 关闭。与 WOL 和 LAN 的 Esc 行为不同，需要由对话框声明，而不是由库统一规定。

**仍未覆盖**：`app/shell_random_map.rs` 第 2185 行之后的生成块和打开种子浏览器的逻辑未读；种子浏览器的布局公式（`layout.rs:1383-1500`）未完整盘点。
