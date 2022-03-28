// @Author: westhide.yzw
// @Date: 2022-03-20 16:15:05
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-20 16:15:05

mod config;
pub mod entity;

use std::collections::HashMap;

use once_cell::sync::OnceCell;
use sea_orm::{
    ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, DatabaseTransaction, DbErr,
    ExecResult, Statement, TransactionTrait,
};

use crate::config::GLOBAL_CONFIG;

pub static DB: OnceCell<HashMap<String, DatabaseConnection>> = OnceCell::new();

pub async fn db_connect(key: &str) -> Result<DatabaseConnection, DbErr> {
    let db_url = GLOBAL_CONFIG.get::<String>(key).unwrap();
    Database::connect(db_url).await
}

pub async fn init_database() -> Result<(), DbErr> {
    let mut db_list = HashMap::<String, DatabaseConnection>::new();

    let db_default = db_connect("MYSQL_URL_DEFAULT").await?;
    let db_laiu8 = db_connect("MYSQL_URL_LAIU8").await?;

    db_list.insert("default".into(), db_default);
    db_list.insert("laiu8".into(), db_laiu8);

    DB.set(db_list)
        .expect("Can not set global database connection list");
    Ok(())
}

pub fn get_db(key: &str) -> &'static DatabaseConnection {
    let db_list = DB.get().expect("Database is not initialized");
    db_list.get(key).expect("Database is not exists")
}

async fn set_time_zone(txn: &DatabaseTransaction, time_zone: &str) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        format!(
            "
            SET time_zone = '{}';
        ",
            time_zone
        ),
    ))
    .await
}

pub async fn get_txn(key: &str) -> Result<DatabaseTransaction, DbErr> {
    let connection = get_db(key);
    let txn = connection.begin().await?;
    set_time_zone(&txn, "+8:00").await?;
    Ok(txn)
}
