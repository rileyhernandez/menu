use std::{fs, path::Path};
use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::config_items::*;

#[cfg(feature = "generate")]
use std::io::Write;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    conveyor_motor: Motor,
    hatch: Hatch,
    photo_eye: PhotoEye,
    pins: Pins,
    dispense: Dispense,
    setpoint: Setpoint,
}
impl Config {
    pub fn read(path: &Path) -> Result<Self, Error> {
        let file_as_string = fs::read_to_string(path).map_err(Error::FileSystem)?;
        toml::from_str(&file_as_string).map_err(Error::TomlRead)
    }
    #[cfg(feature = "generate")]
    fn to_toml_string(&self) -> Result<String, Error> {
        toml::to_string(self).map_err(Error::TomlGeneration)
    }
    #[cfg(feature = "generate")]
    pub fn generate_toml(self, file_path: &Path) -> Result<(), Error> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }
        let mut file = File::create_new(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes())
            .map_err(Error::FileSystem)?;
        Ok(())
    }
    #[cfg(feature = "generate")]
    pub fn overwrite_toml(self, file_path: &Path) -> Result<(), Error> {
        let mut file = File::create(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes()).map_err(Error::FileSystem)?;
        Ok(())
    }
}
