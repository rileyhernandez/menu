use std::fs;
use std::path::Path;
use serde::Deserialize;
use crate::error::Error;

pub trait Read {
    fn read(path: &Path) -> Result<Self, Error>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let file_as_string = fs::read_to_string(path).map_err(Error::FileSystem)?;
        toml::from_str(&file_as_string).map_err(Error::TomlRead)
    }
    fn read_as_vec(path: &Path) -> Result<Vec<Self>, Error>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let file_as_string = fs::read_to_string(path).map_err(Error::FileSystem)?;
        let toml_value: toml::Value = toml::from_str(&file_as_string).map_err(Error::TomlRead)?;

        if let toml::Value::Table(table) = toml_value {
            table
                .into_iter()
                .map(|(_, value)| value.try_into::<Self>())
                .collect::<Result<Vec<Self>, _>>()
                .map_err(Error::TomlRead)
        } else {
            let custom_error = "Expected TOML root to be a table of tables";
            Err(Error::Custom(custom_error.parse().unwrap()))
        }
    }
}