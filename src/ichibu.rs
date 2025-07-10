use crate::ichibu_items::*;
use crate::device::Device;
use crate::read::Read;
use serde::{Deserialize, Serialize};

#[cfg(feature = "write")]
use crate::generate::Generate;
#[cfg(feature = "write")]
use crate::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Ichibu {
    device: Option<Device>,
    conveyor_motor: Motor,
    hatch: Hatch,
    photo_eye: PhotoEye,
    pins: Pins,
    dispense: Dispense,
    setpoint: Setpoint,
}
impl Read for Ichibu {}
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

#[cfg(feature = "write")]
impl Generate<'_> for Ichibu {
    fn to_toml_string(&self) -> Result<String, Error> {
        if self.device.is_none() {
            return Err(Error::NoSerialNumber);
        }
        toml::to_string(self).map_err(Error::TomlGeneration)
    }
}
#[cfg(feature = "write")]
impl Generate<'_> for ScaleConfig {}
#[cfg(feature = "write")]
impl Read for ScaleConfig {}
