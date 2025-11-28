use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

#[derive(
    Clone, Copy, Display, EnumString, EnumIter, PartialEq, Debug, Serialize, Deserialize, Default,
)]
pub enum WaitAfterBuffered {
    #[default]
    None,
    Interruptible,
    Uninterruptible,
}
