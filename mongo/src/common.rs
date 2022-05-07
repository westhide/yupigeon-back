use async_trait::async_trait;
pub use macro_lib::DeriveCollection;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Result,
    Collection,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

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
pub trait DBRefTrait {
    async fn fetch<T>(&self) -> Result<Option<T>>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync;
}

#[async_trait]
impl DBRefTrait for DBRef {
    async fn fetch<T>(&self) -> Result<Option<T>>
    where
        T: Serialize + DeserializeOwned + Unpin + Send + Sync,
    {
        let db = crate::Mongo::database();

        let collection = db.collection::<T>(&self._ref);
        collection.find_one(doc! {"_id":self._id}, None).await
    }
}

pub trait CollectionTrait: Serialize + Sized {
    fn collection_name<'a>() -> &'a str;

    fn primary_key(&self) -> ObjectId;

    fn get_collection_name<'a>(&self) -> &'a str {
        Self::collection_name()
    }

    fn collection() -> Collection<Self> {
        let db = crate::Mongo::database();

        db.collection::<Self>(Self::collection_name())
    }

    fn db_ref(&self) -> DBRef {
        DBRef::new(Self::collection_name(), self.primary_key())
    }
}
