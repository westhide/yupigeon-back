use sea_orm::entity::prelude::*;

use crate::entity::ship_ticket_refund_bill::Model;

pub async fn refund_bill(
    datetime_from: DateTime,
    datetime_end: DateTime,
    where_condition: &str,
) -> Result<Vec<Model>, DbErr> {
    let database = crate::Database::new("laiu8").await?;

    let sql = format!(
        "
            SELECT  id
                ,tb_id
                ,ticket_id
                ,link_ticket_id
                ,ticket_no
                ,refund_type
                ,channel_name
                ,user_type
                ,user_name
                ,refund_method
                ,refund_finish_time
                ,refund_id
                ,refund_amount
                ,fee
                ,order_id
            FROM ship_ticket_refund_bill
            WHERE refund_finish_time BETWEEN '{}' AND '{}'
            {}
            ;
        ",
        datetime_from, datetime_end, where_condition
    );

    database.find_by_sql(&sql).await
}
