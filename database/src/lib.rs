// @Author: westhide.yzw
// @Date: 2022-03-20 16:15:05
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-20 16:15:05

mod config;
pub mod entity;

use sea_orm::{Database, DatabaseConnection};

use crate::config::GLOBAL_CONFIG;

pub async fn init_database() -> DatabaseConnection {
    let db_url = GLOBAL_CONFIG.get::<String>("MYSQL_URL").unwrap();
    Database::connect(db_url).await.unwrap()
}
