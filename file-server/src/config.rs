use config::{Config, ConfigError};

pub fn config() -> Result<Config, ConfigError> {
    let config = Config::builder()
        .add_source(config::File::with_name("file-server/Config"))
        // .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    Ok(config)
}
