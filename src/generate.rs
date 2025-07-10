use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[cfg(feature = "write")]
pub trait Generate<'de> {
    fn from_json(json: &'de str) -> Result<Self, Error>
    where
        Self: Sized,
        Self: Deserialize<'de>,
    {
        serde_json::from_str(json).map_err(Error::SerdeJson)
    }
    fn to_toml_string(&self) -> Result<String, Error>
    where
        Self: Serialize,
    {
        toml::to_string(self).map_err(Error::TomlGeneration)
    }
    fn generate_toml(self, file_path: &Path) -> Result<(), Error>
    where
        Self: Sized,
        Self: Serialize,
    {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }
        let mut file = File::create_new(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes())
            .map_err(Error::FileSystem)?;
        Ok(())
    }
    fn overwrite_toml(self, file_path: &Path) -> Result<(), Error>
    where
        Self: Sized,
        Self: Serialize,
    {
        let mut file = File::create(file_path).map_err(Error::FileSystem)?;
        file.write_all(self.to_toml_string()?.as_bytes())
            .map_err(Error::FileSystem)?;
        Ok(())
    }
    fn add_as_table(self, file_path: &Path, table_name: &str) -> Result<(), Error>
    where
        Self: Sized,
        Self: Serialize,
    {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .map_err(Error::FileSystem)?;
        let mut table_map = std::collections::BTreeMap::new();
        table_map.insert(table_name, self);
        let toml_string = toml::to_string(&table_map).map_err(Error::TomlGeneration)?;
        if file.metadata().map_err(Error::FileSystem)?.len() > 0 {
            file.write_all(b"\n").map_err(Error::FileSystem)?;
        }
        file.write_all(toml_string.as_bytes())
            .map_err(Error::FileSystem)?;

        Ok(())
    }
}
