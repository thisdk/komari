use std::{collections::HashMap, sync::LazyLock};

use backend::UiLanguage;
use dioxus::prelude::*;

use crate::AppState;

/// Translation entries: `(key, Chinese, English)`.
///
/// Keys are the original English UI copy so missing entries safely fall back
/// to English (and finally to the key itself) during incremental migration.
const TRANSLATIONS: &[(&str, &str, &str)] = &[
    // Navigation
    ("Actions", "动作", "Actions"),
    ("Characters", "角色", "Characters"),
    ("Settings", "设置", "Settings"),
    ("Localization", "本地化", "Localization"),
    ("Debug", "调试", "Debug"),
    // Common actions
    ("Create", "创建", "Create"),
    ("Save", "保存", "Save"),
    ("Delete", "删除", "Delete"),
    ("Cancel", "取消", "Cancel"),
    ("Reset", "重置", "Reset"),
    ("Replace", "替换", "Replace"),
    ("Import", "导入", "Import"),
    ("Export", "导出", "Export"),
    ("Copy", "复制", "Copy"),
    ("Update", "更新", "Update"),
    ("Add", "添加", "Add"),
    ("Enabled", "启用", "Enabled"),
    ("Key", "按键", "Key"),
    ("Mode", "模式", "Mode"),
    ("Count", "数量", "Count"),
    ("Amount", "数量", "Amount"),
    ("Width", "宽度", "Width"),
    ("Height", "高度", "Height"),
    ("X", "X", "X"),
    ("Y", "Y", "Y"),
    ("Unknown", "未知", "Unknown"),
    ("None", "无", "None"),
    ("Start", "开始", "Start"),
    ("Stop", "停止", "Stop"),
    ("No", "否", "No"),
    ("Resume", "继续", "Resume"),
    ("Suspend", "暂停", "Suspend"),
    ("Save button", "保存按钮", "Save button"),
    ("Others", "其他", "Others"),
    // Inputs
    ("Enter a name...", "输入名称...", "Enter a name..."),
    ("Click to set", "点击设置", "Click to set"),
    ("Press any key...", "请按下任意按键...", "Press any key..."),
    ("(optional)", "（可选）", "(optional)"),
    // Minimap
    ("State", "状态", "State"),
    ("Position", "坐标", "Position"),
    ("Priority action", "优先级动作", "Priority action"),
    ("Normal action", "普通动作", "Normal action"),
    ("Erda Shower", "艾尔达之雨", "Erda Shower"),
    ("Detected size", "检测尺寸", "Detected size"),
    ("Selected size", "选择尺寸", "Selected size"),
    ("Time Until Stop", "剩余运行时间", "Time Until Stop"),
    ("Run Time", "已运行时间", "Run Time"),
    ("Input method", "输入方式", "Input method"),
    ("Use GPU", "使用 GPU", "Use GPU"),
    ("Lie Detectors", "谎言探测器", "Lie Detectors"),
    ("Re-detect", "重新检测", "Re-detect"),
    ("Bulk import", "批量导入", "Bulk import"),
    ("Create a map...", "创建地图...", "Create a map..."),
    // Settings
    ("Capture", "捕获", "Capture"),
    ("Handle", "句柄", "Handle"),
    ("Refresh handles", "刷新句柄", "Refresh handles"),
    ("Default", "默认", "Default"),
    ("Input", "输入", "Input"),
    ("Method", "方式", "Method"),
    ("RPC server URL", "RPC 服务器地址", "RPC server URL"),
    ("Notifications", "通知", "Notifications"),
    ("Webhook provider", "Webhook 服务商", "Webhook provider"),
    ("Webhook URL", "Webhook 地址", "Webhook URL"),
    (
        "Discord ping user ID",
        "Discord 提醒用户 ID",
        "Discord ping user ID",
    ),
    ("Rune spawns", "符文出现", "Rune spawns"),
    ("Elite boss spawns", "精英怪出现", "Elite boss spawns"),
    ("Player dies", "角色死亡", "Player dies"),
    ("Guildie appears", "公会成员出现", "Guildie appears"),
    ("Stranger appears", "陌生人出现", "Stranger appears"),
    ("Friend appears", "好友出现", "Friend appears"),
    (
        "Detection fails or map changes",
        "检测失败或地图变化",
        "Detection fails or map changes",
    ),
    (
        "Lie detector appears",
        "谎言探测器出现",
        "Lie detector appears",
    ),
    ("Run timer ends", "运行计时结束", "Run timer ends"),
    ("Hotkeys", "快捷键", "Hotkeys"),
    (
        "Toggle start/stop actions",
        "切换开始/停止动作",
        "Toggle start/stop actions",
    ),
    ("Add platform", "添加平台", "Add platform"),
    ("Mark platform start", "标记平台起点", "Mark platform start"),
    ("Mark platform end", "标记平台终点", "Mark platform end"),
    ("Run timer", "运行计时器", "Run timer"),
    (
        "Duration (hh:mm:ss)",
        "时长（hh:mm:ss）",
        "Duration (hh:mm:ss)",
    ),
    ("Enable rune solving", "启用符文破解", "Enable rune solving"),
    (
        "Enable transparent shape solving",
        "启用透明图形破解",
        "Enable transparent shape solving",
    ),
    (
        "Enable Violetta solving",
        "启用 Violetta 破解",
        "Enable Violetta solving",
    ),
    ("Enable panic mode", "启用应急模式", "Enable panic mode"),
    (
        "Stop actions on fail or map changed",
        "失败或地图变化时停止动作",
        "Stop actions on fail or map changed",
    ),
    (
        "Stop actions on player dies",
        "角色死亡时停止动作",
        "Stop actions on player dies",
    ),
    ("Language", "语言", "Language"),
    // Localization (game templates) screen
    ("Info", "说明", "Info"),
    ("Section", "分区", "Section"),
    ("Function", "功能", "Function"),
    ("Template(s)", "模板", "Template(s)"),
    ("Popups", "弹窗", "Popups"),
    (
        "Unstuck player through closing menu, popup, dialog, etc.",
        "通过关闭菜单、弹窗、对话框等为角色解除卡住",
        "Unstuck player through closing menu, popup, dialog, etc.",
    ),
    ("All popups.", "所有弹窗", "All popups."),
    (
        "Go to town confirmation and save familiars setup.",
        "回城确认与保存图鉴设置",
        "Go to town confirmation and save familiars setup.",
    ),
    ("Confirm popup.", "确认弹窗", "Confirm popup."),
    (
        "Respawn on player death.",
        "角色死亡后复活",
        "Respawn on player death.",
    ),
    ("Ok (new) popup.", "确定（新版）弹窗", "Ok (new) popup."),
    ("Familiars", "图鉴", "Familiars"),
    (
        "Sort familiar cards by level before swapping.",
        "切换前按等级对图鉴卡排序",
        "Sort familiar cards by level before swapping.",
    ),
    (
        "Familiar menu setup tab's setup level sort button.",
        "图鉴菜单设置页的等级排序按钮",
        "Familiar menu setup tab's setup level sort button.",
    ),
    (
        "Save familiars setup after swapping.",
        "切换后保存图鉴设置",
        "Save familiars setup after swapping.",
    ),
    (
        "Familiar menu setup tab's save button.",
        "图鉴菜单设置页的保存按钮",
        "Familiar menu setup tab's save button.",
    ),
    ("HEXA", "HEXA", "HEXA"),
    (
        "Open Sol Erda version menu in HEXA Matrix.",
        "在 HEXA 矩阵中打开索尔艾尔达版本菜单",
        "Open Sol Erda version menu in HEXA Matrix.",
    ),
    (
        "Erda conversion button.",
        "艾尔达转化按钮",
        "Erda conversion button.",
    ),
    (
        "Open HEXA Booster exchange menu.",
        "打开 HEXA Booster 交换菜单",
        "Open HEXA Booster exchange menu.",
    ),
    (
        "HEXA Booster button.",
        "HEXA Booster 按钮",
        "HEXA Booster button.",
    ),
    (
        "Select max HEXA Booster amount to exchange.",
        "选择要交换的最大 HEXA Booster 数量",
        "Select max HEXA Booster amount to exchange.",
    ),
    ("Max button.", "最大按钮", "Max button."),
    (
        "Convert Sol Erda to HEXA Booster.",
        "将索尔艾尔达转换为 HEXA Booster",
        "Convert Sol Erda to HEXA Booster.",
    ),
    ("Convert button.", "转换按钮", "Convert button."),
    (
        "Detect whether change channel menu is opened.",
        "检测换线菜单是否打开",
        "Detect whether change channel menu is opened.",
    ),
    ("Change channel text.", "换线文字", "Change channel text."),
    (
        "Detect whether player entered cash shop.",
        "检测角色是否进入现金商城",
        "Detect whether player entered cash shop.",
    ),
    ("Cash shop text.", "现金商城文字", "Cash shop text."),
    (
        "Detect whether Generic/HEXA booster is in use.",
        "检测普通/HEXA 增幅器是否使用中",
        "Detect whether Generic/HEXA booster is in use.",
    ),
    ("Timer text.", "计时器文字", "Timer text."),
    (
        "Detect lie detector event.",
        "检测谎言探测器事件",
        "Detect lie detector event.",
    ),
    (
        "Lie detector title.",
        "谎言探测器标题",
        "Lie detector title.",
    ),
    ("Capture color", "捕获彩色", "Capture color"),
    ("Capture grayscale", "捕获灰度", "Capture grayscale"),
    ("Confirm", "确认", "Confirm"),
    ("Yes", "是", "Yes"),
    ("Next", "继续", "Next"),
    ("End chat", "结束对话", "End chat"),
    ("Ok (new)", "确定（新版）", "Ok (new)"),
    ("Ok (old)", "确定（旧版）", "Ok (old)"),
    ("Cancel (new)", "取消（新版）", "Cancel (new)"),
    ("Cancel (old)", "取消（旧版）", "Cancel (old)"),
    (
        "Erda conversion button",
        "艾尔达转化按钮",
        "Erda conversion button",
    ),
    (
        "HEXA Booster button",
        "HEXA Booster 按钮",
        "HEXA Booster button",
    ),
    ("Max button", "最大按钮", "Max button"),
    ("Convert button", "转换按钮", "Convert button"),
    ("Level sort button", "等级排序按钮", "Level sort button"),
    ("Cash shop", "现金商城", "Cash shop"),
    ("Change channel", "换线", "Change channel"),
    ("Timer", "计时器", "Timer"),
    (
        "Lie detector (new)",
        "谎言探测器（新版）",
        "Lie detector (new)",
    ),
    (
        "Lie detector (old)",
        "谎言探测器（旧版）",
        "Lie detector (old)",
    ),
    (
        "This template is in grayscale.",
        "此模板为灰度图像",
        "This template is in grayscale.",
    ),
    // Debug
    ("Test spin rune", "测试旋转符文", "Test spin rune"),
    ("Test Violetta", "测试 Violetta", "Test Violetta"),
    (
        "Test transparent shape normal",
        "测试透明图形（普通）",
        "Test transparent shape normal",
    ),
    (
        "Test transparent shape hard",
        "测试透明图形（困难）",
        "Test transparent shape hard",
    ),
    (
        "Test transparent shape...",
        "测试透明图形...",
        "Test transparent shape...",
    ),
    ("Stop recording", "停止录制", "Stop recording"),
    ("Start recording", "开始录制", "Start recording"),
    (
        "Stop auto saving rune",
        "停止自动保存符文",
        "Stop auto saving rune",
    ),
    (
        "Start auto saving rune",
        "开始自动保存符文",
        "Start auto saving rune",
    ),
    (
        "Stop auto record lie detector",
        "停止自动录制谎言探测器",
        "Stop auto record lie detector",
    ),
    (
        "Start auto record lie detector",
        "开始自动录制谎言探测器",
        "Start auto record lie detector",
    ),
    // Characters
    (
        "Create a character...",
        "创建角色...",
        "Create a character...",
    ),
    (
        "Use potion and feed pet",
        "使用药水与喂养宠物",
        "Use potion and feed pet",
    ),
    ("Feed key", "喂养键", "Feed key"),
    ("Every (mm:ss)", "每隔（mm:ss）", "Every (mm:ss)"),
    ("Potion key", "药水键", "Potion key"),
    ("HP below", "生命值低于", "HP below"),
    ("HP update every", "生命值更新间隔", "HP update every"),
    ("Use booster", "使用增幅器", "Use booster"),
    ("Generic Booster key", "普通增幅器键", "Generic Booster key"),
    ("HEXA Booster key", "HEXA Booster 键", "HEXA Booster key"),
    (
        "Exchange when Sol Erda",
        "索尔艾尔达交换条件",
        "Exchange when Sol Erda",
    ),
    (
        "Requires HEXA Booster to be visible in quick slots, Sol Erda tracker menu opened and HEXA Matrix configured in the quick menu. Exchange will only happen if there is no HEXA Booster.",
        "需要 HEXA Booster 显示在快捷栏、索尔艾尔达追踪菜单打开且 HEXA 矩阵配置在快捷菜单中。仅当没有 HEXA Booster 时才会交换。",
        "Requires HEXA Booster to be visible in quick slots, Sol Erda tracker menu opened and HEXA Matrix configured in the quick menu. Exchange will only happen if there is no HEXA Booster.",
    ),
    ("Exchange all", "全部交换", "Exchange all"),
    ("Movement", "移动", "Movement"),
    ("Up jump is flight", "上跳为飞行", "Up jump is flight"),
    (
        "Applicable only to mage class or when non-up-arrow up jump key is set.",
        "仅适用于法师职业或设置了非上箭头键的上跳键时",
        "Applicable only to mage class or when non-up-arrow up jump key is set.",
    ),
    (
        "Jump then up jump if possible",
        "尽可能先跳跃再上跳",
        "Jump then up jump if possible",
    ),
    (
        "Applicable only for non-mage class and when non-up-arrow up jump key is set.",
        "仅适用于非法师职业且设置了非上箭头键的上跳键时",
        "Applicable only for non-mage class and when non-up-arrow up jump key is set.",
    ),
    ("Fall teleport range", "下落传送范围", "Fall teleport range"),
    (
        "Maximum y distance to teleport when falling instead of jumping down.",
        "下落而非跳下时进行传送的最大 y 距离",
        "Maximum y distance to teleport when falling instead of jumping down.",
    ),
    (
        "Up jump teleport range",
        "上跳传送范围",
        "Up jump teleport range",
    ),
    (
        "Minimum y distance to use teleport with jump when up jumping.",
        "上跳时结合跳跃使用传送的最小 y 距离",
        "Minimum y distance to use teleport with jump when up jumping.",
    ),
    (
        "Disable teleport on fall",
        "下落时禁用传送",
        "Disable teleport on fall",
    ),
    (
        "Applicable only to mage class.",
        "仅适用于法师职业",
        "Applicable only to mage class.",
    ),
    (
        "Attack when pathing (PingPong)",
        "寻路时攻击（PingPong）",
        "Attack when pathing (PingPong)",
    ),
    (
        "Attacks with the PingPong key while pathing to a target (e.g. rune) until within 5 distance of the target.",
        "在前往目标（例如符文）途中使用 PingPong 键攻击，直到距离目标 5 以内",
        "Attacks with the PingPong key while pathing to a target (e.g. rune) until within 5 distance of the target.",
    ),
    (
        "Disable double jumping",
        "禁用二段跳",
        "Disable double jumping",
    ),
    (
        "Not applicable if an action requires double jumping.",
        "如果某个动作需要二段跳则不适用",
        "Not applicable if an action requires double jumping.",
    ),
    (
        "Disable grapple on double jumping",
        "二段跳时禁用抓钩",
        "Disable grapple on double jumping",
    ),
    (
        "Applicable only if grapple key is set.",
        "仅当设置了抓钩键时适用",
        "Applicable only if grapple key is set.",
    ),
    ("Disable walking", "禁用行走", "Disable walking"),
    (
        "Not applicable if an action requires adjusting.",
        "如果某个动作需要微调则不适用",
        "Not applicable if an action requires adjusting.",
    ),
    ("Swappable slots", "可切换槽位", "Swappable slots"),
    (
        "Swap check every (mm:ss)",
        "切换检查间隔（mm:ss）",
        "Swap check every (mm:ss)",
    ),
    ("Swapping enabled", "启用切换", "Swapping enabled"),
    (
        "Can swap rare familiars",
        "可切换稀有图鉴",
        "Can swap rare familiars",
    ),
    (
        "Can swap epic familiars",
        "可切换史诗图鉴",
        "Can swap epic familiars",
    ),
    ("Link key timing", "连锁键时机", "Link key timing"),
    (
        "Elite boss spawns behavior",
        "精英怪出现行为",
        "Elite boss spawns behavior",
    ),
    ("Key to use", "使用的按键", "Key to use"),
    // Key bindings
    ("Key bindings", "按键绑定", "Key bindings"),
    ("Rope lift", "绳梯", "Rope lift"),
    ("Teleport", "传送", "Teleport"),
    ("Jump", "跳跃", "Jump"),
    ("Up jump", "上跳", "Up jump"),
    (
        "This is meant for classes that have a separate skill to up jump. Classes that use up arrow should set this key to up arrow.",
        "这是为拥有独立上跳技能的职业准备的。使用上箭头键上跳的职业应将此键设置为上箭头",
        "This is meant for classes that have a separate skill to up jump. Classes that use up arrow should set this key to up arrow.",
    ),
    ("Interact", "交互", "Interact"),
    (
        "Cash shop is used to reset spin rune to a normal rune. This only happens if solving rune fails 8 times consecutively.",
        "现金商城用于将旋转符文重置为普通符文。仅在符文破解连续失败 8 次时发生",
        "Cash shop is used to reset spin rune to a normal rune. This only happens if solving rune fails 8 times consecutively.",
    ),
    ("To town", "回城", "To town"),
    (
        "This key must be set to use navigation or run/stop cycle features.",
        "使用导航或运行/停止循环功能时必须设置此键",
        "This key must be set to use navigation or run/stop cycle features.",
    ),
    (
        "This key must be set to use panic mode or elite boss spawns behavior features.",
        "使用应急模式或精英怪出现行为功能时必须设置此键",
        "This key must be set to use panic mode or elite boss spawns behavior features.",
    ),
    ("Familiar menu", "图鉴菜单", "Familiar menu"),
    (
        "This key must be set to use familiars swapping feature.",
        "使用图鉴切换功能时必须设置此键",
        "This key must be set to use familiars swapping feature.",
    ),
    // Buffs
    ("Buffs", "增益", "Buffs"),
    ("Familiar skill", "图鉴技能", "Familiar skill"),
    ("Familiar essence", "图鉴精华", "Familiar essence"),
    ("Sayram's Elixir", "赛拉姆秘药", "Sayram's Elixir"),
    ("Aurelia's Elixir", "奥蕾莉亚秘药", "Aurelia's Elixir"),
    ("2x EXP Coupon", "2 倍经验券", "2x EXP Coupon"),
    ("3x EXP Coupon", "3 倍经验券", "3x EXP Coupon"),
    ("4x EXP Coupon", "4 倍经验券", "4x EXP Coupon"),
    (
        "50% Bonus EXP Coupon",
        "50% 额外经验券",
        "50% Bonus EXP Coupon",
    ),
    ("Legion's Wealth", "联盟财富", "Legion's Wealth"),
    ("Legion's Luck", "联盟幸运", "Legion's Luck"),
    (
        "Wealth Acquisition Potion",
        "财富获取药水",
        "Wealth Acquisition Potion",
    ),
    (
        "EXP Accumulation Potion",
        "经验累积药水",
        "EXP Accumulation Potion",
    ),
    (
        "Small Wealth Acquisition Potion",
        "小型财富获取药水",
        "Small Wealth Acquisition Potion",
    ),
    (
        "Small EXP Accumulation Potion",
        "小型经验累积药水",
        "Small EXP Accumulation Potion",
    ),
    ("For The Guild", "为了公会", "For The Guild"),
    ("Hard Hitter", "强力打击", "Hard Hitter"),
    ("Extreme Red Potion", "极限红药水", "Extreme Red Potion"),
    ("Extreme Blue Potion", "极限蓝药水", "Extreme Blue Potion"),
    ("Extreme Green Potion", "极限绿药水", "Extreme Green Potion"),
    ("Extreme Gold Potion", "极限金药水", "Extreme Gold Potion"),
    // Fixed actions
    ("Fixed actions", "固定动作", "Fixed actions"),
    ("Add action", "添加动作", "Add action"),
    (
        "Modify a fixed action",
        "修改固定动作",
        "Modify a fixed action",
    ),
    (
        "Add a new fixed action",
        "添加新固定动作",
        "Add a new fixed action",
    ),
    ("Use count", "使用次数", "Use count"),
    ("Hold for", "按住时长", "Hold for"),
    ("Holding buffered", "按住缓冲", "Holding buffered"),
    (
        "Require [Wait after buffered] to be enabled and without [Link key]. When enabled, the holding time will be added to [Wait after] during the last key use. Useful for holding down key and moving simultaneously.",
        "需要启用[缓冲后等待]且不设置[连锁键]。启用后，最后一次按键的按住时间会计入[使用后等待]。适合按住按键同时移动",
        "Require [Wait after buffered] to be enabled and without [Link key]. When enabled, the holding time will be added to [Wait after] during the last key use. Useful for holding down key and moving simultaneously.",
    ),
    ("Link key", "连锁键", "Link key"),
    ("Link key type", "连锁键类型", "Link key type"),
    ("Linked action", "连锁动作", "Linked action"),
    ("Use with", "配合使用", "Use with"),
    ("Use every", "每隔", "Use every"),
    ("Wait before use", "使用前等待", "Wait before use"),
    ("Wait random range", "随机等待范围", "Wait random range"),
    ("Wait after use", "使用后等待", "Wait after use"),
    ("Wait after buffered", "缓冲后等待", "Wait after buffered"),
    (
        "After the last key use, instead of waiting inplace, the bot is allowed to execute the next action partially. This can be useful for movable skill with casting animation.",
        "在最后一次按键使用后，机器人可以部分执行下一个动作，而不是原地等待。这对有施法动画的可移动技能很有用",
        "After the last key use, instead of waiting inplace, the bot is allowed to execute the next action partially. This can be useful for movable skill with casting animation.",
    ),
    ("Any", "任意", "Any"),
    ("Stationary", "站立", "Stationary"),
    ("Double jump", "二段跳", "Double jump"),
    ("Adjust", "调整", "Adjust"),
    // Actions
    (
        "Create an actions preset for the selected map...",
        "为所选地图创建动作预设...",
        "Create an actions preset for the selected map...",
    ),
    ("Action legends", "动作图例", "Action legends"),
    ("⟳ - Repeat", "⟳ - 重复", "⟳ - Repeat"),
    ("⏱︎  - Wait", "⏱︎  - 等待", "⏱︎  - Wait"),
    ("ㄨ - No position", "ㄨ - 无坐标", "ㄨ - No position"),
    ("⇈ - Queue to front", "⇈ - 排到最前", "⇈ - Queue to front"),
    ("⇆ - Any direction", "⇆ - 任意方向", "⇆ - Any direction"),
    ("← - Left direction", "← - 向左", "← - Left direction"),
    ("→ - Right direction", "→ - 向右", "→ - Right direction"),
    (
        "⁺ - Buffered wait after",
        "⁺ - 缓冲后等待",
        "⁺ - Buffered wait after",
    ),
    (
        "A ⤓ - Key A is held down",
        "A ⤓ - 按键 A 被按住",
        "A ⤓ - Key A is held down",
    ),
    (
        "A ~ B - Random range between A and B",
        "A ~ B - A 和 B 之间的随机范围",
        "A ~ B - Random range between A and B",
    ),
    (
        "A ↝ B - Use A key then B key",
        "A ↝ B - 先使用 A 键再使用 B 键",
        "A ↝ B - Use A key then B key",
    ),
    (
        "A ↜ B - Use B key then A key",
        "A ↜ B - 先使用 B 键再使用 A 键",
        "A ↜ B - Use B key then A key",
    ),
    (
        "A ↭ B - Use A and B keys at the same time",
        "A ↭ B - 同时使用 A 键和 B 键",
        "A ↭ B - Use A and B keys at the same time",
    ),
    (
        "A ↷ B - Use A key then B key while A is held down",
        "A ↷ B - 按住 A 键时先使用 A 键再使用 B 键",
        "A ↷ B - Use A key then B key while A is held down",
    ),
    ("Normal actions", "普通动作", "Normal actions"),
    (
        "Erda Shower off cooldown priority actions",
        "艾尔达之雨冷却结束优先动作",
        "Erda Shower off cooldown priority actions",
    ),
    (
        "Every milliseconds priority actions",
        "每毫秒优先动作",
        "Every milliseconds priority actions",
    ),
    (
        "Import/export actions",
        "导入/导出动作",
        "Import/export actions",
    ),
    // Action/Platform popups
    ("Modify platform", "修改平台", "Modify platform"),
    ("X start", "X 起点", "X start"),
    ("X end", "X 终点", "X end"),
    (
        "Modify mobbing bound",
        "修改刷怪边界",
        "Modify mobbing bound",
    ),
    ("X offset", "X 偏移", "X offset"),
    ("Y offset", "Y 偏移", "Y offset"),
    ("Modify mobbing key", "修改刷怪按键", "Modify mobbing key"),
    ("normal", "普通", "normal"),
    ("every milliseconds", "每毫秒", "every milliseconds"),
    (
        "Erda Shower off cooldown",
        "艾尔达之雨冷却结束",
        "Erda Shower off cooldown",
    ),
    ("linked", "连锁", "linked"),
    ("Modify a {} action", "修改{}动作", "Modify a {} action"),
    ("Add a new {} action", "添加新{}动作", "Add a new {} action"),
    // Action input
    ("Switch to key", "切换到按键", "Switch to key"),
    ("Switch to move", "切换到移动", "Switch to move"),
    ("X random range", "X 随机范围", "X random range"),
    ("Wait after move", "移动后等待", "Wait after move"),
    ("X range", "X 范围", "X range"),
    ("Positioned", "已定位", "Positioned"),
    ("Use direction", "使用方向", "Use direction"),
    ("Queue to front", "排到最前", "Queue to front"),
    // Platforms
    ("Platforms", "平台", "Platforms"),
    ("Rune pathing", "符文寻路", "Rune pathing"),
    ("Up jump only", "仅上跳", "Up jump only"),
    (
        "Auto-mobbing pathing",
        "自动刷怪寻路",
        "Auto-mobbing pathing",
    ),
    // Rotation
    ("Rotation", "动作循环", "Rotation"),
    ("Update mobbing key", "更新刷怪按键", "Update mobbing key"),
    (
        "Update mobbing bound",
        "更新刷怪边界",
        "Update mobbing bound",
    ),
    (
        "Auto mobbing uses key when pathing",
        "寻路时自动刷怪使用按键",
        "Auto mobbing uses key when pathing",
    ),
    (
        "Pathing means when the player is moving from one quad to another.",
        "寻路是指角色从一个象限移动到另一个象限",
        "Pathing means when the player is moving from one quad to another.",
    ),
    (
        "Detect mobs when pathing every",
        "寻路时检测怪物间隔",
        "Detect mobs when pathing every",
    ),
    (
        "Reset normal actions on Erda Shower resets",
        "艾尔达之雨重置时重置普通动作",
        "Reset normal actions on Erda Shower resets",
    ),
    // Runtime status tokens shown on the minimap
    ("Detecting", "检测中", "Detecting"),
    ("Idle", "空闲", "Idle"),
    ("UseKey", "使用按键", "UseKey"),
    ("Moving", "移动中", "Moving"),
    ("Adjusting", "微调中", "Adjusting"),
    ("DoubleJumping", "二段跳", "DoubleJumping"),
    ("Grappling", "抓钩中", "Grappling"),
    ("Jumping", "跳跃中", "Jumping"),
    ("UpJumping", "上跳中", "UpJumping"),
    ("Falling", "下落中", "Falling"),
    ("Unstucking", "解除卡住中", "Unstucking"),
    ("Stalling", "停顿中", "Stalling"),
    ("SolvingRune", "破解符文", "SolvingRune"),
    ("SolvingShape", "破解透明图形", "SolvingShape"),
    ("SolvingVioletta", "破解 Violetta", "SolvingVioletta"),
    ("CashShopThenExit", "现金商城后退出", "CashShopThenExit"),
    ("FamiliarsSwapping", "图鉴切换中", "FamiliarsSwapping"),
    ("Panicking", "应急中", "Panicking"),
    ("UsingBooster", "使用增幅器", "UsingBooster"),
    ("ExchangingBooster", "交换增幅器", "ExchangingBooster"),
    ("SolveRune", "破解符文", "SolveRune"),
    ("SolveShape", "破解透明图形", "SolveShape"),
    ("SolveVioletta", "破解 Violetta", "SolveVioletta"),
    ("FamiliarsSwap", "切换图鉴", "FamiliarsSwap"),
    ("Panic", "应急", "Panic"),
    ("UseBooster", "使用增幅器", "UseBooster"),
    ("ExchangeBooster", "交换增幅器", "ExchangeBooster"),
    ("Unstuck", "解除卡住", "Unstuck"),
    ("AutoMob", "自动刷怪", "AutoMob"),
    ("PingPong", "往返刷怪", "PingPong"),
    ("SendInput", "系统输入", "SendInput"),
    ("RPC", "RPC", "RPC"),
    ("Left", "左", "Left"),
    ("Right", "右", "Right"),
    ("Waiting", "等待中", "Waiting"),
    ("Solving", "破解中", "Solving"),
    ("Completed", "已完成", "Completed"),
    ("OpenMenu", "打开菜单", "OpenMenu"),
    ("FindSlots", "查找槽位", "FindSlots"),
    ("FreeSlots", "释放槽位", "FreeSlots"),
    ("FindCards", "查找卡片", "FindCards"),
    ("Swapping", "切换中", "Swapping"),
    ("Scrolling", "滚动中", "Scrolling"),
    ("Saving", "保存中", "Saving"),
    ("Completing", "完成中", "Completing"),
    ("Preparing", "准备中", "Preparing"),
    ("Move", "移动", "Move"),
    // Enum option labels rendered by generic selects
    ("BitBlt", "BitBlt", "BitBlt"),
    (
        "Windows 10 (1903 and up)",
        "Windows 10（1903 及以上）",
        "Windows 10 (1903 and up)",
    ),
    ("BitBltArea", "BitBltArea", "BitBltArea"),
    ("Rpc", "RPC", "Rpc"),
    ("Discord", "Discord", "Discord"),
    ("EveryMillis", "每隔", "EveryMillis"),
    ("Percentage", "百分比", "Percentage"),
    ("Full", "已满", "Full"),
    ("AtLeastOne", "至少一个", "AtLeastOne"),
    ("CycleChannel", "循环换线", "CycleChannel"),
    ("All", "全部", "All"),
    ("Last", "最后一个", "Last"),
    ("SecondAndLast", "倒数第二个和最后一个", "SecondAndLast"),
    ("StartToEnd", "起点到终点", "StartToEnd"),
    (
        "StartToEndThenReverse",
        "起点到终点后反向",
        "StartToEndThenReverse",
    ),
    ("AutoMobbing", "自动刷怪", "AutoMobbing"),
    ("DoubleJump", "二段跳", "DoubleJump"),
    ("Interruptible", "可中断", "Interruptible"),
    ("Uninterruptible", "不可中断", "Uninterruptible"),
    ("Before", "前置", "Before"),
    ("After", "后置", "After"),
    ("AtTheSame", "同时", "AtTheSame"),
    ("Along", "伴随", "Along"),
];

static TRANSLATION_MAP: LazyLock<HashMap<&'static str, (&'static str, &'static str)>> =
    LazyLock::new(|| {
        TRANSLATIONS
            .iter()
            .copied()
            .map(|(key, zh, en)| (key, (zh, en)))
            .collect()
    });

/// Translates a static UI key to the current language.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Translator {
    language: UiLanguage,
}

impl Translator {
    pub fn new(language: UiLanguage) -> Self {
        Self { language }
    }

    /// Returns the translation for `key`.
    ///
    /// Falls back to English and then to the key itself so partially migrated
    /// screens still render something readable.
    pub fn t(&self, key: &'static str) -> &'static str {
        match TRANSLATION_MAP.get(key) {
            Some((zh, en)) => match self.language {
                UiLanguage::Zh => zh,
                UiLanguage::En => en,
            },
            None => key,
        }
    }

    /// Returns the translation for `key` with `{}` placeholders replaced by
    /// the given arguments in order.
    pub fn t_fmt(&self, key: &'static str, args: &[&str]) -> String {
        let template = self.t(key);
        let mut result = String::with_capacity(template.len());
        let mut args = args.iter();
        let mut rest = template;
        while let Some(index) = rest.find("{}") {
            result.push_str(&rest[..index]);
            match args.next() {
                Some(arg) => result.push_str(arg),
                None => result.push_str("{}"),
            }
            rest = &rest[index + 2..];
        }
        result.push_str(rest);
        result
    }

    /// Translates a runtime status value (e.g. `Moving(1, 2)` or `PingPong(Left)`)
    /// shown on the minimap. Known tokens are translated, unknown ones are kept.
    pub fn state_text(&self, value: &str) -> String {
        if let Some((zh, en)) = TRANSLATION_MAP.get(value) {
            return match self.language {
                UiLanguage::Zh => (*zh).to_string(),
                UiLanguage::En => (*en).to_string(),
            };
        }

        let (prefix, inner) = match value.split_once('(') {
            Some((prefix, rest)) => (prefix, rest.strip_suffix(')').unwrap_or(rest)),
            None => (value, ""),
        };

        let prefix = self.token(prefix);
        if inner.is_empty() {
            return prefix.to_string();
        }

        let inner = inner
            .split(',')
            .map(|part| self.token(part.trim()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{prefix}({inner})")
    }

    fn token<'a>(&self, token: &'a str) -> &'a str {
        match TRANSLATION_MAP.get(token) {
            Some((zh, en)) => match self.language {
                UiLanguage::Zh => zh,
                UiLanguage::En => en,
            },
            None => token,
        }
    }
}

/// Creates a memoized translator bound to the current UI language setting.
///
/// Reading the returned memo inside component render code subscribes the
/// component to language changes so the UI updates immediately on switch.
pub fn use_translator() -> Memo<Translator> {
    let settings = use_context::<AppState>().settings;
    use_memo(move || {
        Translator::new(
            settings()
                .map(|settings| settings.ui_language)
                .unwrap_or_default(),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_language_is_chinese() {
        let translator = Translator::new(UiLanguage::default());

        assert_eq!(translator.t("Settings"), "设置");
    }

    #[test]
    fn english_keeps_original_copy() {
        let translator = Translator::new(UiLanguage::En);

        assert_eq!(translator.t("Settings"), "Settings");
    }

    #[test]
    fn unknown_key_falls_back_to_english() {
        let translator = Translator::new(UiLanguage::Zh);

        assert_eq!(translator.t("Not translated yet"), "Not translated yet");
    }

    #[test]
    fn format_translation_replaces_placeholders() {
        let translator = Translator::new(UiLanguage::Zh);

        assert_eq!(
            translator.t_fmt("Modify a {} action", &["普通"]),
            "修改普通动作"
        );
    }

    #[test]
    fn state_text_translates_known_tokens_and_keeps_unknown() {
        let translator = Translator::new(UiLanguage::Zh);

        assert_eq!(translator.state_text("PingPong(Left)"), "往返刷怪(左)");
        assert_eq!(
            translator.state_text("SolvingShape(Waiting)"),
            "破解透明图形(等待中)"
        );
        assert_eq!(translator.state_text("Idle(1, 2)"), "空闲(1, 2)");
        assert_eq!(translator.state_text("MysteryState"), "MysteryState");
    }
}
