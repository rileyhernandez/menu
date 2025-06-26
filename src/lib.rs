#[cfg(feature = "generate")]
pub mod backend;
pub mod ichibu;
pub mod ichibu_items;
pub mod device;
pub mod error;
#[cfg(feature = "generate")]
pub mod generate;
pub mod libra;
pub mod read;
#[cfg(feature = "generate")]
pub mod pull;

#[cfg(test)]
mod tests {
    use crate::ichibu::{Ichibu};
    use anyhow::Result;
    use std::path::Path;
    use crate::libra::Config;
    use crate::read::Read;

    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";

    #[test]
    fn load() -> Result<()> {
        let config = Ichibu::read(Path::new(READ_PATH));
        println!("{:?}", config?);

        Ok(())
    }
    #[test]
    fn load_libra() -> Result<()> {
        let config = Config::read_as_vec(Path::new(WRITE_PATH));
        println!("{:?}", config?);
        Ok(())
    }
}

#[cfg(feature = "generate")]
#[cfg(test)]
mod generate_tests {
    use std::array;
    use crate::backend::{CalibrationBackend, ConfigBackend, CONFIG_BACKEND_URL};
    use crate::ichibu::{Ichibu, ScaleConfig};
    use crate::device::Device;
    use crate::device::Model;
    use crate::generate::Generate;
    use crate::libra::{Config, Libra};
    use anyhow::Result;
    use std::path::Path;
    use crate::pull::FromBackend;
    use crate::read::Read;

    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";
    const CALIBRATION_PATH: &str =
        "https://us-west1-calibration-backend.cloudfunctions.net/test-function";
    const CONFIG_PATH: &str = "http://127.0.0.1:8080";

    #[test]
    fn generate() -> Result<()> {
        let mut config = Ichibu::read(Path::new(READ_PATH))?;
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
    #[test]
    fn libra() -> Result<()> {
        // let names: [String; 3] = array::from_fn(|i| (i+3).to_string());
        // for name in names {
        //     let config = Config::test();
        //     let device = Device::new(Model::LibraV0, 0);
        //     let libra = Libra { config, device};
        //     libra.add_as_table(Path::new(WRITE_PATH), &format!("LibraV0-{}", name))?;
        // }
        // let url = CONFIG_PATH;
        // let libra = Libra::pull_from_backend(Device::new(Model::LibraV0, 0), url)?;
        // println!("{:?}", libra);

        let libras = Libra::read_as_vec(Path::new(WRITE_PATH))?;
        println!("{:?}", libras);
        
        Ok(())
    }
}
