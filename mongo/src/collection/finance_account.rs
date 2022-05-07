use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAccount {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: String,
    name: String,
    direction: String,
    subsidiary_group: DBRef,
}
