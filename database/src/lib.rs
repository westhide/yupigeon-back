// @Author: westhide.yzw
// @Date: 2022-03-20 16:15:05
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-20 16:15:05

mod config;
pub mod entity;

use std::collections::HashMap;

use once_cell::sync::OnceCell;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, ExecResult, Statement,
    TransactionTrait,
};

use crate::config::GLOBAL_CONFIG;

pub static DB: OnceCell<HashMap<String, DatabaseConnection>> = OnceCell::new();

pub struct Database {
    txn: DatabaseTransaction,
}

impl Database {
    pub async fn new(key: &str) -> Result<Database, DbErr> {
        let txn = Self::txn(key).await?;
        let database = Database { txn };
        database.set_time_zone("+8:00").await?;
        Ok(database)
    }

    async fn connect(key: &str) -> Result<DatabaseConnection, DbErr> {
        let db_url = GLOBAL_CONFIG.get::<String>(key).unwrap();
        sea_orm::Database::connect(db_url).await
    }

    pub async fn init() -> Result<(), DbErr> {
        let mut db_list = HashMap::<String, DatabaseConnection>::new();

        let db_default = Self::connect("MYSQL_URL_DEFAULT").await?;
        let db_laiu8 = Self::connect("MYSQL_URL_LAIU8").await?;

        db_list.insert("default".into(), db_default);
        db_list.insert("laiu8".into(), db_laiu8);

        DB.set(db_list)
            .expect("Can not set global database connection list");
        Ok(())
    }

    pub fn connection(key: &str) -> &DatabaseConnection {
        let db_list = DB.get().expect("Database is not initialized");
        db_list.get(key).expect("Database is not exists")
    }

    async fn txn(key: &str) -> Result<DatabaseTransaction, DbErr> {
        let connection = Self::connection(key);
        connection.begin().await
    }

    async fn set_time_zone(&self, time_zone: &str) -> Result<ExecResult, DbErr> {
        let sql = format!("SET time_zone = '{}';", time_zone);
        self.execute_sql(&sql).await
    }

    pub async fn execute_sql(&self, sql: &str) -> Result<ExecResult, DbErr> {
        let txn = &self.txn;
        txn.execute(Statement::from_string(
            txn.get_database_backend(),
            sql.into(),
        ))
        .await
    }

    pub async fn execute_multi_sql(&self, multi_sql: Vec<&str>) -> Result<Vec<ExecResult>, DbErr> {
        let txn = &self.txn;
        let mut results = vec![];
        for sql in multi_sql {
            let result = txn
                .execute(Statement::from_string(
                    txn.get_database_backend(),
                    sql.into(),
                ))
                .await?;
            results.push(result)
        }
        Ok(results)
    }
}
