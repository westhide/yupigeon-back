use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::CollectionTrait;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationCompany {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    name: String,
    finance_code: String,
}

impl CollectionTrait for OrganizationCompany {
    fn collection_name<'a>() -> &'a str {
        "OrganizationCompany"
    }

    fn primary_key(&self) -> ObjectId {
        self._id
    }
}
