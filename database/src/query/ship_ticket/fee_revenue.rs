use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRevenue {
    client: String,
    receipt_type: String,
    conductor: String,
    fee_type: String,
    fee_amount: Decimal,
}

pub async fn fee_revenue(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<FeeRevenue>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let sql = r#"
            SELECT  '散客' client
                ,(CASE
                    WHEN trb.user_type='线下' THEN '手续费-散客窗口'
                    ELSE '船票款-网售' END
                ) receipt_type
                ,(CASE
                    WHEN trb.user_type='线下' OR trb.channel_name IN ('驻岛订票','内部订票') THEN trb.user_name
                    ELSE '网售-来游吧' END
                ) conductor
                ,(CASE
                    WHEN tb.line_name IN ('北海-涠洲','涠洲-北海') THEN '手续费（北涠）'
                    ELSE '手续费（北琼）' END
                )fee_type
                ,SUM(trb.fee) fee_amount
            FROM ship_ticket_refund_bill trb
            JOIN ticket_bill tb
            ON trb.tb_id=tb.id
            WHERE trb.refund_finish_time BETWEEN ? AND ?
            GROUP BY  client
                    ,receipt_type
                    ,conductor
                    ,fee_type
            HAVING fee_amount!=0
            ;
        "#;

    database
        .find_by_sql_and_values(sql, [datetime_from.into(), datetime_end.into()])
        .await
}
