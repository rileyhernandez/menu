use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error generating config.toml: {0}")]
    TomlGeneration(toml::ser::Error),
    #[error("Error reading config.toml: {0}")]
    TomlRead(toml::de::Error),
    #[error("Error with file system operation: {0}")]
    FileSystem(std::io::Error),
}
