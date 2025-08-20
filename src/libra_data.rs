use crate::action::Action;
use crate::device::Device;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraData {
    pub device: Device,
    pub location: String,
    pub ingredient: String,
    pub data_action: Action,
    pub amount: f64,
    pub timestamp: OffsetDateTime,
}
