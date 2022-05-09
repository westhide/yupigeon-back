use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistAccountGroup {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    pub code: String,
    pub name: String,
    pub assist_account_refs: Vec<DBRef>,
}
