use sea_orm::{entity::prelude::*, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct ConductorDailyReceipt {
    date: Date,
    user_name: String,
    receipt: Option<Decimal>,
    sum_pay_amount: Option<Decimal>,
    sum_fee: Option<Decimal>,
    just_refund_fee: Option<Decimal>,
    just_change_fee: Option<Decimal>,
    sum_refund_amount: Option<Decimal>,
    just_refund_amount: Option<Decimal>,
    just_change_amount: Option<Decimal>,
    just_compensation_amount: Option<Decimal>,
    just_hcbb_amount: Option<Decimal>,
}

pub async fn conductor_daily_receipt(
    datetime_from: DateTime,
    datetime_end: DateTime,
    where_condition: &str,
) -> Result<Vec<ConductorDailyReceipt>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let set_from = format!("SET @from_time='{}';", datetime_from);
    database.execute_sql(&set_from).await?;

    let set_end = format!("SET @end_time='{}';", datetime_end);
    database.execute_sql(&set_end).await?;

    let sql = format!("
            WITH offr AS (
                SELECT
                    DATE( refund_finish_time ) date,
                    user_name,
                    SUM(IF(refund_amount>0,refund_amount+fee,0)) sum_refund_amount,
                    SUM(IF(refund_type='已退款',IF(refund_amount>0,refund_amount+fee,0),0)) just_refund_amount,
                    SUM(IF(refund_type='改签废票',IF(refund_amount>0,refund_amount+fee,0),0)) just_change_amount,
                    SUM(IF(refund_type='已补差',IF(refund_amount>0,refund_amount+fee,0),0)) just_compensation_amount,
                    SUM(IF(refund_type='已换船',IF(refund_amount>0,refund_amount+fee,0),0)) just_hcbb_amount,
                    SUM(IF(refund_amount>0,fee,0)) sum_repeat_fee,
                    SUM(fee) sum_fee,
                    SUM(IF(refund_type='已退款',fee,0)) just_refund_fee,
                    SUM(IF(refund_type='改签废票',fee,0)) just_change_fee
                FROM ship_ticket_refund_bill
                WHERE
                        user_type='线下'
                        AND refund_finish_time >=@from_time AND refund_finish_time <= @end_time
                GROUP BY date,user_name
            )
            ,offp AS (
                SELECT
                        DATE( create_time ) date,
                        user_name,
                        SUM( pay_amount ) sum_pay_amount
                FROM ticket_bill
                WHERE
                        user_type='线下'
                        AND pay_amount IS NOT NULL
                        AND create_time >=@from_time AND create_time <= @end_time
                GROUP BY date,user_name
            )
            ,off AS (
            SELECT
                offp.date,
                offp.user_name,
                offp.sum_pay_amount,
                offr.sum_fee,
                offr.just_refund_fee,
                offr.just_change_fee,
                offr.sum_refund_amount,
                offr.just_refund_amount,
                offr.just_change_amount,
                offr.just_compensation_amount,
                offr.just_hcbb_amount,
                offr.sum_repeat_fee
            FROM offp LEFT JOIN offr ON offp.date=offr.date AND offp.user_name=offr.user_name
            UNION
            SELECT
                offr.date,
                offr.user_name,
                offp.sum_pay_amount,
                offr.sum_fee,
                offr.just_refund_fee,
                offr.just_change_fee,
                offr.sum_refund_amount,
                offr.just_refund_amount,
                offr.just_change_amount,
                offr.just_compensation_amount,
                offr.just_hcbb_amount,
                offr.sum_repeat_fee
            FROM offp RIGHT JOIN offr ON offp.date=offr.date AND offp.user_name=offr.user_name
            )
            SELECT
                off.date,
                off.user_name,
                IFNULL(off.sum_pay_amount,0) + IFNULL(off.sum_repeat_fee,0) receipt,
                off.sum_pay_amount,
                IF(off.sum_fee>0,off.sum_fee,NULL) sum_fee,
                IF(off.just_refund_fee>0,off.just_refund_fee,NULL) just_refund_fee,
                IF(off.just_change_fee>0,off.just_change_fee,NULL) just_change_fee,
                IF(off.sum_refund_amount>0,off.sum_refund_amount,NULL) sum_refund_amount,
                IF(off.just_refund_amount>0,off.just_refund_amount,NULL) just_refund_amount,
                IF(off.just_change_amount>0,off.just_change_amount,NULL) just_change_amount,
                IF(off.just_compensation_amount>0,off.just_compensation_amount,NULL) just_compensation_amount,
                IF(off.just_hcbb_amount>0,off.just_hcbb_amount,NULL) just_hcbb_amount
            FROM
                off
            WHERE (off.sum_pay_amount>0 OR off.sum_fee>0 OR off.sum_refund_amount>0)
            {}
            ORDER BY off.date DESC,off.user_name
            ;
        ",where_condition);

    database.find_by_sql(&sql).await
}
