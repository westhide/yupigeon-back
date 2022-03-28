use sea_orm::{DatabaseTransaction, DbErr, ExecResult};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    crate::execute_sql(
        txn,
        r#"
            -- TODO: 重命名表为ship_ticket_bill
            DROP TABLE IF EXISTS ticket_bill;
        "#,
    )
    .await
}
