use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::deserialize_with_ok_or_default;
use crate::{KeyBinding, KeyBindingConfiguration, impl_identifiable};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing, default)]
    pub id: Option<i64>,
    pub capture_mode: CaptureMode,
    #[serde(default = "enable_rune_solving_default")]
    pub enable_rune_solving: bool,
    pub enable_panic_mode: bool,
    pub stop_on_fail_or_change_map: bool,
    #[serde(default = "stop_on_player_die_default")]
    pub stop_on_player_die: bool,
    #[serde(default, deserialize_with = "deserialize_with_ok_or_default")]
    pub cycle_run_stop: CycleRunStopMode,
    #[serde(default = "cycle_run_duration_millis_default")]
    pub cycle_run_duration_millis: u64,
    #[serde(default = "cycle_stop_duration_millis_default")]
    pub cycle_stop_duration_millis: u64,
    pub input_method: InputMethod,
    pub input_method_rpc_server_url: String,
    #[serde(default)]
    pub discord_bot_access_token: String,
    pub notifications: Notifications,
    pub familiars: Familiars,
    #[serde(default = "toggle_actions_key_default")]
    pub toggle_actions_key: KeyBindingConfiguration,
    #[serde(default = "platform_start_key_default")]
    pub platform_start_key: KeyBindingConfiguration,
    #[serde(default = "platform_end_key_default")]
    pub platform_end_key: KeyBindingConfiguration,
    #[serde(default = "platform_add_key_default")]
    pub platform_add_key: KeyBindingConfiguration,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: None,
            capture_mode: CaptureMode::default(),
            enable_rune_solving: enable_rune_solving_default(),
            enable_panic_mode: false,
            input_method: InputMethod::default(),
            input_method_rpc_server_url: String::default(),
            stop_on_fail_or_change_map: false,
            stop_on_player_die: stop_on_player_die_default(),
            cycle_run_stop: CycleRunStopMode::default(),
            cycle_run_duration_millis: cycle_run_duration_millis_default(),
            cycle_stop_duration_millis: cycle_stop_duration_millis_default(),
            discord_bot_access_token: String::default(),
            notifications: Notifications::default(),
            familiars: Familiars::default(),
            toggle_actions_key: toggle_actions_key_default(),
            platform_start_key: platform_start_key_default(),
            platform_end_key: platform_end_key_default(),
            platform_add_key: platform_add_key_default(),
        }
    }
}

impl_identifiable!(Settings);

fn stop_on_player_die_default() -> bool {
    true
}

fn cycle_run_duration_millis_default() -> u64 {
    14400000 // 4 hours
}

fn cycle_stop_duration_millis_default() -> u64 {
    3600000 // 1 hour
}

fn enable_rune_solving_default() -> bool {
    true
}

fn toggle_actions_key_default() -> KeyBindingConfiguration {
    KeyBindingConfiguration {
        key: KeyBinding::Comma,
        enabled: false,
    }
}

fn platform_start_key_default() -> KeyBindingConfiguration {
    KeyBindingConfiguration {
        key: KeyBinding::J,
        enabled: false,
    }
}

fn platform_end_key_default() -> KeyBindingConfiguration {
    KeyBindingConfiguration {
        key: KeyBinding::K,
        enabled: false,
    }
}

fn platform_add_key_default() -> KeyBindingConfiguration {
    KeyBindingConfiguration {
        key: KeyBinding::L,
        enabled: false,
    }
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum InputMethod {
    #[default]
    Default,
    Rpc,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum CycleRunStopMode {
    #[default]
    None,
    Once,
    Repeat,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum CaptureMode {
    BitBlt,
    #[strum(to_string = "Windows 10 (1903 and up)")] // Thanks OBS
    #[default]
    WindowsGraphicsCapture,
    BitBltArea,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Notifications {
    pub discord_webhook_url: String,
    pub discord_user_id: String,
    pub notify_on_fail_or_change_map: bool,
    pub notify_on_rune_appear: bool,
    pub notify_on_elite_boss_appear: bool,
    pub notify_on_player_die: bool,
    pub notify_on_player_guildie_appear: bool,
    pub notify_on_player_stranger_appear: bool,
    pub notify_on_player_friend_appear: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Familiars {
    pub enable_familiars_swapping: bool,
    #[serde(default = "familiars_swap_check_millis")]
    pub swap_check_millis: u64,
    pub swappable_familiars: SwappableFamiliars,
    pub swappable_rarities: HashSet<FamiliarRarity>,
}

impl Default for Familiars {
    fn default() -> Self {
        Self {
            enable_familiars_swapping: false,
            swap_check_millis: familiars_swap_check_millis(),
            swappable_familiars: SwappableFamiliars::default(),
            swappable_rarities: HashSet::default(),
        }
    }
}

fn familiars_swap_check_millis() -> u64 {
    300000
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum SwappableFamiliars {
    #[default]
    All,
    Last,
    SecondAndLast,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Hash, Serialize, Deserialize)]
pub enum FamiliarRarity {
    #[default]
    Rare,
    Epic,
}
