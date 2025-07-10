#[cfg(feature = "write")]
use std::fs;
#[cfg(feature = "write")]
use std::fs::File;
#[cfg(feature = "write")]
use std::io::Write;
use crate::device::{Device, Model};
#[cfg(feature = "write")]
use crate::error::Error;
#[cfg(feature = "write")]
use crate::generate::Generate;
#[cfg(feature = "write")]
use crate::pull::FromBackend;
use crate::read::Read;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Libra {
    pub config: Config,
    pub device: Device,
}
#[cfg(feature = "write")]
impl Libra {
    pub fn pull_from_backend(device: Device, url: &str) -> Result<Self, Error> {
        let url = format!("{}/{}", url, device);
        let config = Config::pull(&url)?;
        Ok(Self { config, device })
    }
    pub fn new_config_file(libras: Vec<Self>, file_path: &std::path::Path) -> Result<(), Error> {
        if file_path.exists() {

            return Err(Error::NotImplemented);
        }
        let map: std::collections::BTreeMap<String, Libra> = libras
            .into_iter()
            .map(|libra| (libra.device.to_string(), libra))
            .collect();

        let toml_string = toml::to_string(&map).map_err(Error::TomlGeneration)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }

        let mut file = File::create_new(file_path).map_err(Error::FileSystem)?;
        file.write_all(toml_string.as_bytes())
            .map_err(Error::FileSystem)?;

        Ok(())
    }
    pub fn edit_config_file(self, file_path: &std::path::Path) -> Result<(), Error> {

        if !file_path.exists() {
            return Err(Error::FileNotFound);
        }
        let libras: Vec<Libra> = Libra::read_as_vec(file_path)?.into_iter()
            .map(|mut libra| {
                if PartialEq::eq(&libra.device, &self.device) { libra.config = self.config.clone() }
                Ok(libra)
        }).collect::<Result<Vec<Libra>, Error>>()?;

        fs::remove_file(file_path)?;
        Libra::new_config_file(libras, file_path)?;
        Ok(())
    }
}
impl Default for Libra {
    fn default() -> Self {
        Self {
            config: Config::default(),
            device: Device::new(Model::LibraV0, 0),
        }
    }
}
impl Read for Libra {}
#[cfg(feature = "write")]
impl Generate<'_> for Libra {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub phidget_id: i32,
    pub load_cell_id: i32,
    pub gain: f64,
    pub offset: f64,
    pub location: String,
    pub ingredient: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            phidget_id: 0,
            load_cell_id: 0,
            gain: 1.0,
            offset: 0.0,
            location: "Caldo HQ".into(),
            ingredient: "Fake Chicken Wings".into(),
        }
    }
}
impl Read for Config {}
#[cfg(feature = "write")]
impl FromBackend for Config {}
#[cfg(feature = "write")]
impl Generate<'_> for Config {}
