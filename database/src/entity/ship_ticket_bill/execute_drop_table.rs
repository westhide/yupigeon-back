use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, ExecResult, Statement};

use crate::get_db;

pub async fn execute() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                -- TODO: 重命名表为ship_ticket_bill
                DROP TABLE IF EXISTS ticket_bill;
            "#
            .into(),
        ))
        .await
}
