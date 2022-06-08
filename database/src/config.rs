// @Author: westhide.yzw
// @Date: 2022-03-19 22:43:42
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:43:42

use config::{Config, ConfigError};
use once_cell::sync::Lazy;
use sea_orm::DbErr;

pub static GLOBAL_CONFIG: Lazy<Result<Config, ConfigError>> = Lazy::new(config);

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("database/Config"))
        .build()
}

pub fn get_global_config(key: &str) -> Result<String, DbErr> {
    match GLOBAL_CONFIG.as_ref() {
        Ok(config) => config.get(key).map_err(|e| DbErr::Custom(e.to_string())),
        Err(e) => Err(DbErr::Custom(e.to_string())),
    }
}
