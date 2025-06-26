#[cfg(feature = "generate")]
use crate::generate::Generate;
use serde::{Deserialize, Serialize};
use crate::device::Device;
#[cfg(feature = "generate")]
use crate::pull::FromBackend;
use crate::read::Read;
#[cfg(feature = "generate")]
use crate::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Libra {
    pub config: Config,
    pub device: Device
}
#[cfg(feature = "generate")]
impl Libra {
    
    pub fn pull_from_backend(device: Device, url: &str) -> Result<Self, Error> {
        let url = format!("{}/{}", url, device);
        let config = Config::pull(&url)?;
        Ok(Self { config, device})
    }
}
impl Read for Libra {}
#[cfg(feature = "generate")]
impl Generate<'_> for Libra {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "phidget-id")]
    pub phidget_id: i32,
    #[serde(rename = "load-cell-id")]
    pub load_cell_id: i32,
    pub gain: f64,
    pub offset: f64,
}
impl Config {
    pub fn test() -> Self {
        Self {
            phidget_id: 69420,
            load_cell_id: 0,
            gain: 1.0,
            offset: 0.0,
        }
    }
}
impl Read for Config {}
#[cfg(feature = "generate")]
impl FromBackend for Config {}
#[cfg(feature = "generate")]
impl Generate<'_> for Config {}
