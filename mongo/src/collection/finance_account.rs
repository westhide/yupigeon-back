use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    pub code: String,
    pub name: String,
    pub direction: String,
    pub assist_account_group: Option<DBRef>,
}
