use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::CollectionTrait;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistSupplier {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: String,
    name: String,
}

impl CollectionTrait for FinanceAssistSupplier {
    fn collection_name<'a>() -> &'a str {
        "FinanceAssistSupplier"
    }

    fn primary_key(&self) -> ObjectId {
        self._id
    }
}
