use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Action {
    Served,
    RanOut,
    Refilled,
    Starting,
    Heartbeat,
    Offline,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Served => write!(f, "Served"),
            Action::RanOut => write!(f, "RanOut"),
            Action::Refilled => write!(f, "Refilled"),
            Action::Starting => write!(f, "Starting"),
            Action::Heartbeat => write!(f, "Heartbeat"),
            Action::Offline => write!(f, "Offline"),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Served => write!(f, "Served"),
            Action::RanOut => write!(f, "RanOut"),
            Action::Refilled => write!(f, "Refilled"),
            Action::Starting => write!(f, "Starting"),
            Action::Heartbeat => write!(f, "Heartbeat"),
            Action::Offline => write!(f, "Offline"),
        }
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Served" => Ok(Action::Served),
            "RanOut" => Ok(Action::RanOut),
            "Refilled" => Ok(Action::Refilled),
            "Starting" => Ok(Action::Starting),
            "Heartbeat" => Ok(Action::Heartbeat),
            "Offline" => Ok(Action::Offline),
            _ => Err(format!("Invalid action: {s}")),
        }
    }
}
