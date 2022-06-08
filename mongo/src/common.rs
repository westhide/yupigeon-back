use std::marker::PhantomData;

pub use macro_lib::DeriveCollection;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBRef<T> {
    #[serde(rename = "$ref")]
    pub ref_name: String,
    #[serde(rename = "$id")]
    pub ref_id: ObjectId,
    #[serde(skip)]
    _unused: PhantomData<T>,
}

impl<T> DBRef<T> {
    pub fn new(ref_name: &str, ref_id: ObjectId) -> Self {
        Self {
            ref_name: ref_name.to_string(),
            ref_id,
            _unused: PhantomData,
        }
    }
}

pub trait CollectionTrait: Serialize + Sized + Send + Sync {
    fn collection_name<'a>() -> &'a str;

    fn primary_key(&self) -> ObjectId;

    fn get_collection_name<'a>(&self) -> &'a str {
        Self::collection_name()
    }

    fn collection() -> Result<Collection<Self>> {
        let db = crate::MongoPool::database()?;

        Ok(db.collection::<Self>(Self::collection_name()))
    }

    fn db_ref<T>(&self) -> DBRef<T> {
        DBRef::new(Self::collection_name(), self.primary_key())
    }
}
