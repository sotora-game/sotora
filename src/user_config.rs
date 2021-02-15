use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::{fs, io};

use bevy::prelude::*;
use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct KeyBinds {
    pub move_forward: KeyCode,
    pub move_left: KeyCode,
    pub move_backward: KeyCode,
    pub move_right: KeyCode,
}

impl Default for KeyBinds {
    fn default() -> Self {
        KeyBinds {
            move_forward: KeyCode::W,
            move_left: KeyCode::A,
            move_backward: KeyCode::S,
            move_right: KeyCode::D,
        }
    }
}

impl UserConfig for KeyBinds {
    const FILE_NAME: &'static str = "key_binds";
}

/// A configuration object that is persisted to disk as a JSON file
pub trait UserConfig: Serialize + DeserializeOwned + Default {
    /// Unique filename to store in the user data directory
    ///
    /// Should not include a file extension.
    const FILE_NAME: &'static str;

    /// Load the config data
    ///
    /// If no file exists yet, this will create a new config file with the
    /// `Default::default()` values
    fn load() -> Self {
        let config = load_user_config::<Self>();
        config.save();
        config
    }

    /// Saves the config data
    fn save(&self) {
        save_user_config(self);
    }
}

/// Loads the user config from disk, or returns the default values if the file does not exist
fn load_user_config<C: UserConfig>() -> C {
    let file = OpenOptions::new()
        .read(true)
        .open(get_user_config_dir().join(format!("{}.json", C::FILE_NAME)));

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).expect("Could not read config file")
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Default::default(),
        Err(e) => panic!(e),
    }
}

/// Saves the user config data to disk, overwriting any existing config files
fn save_user_config<C: UserConfig>(data: &C) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_user_config_dir().join(format!("{}.json", C::FILE_NAME)))
        .expect("Could not open user config file for writing");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, data).expect("Could not write config file");
}

/// Gets the platform-specific user config directory
fn get_user_config_dir() -> PathBuf {
    let dirs = ProjectDirs::from("", "bevy-community", "sotora")
        .expect("Could not access user config dirs");

    fs::create_dir_all(dirs.config_dir()).expect("Could not create user config directory");

    dirs.config_dir().to_owned()
}
