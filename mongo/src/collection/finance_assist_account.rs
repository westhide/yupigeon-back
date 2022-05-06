use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::{CollectionTrait, DBRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: Option<String>,
    name: String,
    pub assist_items: Option<Vec<DBRef>>,
}

impl CollectionTrait for FinanceAssistAccount {
    fn collection_name<'a>() -> &'a str {
        "FinanceAssistAccount"
    }

    fn primary_key(&self) -> ObjectId {
        self._id
    }
}
