use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoucherRevenue {
    client: String,
    receipt_type: String,
    conductor: String,
    ship_line: String,
    ship: String,
    sum_ticket_price: Decimal,
}

pub async fn voucher_revenue(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<VoucherRevenue>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let set_from = format!("SET @from_time='{}';", datetime_from);
    database.execute_sql(&set_from).await?;

    let set_end = format!("SET @end_time='{}';", datetime_end);
    database.execute_sql(&set_end).await?;

    database.find_by_sql("
        WITH ts AS
        (
            SELECT  link_id
                ,line_name ship_line
                ,ship_name ship
            FROM ticket_bill
            WHERE departure_datetime >= @from_time
            AND departure_datetime <= @end_time
            AND ticket_status IN ('一检', '二检', '出票成功')
        ) , tr AS
        (
            SELECT  trb.tb_id
                ,SUM(trb.refund_amount) sum_refund
                ,SUM(trb.fee) sum_fee
            FROM ticket_bill tb
            JOIN ts
            ON tb.link_id=ts.link_id
            JOIN ship_ticket_refund_bill trb
            ON trb.tb_id=tb.id
            GROUP BY  trb.tb_id
        )
        SELECT  '散客' client
            ,(CASE
                WHEN user_type='线下' AND channel_name='航线' THEN '升舱票款'
                WHEN user_type='线下' AND channel_name!='航线' THEN '船票款-散客窗口'
                ELSE '船票款-网售' END
            ) receipt_type
            ,(CASE
                WHEN user_type='线下' OR channel_name IN ('驻岛订票','内部订票') THEN user_name
                ELSE '网售-来游吧' END
            ) conductor
            ,ts.ship_line
            ,ts.ship
            ,SUM(IFNULL(tb.pay_amount,0)-IFNULL(tr.sum_refund,0)-IFNULL(tr.sum_fee,0)) sum_ticket_price
        -- , SUM(tb.pay_amount) sum_pay_amount
        -- , SUM(tr.sum_refund) sum_refund_amount
        -- , SUM(tr.sum_fee) sum_fee
        FROM ticket_bill tb
        JOIN ts
        ON tb.link_id=ts.link_id
        LEFT JOIN tr
        ON tb.id=tr.tb_id
        GROUP BY  client
                ,receipt_type
                ,conductor
                ,ship_line
                ,ship
        HAVING sum_ticket_price!=0
        ;
    ").await
}
