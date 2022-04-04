use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct DailyReceipt {
    date: Date,
    pay_method: Option<String>,
    sum_pay_amount: Option<Decimal>,
    sum_refund_amount: Option<Decimal>,
}

pub async fn daily_receipt(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailyReceipt>, DbErr> {
    let txn = crate::get_txn("laiu8").await?;
    DailyReceipt::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
            WITH offr AS
            (
                SELECT  DATE( refund_finish_time ) date
                    ,IF(user_type='线下',IF(channel_name='航线','航线','窗口收款'),refund_method) pay_method
                    ,SUM(refund_amount) sum_refund_amount
                FROM ship_ticket_refund_bill
                WHERE refund_finish_time >=?
                AND refund_finish_time <= ?
                GROUP BY  date
                        ,pay_method
            ) , offp AS
            (
                SELECT  DATE( payment_time ) date
                    ,IFNULL(u8_payment_method,IF(user_type='线下',IF(channel_name='航线','航线','窗口收款'),payment_method)) pay_method
                    ,SUM( pay_amount ) sum_pay_amount
                FROM ticket_bill
                WHERE pay_amount IS NOT NULL
                AND payment_time >=?
                AND payment_time <= ?
                GROUP BY  date
                        ,pay_method
            ) , off AS
            (
                SELECT  offp.date
                    ,offp.pay_method
                    ,offp.sum_pay_amount
                    ,offr.sum_refund_amount
                FROM offp
                LEFT JOIN offr
                ON offp.date=offr.date AND offp.pay_method=offr.pay_method UNION
                SELECT  offr.date
                    ,offr.pay_method
                    ,offp.sum_pay_amount
                    ,offr.sum_refund_amount
                FROM offp
                RIGHT JOIN offr
                ON offp.date=offr.date AND offp.pay_method=offr.pay_method
            )
            SELECT  off.date
                ,off.pay_method
                ,off.sum_pay_amount
                ,off.sum_refund_amount
            FROM off
            WHERE off.sum_pay_amount>0 OR off.sum_refund_amount>0
            ORDER BY off.date DESC, off.pay_method
        ;
        "#,
        vec![
            datetime_from.into(),
            datetime_end.into(),
            datetime_from.into(),
            datetime_end.into(),
        ],
    ))
    .all(&txn)
    .await
}
