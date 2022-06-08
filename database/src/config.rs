// @Author: westhide.yzw
// @Date: 2022-03-19 22:43:42
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:43:42

use config::{Config, ConfigError};
use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Result<Config, ConfigError>> = Lazy::new(config);

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("database/Config"))
        .build()
}

pub fn get_config(key: &str) -> Result<String, ConfigError> {
    match GLOBAL_CONFIG.as_ref() {
        Ok(config) => config.get(key),
        Err(e) => Err(ConfigError::Message(e.to_string())),
    }
}
