use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "investment_real_estates_lease_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    investment_real_estates_id: u32,
    serial_no: u32,
    client: String,
    status: String,
    brand: Option<String>,
    business_type: Option<String>,
    lease_commencement_date: Date,
    lease_end_date: Date,
    canceling_date: Option<Date>,
    term: u32,
    total_rent: Option<Decimal>,
    each_term_rent: Option<Decimal>,
    attachment: Option<String>,
    remark: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
