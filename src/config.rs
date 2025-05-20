use std::{fs, path::Path};
use serde::{Deserialize, Serialize};
use crate::error::Error;
use crate::config_items::*;

#[cfg(feature = "generate")]
use std::fs::File;
#[cfg(feature = "generate")]
use std::io::Write;
use crate::device::Device;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    device: Option<Device>,
    conveyor_motor: Motor,
    hatch: Hatch,
    photo_eye: PhotoEye,
    pins: Pins,
    dispense: Dispense,
    setpoint: Setpoint,
}

pub trait Read {
    fn read(path: &Path) -> Result<Self, Error> where Self: Sized, for<'de> Self: Deserialize<'de> {
        let file_as_string = fs::read_to_string(path).map_err(Error::FileSystem)?;
        toml::from_str(&file_as_string).map_err(Error::TomlRead)
    }
}
impl Read for Config {}
#[derive(Deserialize, Serialize, Debug)]
pub struct ScaleConfig {
    phidget_id: isize,
    coefficients: Option<[f64; 4]>,
}
impl ScaleConfig {
    pub fn update_coefficients(&mut self, coefficients: [f64; 4]) {
        self.coefficients = Some(coefficients);
    }
    pub fn has_coefficients(&self) -> bool {
        self.coefficients.is_some()
    }
}

#[cfg(feature = "generate")]
pub trait Generate {
    fn to_toml_string(&self) -> Result<String, Error> where Self: Serialize {
        toml::to_string(self).map_err(Error::TomlGeneration)
    }
    fn generate_toml(self, file_path: &Path) -> Result<(), Error> where Self: Sized, Self: Serialize {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }
        let mut file = File::create_new(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes())
            .map_err(Error::FileSystem)?;
        Ok(())
    }
    fn overwrite_toml(self, file_path: &Path) -> Result<(), Error> where Self: Sized, Self: Serialize {
        let mut file = File::create(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes()).map_err(Error::FileSystem)?;
        Ok(())
    }
}
#[cfg(feature = "generate")]
impl Generate for Config {
    fn to_toml_string(&self) -> Result<String, Error> {
        if self.device.is_none() {
            return Err(Error::NoSerialNumber)
        }
        toml::to_string(self).map_err(Error::TomlGeneration)
    }
}
#[cfg(feature = "generate")]
impl Generate for ScaleConfig {}
#[cfg(feature = "generate")]
impl Read for ScaleConfig {}

