// @Author: westhide.yzw
// @Date: 2022-03-19 22:43:42
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:43:42

use config::{Config, ConfigError};
use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(|| config().unwrap());

pub fn config() -> Result<Config, ConfigError> {
    Config::builder()
        .add_source(config::File::with_name("yupigeon-server/Config"))
        // .add_source(config::Environment::with_prefix("APP"))
        .build()
}
