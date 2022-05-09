use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceAssistPayment {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    code: String,
    name: String,
}
