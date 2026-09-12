use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::LazyLock,
    time::{SystemTime, UNIX_EPOCH},
};

use opencv::{core::ToInputArray, imgcodecs::imwrite_def};

/// Environment variable overriding [`data_dir`].
pub const DATA_DIR_ENV: &str = "KOMARI_DATA_DIR";

/// The directory holding every file the program produces at runtime.
///
/// This is the directory of the running executable so that the configuration,
/// the cache and the logs stay in one place, next to the program itself: the
/// whole state can be inspected, backed up or deleted by removing that single
/// directory. Set [`DATA_DIR_ENV`] to override it, which is useful during
/// development because `dx build` wipes the directory holding the debug
/// executable (and therefore its data).
pub fn data_dir() -> PathBuf {
    static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        if let Some(dir) = env::var_os(DATA_DIR_ENV).filter(|dir| !dir.is_empty()) {
            return PathBuf::from(dir);
        }

        env::current_exe()
            .ok()
            .and_then(|exe| exe.parent().map(Path::to_path_buf))
            .unwrap_or_else(|| PathBuf::from("."))
    });

    DATA_DIR.clone()
}

/// The directory holding the database and every image or recording the program produces.
pub fn dataset_dir() -> PathBuf {
    static DATASET_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
        let dir = data_dir().join("dataset");
        if let Err(error) = fs::create_dir_all(&dir) {
            log::error!("failed to create data directory {}: {error}", dir.display());
        }
        dir
    });

    DATASET_DIR.clone()
}

#[cfg(debug_assertions)]
static DATASET_RECORDINGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = dataset_dir().join("recordings");
    fs::create_dir_all(dir.clone()).unwrap();
    dir
});

#[cfg(debug_assertions)]
static DATASET_RUNE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = dataset_dir().join("rune");
    fs::create_dir_all(dir.clone()).unwrap();
    dir
});

#[derive(Debug)]
pub enum DatasetDir {
    Root,
    #[cfg(debug_assertions)]
    Recordings,
    #[cfg(debug_assertions)]
    Rune,
}

impl DatasetDir {
    pub fn to_folder(&self) -> PathBuf {
        match self {
            DatasetDir::Root => dataset_dir(),
            #[cfg(debug_assertions)]
            DatasetDir::Recordings => DATASET_RECORDINGS_DIR.clone(),
            #[cfg(debug_assertions)]
            DatasetDir::Rune => DATASET_RUNE_DIR.clone(),
        }
    }
}

pub fn save_image_to_default(mat: &impl ToInputArray, dir: DatasetDir) {
    save_image_to(mat, dir, format!("{}.png", epoch_millis_as_string()));
}

pub fn save_image_to<P: AsRef<Path>>(mat: &impl ToInputArray, dir: DatasetDir, relative: P) {
    let folder = dir.to_folder();
    let mut image = folder.join(relative);
    if image.extension().is_none_or(|ext| ext != "png") {
        image = image.join(format!("{}.png", epoch_millis_as_string()));
    }

    if !image.exists() {
        fs::create_dir_all(image.parent().unwrap()).unwrap();
    }

    let _ = imwrite_def(image.to_str().unwrap(), mat);
}

#[cfg(debug_assertions)]
pub fn save_file_to<P: AsRef<Path>, C: AsRef<[u8]>>(contents: C, dir: DatasetDir, relative: P) {
    let folder = dir.to_folder();
    let file = folder.join(relative);
    if !file.exists() {
        fs::create_dir_all(file.parent().unwrap()).unwrap();
    }

    let _ = fs::write(file, contents);
}

fn epoch_millis_as_string() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
