use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationGroup {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    name: String,
    company_items: Vec<DBRef>,
}
