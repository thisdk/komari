// TODO: Move related character models here

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum EliteBossBehavior {
    #[default]
    None,
    CycleChannel,
    UseKey,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum ExchangeHexaBoosterCondition {
    #[default]
    None,
    Full,
    AtLeastOne,
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
