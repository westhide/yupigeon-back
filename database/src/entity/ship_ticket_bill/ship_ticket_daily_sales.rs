use sea_orm::{entity::prelude::*, ConnectionTrait, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
pub struct DailySales {
    date: Date,
    times: i64,
    sales: Option<Decimal>,
}

pub async fn daily_sales(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailySales>, DbErr> {
    let txn = crate::get_txn("laiu8").await?;
    DailySales::find_by_statement(Statement::from_sql_and_values(
        txn.get_database_backend(),
        r#"
        WITH td AS (
            SELECT
            link_order_id 订单号,
            channel_name 渠道名称,
            serial_no 变更序列,
            ticket_status 票状态,
            line_name 航线,
            departure_datetime 航班时间,
            ship_name 船舶,
            ticket_type_name 票型,
            ticket_price 票价,
            cabin_name 舱位,
            seat_memo 座位号,
            passenger_name 乘船人,
            passenger_id_no 证件号,
            user_name 出票人,
            payment_method 支付方式,
            pay_amount 支付金额,
            payment_time 支付时间,
            pay_id 支付ID,
            ticket_id 票ID,
            ticket_no 票号
            FROM ticket_bill
            -- 可自行修改日期查询期间
            WHERE departure_datetime >=? AND departure_datetime <= ?
            )

        SELECT date(航班时间) date,count(1) times,sum(票价) sales FROM td
        WHERE 票状态 IN ('一检','二检','出票成功')
        GROUP BY date
        ORDER BY date
        ;
        "#,
        vec![datetime_from.into(), datetime_end.into()],
    ))
    .all(&txn)
    .await
}
