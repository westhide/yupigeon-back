use mongodb::{error::Result, results::InsertManyResult};
use serde::Serialize;

pub async fn insert_many<T>(items: Vec<T>, collection: &str) -> Result<InsertManyResult>
where
    T: Serialize,
{
    let db = crate::Mongo::database().await?;

    let collection = db.collection::<T>(collection);
    collection.insert_many(items, None).await
}
