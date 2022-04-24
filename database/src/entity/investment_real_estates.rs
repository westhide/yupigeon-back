use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "investment_real_estates")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    name: String,
    location: Option<String>,
    code: String,
    floor: Option<String>,
    leasable_area: Decimal,
    status: Option<String>,
    client: Option<String>,
    brand: Option<String>,
    business_type: Option<String>,
    lease_commencement_date: Option<Date>,
    lease_end_date: Option<Date>,
    term: Option<u32>,
    total_rent: Option<Decimal>,
    each_term_rent: Decimal,
    attachment: Option<String>,
    remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
