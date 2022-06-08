use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::OrganizationCompany;
use crate::mongo::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct OrganizationGroup {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    name: String,
    organization_company_refs: Vec<DBRef<OrganizationCompany>>,
}
