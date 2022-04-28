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
    #[serde(rename(serialize = "C__taxRate"))]
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
            WITH irelh_latest AS
            (
                SELECT  investment_real_estates_id ire_id
                    ,MAX( serial_no ) max_serial_no
                FROM investment_real_estates_lease_history
                GROUP BY  ire_id
            ) , ireb AS
            (
                SELECT  ire.id
                    ,ire.name
                    ,ire.location
                    ,ire.code
                    ,ire.floor
                    ,ire.leasable_area
                    ,ire.status
                    ,ire.attachment
                    ,ire.remark
                    ,irelh.client
                    ,irelh.brand
                    ,irelh.business_type
                    ,irelh.status signing_status
                    ,irelh.lease_commencement_date
                    ,irelh.lease_end_date
                    ,irelh.canceling_date
                    ,irelh.term
                    ,irelh.total_rent
                    ,irelh.each_term_rent
                FROM investment_real_estates ire
                LEFT JOIN irelh_latest
                ON irelh_latest.ire_id = ire.id
                LEFT JOIN investment_real_estates_lease_history irelh
                ON irelh.investment_real_estates_id = ire.id AND irelh.serial_no = irelh_latest.max_serial_no
            )
            SELECT  CONCAT(name,'(',code,')','-',brand,'-',client) subsidiary_abstract
                ,client debit_client
                ,client credit_client
                ,"租赁" debit_receipt_type
                ,"租赁" credit_receipt_type
                ,"9%" credit_tax_rate
                ,"其他" credit_business_type
                ,SUM( each_term_rent ) monthly_rent
                ,0.09 tax_rate
            FROM ireb
            WHERE status='已租赁'
            AND signing_status='已签约'
            GROUP BY  subsidiary_abstract
                    ,client
            ;
        "#;

    database.find_by_sql(sql).await
}
