#[cfg(feature = "write")]
use reqwest;
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error generating config.toml: {0}")]
    TomlGeneration(toml::ser::Error),
    #[error("Error reading config.toml: {0}")]
    TomlRead(toml::de::Error),
    #[error("Error with file system operation: {0}")]
    FileSystem(#[from] std::io::Error),
    #[error("Feature not yet implemented!")]
    NotImplemented,
    #[error("{0}")]
    Custom(String),
    #[cfg(feature = "write")]
    #[error("Error reaching backend: {0}")]
    Reqwest(reqwest::Error),
    #[cfg(feature = "write")]
    #[error("Device must be assigned a serial number before compiling config file!")]
    NoSerialNumber,
    #[cfg(feature = "write")]
    #[error("Error serializing from backend: {0}")]
    SerdeJson(serde_json::Error),
    #[error("File Already Exists")]
    FileExists,
    #[error("File Does Not Exist")]
    FileNotFound,
    #[error("Libra not found in config file!")]
    LibraNotFound,
    #[error("Libra already exists in config file!")]
    LibraAlreadyExists,
    #[error("Couldn't access environment variable: {0}")]
    Env(#[from] std::env::VarError),
    #[cfg(feature = "write")]
    #[error("Backend Error: {0:?}")]
    Backend(reqwest::StatusCode)
}
