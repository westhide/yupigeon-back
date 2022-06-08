use mongodb::{
    options::{ClientOptions, Compressor},
    Client, Database,
};
use once_cell::sync::OnceCell;

use crate::{
    config::get_config,
    mongo::error::{MongoErr, Result},
};

pub struct MongoPool {
    database: Database,
}

pub static MONGO_POOL: OnceCell<MongoPool> = OnceCell::new();

impl MongoPool {
    fn set_mongo_pool(pool: Self) -> Result<()> {
        MONGO_POOL
            .set(pool)
            .map_err(|_| MongoErr::message_error("Can Not Set MONGO_POOL twice"))
    }

    fn get_mongo_pool<'a>() -> Result<&'a Self> {
        MONGO_POOL
            .get()
            .ok_or_else(|| MongoErr::message_error("MONGO_POOL Not Found"))
    }

    pub async fn init() -> Result<()> {
        let db_url = get_config("MONGODB_URL").map_err(|e| MongoErr::Message(e.to_string()))?;

        let mut client_options = ClientOptions::parse(db_url).await?;
        client_options.app_name = Some("Mongo".to_string());
        client_options.compressors = Some(vec![Compressor::Zstd { level: Some(3) }]);

        let client = Client::with_options(client_options)?;
        let database = client.database("yupigeon01");

        let mongo_pool = Self { database };
        Self::set_mongo_pool(mongo_pool)
    }

    pub fn database<'a>() -> Result<&'a Database> {
        let mongo_pool = Self::get_mongo_pool()?;
        Ok(&mongo_pool.database)
    }
}
