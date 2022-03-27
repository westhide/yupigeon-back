use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, ExecResult, Statement};

use crate::get_db;

pub async fn execute() -> Result<ExecResult, DbErr> {
    update_link_order_id().await?;
    update_order_info().await
}

async fn update_link_order_id() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                UPDATE ticket_bill tb
                LEFT JOIN bt_ticket t ON tb.ticket_id = t.id
                SET tb.link_order_id = t.order_id;
            "#
            .into(),
        ))
        .await
}

async fn update_order_info() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
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
            "#
            .into(),
        ))
        .await
}
