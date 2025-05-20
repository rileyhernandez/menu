pub mod config;
pub mod error;
pub mod config_items;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::path::Path;
    use crate::config::Config;
    const READ_PATH: &str = "config.toml";
    const WRITE_PATH: &str = "/home/riley/Downloads/test/config.toml";

    #[test]
    fn load() -> Result<()> {
        let config = Config::read(Path::new(READ_PATH));
        println!("{:?}", config?);

        Ok(())
    }
    
    #[cfg(feature = "generate")]
    #[test]
    fn generate() -> Result<()> {
        let config = Config::read(Path::new(READ_PATH))?;
        config.generate_toml(Path::new(WRITE_PATH))?;
        Ok(())
    }
}
