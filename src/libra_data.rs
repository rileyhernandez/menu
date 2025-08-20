use crate::action::Action;
use crate::device::Device;
use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Debug, Deserialize)]
pub struct LibraData {
    pub device: Device,
    pub location: String,
    pub ingredient: String,
    pub data_action: Action,
    pub amount: f64,
    pub timestamp: OffsetDateTime,
}
