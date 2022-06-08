use config::{Config, ConfigError};
use once_cell::sync::Lazy;

use crate::error::MongoErr;

pub static GLOBAL_CONFIG: Lazy<Result<Config, ConfigError>> = Lazy::new(config);

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("mongo/Config"))
        .build()
}

pub fn get_global_config(key: &str) -> Result<String, MongoErr> {
    match GLOBAL_CONFIG.as_ref() {
        Ok(config) => config
            .get(key)
            .map_err(|e| MongoErr::Message(e.to_string())),
        Err(e) => Err(MongoErr::Message(e.to_string())),
    }
}
