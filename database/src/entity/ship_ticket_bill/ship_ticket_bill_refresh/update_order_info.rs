use sea_orm::{DatabaseTransaction, DbErr, ExecResult};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    update_link_id(txn).await?;
    update_order_info(txn).await
}

async fn update_link_id(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    crate::execute_sql(
        txn,
        r#"
            WITH lid AS (
                SELECT
                    id
                    ,link_ticket_id
                FROM ticket_bill
                WHERE serial_no =1
            )
            UPDATE ticket_bill tb
            LEFT JOIN lid ON tb.link_ticket_id=lid.link_ticket_id
            LEFT JOIN bt_ticket t ON tb.ticket_id=t.id
            SET tb.link_order_id = t.order_id
                ,tb.link_id = lid.id;
            "#,
    )
    .await
}

async fn update_order_info(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    crate::execute_sql(
        txn,
        r#"
                UPDATE ticket_bill tb
                LEFT JOIN bt_ticket t ON tb.ticket_id = t.id
                LEFT JOIN bt_order o ON t.order_id = o.id
                LEFT JOIN bt_channel c ON o.from_channel_id = c.id
                LEFT JOIN sys_user u ON o.create_user_code = u.code

                SET tb.order_id = o.id
                    , tb.channel_id = c.id
                    , tb.channel_name = c.name
                    , tb.user_id = u.code	-- ! sys_user.id非主键,存在null值;用sys_user.code代替主键
                    , tb.user_type = c.category_id
                    , tb.user_name = u.user_name
                    , tb.payment_time = o.payment_time
                    , tb.payment_method = o.payment_method
                WHERE tb.serial_no = 1;
            "#,
    )
    .await
}
