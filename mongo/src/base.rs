use mongodb::{
    error::Result,
    options::{ClientOptions, Compressor},
    Client, Collection, Database,
};
use once_cell::sync::OnceCell;
use serde::Serialize;

use crate::{config::GLOBAL_CONFIG, query::common::CollectionTrait};

pub struct Mongo {
    database: Database,
}

pub static MONGO: OnceCell<Mongo> = OnceCell::new();

impl Mongo {
    pub async fn init() -> Result<()> {
        let db_url = GLOBAL_CONFIG.get::<String>("MONGODB_URL").unwrap();

        let mut client_options = ClientOptions::parse(db_url).await?;
        client_options.app_name = Some("Mongo".to_string());
        client_options.compressors = Some(vec![Compressor::Zstd { level: Some(3) }]);

        let client = Client::with_options(client_options)?;
        let database = client.database("yupigeon");

        let mongo = Mongo { database };
        MONGO.set(mongo).ok();
        Ok(())
    }

    pub fn database<'a>() -> &'a Database {
        let mongo = MONGO.get().expect("Mongo is not exists");
        &mongo.database
    }

    pub fn collection<T>() -> Collection<T>
    where
        T: Serialize + CollectionTrait,
    {
        T::collection()
    }
}
