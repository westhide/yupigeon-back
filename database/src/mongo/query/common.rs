use async_trait::async_trait;
use futures::stream::{TryStream, TryStreamExt};
use mongodb::{bson::doc, error::Error, results::InsertManyResult, Cursor};
use serde::{de::DeserializeOwned, Serialize};

use crate::mongo::{
    common::{CollectionTrait, DBRef},
    error::{MongoErr, Result},
    MongoPool,
};

#[async_trait]
pub trait DBRefTrait<T> {
    async fn fetch(&self) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync;
}

#[async_trait]
impl<T: CollectionTrait> DBRefTrait<T> for DBRef<T> {
    async fn fetch(&self) -> Result<T>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        T::collection()?
            .find_one(doc! {"_id":self.ref_id}, None)
            .await?
            .ok_or_else(|| {
                MongoErr::message_error(&format!(
                    "DBRef Not Found: {}._id='{}' ",
                    self.ref_name, self.ref_id
                ))
            })
    }
}

#[async_trait]
pub trait QueryTrait: CollectionTrait {
    async fn insert_many(items: Vec<Self>) -> Result<InsertManyResult> {
        let collection = Self::collection()?;
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

    async fn find_all_as_db_ref() -> Result<Vec<DBRef<Self>>>
    where
        Cursor<Self>: TryStream,
        Vec<Self>: Extend<<Cursor<Self> as TryStream>::Ok>,
        Error: From<<Cursor<Self> as TryStream>::Error>,
    {
        let db_refs = Self::find_all()
            .await?
            .iter()
            .map(|item| item.db_ref())
            .collect::<Vec<DBRef<Self>>>();
        Ok(db_refs)
    }
}

impl<T: CollectionTrait> QueryTrait for T {}

pub async fn find_all_by_collection<T: Serialize>(name: &str) -> Result<Vec<T>>
where
    Cursor<T>: TryStream,
    Vec<T>: Extend<<Cursor<T> as TryStream>::Ok>,
    Error: From<<Cursor<T> as TryStream>::Error>,
{
    let collection = MongoPool::database()?.collection::<T>(name);
    let res = collection
        .find(None, None)
        .await?
        .try_collect::<Vec<T>>()
        .await
        .map_err(Into::<Error>::into)?;
    Ok(res)
}
