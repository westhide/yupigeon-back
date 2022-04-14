use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientSales {
    user_type: String,
    channel_name: String,
    client_type: String,
    client_name: String,
    sum_ticket_price: Decimal,
    sum_pay_amount: Option<Decimal>,
    sum_past_pay_amount: Option<Decimal>,
    sum_now_pay_amount: Option<Decimal>,
    // sum_future_pay_amount: Option<Decimal>,
    sum_refund_amount: Option<Decimal>,
    sum_past_refund: Option<Decimal>,
    sum_now_refund: Option<Decimal>,
    sum_future_refund: Option<Decimal>,
    sum_fee: Option<Decimal>,
    sum_past_fee: Option<Decimal>,
    sum_now_fee: Option<Decimal>,
    sum_future_fee: Option<Decimal>,
}

pub async fn client_sales(
    datetime_from: DateTime,
    datetime_end: DateTime,
    where_condition: &str,
) -> Result<Vec<ClientSales>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let set_from = format!("SET @from_time='{}';", datetime_from);
    database.execute_sql(&set_from).await?;

    let set_end = format!("SET @end_time='{}';", datetime_end);
    database.execute_sql(&set_end).await?;

    let sql = format!("
        WITH ts AS
        (
            SELECT  link_id
            FROM ticket_bill
            WHERE departure_datetime >= @from_time
            AND departure_datetime <= @end_time
            AND ticket_status IN ('一检', '二检', '出票成功')
        ) , tr AS
        (
            SELECT  trb.tb_id
                ,SUM(trb.refund_amount) sum_refund
                ,SUM(IF(refund_finish_time<@from_time,trb.refund_amount,0)) past_refund
                ,SUM(IF(refund_finish_time BETWEEN @from_time AND @end_time,trb.refund_amount,0)) now_refund
                ,SUM(IF(refund_finish_time>@end_time,trb.refund_amount,0)) future_refund
                ,SUM(trb.fee) sum_fee
                ,SUM(IF(refund_finish_time<@from_time,trb.fee,0)) past_fee
                ,SUM(IF(refund_finish_time BETWEEN @from_time AND @end_time,trb.fee,0)) now_fee
                ,SUM(IF(refund_finish_time>@end_time,trb.fee,0)) future_fee
            FROM ticket_bill tb
            JOIN ts
            ON tb.link_id=ts.link_id
            JOIN ship_ticket_refund_bill trb
            ON trb.tb_id=tb.id
            GROUP BY  trb.tb_id
        )
        SELECT  tb.user_type
            ,tb.channel_name
            ,(CASE u8_user_type
                    WHEN 'OTA' THEN 'OTA'
                    WHEN 'VIP' THEN 'VIP'
                    ELSE '散客' END
                ) client_type
            ,(CASE u8_user_type
                    WHEN 'OTA' THEN IFNULL(u8_vip_pact,IFNULL(u8_nickname,u8_user_name) )
                    WHEN 'VIP' THEN IFNULL(u8_vip_pact,IFNULL(u8_nickname,u8_user_name) )
                    ELSE '散客' END
                ) client_name
            ,SUM(IFNULL(tb.pay_amount,0)-IFNULL(tr.sum_refund,0)-IFNULL(tr.sum_fee,0)) sum_ticket_price
            ,SUM(tb.pay_amount) sum_pay_amount
            ,SUM(IF(payment_time<@from_time,tb.pay_amount,0)) sum_past_pay_amount
            ,SUM(IF(payment_time BETWEEN @from_time AND @end_time,tb.pay_amount,0)) sum_now_pay_amount
            ,SUM(IF(payment_time>@end_time,tb.pay_amount,0)) sum_future_pay_amount
            ,SUM(tr.sum_refund) sum_refund_amount
            ,SUM(tr.past_refund) sum_past_refund
            ,SUM(tr.now_refund) sum_now_refund
            ,SUM(tr.future_refund) sum_future_refund
            ,SUM(tr.sum_fee) sum_fee
            ,SUM(tr.past_fee) sum_past_fee
            ,SUM(tr.now_fee) sum_now_fee
            ,SUM(tr.future_fee) sum_future_fee
        FROM ticket_bill tb
        JOIN ts
        ON tb.link_id=ts.link_id
        LEFT JOIN tr
        ON tb.id=tr.tb_id
        {}
        GROUP BY  user_type
                ,channel_name
                ,client_type
                ,client_name
        ORDER BY user_type
                ,channel_name
                ,client_type
                ,client_name
        ;
    ",where_condition);

    database.find_by_sql(&sql).await
}
