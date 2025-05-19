mod config;
mod error;
mod config_items;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::fs;

    use crate::config::Config;

    fn load_file(file: String) -> Result<Config> {
        let file_as_string = fs::read_to_string(file)?;
        let config: Config = toml::from_str(&file_as_string)?;
        Ok(config)
    }

    #[test]
    fn load() -> Result<()> {
        let config = load_file("config.toml".into());
        assert!(config.is_ok());
        println!("{:?}", config?);

        Ok(())
    }
    #[test]
    fn generate() -> Result<()> {
        let config = load_file("config.toml".into())?;
        config.generate_toml("test.toml")?;

        Ok(())
    }
}
