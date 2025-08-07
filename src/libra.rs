use crate::device::{Device, Model};
#[cfg(feature = "write")]
use crate::error::Error;
#[cfg(feature = "write")]
use crate::generate::Generate;
use crate::read::Read;
use serde::{Deserialize, Serialize};
#[cfg(feature = "write")]
use std::fs;
#[cfg(feature = "write")]
use std::fs::File;
#[cfg(feature = "write")]
use std::io::Write;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Libra {
    pub config: Config,
    pub device: Device,
}
#[cfg(feature = "write")]
impl Libra {
    pub fn new_config_file(libras: Vec<Self>, file_path: &std::path::Path) -> Result<(), Error> {
        if file_path.exists() {
            return Err(Error::NotImplemented);
        }
        let map: std::collections::BTreeMap<String, Libra> = libras
            .into_iter()
            .map(|libra| (libra.device.to_string(), libra))
            .collect();

        let toml_string = toml::to_string(&map).map_err(Error::TomlGeneration)?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).map_err(Error::FileSystem)?;
        }

        let mut file = File::create_new(file_path).map_err(Error::FileSystem)?;
        file.write_all(toml_string.as_bytes())
            .map_err(Error::FileSystem)?;

        Ok(())
    }
    pub fn edit_config_file(self, file_path: &std::path::Path) -> Result<(), Error> {
        if !file_path.exists() {
            return Err(Error::FileNotFound);
        }
        let mut libras_from_file = Libra::read_as_vec(file_path)?;
        let mut found = false;
        for libra in libras_from_file.iter_mut() {
            if PartialEq::eq(&libra.device, &self.device) {
                libra.config = self.config.clone();
                found = true;
                break;
            }
        }
        if !found {
            return Err(Error::LibraNotFound);
        }
        fs::remove_file(file_path)?;
        Libra::new_config_file(libras_from_file, file_path)?;
        Ok(())
    }
    pub fn add_to_config_file(self, file_path: &std::path::Path) -> Result<(), Error> {
        if !file_path.exists() {
            return Err(Error::FileNotFound);
        }
        let mut libras_from_file = Libra::read_as_vec(file_path)?;
        let mut already_exists = false;
        for libra in libras_from_file.iter_mut() {
            if PartialEq::eq(&libra.device, &self.device) {
                already_exists = true;
                break;
            }
        }
        if already_exists {
            return Err(Error::LibraAlreadyExists);
        }
        fs::remove_file(file_path)?;
        libras_from_file.push(self);
        Libra::new_config_file(libras_from_file, file_path)?;
        Ok(())
    }
    pub fn remove_from_config_file(
        device: Device,
        file_path: &std::path::Path,
    ) -> Result<(), Error> {
        if !file_path.exists() {
            return Err(Error::FileNotFound);
        }
        let mut libras_from_file = Libra::read_as_vec(file_path)?;
        let original_length = libras_from_file.len();
        libras_from_file.retain(|x| x.device != device);
        if original_length == libras_from_file.len() {
            return Err(Error::LibraNotFound);
        }
        fs::remove_file(file_path)?;
        Libra::new_config_file(libras_from_file, file_path)?;
        Ok(())
    }
}
impl Default for Libra {
    fn default() -> Self {
        Self {
            config: Config::default(),
            device: Device::new(Model::LibraV0, "Lib0"),
        }
    }
}
impl Read for Libra {}
#[cfg(feature = "write")]
impl Generate<'_> for Libra {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub phidget_id: i32,
    pub load_cell_id: i32,
    pub gain: f64,
    pub offset: f64,
    pub location: String,
    pub ingredient: String,
    pub heartbeat_period: Duration,
    pub buffer_length: usize,
    pub max_noise: f64,
    pub phidget_sample_period: Duration,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            phidget_id: 0,
            load_cell_id: 0,
            gain: 1.0,
            offset: 0.0,
            location: "Caldo HQ".into(),
            ingredient: "Fake Chicken Wings".into(),
            heartbeat_period: Duration::from_secs(60),
            buffer_length: 20,
            max_noise: 3.,
            phidget_sample_period: Duration::from_millis(250),
        }
    }
}
impl Read for Config {}
#[cfg(feature = "write")]
impl Generate<'_> for Config {}

#[cfg(test)]
#[cfg(feature = "write")]
mod tests {
    use super::*;
    use crate::device::{Device, Model};
    use crate::error::Error;
    use std::collections::BTreeMap;
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};

    struct TestFile {
        path: PathBuf,
    }

    impl TestFile {
        fn new(name: &str) -> Self {
            let mut path = std::env::temp_dir();
            path.push(format!("menu-test-{}", name));

            if path.exists() {
                let _ = fs::remove_file(&path);
            }

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            if self.path.exists() {
                let _ = fs::remove_file(&self.path);
            }
        }
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.phidget_id, 0);
        assert_eq!(config.load_cell_id, 0);
        assert_eq!(config.gain, 1.0);
        assert_eq!(config.offset, 0.0);
        assert_eq!(config.location, "Caldo HQ");
        assert_eq!(config.ingredient, "Fake Chicken Wings");
        assert_eq!(config.heartbeat_period, Duration::from_secs(60));
        assert_eq!(config.buffer_length, 20);
        assert_eq!(config.max_noise, 3.0);
        assert_eq!(config.phidget_sample_period, Duration::from_millis(250));
    }

    #[test]
    fn test_libra_default() {
        let libra = Libra::default();
        assert_eq!(libra.config, Config::default());
        assert_eq!(libra.device, Device::new(Model::LibraV0, "Lib0"));
    }

    #[test]
    fn test_new_config_file_success() {
        let test_file = TestFile::new("new_config_success.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default();
        let mut libra2 = Libra::default();
        libra2.device = Device::new(Model::LibraV0, "L1");
        let libras = vec![libra1.clone(), libra2.clone()];

        let result = Libra::new_config_file(libras, file_path);
        assert!(result.is_ok());
        assert!(file_path.exists());

        let content = fs::read_to_string(file_path).unwrap();
        let mut expected_map = BTreeMap::new();
        expected_map.insert(libra1.device.to_string(), libra1);
        expected_map.insert(libra2.device.to_string(), libra2);
        let expected_content = toml::to_string(&expected_map).unwrap();

        assert_eq!(content, expected_content);
    }

    #[test]
    fn test_new_config_file_already_exists() {
        let test_file = TestFile::new("new_config_exists.toml");
        let file_path = test_file.path();
        File::create(file_path).unwrap();

        let result = Libra::new_config_file(vec![], file_path);
        assert!(result.is_err());
        match result.unwrap_err() {
            // This currently returns NotImplemented, which seems incorrect.
            Error::NotImplemented => {}
            other => panic!("Expected Error::NotImplemented, got {:?}", other),
        }
    }

    #[test]
    fn test_edit_config_file_success() {
        let test_file = TestFile::new("edit_config_success.toml");
        let file_path = test_file.path();

        let mut libra1 = Libra::default();
        libra1.device = Device::new(Model::LibraV0, "L1");
        let mut libra2 = Libra::default();
        libra2.device = Device::new(Model::LibraV0, "L2");
        Libra::new_config_file(vec![libra1.clone(), libra2.clone()], file_path).unwrap();

        let mut edited_libra1 = libra1.clone();
        edited_libra1.config.ingredient = "New Ingredient".to_string();

        let result = edited_libra1.clone().edit_config_file(file_path);
        assert!(result.is_ok());

        let libras_from_file = Libra::read_as_vec(file_path).unwrap();
        assert_eq!(libras_from_file.len(), 2);
        assert!(libras_from_file.contains(&edited_libra1));
        assert!(libras_from_file.contains(&libra2));
    }

    #[test]
    fn test_edit_config_file_not_found() {
        let test_file = TestFile::new("edit_config_not_found.toml");
        let libra = Libra::default();
        let result = libra.edit_config_file(test_file.path());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FileNotFound));
    }

    #[test]
    fn test_edit_config_file_libra_not_in_file() {
        let test_file = TestFile::new("edit_config_libra_not_in_file.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default(); // device number 0
        Libra::new_config_file(vec![libra1], file_path).unwrap();

        let mut libra_to_edit = Libra::default();
        libra_to_edit.device = Device::new(Model::LibraV0, "L1"); // different device
        libra_to_edit.config.ingredient = "New Ingredient".to_string();

        let result = libra_to_edit.edit_config_file(file_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::LibraNotFound));
    }

    #[test]
    fn test_add_to_config_file_success() {
        let test_file = TestFile::new("add_config_success.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default();
        Libra::new_config_file(vec![libra1.clone()], file_path).unwrap();

        let mut libra2 = Libra::default();
        libra2.device = Device::new(Model::LibraV0, "L1");

        let result = libra2.clone().add_to_config_file(file_path);
        assert!(result.is_ok());

        let libras_from_file = Libra::read_as_vec(file_path).unwrap();
        assert_eq!(libras_from_file.len(), 2);
        assert!(libras_from_file.contains(&libra1));
        assert!(libras_from_file.contains(&libra2));
    }

    #[test]
    fn test_add_to_config_file_not_found() {
        let test_file = TestFile::new("add_config_not_found.toml");
        let libra = Libra::default();
        let result = libra.add_to_config_file(test_file.path());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FileNotFound));
    }

    #[test]
    fn test_add_to_config_file_already_exists() {
        let test_file = TestFile::new("add_config_already_exists.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default();
        Libra::new_config_file(vec![libra1.clone()], file_path).unwrap();

        let result = libra1.add_to_config_file(file_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::LibraAlreadyExists));
    }

    #[test]
    fn test_remove_from_config_file_success() {
        let test_file = TestFile::new("remove_config_success.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default();
        let mut libra2 = Libra::default();
        libra2.device = Device::new(Model::LibraV0, "L1");
        Libra::new_config_file(vec![libra1.clone(), libra2.clone()], file_path).unwrap();

        let result = Libra::remove_from_config_file(libra1.device.clone(), file_path);
        assert!(result.is_ok());

        let libras_from_file = Libra::read_as_vec(file_path).unwrap();
        assert_eq!(libras_from_file.len(), 1);
        assert!(!libras_from_file.contains(&libra1));
        assert!(libras_from_file.contains(&libra2));
    }

    #[test]
    fn test_remove_from_config_file_not_found() {
        let test_file = TestFile::new("remove_config_not_found.toml");
        let device = Device::new(Model::LibraV0, "L1");
        let result = Libra::remove_from_config_file(device, test_file.path());
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::FileNotFound));
    }

    #[test]
    fn test_remove_from_config_file_libra_not_in_file() {
        let test_file = TestFile::new("remove_config_libra_not_in_file.toml");
        let file_path = test_file.path();

        let libra1 = Libra::default();
        Libra::new_config_file(vec![libra1], file_path).unwrap();

        let device_to_remove = Device::new(Model::LibraV0, "L99"); // Not in file

        let result = Libra::remove_from_config_file(device_to_remove, file_path);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::LibraNotFound));
    }
}
