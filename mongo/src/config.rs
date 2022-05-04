use config::{Config, ConfigError};
use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| config().unwrap());

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("mongo/Config"))
        .build()
}
