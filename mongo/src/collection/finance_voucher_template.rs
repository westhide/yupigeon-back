use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::query::common::{CollectionTrait, DBRef};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FinanceVoucherTemplate {
    #[serde(rename = "_id")]
    pub _id: ObjectId,
    code: String,
    name: String,
    business_date: Option<String>,
    bookkeeping_date: Option<String>,
    organization_company: DBRef,
    book_code: Option<String>,
    book_name: Option<String>,
    voucher_code: String,
    voucher_type: String,
    voucher_no: Option<String>,
    creator_code: Option<String>,
    creator_name: Option<String>,
    r#abstract: Option<String>,
    debit_finance_account: DBRef,
    credit_finance_account: DBRef,
    due_date: Option<String>,
    business_no: Option<String>,
    measure_unit_code: Option<String>,
    measure_unit_name: Option<String>,
    quantity: Option<f32>,
    unit_price: Option<f32>,
    currency_code: String,
    currency_name: String,
    amount: Option<f32>,
    cashflow: DBRef,
    supplement: DBRef,
}

impl CollectionTrait for FinanceVoucherTemplate {
    fn collection_name<'a>() -> &'a str {
        "FinanceVoucherTemplate"
    }

    fn primary_key(&self) -> ObjectId {
        self._id
    }
}
