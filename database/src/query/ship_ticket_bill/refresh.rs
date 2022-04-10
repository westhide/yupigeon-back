use sea_orm::entity::prelude::*;

pub async fn refresh() -> Result<(), DbErr> {
    let database = crate::Database::new("laiu8").await?;
    use super::refresh_sql::{bt_ticket_info, laiu8_info, refund_info};
    database
        .execute_multi_sql(vec![
            bt_ticket_info::DROP_TABLE,
            bt_ticket_info::CREATE_TABLE,
            bt_ticket_info::INSERT_KEY_RECORD,
            bt_ticket_info::UPDATE_LINK_ID,
            bt_ticket_info::UPDATE_ORDER_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_PAY_AMOUNT,
            bt_ticket_info::UPDATE_DEPARTURE_INFO,
            bt_ticket_info::UPDATE_RELATED_INFO,
            bt_ticket_info::UPDATE_U8_TICKET_KEY,
            laiu8_info::DROP_TEMP_TABLE,
            laiu8_info::CREATE_TEMP_TABLE,
            laiu8_info::CREATE_INDEX_ID,
            laiu8_info::CREATE_INDEX_OLD_ID,
            laiu8_info::UPDATE_LAIU8_INFO,
            laiu8_info::UPDATE_EDGE_CASE,
            laiu8_info::UPDATE_RELATED_INFO,
            laiu8_info::UPDATE_MINI_PROGRAM_INFO,
            laiu8_info::UPDATE_MINI_PROGRAM_PAY_ID,
            refund_info::DELETE_TABLE,
            refund_info::INSERT_REFUND_RECORD,
            refund_info::INSERT_OTHER_RECORD,
            refund_info::UPDATE_RELATED_INFO,
            refund_info::UPDATE_TICKET_REFUND_INFO,
        ])
        .await?;
    database.txn.commit().await
}
