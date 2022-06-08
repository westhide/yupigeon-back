// @Author: westhide.yzw
// @Date: 2022-03-19 22:43:42
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:43:42

use config::{Config, ConfigError};
use once_cell::sync::Lazy;

use crate::service::error::WrapError;

pub static GLOBAL_CONFIG: Lazy<Result<Config, ConfigError>> = Lazy::new(config);

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("server/Config"))
        .build()
}

pub fn get_config(key: &str) -> Result<String, WrapError> {
    match GLOBAL_CONFIG.as_ref() {
        Ok(config) => config
            .get(key)
            .map_err(|e| WrapError::Message(e.to_string())),
        Err(e) => Err(WrapError::Message(e.to_string())),
    }
}
