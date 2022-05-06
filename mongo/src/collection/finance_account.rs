use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::{CollectionTrait, DBRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: String,
    name: String,
    direction: String,
    subsidiary_group: DBRef,
}

impl CollectionTrait for FinanceAccount {
    fn collection_name<'a>() -> &'a str {
        "FinanceAccount"
    }
}
