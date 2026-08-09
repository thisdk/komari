use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

/// The language used for the desktop UI copy.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    Display,
    EnumString,
)]
pub enum UiLanguage {
    #[default]
    #[strum(to_string = "中文")]
    Zh,
    #[strum(to_string = "English")]
    En,
}
