#[cfg(feature = "write")]
pub mod backend;
pub mod device;
pub mod error;
#[cfg(feature = "write")]
pub mod generate;
pub mod ichibu;
pub mod ichibu_items;
pub mod libra;
#[cfg(feature = "write")]
pub mod pull;
pub mod read;

#[cfg(test)]
mod tests {
    use crate::ichibu::Ichibu;
    use crate::libra::Config;
    use crate::read::Read;
    use anyhow::Result;
    use std::path::Path;

    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";

    // #[test]
    // fn load() -> Result<()> {
    //     let config = Ichibu::read(Path::new(READ_PATH));
    //     println!("{:?}", config?);
    //
    //     Ok(())
    // }
    // #[test]
    // fn load_libra() -> Result<()> {
    //     let config = Config::read_as_vec(Path::new(WRITE_PATH));
    //     println!("{:?}", config?);
    //     Ok(())
    // }
}

#[cfg(feature = "write")]
#[cfg(test)]
mod libra_write_tests {
    use crate::backend::{CONFIG_BACKEND_URL, CalibrationBackend, ConfigBackend};
    use crate::device::Device;
    use crate::device::Model;
    use crate::generate::Generate;
    use crate::ichibu::{Ichibu, ScaleConfig};
    use crate::libra::{Config, Libra};
    use crate::pull::FromBackend;
    use crate::read::Read;
    use anyhow::Result;
    use std::path::{Path, PathBuf};
    use std::{env, fs};

    fn create_test_directory(directory_name: &str) -> Result<PathBuf> {
        let home = env::var("HOME")?;
        let path = Path::new(&env::var("HOME")?).join(".config").join(directory_name);
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir(&path)?;
        Ok(path)
    }
    fn make_default_config(config_path: &Path) -> Result<()> {
        let libras = vec![Libra::default()];
        Libra::new_config_file(libras, config_path)?;
        Ok(())
    }

    #[test]
    fn generate_config() -> Result<()> {
        let directory = create_test_directory("generate_config")?;
        let config_path = directory.join("config.toml");
        make_default_config(&config_path)?;

        let model_libra = vec![Libra::default()];
        let generated_libras = Libra::read_as_vec(&config_path)?;
        assert_eq!(model_libra, Libra::read_as_vec(&config_path)?);
        fs::remove_dir_all(directory)?;
        Ok(())
    }
    #[test]
    fn edit_config() -> Result<()> {
        let directory = create_test_directory("edit_config")?;
        let config_path = directory.join("config.toml");
        make_default_config(&config_path)?;

        let mut edited_libra = Libra::default();
        edited_libra.config.location = "New Location".into();
        let edited_clone = vec![edited_libra.clone()];
        edited_libra.edit_config_file(&config_path)?;

        let libra_from_new_file = Libra::read_as_vec(&config_path)?;
        assert_eq!(edited_clone, libra_from_new_file);
        fs::remove_dir_all(directory)?;
        Ok(())
    }
}

#[cfg(feature = "write")]
#[cfg(test)]
mod generate_tests {
    use crate::backend::{CONFIG_BACKEND_URL, CalibrationBackend, ConfigBackend};
    use crate::device::Device;
    use crate::device::Model;
    use crate::generate::Generate;
    use crate::ichibu::{Ichibu, ScaleConfig};
    use crate::libra::{Config, Libra};
    use crate::pull::FromBackend;
    use crate::read::Read;
    use anyhow::Result;
    use std::array;
    use std::path::Path;

    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";
    const CALIBRATION_PATH: &str =
        "https://us-west1-calibration-backend.cloudfunctions.net/test-function";
    const CONFIG_PATH: &str = "http://127.0.0.1:8080";

    // #[test]
    // fn generate() -> Result<()> {
    //     let mut config = Ichibu::read(Path::new(READ_PATH))?;
    //     config.overwrite_toml(Path::new(WRITE_PATH))?;
    //     Ok(())
    // }
    // #[test]
    // fn calibration_backend() -> Result<()> {
    //     let backend = CalibrationBackend::new(CALIBRATION_PATH.into());
    //     let payload = backend.get_config(69420)?;
    //     println!("{:?}", payload);
    //     Ok(())
    // }
    // #[test]
    // fn config_backend() -> Result<()> {
    //     let backend = ConfigBackend::new(CONFIG_PATH.into());
    //     let device = Device::new(Model::IchibuV2, 0);
    //     let payload = backend.get_config(device)?;
    //     println!("{:?}", payload);
    //     Ok(())
    // }
    // #[test]
    // fn libra() -> Result<()> {
    //
    //     let libras = Libra::read_as_vec(Path::new(WRITE_PATH))?;
    //     println!("{:?}", libras);
    //
    //     Ok(())
    // }
}
