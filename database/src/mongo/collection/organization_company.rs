use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::mongo::common::{CollectionTrait, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationCompany {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    pub name: String,
    pub finance_code: String,
}
