use std::fs;
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
    pub fn new(path: &str) -> Result<Self, Error> {
        let file_as_string = fs::read_to_string(path).map_err(Error::FileSystem)?;
        toml::from_str(&file_as_string).map_err(Error::TomlRead)
    }
    #[cfg(feature = "generate")]
    pub fn generate_toml(self, file_path: &str) -> Result<(), Error> {
        let toml_string = toml::to_string(&self).map_err(Error::TomlGeneration)?;
        let mut file = File::create(file_path).map_err(Error::FileSystem)?;
        file.write_all(toml_string.as_bytes())
            .map_err(Error::FileSystem)?;
        Ok(())
    }
}
