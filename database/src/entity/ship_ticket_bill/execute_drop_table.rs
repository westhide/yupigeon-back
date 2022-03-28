use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult, Statement,
};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                -- TODO: 重命名表为ship_ticket_bill
                DROP TABLE IF EXISTS ticket_bill;
            "#
        .into(),
    ))
    .await
}
