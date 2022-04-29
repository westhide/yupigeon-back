use sea_orm::{entity::prelude::*, FromQueryResult, InsertResult};
use serde::{Deserialize, Serialize};

pub async fn insert_many<E, A>(models: Vec<E::Model>) -> Result<InsertResult<A>, DbErr>
where
    E: EntityTrait,
    A: ActiveModelTrait<Entity = E> + From<E::Model>,
{
    let txn = crate::Database::new("default").await?.txn;

    let records: Vec<A> = models.iter().map(|model| model.to_owned().into()).collect();

    let result = E::insert_many(records).exec(&txn).await?;

    txn.commit().await?;
    Ok(result)
}

#[derive(Debug, FromQueryResult, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySales {
    date: Date,
    channel: String,
    operator: String,
    payment_method: String,
    client: String,
    ticket_type: String,
    ticket_price: Decimal,
    sum_ticket_num: Decimal,
    sum_ticket_amount: Decimal,
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
                SELECT  DISTINCT *
                FROM canyon_offline_ticket_bill
            ), ont AS
            (
                SELECT  DISTINCT *
                FROM canyon_online_ticket_bill
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
                UNION ALL(
                SELECT  'sale' trade_type
                    ,ont.check_in_datetime trade_time
                    ,'线上' channel
                    ,ont.client operator
                    ,( CASE tc.type WHEN 'online' THEN '挂账' WHEN 'travelAgency' THEN '返利' END ) payment_method
                    ,ont.client
                    ,ont.ticket_type ticket_type_raw
                    ,ont.ticket_price
                    ,ont.ticket_num
                    ,ont.ticket_amount
                FROM ont
                LEFT JOIN canyon_ticket_client tc
                ON tc.name=ont.client )
            )
            SELECT  DATE( trade_time ) date
                ,channel
                ,operator
                ,payment_method
                ,client
                ,IFNULL(mdv.to_value,tb.ticket_type_raw) ticket_type
                ,ticket_price
                ,SUM( CASE trade_type WHEN 'sale' THEN ticket_num WHEN 'refund' THEN - ticket_num END ) sum_ticket_num
                ,SUM( CASE trade_type WHEN 'sale' THEN ticket_amount WHEN 'refund' THEN - ticket_amount END ) sum_ticket_amount
            FROM tb
            LEFT JOIN mapper_domain_value mdv
            ON mdv.domain='CanyonTicket' AND mdv.type='ticket_type' AND tb.ticket_type_raw=mdv.from_value
            WHERE trade_time BETWEEN ? AND ?
            {}
            GROUP BY  date
                    ,channel
                    ,operator
                    ,payment_method
                    ,client
                    ,ticket_type
                    ,ticket_price
            ;
        ",where_condition);

    database
        .find_by_sql_and_values(&sql, vec![datetime_from.into(), datetime_end.into()])
        .await
}
