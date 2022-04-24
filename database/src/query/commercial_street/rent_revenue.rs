use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct RentRevenue {
    #[serde(rename(serialize = "subsidiaryAbstract"))]
    subsidiary_abstract: String,
    #[serde(rename(serialize = "D__client"))]
    debit_client: String,
    #[serde(rename(serialize = "C__client"))]
    credit_client: String,
    #[serde(rename(serialize = "D__receiptType"))]
    debit_receipt_type: String,
    #[serde(rename(serialize = "C__receiptType"))]
    credit_receipt_type: String,
    #[serde(rename(serialize = "C_taxRate"))]
    credit_tax_rate: String,
    #[serde(rename(serialize = "C__businessType"))]
    credit_business_type: String,
    #[serde(rename(serialize = "monthlyRent"))]
    monthly_rent: Decimal,
    #[serde(rename(serialize = "taxRate"))]
    tax_rate: Decimal,
}

pub async fn rent_revenue() -> Result<Vec<RentRevenue>, DbErr> {
    let database = crate::Database::new("default").await?;

    let sql = r#"
            SELECT  CONCAT(name,'(',code,')','-',brand,'-',client) subsidiary_abstract
                ,client debit_client
                ,client credit_client
                ,"租赁" debit_receipt_type
                ,"租赁" credit_receipt_type
                ,"9%" credit_tax_rate
                ,"其他" credit_business_type
                ,SUM( each_term_rent ) monthly_rent
                ,0.09 tax_rate
            FROM investment_real_estates
            WHERE status='已签约'
            GROUP BY subsidiary_abstract , client
            ;
        "#;

    database.find_by_sql(sql).await
}
