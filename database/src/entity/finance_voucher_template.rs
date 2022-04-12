use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "finance_voucher_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    code: String,
    name: String,
    company_code: Option<String>,
    bookkeeping_date: Option<Date>,
    business_date: Option<Date>,
    period: Option<String>,
    voucher_type: String,
    voucher_id: Option<String>,
    entry_no: Option<String>,
    r#abstract: Option<String>,
    finance_account_code: String,
    currency_type: Option<String>,
    currency_rate: Option<f32>,
    direction: Option<String>,
    currency_amount: Option<f32>,
    quantity: i64,
    unit_price: f32,
    debit_amount: Option<f32>,
    credit_amount: Option<f32>,
    creator: Option<String>,
    confirmor: Option<String>,
    auditor: Option<String>,
    is_cashflow: Option<String>,
    cashflow_mark: Option<String>,
    subsidiary_business_date: Option<Date>,
    due_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
