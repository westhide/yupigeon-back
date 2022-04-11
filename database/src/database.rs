use std::collections::HashMap;

use once_cell::sync::OnceCell;
use sea_orm::{
    sea_query::Value, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, ExecResult,
    FromQueryResult, Statement, TransactionTrait,
};

use crate::config::GLOBAL_CONFIG;

pub static DB: OnceCell<HashMap<String, DatabaseConnection>> = OnceCell::new();

pub struct Database {
    pub txn: DatabaseTransaction,
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
        let db_backend = txn.get_database_backend();
        txn.execute(Statement::from_string(db_backend, sql.into()))
            .await
    }

    pub async fn execute_multi_sql(&self, multi_sql: Vec<&str>) -> Result<Vec<ExecResult>, DbErr> {
        let txn = &self.txn;
        let db_backend = txn.get_database_backend();
        let mut results = vec![];
        for sql in multi_sql {
            let result = txn
                .execute(Statement::from_string(db_backend, sql.into()))
                .await?;
            results.push(result)
        }
        Ok(results)
    }

    pub async fn find_by_sql<T: FromQueryResult>(&self, sql: &str) -> Result<Vec<T>, DbErr> {
        let db_backend = self.txn.get_database_backend();
        T::find_by_statement(Statement::from_string(db_backend, sql.into()))
            .all(&self.txn)
            .await
    }

    pub async fn find_by_sql_and_values<T, V>(&self, sql: &str, values: V) -> Result<Vec<T>, DbErr>
    where
        T: FromQueryResult,
        V: IntoIterator<Item = Value>,
    {
        let db_backend = self.txn.get_database_backend();
        T::find_by_statement(Statement::from_sql_and_values(db_backend, sql, values))
            .all(&self.txn)
            .await
    }
}
