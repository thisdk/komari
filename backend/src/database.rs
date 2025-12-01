use std::{
    collections::HashMap,
    env,
    sync::{LazyLock, Mutex},
};

use anyhow::{Result, bail};
use opencv::core::Rect;
use rusqlite::{Connection, Params, Statement, types::Null};
use serde::{Deserialize, Deserializer, Serialize, de::DeserializeOwned};
use serde_json::Value;
use strum::{Display, EnumIter, EnumString};
use tokio::sync::broadcast::{Receiver, Sender, channel};

use crate::{models::*, pathing};

const MAPS: &str = "maps";
const NAVIGATION_PATHS: &str = "navigation_paths";
const CHARACTERS: &str = "characters";
const SETTINGS: &str = "settings";
const SEEDS: &str = "seeds";
const LOCALIZATIONS: &str = "localizations";

static CONNECTION: LazyLock<Mutex<Connection>> = LazyLock::new(|| {
    let path = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("local.db")
        .to_path_buf();
    let conn = Connection::open(path.to_str().unwrap()).expect("failed to open local.db");
    conn.execute_batch(
        format!(
            r#"
            CREATE TABLE IF NOT EXISTS {MAPS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {NAVIGATION_PATHS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {CHARACTERS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {SETTINGS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {SEEDS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS {LOCALIZATIONS} (
                id INTEGER PRIMARY KEY,
                data TEXT NOT NULL
            );
            "#
        )
        .as_str(),
    )
    .unwrap();
    Mutex::new(conn)
});
static EVENT: LazyLock<Sender<DatabaseEvent>> = LazyLock::new(|| channel(5).0);

#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum DatabaseEvent {
    MinimapUpdated(Minimap),
    MinimapDeleted(i64),
    NavigationPathsUpdated,
    NavigationPathsDeleted,
    SettingsUpdated(Settings),
    LocalizationUpdated(Localization),
    CharacterUpdated(Character),
    CharacterDeleted(i64),
}

pub trait Identifiable {
    fn id(&self) -> Option<i64>;

    fn set_id(&mut self, id: i64);
}

#[macro_export]
macro_rules! impl_identifiable {
    ($type:ty) => {
        impl $crate::database::Identifiable for $type {
            fn id(&self) -> Option<i64> {
                self.id
            }

            fn set_id(&mut self, id: i64) {
                self.id = Some(id);
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Seeds {
    #[serde(skip_serializing, default)]
    pub id: Option<i64>,
    #[serde(alias = "seed")]
    pub rng_seed: [u8; 32],
    #[serde(default = "perlin_seed_default")]
    pub perlin_seed: u32,
}

impl Default for Seeds {
    fn default() -> Self {
        Self {
            id: None,
            rng_seed: rand::random(),
            perlin_seed: perlin_seed_default(),
        }
    }
}

impl_identifiable!(Seeds);

fn perlin_seed_default() -> u32 {
    rand::random()
}

#[derive(Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize)]
pub struct Bound {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

// TODO: Should be part of auto-mobbing or ping-pong logics, not here
impl From<Bound> for Rect {
    fn from(value: Bound) -> Self {
        Self::new(value.x, value.y, value.width, value.height)
    }
}

impl From<Rect> for Bound {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum RotationMode {
    StartToEnd,
    #[default]
    StartToEndThenReverse,
    AutoMobbing,
    PingPong,
}

impl_identifiable!(Character);

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Minimap {
    #[serde(skip_serializing)]
    pub id: Option<i64>,
    pub name: String,
    pub width: i32,
    pub height: i32,
    #[serde(default, deserialize_with = "deserialize_with_ok_or_default")]
    pub rotation_mode: RotationMode,
    #[serde(default)]
    pub rotation_ping_pong_bound: Bound,
    #[serde(default)]
    pub rotation_auto_mob_bound: Bound,
    #[serde(default)]
    pub rotation_mobbing_key: MobbingKey,
    pub platforms: Vec<Platform>,
    pub rune_platforms_pathing: bool,
    pub rune_platforms_pathing_up_jump_only: bool,
    pub auto_mob_platforms_pathing: bool,
    pub auto_mob_platforms_pathing_up_jump_only: bool,
    pub auto_mob_platforms_bound: bool,
    #[serde(default)]
    pub auto_mob_use_key_when_pathing: bool,
    #[serde(default)]
    pub auto_mob_use_key_when_pathing_update_millis: u64,
    pub actions_any_reset_on_erda_condition: bool,
    pub actions: HashMap<String, Vec<Action>>,
    // Not FK, loose coupling to another navigation paths and its index
    #[serde(default)]
    pub paths_id_index: Option<(i64, usize)>,
}

impl_identifiable!(Minimap);

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPaths {
    #[serde(skip_serializing, default)]
    pub id: Option<i64>,
    pub name: String,
    pub paths: Vec<NavigationPath>,
}

impl_identifiable!(NavigationPaths);

#[derive(PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPath {
    pub minimap_snapshot_base64: String,
    #[serde(default)]
    pub minimap_snapshot_grayscale: bool,
    pub name_snapshot_base64: String,
    pub name_snapshot_width: i32,
    pub name_snapshot_height: i32,
    pub points: Vec<NavigationPoint>,
}

#[derive(PartialEq, Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct NavigationPoint {
    // Not FK, loose coupling to another navigation paths and its index
    pub next_paths_id_index: Option<(i64, usize)>,
    pub x: i32,
    pub y: i32,
    pub transition: NavigationTransition,
}

#[derive(
    Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize, EnumIter, Display, EnumString,
)]
pub enum NavigationTransition {
    #[default]
    Portal,
}

fn deserialize_with_ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let value = Value::deserialize(deserializer)?;
    Ok(T::deserialize(value).unwrap_or_default())
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Platform {
    pub x_start: i32,
    pub x_end: i32,
    pub y: i32,
}

// TODO: Should be part of pathing logics, not here
impl From<Platform> for pathing::Platform {
    fn from(value: Platform) -> Self {
        Self::new(value.x_start..value.x_end, value.y)
    }
}

pub fn database_event_receiver() -> Receiver<DatabaseEvent> {
    EVENT.subscribe()
}

pub fn query_and_upsert_seeds() -> Seeds {
    let mut seeds = query_from_table::<Seeds>(SEEDS)
        .unwrap()
        .into_iter()
        .next()
        .unwrap_or_default();
    upsert_to_table(SEEDS, &mut seeds).unwrap();
    seeds
}

pub fn query_or_upsert_localization() -> Localization {
    let mut localization = query_from_table::<Localization>(LOCALIZATIONS)
        .unwrap()
        .into_iter()
        .next()
        .unwrap_or_default();
    if localization.id.is_none() {
        upsert_to_table(LOCALIZATIONS, &mut localization).unwrap();
    }
    localization
}

pub fn upsert_localization(localization: &mut Localization) -> Result<()> {
    upsert_to_table(LOCALIZATIONS, localization).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::LocalizationUpdated(localization.clone()));
    })
}

pub fn query_settings() -> Settings {
    let mut settings = query_from_table::<Settings>(SETTINGS)
        .unwrap()
        .into_iter()
        .next()
        .unwrap_or_default();
    if settings.id.is_none() {
        upsert_settings(&mut settings).unwrap();
    }
    settings
}

pub fn upsert_settings(settings: &mut Settings) -> Result<()> {
    upsert_to_table(SETTINGS, settings).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::SettingsUpdated(settings.clone()));
    })
}

pub fn query_characters() -> Result<Vec<Character>> {
    query_from_table(CHARACTERS)
}

pub fn upsert_character(character: &mut Character) -> Result<()> {
    upsert_to_table(CHARACTERS, character).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::CharacterUpdated(character.clone()));
    })
}

pub fn delete_character(character: &Character) -> Result<()> {
    delete_from_table(CHARACTERS, character).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::CharacterDeleted(
            character.id.expect("valid id if deleted"),
        ));
    })
}

pub fn query_minimaps() -> Result<Vec<Minimap>> {
    query_from_table(MAPS)
}

pub fn upsert_minimap(minimap: &mut Minimap) -> Result<()> {
    upsert_to_table(MAPS, minimap).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::MinimapUpdated(minimap.clone()));
    })
}

pub fn delete_minimap(minimap: &Minimap) -> Result<()> {
    delete_from_table(MAPS, minimap).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::MinimapDeleted(
            minimap.id.expect("valid id if deleted"),
        ));
    })
}

pub fn query_navigation_paths() -> Result<Vec<NavigationPaths>> {
    query_from_table(NAVIGATION_PATHS)
}

pub fn upsert_navigation_paths(paths: &mut NavigationPaths) -> Result<()> {
    upsert_to_table(NAVIGATION_PATHS, paths).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::NavigationPathsUpdated);
    })
}

pub fn delete_navigation_paths(paths: &NavigationPaths) -> Result<()> {
    delete_from_table(NAVIGATION_PATHS, paths).inspect(|_| {
        let _ = EVENT.send(DatabaseEvent::NavigationPathsDeleted);
    })
}

fn map_data<T>(mut stmt: Statement<'_>, params: impl Params) -> Result<Vec<T>>
where
    T: DeserializeOwned + Identifiable + Default,
{
    Ok(stmt
        .query_map::<T, _, _>(params, |row| {
            let id = row.get::<_, i64>(0).unwrap();
            let data = row.get::<_, String>(1).unwrap();
            let mut value = serde_json::from_str::<'_, T>(data.as_str()).unwrap_or_default();
            value.set_id(id);
            Ok(value)
        })?
        .filter_map(|c| c.ok())
        .collect::<Vec<_>>())
}

fn query_from_table<T>(table: &str) -> Result<Vec<T>>
where
    T: DeserializeOwned + Identifiable + Default,
{
    let conn = CONNECTION.lock().unwrap();
    let stmt = format!("SELECT id, data FROM {table};");
    let stmt = conn.prepare(&stmt).unwrap();
    map_data(stmt, [])
}

fn upsert_to_table<T>(table: &str, data: &mut T) -> Result<()>
where
    T: Serialize + Identifiable,
{
    let json = serde_json::to_string(&data).unwrap();
    let conn = CONNECTION.lock().unwrap();
    let stmt = format!(
        "INSERT INTO {table} (id, data) VALUES (?1, ?2) ON CONFLICT (id) DO UPDATE SET data = ?2;",
    );
    match data.id() {
        Some(id) => {
            if conn.execute(&stmt, (id, &json))? > 0 {
                Ok(())
            } else {
                bail!("no row was updated")
            }
        }
        None => {
            if conn.execute(&stmt, (Null, &json))? > 0 {
                data.set_id(conn.last_insert_rowid());
                Ok(())
            } else {
                bail!("no row was inserted")
            }
        }
    }
}

fn delete_from_table<T: Identifiable>(table: &str, data: &T) -> Result<()> {
    fn inner(table: &str, id: Option<i64>) -> Result<()> {
        if let Some(id) = id {
            let conn = CONNECTION.lock().unwrap();
            let stmt = format!("DELETE FROM {table} WHERE id = ?1;");
            let deleted = conn.execute(&stmt, [id])?;

            if deleted > 0 {
                return Ok(());
            }
        }
        bail!("no row was deleted")
    }

    inner(table, data.id())
}
