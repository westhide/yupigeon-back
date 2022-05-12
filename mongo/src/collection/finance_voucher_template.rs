use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateBase {
    pub code: String,
    pub name: String,
    pub business_date: Option<String>,
    pub bookkeeping_date: Option<String>,
    pub book_code: Option<String>,
    pub book_name: Option<String>,
    pub voucher_code: String,
    pub voucher_type: String,
    pub voucher_no: Option<String>,
    pub creator_code: Option<String>,
    pub creator_name: Option<String>,
    pub r#abstract: Option<String>,
    pub due_date: Option<String>,
    pub business_no: Option<String>,
    pub measure_unit_code: Option<String>,
    pub measure_unit_name: Option<String>,
    pub quantity: Option<f32>,
    pub unit_price: Option<f32>,
    pub currency_code: String,
    pub currency_name: String,
    pub currency_rate: f32,
    pub amount: Option<f32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceVoucherTemplate {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    #[serde(flatten)]
    pub template_base: TemplateBase,
    pub organization_company_ref: DBRef,
    pub debit_finance_account_ref: DBRef,
    pub credit_finance_account_ref: DBRef,
    pub cashflow_ref: Option<DBRef>,
}
