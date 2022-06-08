use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    id: u32,
    name: String,
    location: Option<String>,
    code: String,
    floor: Option<String>,
    leasable_area: Decimal,
    status: Option<String>,
    attachment: Option<String>,
    remark: Option<String>,
    client: Option<String>,
    brand: Option<String>,
    business_type: Option<String>,
    signing_status: Option<String>,
    lease_commencement_date: Option<Date>,
    lease_end_date: Option<Date>,
    canceling_date: Option<Date>,
    term: Option<u32>,
    total_rent: Option<Decimal>,
    each_term_rent: Option<Decimal>,
}

pub async fn bill() -> Result<Vec<Bill>, DbErr> {
    let database = crate::mysql::Database::new("default").await?;

    let sql = r#"
            WITH irelh_latest AS
            (
                SELECT  investment_real_estates_id ire_id
                    ,MAX( serial_no ) max_serial_no
                FROM investment_real_estates_lease_history
                GROUP BY  ire_id
            )
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
            ORDER BY ire.code
            ;
        "#;

    database.find_by_sql(sql).await
}
