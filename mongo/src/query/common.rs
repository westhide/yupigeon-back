use async_trait::async_trait;
use futures::stream::{TryStream, TryStreamExt};
use mongodb::{error::Error, results::InsertManyResult, Cursor};
use serde::Serialize;

use crate::{
    common::CollectionTrait,
    error::{MongoErr, Result},
};

#[async_trait]
pub trait QueryTrait: CollectionTrait {
    async fn insert_many(items: Vec<Self>) -> Result<InsertManyResult> {
        let collection = Self::collection();
        collection
            .insert_many(items, None)
            .await
            .map_err(Into::<MongoErr>::into)
    }

    async fn find_all() -> Result<Vec<Self>>
    where
        Cursor<Self>: TryStream,
        Vec<Self>: Extend<<Cursor<Self> as TryStream>::Ok>,
        Error: From<<Cursor<Self> as TryStream>::Error>,
    {
        find_all_by_collection(Self::collection_name()).await
    }
}

impl<T: CollectionTrait> QueryTrait for T {}

pub async fn find_all_by_collection<T: Serialize>(name: &str) -> Result<Vec<T>>
where
    Cursor<T>: TryStream,
    Vec<T>: Extend<<Cursor<T> as TryStream>::Ok>,
    Error: From<<Cursor<T> as TryStream>::Error>,
{
    let collection = crate::Mongo::database().collection::<T>(name);
    let res = collection
        .find(None, None)
        .await?
        .try_collect::<Vec<T>>()
        .await
        .map_err(Into::<Error>::into)?;
    Ok(res)
}
