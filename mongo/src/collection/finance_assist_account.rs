use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub code: String,
    pub name: String,
    pub collection_name: String,
    pub assist_items: Vec<DBRef>,
}
