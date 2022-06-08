use std::collections::HashMap;

use once_cell::sync::OnceCell;
use sea_orm::{
    sea_query::Value, ConnectionTrait, DatabaseConnection, DatabaseTransaction, DbErr, ExecResult,
    FromQueryResult, Statement, TransactionTrait,
};

use crate::config::get_config;

type DatabasePool = HashMap<String, DatabaseConnection>;
pub static DATABASE_POOL: OnceCell<DatabasePool> = OnceCell::new();

fn set_database_pool(db_pool: DatabasePool) -> Result<(), DbErr> {
    DATABASE_POOL
        .set(db_pool)
        .map_err(|_| DbErr::Custom("Can Not Set DATABASE_POOL twice".to_string()))
}

fn get_database_pool<'a>() -> Result<&'a DatabasePool, DbErr> {
    DATABASE_POOL
        .get()
        .ok_or_else(|| DbErr::Custom("DATABASE_POOL Not Found".to_string()))
}

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
        let db_url = get_config(key).map_err(|e| DbErr::Custom(e.to_string()))?;
        sea_orm::Database::connect(db_url).await
    }

    pub async fn init() -> Result<(), DbErr> {
        let mut db_pool = HashMap::<String, DatabaseConnection>::new();

        let db_default = Self::connect("MYSQL_URL_DEFAULT").await?;
        let db_laiu8 = Self::connect("MYSQL_URL_LAIU8").await?;

        db_pool.insert("default".into(), db_default);
        db_pool.insert("laiu8".into(), db_laiu8);

        set_database_pool(db_pool)
    }

    pub fn connection(key: &str) -> Result<&DatabaseConnection, DbErr> {
        let db_pool = get_database_pool()?;
        db_pool
            .get(key)
            .ok_or_else(|| DbErr::Custom("Database Connection Not Found".to_string()))
    }

    async fn txn(key: &str) -> Result<DatabaseTransaction, DbErr> {
        let connection = Self::connection(key)?;
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
