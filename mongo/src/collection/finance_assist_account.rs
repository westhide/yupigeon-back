use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: Option<String>,
    name: String,
    pub assist_items: Option<Vec<DBRef>>,
}
