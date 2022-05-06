use async_trait::async_trait;
use mongodb::{bson::oid::ObjectId, error::Result, results::InsertManyResult, Collection};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBRef {
    #[serde(rename = "$ref")]
    _ref: String,
    #[serde(rename = "$id")]
    _id: ObjectId,
}

impl DBRef {
    pub fn new(_ref: &str, _id: ObjectId) -> Self {
        Self {
            _ref: _ref.to_string(),
            _id,
        }
    }
}

#[async_trait]
pub trait CollectionTrait: Serialize {
    fn collection_name<'a>() -> &'a str;

    fn primary_key(&self) -> ObjectId;

    fn get_collection_name<'a>(&self) -> &'a str {
        Self::collection_name()
    }

    fn collection() -> Collection<Self>
    where
        Self: Sized,
    {
        let db = crate::Mongo::database();

        db.collection::<Self>(Self::collection_name())
    }

    fn db_ref(&self) -> DBRef {
        DBRef::new(Self::collection_name(), self.primary_key())
    }

    async fn insert_many(items: Vec<Self>) -> Result<InsertManyResult>
    where
        Self: Send + Sync + Sized,
    {
        let collection = Self::collection();
        collection.insert_many(items, None).await
    }
}
