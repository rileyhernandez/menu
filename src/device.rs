use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Device {
    pub model: Model,
    pub serial_number: String,
}

impl Device {
    pub fn new(model: Model, serial_number: &str) -> Self {
        Self {
            model,
            serial_number: serial_number.to_string(),
        }
    }
}
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{}", self.model, self.serial_number,)
    }
}

impl FromStr for Device {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid format: {}", s));
        }
        let model = Model::from_str(parts[0])?;
        let serial_number = parts[1].to_string();
        Ok(Device::new(model, &serial_number))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Model {
    IchibuV1,
    IchibuV2,
    LibraV0,
}

impl FromStr for Model {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IchibuV1" => Ok(Model::IchibuV1),
            "IchibuV2" => Ok(Model::IchibuV2),
            "LibraV0" => Ok(Model::LibraV0),
            _ => Err(format!("Invalid model: {}", s)),
        }
    }
}
