use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct DailyReceipt {
    date: Date,
    sum_settlement_total: Option<Decimal>,
    sum_staff_canteen_pay_amount: Option<Decimal>,
    sum_pay_amount: Option<Decimal>,
    sum_ship_ticket_pay_amount: Option<Decimal>,
    sum_product_ticket_pay_amount: Option<Decimal>,
    sum_product_ticket_coupon_fee: Option<Decimal>,
    sum_diff: Option<Decimal>,
}

pub async fn daily_receipt(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailyReceipt>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let sql = r#"
        WITH u8pb AS
        (
            SELECT  u8p.callback_trade_no pay_id
                ,"网售产品" ticket_type
                ,u8pd.price pay_amount
                ,u8pd.coupon_fee
            FROM u8_order_common_expand u8pd
            LEFT JOIN u8_order u8o
            ON u8o.id = u8pd.order_id
            LEFT JOIN u8_user u8u
            ON u8u.id = u8pd.user_id
            LEFT JOIN u8_tickets_orgsign u8org
            ON u8org.id = u8pd.ota_id
            LEFT JOIN u8_order_payment u8p
            ON u8p.trade_no = u8o.trade_no AND FIND_IN_SET(u8o.id, u8p.orders)
            WHERE u8pd.create_time BETWEEN UNIX_TIMESTAMP(?) AND UNIX_TIMESTAMP(?)
            AND IFNULL(u8p.callback_trade_no, '')!=''
            UNION ALL
            SELECT  pay_id
                ,"船票" ticket_type
                ,pay_amount
                ,0
            FROM ticket_bill tb
            WHERE pay_id IS NOT NULL
        ) , pb AS
        (
            SELECT  pay_id
                ,SUM(pay_amount) amount
                ,SUM(IF(u8pb.ticket_type='船票',u8pb.pay_amount,0)) ship_ticket_pay_amount
                ,SUM(IF(u8pb.ticket_type='网售产品',u8pb.pay_amount,0)) product_ticket_pay_amount
                ,SUM(IF(u8pb.ticket_type='网售产品',u8pb.coupon_fee,0)) product_ticket_coupon_fee
            FROM u8pb
            GROUP BY  pay_id
        ) , tb AS
        (
            SELECT  link_order_id
                ,pay_id
                ,channel_name
            FROM ticket_bill
            GROUP BY  pay_id
        ) , pc AS
        (
            SELECT  tp.trade_time
                ,tp.transaction_id
                ,pb.pay_id
                ,pb.amount
                ,pb.ship_ticket_pay_amount
                ,pb.product_ticket_pay_amount
                ,pb.product_ticket_coupon_fee
                ,tp.settlement_total
                ,IF(goods_name='内部订餐',tp.settlement_total,0) staff_canteen_pay_amount
                ,pb.amount + IF(goods_name='内部订餐',tp.settlement_total,0) - tp.settlement_total - pb.product_ticket_coupon_fee diff
                ,tp.goods_name
                ,tb.link_order_id
                ,tb.channel_name
            FROM tenpay_bill tp
            LEFT JOIN pb
            ON pb.pay_id=tp.transaction_id
            LEFT JOIN tb
            ON tb.pay_id=pb.pay_id
            WHERE tp.trade_state='SUCCESS'
            AND tp.trade_time BETWEEN ? AND ?
        )
        SELECT  date(pc.trade_time) date
            ,SUM(pc.settlement_total) sum_settlement_total
            ,SUM(pc.staff_canteen_pay_amount) sum_staff_canteen_pay_amount
            ,SUM(pc.amount) sum_pay_amount
        , SUM(pc.ship_ticket_pay_amount) sum_ship_ticket_pay_amount , SUM(pc.product_ticket_pay_amount) sum_product_ticket_pay_amount , SUM(pc.product_ticket_coupon_fee) sum_product_ticket_coupon_fee , SUM(pc.diff) sum_diff
        FROM pc
        GROUP BY date
        ORDER BY date DESC
        ;
    "#;

    database
        .find_by_sql_and_values(
            sql,
            vec![
                datetime_from.into(),
                datetime_end.into(),
                datetime_from.into(),
                datetime_end.into(),
            ],
        )
        .await
}
