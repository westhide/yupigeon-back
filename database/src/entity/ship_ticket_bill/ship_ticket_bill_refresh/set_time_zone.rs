use sea_orm::{ConnectionTrait, DatabaseTransaction, DbErr, ExecResult, Statement};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        txn.get_database_backend(),
        r#"
            SET time_zone = '+8:00';
        "#
        .into(),
    ))
    .await
}
