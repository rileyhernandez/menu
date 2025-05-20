use serde::{Deserialize, Serialize};

pub type Io = usize;
#[derive(Deserialize, Serialize, Debug)]
pub struct Motor {
    id: Io,
    scale: usize,
    acceleration: f64,
    velocity: f64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Dispense {
    timeout: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Setpoint {
    empty: f64,
    filling_threshold: f64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Hatch {
    motor: Motor,
    open_input: Io,
    close_input: Io,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PhotoEye {
    input: Io,
    sample_number: usize,
    sample_period: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Pins {
    manager: String,
    operator: String,
    sudo: String,
}