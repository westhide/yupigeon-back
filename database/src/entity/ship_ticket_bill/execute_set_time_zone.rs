use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult, Statement,
};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
            SET time_zone = '+8:00';
        "#
        .into(),
    ))
    .await
}
