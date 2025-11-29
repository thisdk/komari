use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use super::{KeyBinding, LinkKeyBinding, deserialize_with_ok_or_default};

#[derive(
    Clone, Copy, Display, EnumString, EnumIter, PartialEq, Debug, Serialize, Deserialize, Default,
)]
pub enum WaitAfterBuffered {
    #[default]
    None,
    Interruptible,
    Uninterruptible,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, EnumIter, Display, EnumString)]
pub enum Action {
    Move(ActionMove),
    Key(ActionKey),
}

impl Action {
    pub fn condition(&self) -> ActionCondition {
        match self {
            Action::Move(action) => action.condition,
            Action::Key(action) => action.condition,
        }
    }

    pub fn with_condition(&self, condition: ActionCondition) -> Action {
        match self {
            Action::Move(action) => Action::Move(ActionMove {
                condition,
                ..*action
            }),
            Action::Key(action) => Action::Key(ActionKey {
                condition,
                ..*action
            }),
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActionMove {
    pub position: Position,
    pub condition: ActionCondition,
    pub wait_after_move_millis: u64,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct ActionKey {
    pub key: KeyBinding,
    #[serde(default)]
    pub key_hold_millis: u64,
    #[serde(default)]
    pub key_hold_buffered_to_wait_after: bool,
    #[serde(default, deserialize_with = "deserialize_with_ok_or_default")]
    pub link_key: LinkKeyBinding,
    #[serde(default = "count_default")]
    pub count: u32,
    pub position: Option<Position>,
    pub condition: ActionCondition,
    pub direction: ActionKeyDirection,
    pub with: ActionKeyWith,
    pub wait_before_use_millis: u64,
    pub wait_before_use_millis_random_range: u64,
    pub wait_after_use_millis: u64,
    pub wait_after_use_millis_random_range: u64,
    #[serde(default, deserialize_with = "deserialize_with_ok_or_default")]
    pub wait_after_buffered: WaitAfterBuffered,
    pub queue_to_front: Option<bool>,
}

impl Default for ActionKey {
    fn default() -> Self {
        Self {
            key: KeyBinding::default(),
            key_hold_millis: 0,
            key_hold_buffered_to_wait_after: false,
            link_key: LinkKeyBinding::None,
            count: count_default(),
            position: None,
            condition: ActionCondition::default(),
            direction: ActionKeyDirection::default(),
            with: ActionKeyWith::default(),
            wait_before_use_millis: 0,
            wait_before_use_millis_random_range: 0,
            wait_after_use_millis: 0,
            wait_after_use_millis_random_range: 0,
            wait_after_buffered: WaitAfterBuffered::None,
            queue_to_front: None,
        }
    }
}

fn count_default() -> u32 {
    1
}

#[derive(Clone, Copy, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: i32,
    pub x_random_range: i32,
    pub y: i32,
    pub allow_adjusting: bool,
}

#[derive(
    Clone, Copy, Default, PartialEq, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum ActionCondition {
    #[default]
    Any,
    EveryMillis(u64),
    ErdaShowerOffCooldown,
    Linked,
}

#[derive(
    Clone, Copy, Default, PartialEq, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum ActionKeyWith {
    #[default]
    Any,
    Stationary,
    DoubleJump,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum ActionKeyDirection {
    #[default]
    Any,
    Left,
    Right,
}
