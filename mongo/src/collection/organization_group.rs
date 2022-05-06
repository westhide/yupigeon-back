use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::{CollectionTrait, DBRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationGroup {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    name: String,
    company_items: Vec<DBRef>,
}

impl CollectionTrait for OrganizationGroup {
    fn collection_name<'a>() -> &'a str {
        "OrganizationGroup"
    }
}
