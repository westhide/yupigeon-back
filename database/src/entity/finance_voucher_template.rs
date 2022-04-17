use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "finance_voucher_template")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: i32,
    pub code: String,
    name: String,
    company_code: Option<String>,
    bookkeeping_date: Option<Date>,
    business_date: Option<Date>,
    period: Option<String>,
    voucher_type: String,
    voucher_id: Option<String>,
    entry_no: Option<String>,
    r#abstract: Option<String>,
    pub debit_finance_account_code: String,
    debit_finance_account_name: Option<String>,
    pub credit_finance_account_code: String,
    credit_finance_account_name: Option<String>,
    currency_type: Option<String>,
    currency_rate: f32,
    direction: Option<String>,
    currency_amount: Option<f32>,
    quantity: i64,
    unit_price: f32,
    amount: Option<f32>,
    creator: Option<String>,
    confirmor: Option<String>,
    auditor: Option<String>,
    attachment_num: i64,
    confirm_mark: bool,
    system_module: Option<String>,
    delete_mark: bool,
    voucher_no: Option<String>,
    unit: Option<String>,
    reference: Option<String>,
    is_cashflow: Option<String>,
    cashflow_mark: Option<String>,
    business_no: Option<String>,
    settlement_method: Option<String>,
    settlement_no: Option<String>,
    subsidiary_business_date: Option<Date>,
    due_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
