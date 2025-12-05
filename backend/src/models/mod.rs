use serde::{Deserialize, Deserializer};
use serde_json::Value;

mod actions;
mod character;
mod keys;
mod localization;
mod map;
mod settings;

pub use actions::*;
pub use character::*;
pub use keys::*;
pub use localization::*;
pub use map::*;
pub use settings::*;

pub(crate) fn deserialize_with_ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let value = Value::deserialize(deserializer)?;
    Ok(T::deserialize(value).unwrap_or_default())
}
