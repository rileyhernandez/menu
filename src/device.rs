use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    model: Model,
    number: usize,
}
impl Device {
    pub fn new(model: Model, number: usize) -> Self {
        Self { model, number }
    }
}
impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}-{}", self.model, self.number,)
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Model {
    IchibuV1,
    IchibuV2,
    LibraV0,
}
