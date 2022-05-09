use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::common::{CollectionTrait, DBRef, DeriveCollection};

#[derive(Clone, Debug, Deserialize, Serialize, DeriveCollection)]
#[serde(rename_all = "camelCase")]
pub struct FinanceVoucherTemplate {
    #[serde(rename = "_id", default)]
    pub _id: ObjectId,
    code: String,
    name: String,
    business_date: Option<String>,
    bookkeeping_date: Option<String>,
    pub organization_company_ref: DBRef,
    book_code: Option<String>,
    book_name: Option<String>,
    voucher_code: String,
    voucher_type: String,
    voucher_no: Option<String>,
    creator_code: Option<String>,
    creator_name: Option<String>,
    r#abstract: Option<String>,
    pub debit_finance_account_ref: DBRef,
    pub credit_finance_account_ref: DBRef,
    due_date: Option<String>,
    business_no: Option<String>,
    measure_unit_code: Option<String>,
    measure_unit_name: Option<String>,
    quantity: Option<f32>,
    unit_price: Option<f32>,
    currency_code: String,
    currency_name: String,
    amount: Option<f32>,
    pub cashflow_ref: Option<DBRef>,
    pub supplement_ref: Option<DBRef>,
}
