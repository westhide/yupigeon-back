use oracle::{Result, RowValue};
use serde::{Deserialize, Serialize};

use crate::oracle::query::common::QueryTrait;

#[derive(RowValue, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketType {
    #[row_value(rename = "operatorName")]
    value: String,
}

pub fn operators() -> Result<Vec<TicketType>> {
    let sql = "
            SELECT
            so.operatorName
            FROM
                Sys_Operator so
            ";

    TicketType::query(sql, &[])
}
