use async_trait::async_trait;
use mongodb::{error::Result, results::InsertManyResult};

use crate::common::CollectionTrait;

#[async_trait]
pub trait QueryTrait: CollectionTrait {
    async fn insert_many(items: Vec<Self>) -> Result<InsertManyResult>
    where
        Self: Send + Sync,
    {
        let collection = Self::collection();
        collection.insert_many(items, None).await
    }
}

impl<T: CollectionTrait> QueryTrait for T {}
