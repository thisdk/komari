//! UI internationalization.
//!
//! Every user-visible string lives in the [`Key`] table below together with its
//! English and Simplified Chinese translation. Components translate through
//! [`I18n::t`], which is backed by the persisted [`Language`] setting so the UI
//! re-renders as soon as the language is changed.
//!
//! Game-specific vocabulary (item, skill and system names such as `Sol Erda`,
//! `HEXA Booster`, `Erda Shower`, `Familiar` or `PingPong`) is deliberately left
//! in English: those names are what the game client and the bundled detection
//! templates show, so translating them would make the settings harder to match
//! against the game.
//!
//! Entries whose text contains `{name}` are templates; use [`I18n::format`].

// Translation keys are named after the text they hold, so several of them
// legitimately end in `Key` (`Key::CommonKey` is the "Key" label and
// `Key::ActionLinkKey` the "Link key" one). That repetition is the content of
// the table rather than the redundant naming this lint looks for, and the names
// are what makes the table greppable from the game's own wording.
#![allow(clippy::enum_variant_names)]

use backend::{
    ActionKeyDirection, ActionKeyWith, CaptureMode, EliteBossBehavior,
    ExchangeHexaBoosterCondition, InputMethod, Language, LinkKeyBinding, PotionMode, RotationMode,
    SwappableFamiliars, WaitAfterBuffered, WebhookProvider,
};
use dioxus::prelude::*;

/// The placeholder replaced by [`I18n::format`].
const ARG: &str = "{name}";

macro_rules! translations {
    ($( $(#[$attr:meta])* $key:ident => ($en:literal, $zh:literal) ),* $(,)?) => {
        /// A user-visible string.
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum Key {
            $( $(#[$attr])* $key ),*
        }

        impl Key {
            /// Translates this key into `language`.
            pub fn translate(self, language: Language) -> &'static str {
                match language {
                    Language::English => match self { $( $(#[$attr])* Key::$key => $en ),* },
                    Language::SimplifiedChinese => match self { $( $(#[$attr])* Key::$key => $zh ),* },
                }
            }
        }
    };
}

translations! {
    // Tabs
    TabActions => ("Actions", "动作"),
    TabCharacters => ("Characters", "角色"),
    TabSettings => ("Settings", "设置"),
    TabLocalization => ("Localization", "模板本地化"),
    #[cfg(debug_assertions)]
    TabDebug => ("Debug", "调试"),

    // Shared
    CommonEnabled => ("Enabled", "启用"),
    CommonSave => ("Save", "保存"),
    CommonAdd => ("Add", "添加"),
    CommonCancel => ("Cancel", "取消"),
    CommonCopy => ("Copy", "复制"),
    CommonImport => ("Import", "导入"),
    CommonExport => ("Export", "导出"),
    CommonUpdate => ("Update", "更新"),
    CommonDelete => ("Delete", "删除"),
    CommonCreate => ("Create", "创建"),
    CommonReset => ("Reset", "重置"),
    CommonReplace => ("Replace", "替换"),
    CommonMode => ("Mode", "模式"),
    CommonAmount => ("Amount", "数量"),
    CommonCount => ("Count", "数量"),
    CommonKey => ("Key", "按键"),
    CommonNone => ("None", "无"),
    CommonUnknown => ("Unknown", "未知"),
    CommonYes => ("Yes", "是"),
    CommonNo => ("No", "否"),
    CommonOptional => ("(optional)", "(可选)"),
    CommonEnterName => ("Enter a name...", "请输入名称..."),
    CommonClickToSet => ("Click to set", "点击设置"),
    CommonPressAnyKey => ("Press any key...", "请按下任意键..."),
    CommonAny => ("Any", "任意"),
    CommonStationary => ("Stationary", "静止"),
    CommonDoubleJump => ("Double jump", "二段跳"),
    CommonAddAction => ("Add action", "添加动作"),
    CommonAdjustSuffix => (" / Adjust", " / 调整"),

    // Action condition names and popup titles
    ActionNameNormal => ("normal", "普通"),
    ActionNameEveryMillis => ("every milliseconds", "定时"),
    ActionNameErdaShower => ("Erda Shower off cooldown", "Erda Shower 冷却"),
    ActionNameLinked => ("linked", "链接"),
    ActionTitleModify => ("Modify a {name} action", "修改{name}动作"),
    ActionTitleAddNew => ("Add a new {name} action", "添加{name}动作"),
    ActionSwitchToKey => ("Switch to key", "切换为按键"),
    ActionSwitchToMove => ("Switch to move", "切换为移动"),
    ActionAdjust => ("Adjust", "自动调整"),
    ActionXRandomRange => ("X random range", "X 随机范围"),
    ActionXRange => ("X range", "X 范围"),
    ActionWaitAfterMove => ("Wait after move", "移动后等待"),
    ActionLinkedAction => ("Linked action", "链接动作"),
    ActionUseCount => ("Use count", "使用次数"),
    ActionHoldFor => ("Hold for", "长按"),
    ActionHoldingBuffered => ("Holding buffered", "长按缓冲"),
    ActionHoldingBufferedTooltip => (
        "Require [Wait after buffered] to be enabled and without [Link key]. When enabled, the holding time will be added to [Wait after] during the last key use. Useful for holding down key and moving simultaneously.",
        "需要启用 [使用后缓冲等待] 且未设置 [链接按键]。启用后，最后一次按键的长按时长会计入 [使用后等待]，适合需要按住按键同时移动的技能。"
    ),
    ActionLinkKey => ("Link key", "链接按键"),
    ActionLinkKeyType => ("Link key type", "链接方式"),
    ActionUseWith => ("Use with", "使用条件"),
    ActionUseDirection => ("Use direction", "使用方向"),
    ActionQueueToFront => ("Queue to front", "优先执行"),
    ActionUseEvery => ("Use every", "使用间隔"),
    ActionWaitBeforeUse => ("Wait before use", "使用前等待"),
    ActionWaitRandomRange => ("Wait random range", "随机等待范围"),
    ActionWaitAfterUse => ("Wait after use", "使用后等待"),
    ActionWaitAfterBuffered => ("Wait after buffered", "使用后缓冲等待"),
    ActionWaitAfterBufferedTooltip => (
        "After the last key use, instead of waiting inplace, the bot is allowed to execute the next action partially. This can be useful for movable skill with casting animation.",
        "最后一次按键后不再原地等待，而是允许提前执行下一个动作。适合带有施法动作的可移动技能。"
    ),
    ActionPositioned => ("Positioned", "指定坐标"),

    // Action legends
    ActionLegends => ("Action legends", "动作图例"),
    LegendRepeat => ("⟳ - Repeat", "⟳ - 循环"),
    LegendWait => ("⏱︎  - Wait", "⏱︎  - 等待"),
    LegendNoPosition => ("ㄨ - No position", "ㄨ - 无坐标"),
    LegendQueueToFront => ("⇈ - Queue to front", "⇈ - 优先执行"),
    LegendAnyDirection => ("⇆ - Any direction", "⇆ - 任意方向"),
    LegendLeftDirection => ("← - Left direction", "← - 向左"),
    LegendRightDirection => ("→ - Right direction", "→ - 向右"),
    LegendBufferedWait => ("⁺ - Buffered wait after", "⁺ - 使用后缓冲等待"),
    LegendHeldKey => ("A ⤓ - Key A is held down", "A ⤓ - 长按 A 键"),
    LegendRandomRange => ("A ~ B - Random range between A and B", "A ~ B - 在 A 与 B 之间随机"),
    LegendLinkBefore => ("A ↝ B - Use A key then B key", "A ↝ B - 先按 A 再按 B"),
    LegendLinkAfter => ("A ↜ B - Use B key then A key", "A ↜ B - 先按 B 再按 A"),
    LegendLinkSame => ("A ↭ B - Use A and B keys at the same time", "A ↭ B - 同时按下 A 和 B"),
    LegendLinkAlong => ("A ↷ B - Use A key then B key while A is held down", "A ↷ B - 按住 A 的同时按 B"),

    // Actions screen
    ActionsPresetPlaceholder => ("Create an actions preset for the selected map...", "为所选地图创建动作预设..."),
    SectionNormalActions => ("Normal actions", "普通动作"),
    SectionErdaPriorityActions => ("Erda Shower off cooldown priority actions", "Erda Shower 冷却优先动作"),
    SectionMillisPriorityActions => ("Every milliseconds priority actions", "定时优先动作"),
    SectionImportExportActions => ("Import/export actions", "导入/导出动作"),
    SectionPlatforms => ("Platforms", "平台"),
    PlatformsRunePathing => ("Rune pathing", "符文寻路"),
    PlatformsUpJumpOnly => ("Up jump only", "仅上跳"),
    PlatformsAutoMobbingPathing => ("Auto-mobbing pathing", "自动刷怪寻路"),
    PlatformsAddPlatform => ("Add platform", "添加平台"),

    // Platform / mobbing popups
    PopupModifyPlatform => ("Modify platform", "修改平台"),
    PopupXStart => ("X start", "X 起点"),
    PopupXEnd => ("X end", "X 终点"),
    PopupModifyMobbingBound => ("Modify mobbing bound", "修改刷怪范围"),
    PopupXOffset => ("X offset", "X 偏移"),
    PopupYOffset => ("Y offset", "Y 偏移"),
    PopupWidth => ("Width", "宽度"),
    PopupHeight => ("Height", "高度"),
    PopupModifyMobbingKey => ("Modify mobbing key", "修改刷怪按键"),

    // Rotation
    SectionRotation => ("Rotation", "循环"),
    RotationUpdateMobbingKey => ("Update mobbing key", "修改刷怪按键"),
    RotationUpdateMobbingBound => ("Update mobbing bound", "修改刷怪范围"),
    RotationAutoMobbingUsesKey => ("Auto mobbing uses key when pathing", "寻路时自动刷怪使用按键"),
    RotationPathingTooltip => (
        "Pathing means when the player is moving from one quad to another.",
        "寻路指角色从一个区域移动到另一个区域的过程。"
    ),
    RotationDetectMobsEvery => ("Detect mobs when pathing every", "寻路时怪物检测间隔"),
    RotationResetNormalActions => ("Reset normal actions on Erda Shower resets", "Erda Shower 重置时重置普通动作"),

    // Settings
    SettingsLanguage => ("Language", "语言"),
    SettingsInterfaceLanguage => ("Interface language", "界面语言"),
    SettingsLanguageTooltip => (
        "The language the interface is displayed in. Takes effect immediately.",
        "界面显示所使用的语言，修改后立即生效。"
    ),
    SettingsCapture => ("Capture", "画面捕获"),
    SettingsHandle => ("Handle", "捕获窗口"),
    SettingsDefault => ("Default", "默认"),
    SettingsRefreshHandles => ("Refresh handles", "刷新窗口"),
    SettingsInput => ("Input", "输入"),
    SettingsMethod => ("Method", "方式"),
    SettingsRpcServerUrl => ("RPC server URL", "RPC 服务器地址"),
    SettingsNotifications => ("Notifications", "通知"),
    SettingsWebhookProvider => ("Webhook provider", "Webhook 服务商"),
    SettingsWebhookUrl => ("Webhook URL", "Webhook 地址"),
    SettingsDiscordUserId => ("Discord ping user ID", "Discord 提醒用户 ID"),
    SettingsNotifyRuneSpawns => ("Rune spawns", "符文出现"),
    SettingsNotifyEliteBoss => ("Elite boss spawns", "精英 boss 出现"),
    SettingsNotifyPlayerDies => ("Player dies", "角色死亡"),
    SettingsNotifyGuildie => ("Guildie appears", "公会成员出现"),
    SettingsNotifyStranger => ("Stranger appears", "陌生人出现"),
    SettingsNotifyFriend => ("Friend appears", "好友出现"),
    SettingsNotifyFailOrChangeMap => ("Detection fails or map changes", "检测失败或切换地图"),
    SettingsNotifyLieDetector => ("Lie detector appears", "测谎仪出现"),
    SettingsNotifyRunTimerEnd => ("Run timer ends", "计时结束"),
    SettingsHotkeys => ("Hotkeys", "快捷键"),
    SettingsHotkeyToggleActions => ("Toggle start/stop actions", "开始/停止动作"),
    SettingsHotkeyAddPlatform => ("Add platform", "添加平台"),
    SettingsHotkeyPlatformStart => ("Mark platform start", "标记平台起点"),
    SettingsHotkeyPlatformEnd => ("Mark platform end", "标记平台终点"),
    SettingsRunTimer => ("Run timer", "运行计时"),
    SettingsRunTimerDuration => ("Duration (hh:mm:ss)", "时长 (hh:mm:ss)"),
    SettingsOthers => ("Others", "其他"),
    SettingsEnableRuneSolving => ("Enable rune solving", "启用符文解谜"),
    SettingsEnableTransparentShapeSolving => ("Enable transparent shape solving", "启用透明图形解谜"),
    SettingsEnableViolettaSolving => ("Enable Violetta solving", "启用 Violetta 解谜"),
    SettingsEnablePanicMode => ("Enable panic mode", "启用紧急模式"),
    SettingsStopOnFailOrChangeMap => ("Stop actions on fail or map changed", "检测失败或切换地图时停止"),
    SettingsStopOnPlayerDies => ("Stop actions on player dies", "角色死亡时停止"),

    // Minimap
    MinimapCreateMap => ("Create a map...", "创建地图..."),
    MinimapBulkImport => ("Bulk import", "批量导入"),
    MinimapStart => ("Start", "开始"),
    MinimapStop => ("Stop", "停止"),
    MinimapSuspend => ("Suspend", "暂停"),
    MinimapResume => ("Resume", "继续"),
    MinimapRedetect => ("Re-detect", "重新检测"),
    InfoState => ("State", "状态"),
    InfoPosition => ("Position", "坐标"),
    InfoPriorityAction => ("Priority action", "优先动作"),
    InfoNormalAction => ("Normal action", "普通动作"),
    InfoErdaShower => ("Erda Shower", "Erda Shower"),
    InfoDetectedSize => ("Detected size", "检测尺寸"),
    InfoSelectedSize => ("Selected size", "地图尺寸"),
    InfoTimeUntilStop => ("Time Until Stop", "剩余时间"),
    InfoRunTime => ("Run Time", "运行时长"),
    InfoInputMethod => ("Input method", "输入方式"),
    InfoUseGpu => ("Use GPU", "GPU 加速"),
    InfoLieDetectors => ("Lie Detectors", "测谎仪次数"),

    // Characters
    CharactersCreate => ("Create a character...", "创建角色..."),
    SectionUsePotionAndFeedPet => ("Use potion and feed pet", "使用药水与喂养宠物"),
    CharactersFeedKey => ("Feed key", "喂养按键"),
    CharactersEveryMmSs => ("Every (mm:ss)", "间隔 (mm:ss)"),
    CharactersPotionKey => ("Potion key", "药水按键"),
    CharactersHpBelow => ("HP below", "HP 低于"),
    CharactersHpUpdateEvery => ("HP update every", "HP 检测间隔"),
    SectionUseBooster => ("Use booster", "使用 Booster"),
    CharactersGenericBoosterKey => ("Generic Booster key", "通用 Booster 按键"),
    CharactersHexaBoosterKey => ("HEXA Booster key", "HEXA Booster 按键"),
    CharactersExchangeWhenSolErda => ("Exchange when Sol Erda", "Sol Erda 兑换条件"),
    CharactersExchangeTooltip => (
        "Requires HEXA Booster to be visible in quick slots, Sol Erda tracker menu opened and HEXA Matrix configured in the quick menu. Exchange will only happen if there is no HEXA Booster.",
        "需要快捷栏中可见 HEXA Booster、已打开 Sol Erda 追踪菜单，并在快捷菜单中配置了 HEXA Matrix。仅在当前没有 HEXA Booster 时才会兑换。"
    ),
    CharactersExchangeAll => ("Exchange all", "全部兑换"),
    SectionMovement => ("Movement", "移动"),
    MovementUpJumpIsFlight => ("Up jump is flight", "上跳为飞行"),
    MovementUpJumpIsFlightTooltip => (
        "Applicable only to mage class or when non-up-arrow up jump key is set.",
        "仅适用于法师职业，或上跳按键不是方向上键时。"
    ),
    MovementJumpThenUpJump => ("Jump then up jump if possible", "尽可能先跳跃再上跳"),
    MovementJumpThenUpJumpTooltip => (
        "Applicable only for non-mage class and when non-up-arrow up jump key is set.",
        "仅适用于非法师职业，且上跳按键不是方向上键时。"
    ),
    MovementFallTeleportRange => ("Fall teleport range", "下落传送范围"),
    MovementFallTeleportTooltip => (
        "Maximum y distance to teleport when falling instead of jumping down.",
        "下落时使用传送代替向下跳跃的最大 y 距离。"
    ),
    MovementUpJumpTeleportRange => ("Up jump teleport range", "上跳传送范围"),
    MovementUpJumpTeleportTooltip => (
        "Minimum y distance to use teleport with jump when up jumping.",
        "上跳时配合跳跃使用传送的最小 y 距离。"
    ),
    MovementDisableTeleportOnFall => ("Disable teleport on fall", "下落时禁用传送"),
    MovementDisableTeleportTooltip => ("Applicable only to mage class.", "仅适用于法师职业。"),
    MovementAttackWhenPathing => ("Attack when pathing (PingPong)", "寻路时攻击 (PingPong)"),
    MovementAttackWhenPathingTooltip => (
        "Attacks with the PingPong key while pathing to a target (e.g. rune) until within 5 distance of the target.",
        "寻路前往目标（例如符文）时使用 PingPong 按键攻击，直到距离目标 5 以内。"
    ),
    MovementDisableDoubleJumping => ("Disable double jumping", "禁用二段跳"),
    MovementDisableDoubleJumpingTooltip => (
        "Not applicable if an action requires double jumping.",
        "如果有动作需要二段跳则不生效。"
    ),
    MovementDisableGrapple => ("Disable grapple on double jumping", "二段跳时禁用绳索"),
    MovementDisableGrappleTooltip => ("Applicable only if grapple key is set.", "仅在设置了绳索按键时生效。"),
    MovementDisableWalking => ("Disable walking", "禁用行走"),
    MovementDisableWalkingTooltip => (
        "Not applicable if an action requires adjusting.",
        "如果有动作需要自动调整则不生效。"
    ),
    SectionFamiliars => ("Familiars", "Familiar"),
    CharactersSwappableSlots => ("Swappable slots", "可交换槽位"),
    CharactersSwapCheckEvery => ("Swap check every (mm:ss)", "交换检测间隔 (mm:ss)"),
    CharactersSwappingEnabled => ("Swapping enabled", "启用交换"),
    CharactersCanSwapRare => ("Can swap rare familiars", "允许交换稀有 Familiar"),
    CharactersCanSwapEpic => ("Can swap epic familiars", "允许交换史诗 Familiar"),
    CharactersLinkKeyTiming => ("Link key timing", "链接按键间隔"),
    CharactersEliteBossBehavior => ("Elite boss spawns behavior", "精英 boss 出现行为"),
    CharactersKeyToUse => ("Key to use", "使用的按键"),

    // Key bindings
    SectionKeyBindings => ("Key bindings", "按键设置"),
    BindingsRopeLift => ("Rope lift", "绳索"),
    BindingsTeleport => ("Teleport", "传送"),
    BindingsJump => ("Jump", "跳跃"),
    BindingsUpJump => ("Up jump", "上跳"),
    BindingsUpJumpTooltip => (
        "This is meant for classes that have a separate skill to up jump. Classes that use up arrow should set this key to up arrow.",
        "用于拥有独立上跳技能的职业。使用方向上键上跳的职业，请将此按键设置为方向上键。"
    ),
    BindingsInteract => ("Interact", "交互"),
    BindingsCashShop => ("Cash shop", "现金商店"),
    BindingsCashShopTooltip => (
        "Cash shop is used to reset spin rune to a normal rune. This only happens if solving rune fails 8 times consecutively.",
        "现金商店用于将旋转符文重置为普通符文，仅在连续 8 次符文解谜失败时触发。"
    ),
    BindingsToTown => ("To town", "回城"),
    BindingsToTownTooltip => (
        "This key must be set to use navigation or run/stop cycle features.",
        "使用导航或运行/停止循环功能时必须设置此按键。"
    ),
    BindingsChangeChannel => ("Change channel", "切换频道"),
    BindingsChangeChannelTooltip => (
        "This key must be set to use panic mode or elite boss spawns behavior features.",
        "使用紧急模式或精英 boss 出现行为功能时必须设置此按键。"
    ),
    BindingsFamiliarMenu => ("Familiar menu", "Familiar 菜单"),
    BindingsFamiliarMenuTooltip => (
        "This key must be set to use familiars swapping feature.",
        "使用 Familiar 交换功能时必须设置此按键。"
    ),

    // Buffs
    SectionBuffs => ("Buffs", "增益"),
    BuffsFamiliarSkill => ("Familiar skill", "Familiar 技能"),
    BuffsFamiliarEssence => ("Familiar essence", "Familiar 精华"),

    // Fixed actions
    SectionFixedActions => ("Fixed actions", "固定动作"),
    CharactersModifyFixedAction => ("Modify a fixed action", "修改固定动作"),
    CharactersAddNewFixedAction => ("Add a new fixed action", "添加固定动作"),

    // Template localization
    LocalizationInfo => ("Info", "说明"),
    LocalizationHeaderSection => ("Section", "分类"),
    LocalizationHeaderFunction => ("Function", "功能"),
    LocalizationHeaderTemplates => ("Template(s)", "模板"),
    LocalizationSectionPopups => ("Popups", "弹窗"),
    LocalizationDescUnstuck => (
        "Unstuck player through closing menu, popup, dialog, etc.",
        "通过关闭菜单、弹窗、对话框等方式解除角色卡死。"
    ),
    LocalizationDescAllPopups => ("All popups.", "所有弹窗。"),
    LocalizationDescGoToTown => (
        "Go to town confirmation and save familiars setup.",
        "回城确认弹窗，并保存 Familiar 配置。"
    ),
    LocalizationDescConfirmPopup => ("Confirm popup.", "确认弹窗。"),
    LocalizationDescRespawn => ("Respawn on player death.", "角色死亡后复活。"),
    LocalizationDescOkNewPopup => ("Ok (new) popup.", "Ok（新版）弹窗。"),
    LocalizationSectionFamiliars => ("Familiars", "Familiar"),
    LocalizationDescSortFamiliar => (
        "Sort familiar cards by level before swapping.",
        "交换前按等级排序 Familiar 卡片。"
    ),
    LocalizationDescLevelSortButton => (
        "Familiar menu setup tab's setup level sort button.",
        "Familiar 菜单设置页的等级排序按钮。"
    ),
    LocalizationDescSaveFamiliars => ("Save familiars setup after swapping.", "交换后保存 Familiar 配置。"),
    LocalizationDescSaveButton => (
        "Familiar menu setup tab's save button.",
        "Familiar 菜单设置页的保存按钮。"
    ),
    LocalizationSectionHexa => ("HEXA", "HEXA"),
    LocalizationDescErdaMenu => (
        "Open Sol Erda version menu in HEXA Matrix.",
        "在 HEXA Matrix 中打开 Sol Erda 版本菜单。"
    ),
    LocalizationDescErdaConversionButton => ("Erda conversion button.", "Erda 兑换按钮。"),
    LocalizationDescBoosterExchangeMenu => (
        "Open HEXA Booster exchange menu.",
        "打开 HEXA Booster 兑换菜单。"
    ),
    LocalizationDescHexaBoosterButton => ("HEXA Booster button.", "HEXA Booster 按钮。"),
    LocalizationDescMaxAmount => (
        "Select max HEXA Booster amount to exchange.",
        "选择要兑换的 HEXA Booster 最大数量。"
    ),
    LocalizationDescMaxButton => ("Max button.", "Max 按钮。"),
    LocalizationDescConvert => (
        "Convert Sol Erda to HEXA Booster.",
        "将 Sol Erda 兑换为 HEXA Booster。"
    ),
    LocalizationDescConvertButton => ("Convert button.", "兑换按钮。"),
    LocalizationSectionOthers => ("Others", "其他"),
    LocalizationDescChangeChannelMenu => (
        "Detect whether change channel menu is opened.",
        "检测是否已打开切换频道菜单。"
    ),
    LocalizationDescChangeChannelText => ("Change channel text.", "切换频道文字。"),
    LocalizationDescCashShop => (
        "Detect whether player entered cash shop.",
        "检测角色是否进入了现金商店。"
    ),
    LocalizationDescCashShopText => ("Cash shop text.", "现金商店文字。"),
    LocalizationDescBoosterInUse => (
        "Detect whether Generic/HEXA booster is in use.",
        "检测是否正在使用通用/HEXA Booster。"
    ),
    LocalizationDescTimerText => ("Timer text.", "计时器文字。"),
    LocalizationDescLieDetector => ("Detect lie detector event.", "检测测谎仪事件。"),
    LocalizationDescLieDetectorTitle => ("Lie detector title.", "测谎仪标题。"),
    LocalizationCaptureColor => ("Capture color", "截取彩色画面"),
    LocalizationCaptureGrayscale => ("Capture grayscale", "截取灰度画面"),
    LocalizationGrayscaleTooltip => ("This template is in grayscale.", "此模板为灰度图。"),
    LocalizationErdaConversionButton => ("Erda conversion button", "Erda 兑换按钮"),
    LocalizationHexaBoosterButton => ("HEXA Booster button", "HEXA Booster 按钮"),
    LocalizationMaxButton => ("Max button", "Max 按钮"),
    LocalizationConvertButton => ("Convert button", "兑换按钮"),
    LocalizationLevelSortButton => ("Level sort button", "等级排序按钮"),
    LocalizationSaveButton => ("Save button", "保存按钮"),
    LocalizationCashShop => ("Cash shop", "现金商店"),
    LocalizationChangeChannel => ("Change channel", "切换频道"),
    LocalizationTimer => ("Timer", "计时器"),
    LocalizationLieDetectorNew => ("Lie detector (new)", "测谎仪（新版）"),
    LocalizationLieDetectorOld => ("Lie detector (old)", "测谎仪（旧版）"),

    // Debug
    #[cfg(debug_assertions)]
    DebugSection => ("Debug", "调试"),
    #[cfg(debug_assertions)]
    DebugTestSpinRune => ("Test spin rune", "测试旋转符文"),
    #[cfg(debug_assertions)]
    DebugTestVioletta => ("Test Violetta", "测试 Violetta"),
    #[cfg(debug_assertions)]
    DebugTestShapeNormal => ("Test transparent shape normal", "测试透明图形（简单）"),
    #[cfg(debug_assertions)]
    DebugTestShapeHard => ("Test transparent shape hard", "测试透明图形（困难）"),
    #[cfg(debug_assertions)]
    DebugTestShapeFile => ("Test transparent shape...", "测试透明图形..."),
    #[cfg(debug_assertions)]
    DebugStartRecording => ("Start recording", "开始录制"),
    #[cfg(debug_assertions)]
    DebugStopRecording => ("Stop recording", "停止录制"),
    #[cfg(debug_assertions)]
    DebugStartAutoSaveRune => ("Start auto saving rune", "开始自动保存符文"),
    #[cfg(debug_assertions)]
    DebugStopAutoSaveRune => ("Stop auto saving rune", "停止自动保存符文"),
    #[cfg(debug_assertions)]
    DebugStartAutoRecordLieDetector => ("Start auto record lie detector", "开始自动录制测谎仪"),
    #[cfg(debug_assertions)]
    DebugStopAutoRecordLieDetector => ("Stop auto record lie detector", "停止自动录制测谎仪"),

    // Values of settings enums
    EnumCaptureModeBitBlt => ("BitBlt", "BitBlt"),
    EnumCaptureModeWindowsGraphicsCapture => ("Windows 10 (1903 and up)", "Windows 10（1903 及以上）"),
    EnumCaptureModeBitBltArea => ("BitBlt (area)", "BitBlt（区域）"),
    EnumInputMethodDefault => ("Default", "默认"),
    EnumInputMethodRpc => ("RPC", "RPC"),
    EnumWebhookProviderDiscord => ("Discord", "Discord"),
    EnumRotationStartToEnd => ("Start to end", "起点到终点"),
    EnumRotationStartToEndThenReverse => ("Start to end then reverse", "起点到终点再返回"),
    EnumRotationAutoMobbing => ("Auto-mobbing", "自动刷怪"),
    EnumRotationPingPong => ("PingPong", "PingPong"),
    EnumDirectionLeft => ("Left", "向左"),
    EnumDirectionRight => ("Right", "向右"),
    EnumWaitAfterBufferedInterruptible => ("Interruptible", "可中断"),
    EnumWaitAfterBufferedUninterruptible => ("Uninterruptible", "不可中断"),
    EnumEliteBossCycleChannel => ("Cycle channel", "切换频道"),
    EnumEliteBossUseKey => ("Use key", "使用按键"),
    EnumExchangeFull => ("Full", "满时"),
    EnumExchangeAtLeastOne => ("At least one", "至少一个"),
    EnumSwappableAll => ("All", "全部"),
    EnumSwappableLast => ("Last", "最后一个"),
    EnumSwappableSecondAndLast => ("Second and last", "倒数第二与最后一个"),
    EnumPotionEveryMillis => ("Every milliseconds", "按间隔"),
    EnumPotionPercentage => ("HP percentage", "按 HP 百分比"),
    EnumLinkKeyBefore => ("Before ({name})", "先按 {name}"),
    EnumLinkKeyAtTheSame => ("At the same time ({name})", "同时按 {name}"),
    EnumLinkKeyAfter => ("After ({name})", "后按 {name}"),
    EnumLinkKeyAlong => ("Along ({name})", "按住 {name}"),
}

/// The currently selected language, provided as a Dioxus context.
#[derive(Clone, Copy, PartialEq)]
pub struct I18n {
    language: Memo<Language>,
}

impl I18n {
    /// Creates a translator for `language`.
    pub fn new(language: Memo<Language>) -> Self {
        Self { language }
    }

    /// The language translations are currently rendered in.
    #[inline]
    pub fn language(self) -> Language {
        (self.language)()
    }

    /// Translates `key`.
    #[inline]
    pub fn t(self, key: Key) -> &'static str {
        key.translate(self.language())
    }

    /// Translates the `key` template, replacing its `{name}` placeholder with `name`.
    pub fn format(self, key: Key, name: &str) -> String {
        self.t(key).replace(ARG, name)
    }

    /// The label of a value shown in a select.
    #[inline]
    pub fn label<T: LocalizedLabel>(self, value: &T) -> String {
        value.localized_label(self)
    }
}

/// Returns the translator provided by the application root.
///
/// # Panics
///
/// Panics when called outside of [`crate::App`].
pub fn use_i18n() -> I18n {
    use_context::<I18n>()
}

/// The name of a language written in that language, so it stays readable no
/// matter which language is currently selected.
pub fn language_name(language: Language) -> &'static str {
    match language {
        Language::SimplifiedChinese => "简体中文",
        Language::English => "English",
    }
}

/// A value that can be shown in the UI under the current language.
pub trait LocalizedLabel {
    fn localized_label(&self, i18n: I18n) -> String;
}

impl LocalizedLabel for CaptureMode {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            CaptureMode::BitBlt => Key::EnumCaptureModeBitBlt,
            CaptureMode::WindowsGraphicsCapture => Key::EnumCaptureModeWindowsGraphicsCapture,
            CaptureMode::BitBltArea => Key::EnumCaptureModeBitBltArea,
        })
        .to_string()
    }
}

impl LocalizedLabel for InputMethod {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            InputMethod::Default => Key::EnumInputMethodDefault,
            InputMethod::Rpc => Key::EnumInputMethodRpc,
        })
        .to_string()
    }
}

impl LocalizedLabel for WebhookProvider {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            WebhookProvider::Discord => Key::EnumWebhookProviderDiscord,
        })
        .to_string()
    }
}

impl LocalizedLabel for RotationMode {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            RotationMode::StartToEnd => Key::EnumRotationStartToEnd,
            RotationMode::StartToEndThenReverse => Key::EnumRotationStartToEndThenReverse,
            RotationMode::AutoMobbing => Key::EnumRotationAutoMobbing,
            RotationMode::PingPong => Key::EnumRotationPingPong,
        })
        .to_string()
    }
}

impl LocalizedLabel for ActionKeyWith {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            ActionKeyWith::Any => Key::CommonAny,
            ActionKeyWith::Stationary => Key::CommonStationary,
            ActionKeyWith::DoubleJump => Key::CommonDoubleJump,
        })
        .to_string()
    }
}

impl LocalizedLabel for ActionKeyDirection {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            ActionKeyDirection::Any => Key::CommonAny,
            ActionKeyDirection::Left => Key::EnumDirectionLeft,
            ActionKeyDirection::Right => Key::EnumDirectionRight,
        })
        .to_string()
    }
}

impl LocalizedLabel for WaitAfterBuffered {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            WaitAfterBuffered::None => Key::CommonNone,
            WaitAfterBuffered::Interruptible => Key::EnumWaitAfterBufferedInterruptible,
            WaitAfterBuffered::Uninterruptible => Key::EnumWaitAfterBufferedUninterruptible,
        })
        .to_string()
    }
}

impl LocalizedLabel for EliteBossBehavior {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            EliteBossBehavior::None => Key::CommonNone,
            EliteBossBehavior::CycleChannel => Key::EnumEliteBossCycleChannel,
            EliteBossBehavior::UseKey => Key::EnumEliteBossUseKey,
        })
        .to_string()
    }
}

impl LocalizedLabel for ExchangeHexaBoosterCondition {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            ExchangeHexaBoosterCondition::None => Key::CommonNone,
            ExchangeHexaBoosterCondition::Full => Key::EnumExchangeFull,
            ExchangeHexaBoosterCondition::AtLeastOne => Key::EnumExchangeAtLeastOne,
        })
        .to_string()
    }
}

impl LocalizedLabel for SwappableFamiliars {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            SwappableFamiliars::All => Key::EnumSwappableAll,
            SwappableFamiliars::Last => Key::EnumSwappableLast,
            SwappableFamiliars::SecondAndLast => Key::EnumSwappableSecondAndLast,
        })
        .to_string()
    }
}

impl LocalizedLabel for PotionMode {
    fn localized_label(&self, i18n: I18n) -> String {
        i18n.t(match self {
            PotionMode::EveryMillis(_) => Key::EnumPotionEveryMillis,
            PotionMode::Percentage(_) => Key::EnumPotionPercentage,
        })
        .to_string()
    }
}

impl LocalizedLabel for LinkKeyBinding {
    fn localized_label(&self, i18n: I18n) -> String {
        match self {
            LinkKeyBinding::None => i18n.t(Key::CommonNone).to_string(),
            LinkKeyBinding::Before(key) => i18n.format(Key::EnumLinkKeyBefore, &key.to_string()),
            LinkKeyBinding::AtTheSame(key) => {
                i18n.format(Key::EnumLinkKeyAtTheSame, &key.to_string())
            }
            LinkKeyBinding::After(key) => i18n.format(Key::EnumLinkKeyAfter, &key.to_string()),
            LinkKeyBinding::Along(key) => i18n.format(Key::EnumLinkKeyAlong, &key.to_string()),
        }
    }
}
