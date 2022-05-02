use sea_orm::{entity::prelude::*, sea_query::Expr, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::entity::{
    canyon_daily_sales_append as DailySalesAppend, canyon_offline_ticket_bill as OfflineTicketBill,
    canyon_online_ticket_bill as OnlineTicketBill,
};

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySales {
    source: String,
    id: Option<u32>,
    date: Date,
    channel: String,
    operator: String,
    payment_method: String,
    client: String,
    ticket_type: String,
    ticket_price: Decimal,
    sum_ticket_num: Decimal,
    sum_ticket_amount: Decimal,
    remark: Option<String>,
}

pub async fn daily_sales(
    datetime_from: DateTime,
    datetime_end: DateTime,
    where_condition: &str,
) -> Result<Vec<DailySales>, DbErr> {
    let database = crate::Database::new("default").await?;

    let sql = format!("
            WITH offt AS
            (
                SELECT  *
                FROM canyon_offline_ticket_bill
                WHERE is_deleted=0
            ), ont AS
            (
                SELECT  *
                FROM canyon_online_ticket_bill
                WHERE is_deleted=0
            ), tb AS
            (
                SELECT  trade_type
                    ,trade_time
                    ,'窗口' channel
                    ,operator
                    ,payment_method
                    ,client
                    ,ticket_type ticket_type_raw
                    ,ticket_price
                    ,ticket_num
                    ,ticket_amount
                FROM offt
                UNION ALL
                SELECT  'sale' trade_type
                    ,ont.check_in_datetime trade_time
                    ,'线上' channel
                    ,ont.client operator
                    ,IFNULL(tc.payment_type,'') payment_method
                    ,ont.client
                    ,ont.ticket_type ticket_type_raw
                    ,ont.ticket_price
                    ,ont.ticket_num
                    ,ont.ticket_amount
                FROM ont
                LEFT JOIN canyon_ticket_client tc
                ON tc.name=ont.client
            ) , ds AS
            (
                SELECT  'system' source
                    ,NULL id
                    ,DATE( trade_time ) date
                    ,channel
                    ,operator
                    ,payment_method
                    ,client
                    ,IFNULL(mdv.to_value,tb.ticket_type_raw) ticket_type
                    ,ticket_price
                    ,SUM( CASE trade_type WHEN 'sale' THEN ticket_num WHEN 'refund' THEN - ticket_num END ) sum_ticket_num
                    ,SUM( CASE trade_type WHEN 'sale' THEN ticket_amount WHEN 'refund' THEN - ticket_amount END ) sum_ticket_amount
                    ,NULL remark
                FROM tb
                LEFT JOIN mapper_domain_value mdv
                ON mdv.domain='CanyonTicket' AND mdv.type='ticket_type' AND tb.ticket_type_raw=mdv.from_value
                GROUP BY  date
                        ,channel
                        ,operator
                        ,payment_method
                        ,client
                        ,ticket_type
                        ,ticket_price
                UNION ALL
                SELECT  'append' source
                    ,id
                    ,date
                    ,channel
                    ,operator
                    ,payment_method
                    ,client
                    ,ticket_type
                    ,ticket_price
                    ,ticket_num sum_ticket_num
                    ,ticket_amount sum_ticket_amount
                    ,remark
                FROM canyon_daily_sales_append
                WHERE is_append = 1
            )
            SELECT  *
            FROM ds
            WHERE date BETWEEN DATE(?) AND DATE(?) {}
            ;
        ",where_condition);

    database
        .find_by_sql_and_values(&sql, vec![datetime_from.into(), datetime_end.into()])
        .await
}

pub async fn daily_sales_appends(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<Vec<DailySalesAppend::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    DailySalesAppend::Entity::find()
        .filter(DailySalesAppend::Column::Date.between(datetime_from, datetime_end))
        .filter(DailySalesAppend::Column::IsAppend.eq(true))
        .all(&txn)
        .await
}

pub async fn delete_ticket_bill(
    datetime_from: DateTime,
    datetime_end: DateTime,
) -> Result<(), DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    OfflineTicketBill::Entity::update_many()
        .col_expr(
            OfflineTicketBill::Column::IsDeleted,
            Expr::value(Value::TinyInt(Some(1))),
        )
        .filter(OfflineTicketBill::Column::TradeTime.between(datetime_from, datetime_end))
        .filter(OfflineTicketBill::Column::IsDeleted.eq(0))
        .exec(&txn)
        .await?;

    OnlineTicketBill::Entity::update_many()
        .col_expr(
            OnlineTicketBill::Column::IsDeleted,
            Expr::value(Value::TinyInt(Some(1))),
        )
        .filter(OnlineTicketBill::Column::CheckInDatetime.between(datetime_from, datetime_end))
        .filter(OnlineTicketBill::Column::IsDeleted.eq(0))
        .exec(&txn)
        .await?;

    txn.commit().await
}
