use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::{CollectionTrait, DBRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistAccountGroup {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: String,
    name: String,
    assist_account_items: Vec<DBRef>,
}

impl CollectionTrait for FinanceAssistAccountGroup {
    fn collection_name<'a>() -> &'a str {
        "FinanceAssistAccountGroup"
    }
}
