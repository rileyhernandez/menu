pub mod config;
pub mod error;
pub mod config_items;
#[cfg(feature = "generate")]
pub mod backend;
#[cfg(feature = "generate")]
pub mod device;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::path::Path;
    use crate::config::{Config, Read};
    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";

    #[test]
    fn load() -> Result<()> {
        let config = Config::read(Path::new(READ_PATH));
        println!("{:?}", config?);

        Ok(())
    }
}

#[cfg(feature = "generate")]
#[cfg(test)]
mod generate_tests {
    use std::path::Path;
    use crate::backend::{CalibrationBackend, ConfigBackend};
    use crate::config::{Config, Generate, Read, ScaleConfig};
    use anyhow::Result;
    use crate::device::Model;
    use crate::device::Device;


    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";
    const CALIBRATION_PATH: &str = "https://us-west1-calibration-backend.cloudfunctions.net/test-function";
    const CONFIG_PATH: &str = "http://127.0.0.1:8080";

    #[test]
    fn generate() -> Result<()> {
        let mut config = Config::read(Path::new(READ_PATH))?;
        config.overwrite_toml(Path::new(WRITE_PATH))?;
        Ok(())
    }
    #[test]
    fn calibration_backend() -> Result<()> {
        let backend = CalibrationBackend::new(CALIBRATION_PATH.into());
        let payload = backend.get_config(69420)?;
        println!("{:?}", payload);
        Ok(())
    }
    #[test]
    fn config_backend() -> Result<()> {
        let backend = ConfigBackend::new(CONFIG_PATH.into());
        let device = Device::new(Model::IchibuV2, 0);
        let payload = backend.get_config(device)?;
        println!("{:?}", payload);
        Ok(())
    }
}