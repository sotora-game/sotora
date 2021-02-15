use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::PathBuf;

use bevy::prelude::*;
use directories::ProjectDirs;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Configurable key bindings
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
        .open(get_config_file_path(C::FILE_NAME));

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            ron::de::from_reader(reader).expect("Could not read config file")
        }
        Err(e) if e.kind() == ErrorKind::NotFound => Default::default(),
        Err(e) => panic!("{}", e),
    }
}

/// Saves the user config data to disk, overwriting any existing config files
fn save_user_config<C: UserConfig>(data: &C) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(get_config_file_path(C::FILE_NAME))
        .expect("Could not open user config file for writing");

    let writer = BufWriter::new(file);
    ron::ser::to_writer_pretty(writer, data, Default::default())
        .expect("Could not write config file");
}

/// Gets the platform-specific user config directory
pub(self) fn get_config_file_path(file_name: &str) -> PathBuf {
    let dirs = ProjectDirs::from("", "bevy-community", "sotora")
        .expect("Could not access user config dirs");

    fs::create_dir_all(dirs.config_dir()).expect("Could not create user config directory");

    dirs.config_dir().join(format!("{}.ron", file_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::io::Error;

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct Foo {
        value_a: bool,
        value_b: String,
        value_c: Bar,
    }

    #[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
    struct Bar(usize);

    impl UserConfig for Foo {
        const FILE_NAME: &'static str = "test";
    }

    #[test]
    fn test() {
        // Delete persisted file from last time, if necessary
        if let Err(err) = std::fs::remove_file(get_config_file_path(Foo::FILE_NAME)) {
            if err.kind() != ErrorKind::NotFound {
                panic!("Could not remove file: {}", err);
            }
        }

        // Make sure that loading a non-existent file returns the default values
        let loaded_data = Foo::load();
        assert_eq!(loaded_data, Foo::default());

        // Make sure that saving and subsequently loading a config file returns the same data
        let new_data = Foo {
            value_a: true,
            value_b: "Hello World".to_string(),
            value_c: Bar(12),
        };

        new_data.save();
        let reloaded_data = Foo::load();
        assert_eq!(new_data, reloaded_data);
    }
}
