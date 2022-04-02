use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
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
) -> Result<Vec<ClientSales>, DbErr> {
    let txn = crate::get_txn("laiu8").await?;
    ClientSales::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
            WITH ts AS
            (
                SELECT  link_id
                    ,user_type
                    ,channel_name
                    ,u8_user_type
                    ,u8_vip_pact
                    ,u8_nickname
                    ,u8_user_name
                    ,ticket_price
                FROM ticket_bill
                WHERE departure_datetime >= ?
                AND departure_datetime <= ?
                AND ticket_status IN ('一检', '二检', '出票成功')
            ) , tr AS
            (
                SELECT  tb.link_id
                    ,SUM(IF(refund_finish_time<?,trb.refund_amount,0)) past_refund
                    ,SUM(IF(refund_finish_time BETWEEN ? AND ?,trb.refund_amount,0)) now_refund
                    ,SUM(IF(refund_finish_time>?,trb.refund_amount,0)) future_refund
                    ,SUM(IF(refund_finish_time<?,trb.fee,0)) past_fee
                    ,SUM(IF(refund_finish_time BETWEEN ? AND ?,trb.fee,0)) now_fee
                    ,SUM(IF(refund_finish_time>?,trb.fee,0)) future_fee
                FROM ticket_bill tb
                JOIN ts
                ON tb.link_id=ts.link_id
                JOIN ship_ticket_refund_bill trb
                ON trb.tb_id=tb.id
                GROUP BY  tb.link_id
            ) , tp AS
            (
                SELECT  tb.link_id
                    ,SUM(tb.pay_amount) pay_amount
                    ,SUM(IF(payment_time<?,tb.pay_amount,0)) past_pay_amount
                    ,SUM(IF(payment_time BETWEEN ? AND ?,tb.pay_amount,0)) now_pay_amount
                    ,SUM(IF(payment_time>?,tb.pay_amount,0)) future_pay_amount
                    ,SUM(tb.refund_amount) refund_amount
                    ,SUM(tb.fee) fee
                FROM ticket_bill tb
                JOIN ts
                ON tb.link_id=ts.link_id
                GROUP BY  tb.link_id
            )
            SELECT  user_type
                ,channel_name
                ,(CASE u8_user_type WHEN "OTA" THEN "OTA" WHEN "VIP" THEN "VIP" ELSE "散客" END ) client_type
                ,(CASE u8_user_type WHEN "OTA" THEN IFNULL(u8_vip_pact,IFNULL(u8_nickname,u8_user_name)) WHEN "VIP" THEN IFNULL(u8_vip_pact,IFNULL(u8_nickname,u8_user_name)) ELSE "散客" END ) client_name
                ,SUM(ticket_price) sum_ticket_price
                ,SUM(tp.pay_amount) sum_pay_amount
                ,SUM(tp.past_pay_amount) sum_past_pay_amount
                ,SUM(tp.now_pay_amount) sum_now_pay_amount
                ,SUM(tp.future_pay_amount) sum_future_pay_amount
                ,SUM(tp.refund_amount) sum_refund_amount
                ,SUM(tr.past_refund) sum_past_refund
                ,SUM(tr.now_refund) sum_now_refund
                ,SUM(tr.future_refund) sum_future_refund
                ,SUM(tp.fee) sum_fee
                ,SUM(tr.past_fee) sum_past_fee
                ,SUM(tr.now_fee) sum_now_fee
                ,SUM(tr.future_fee) sum_future_fee
            FROM ts
            LEFT JOIN tp
            ON tp.link_id=ts.link_id
            LEFT JOIN tr
            ON tr.link_id=ts.link_id
            GROUP BY  user_type
                    ,channel_name
                    ,client_type
                    ,client_name
            ORDER BY user_type
                    ,channel_name
                    ,client_type
                    ,client_name
            ;
        "#,
        vec![
            datetime_from.into()
            ,datetime_end.into()
            ,datetime_from.into()
            ,datetime_from.into()
            ,datetime_end.into()
            ,datetime_end.into()
            ,datetime_from.into()
            ,datetime_from.into()
            ,datetime_end.into()
            ,datetime_end.into()
            ,datetime_from.into()
            ,datetime_from.into()
            ,datetime_end.into()
            ,datetime_end.into()
            ],
    ))
    .all(&txn)
    .await
}
