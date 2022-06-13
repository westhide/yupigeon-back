use oracle::{Result, RowValue};
use serde::{Deserialize, Serialize};

use crate::oracle::OracleDatabase;

#[derive(RowValue, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketType {
    #[row_value(rename = "ticketModelCode")]
    ticket_model_code: String,
    #[row_value(rename = "ticketModelName")]
    ticket_model_name: String,
    #[row_value(rename = "ticketModelPrice")]
    ticket_model_price: f32,
    #[row_value(rename = "ticketId")]
    ticket_id: String,
    #[row_value(rename = "ticketFullName")]
    ticket_full_name: String,
    #[row_value(rename = "ticketShortName")]
    ticket_short_name: String,
    #[row_value(rename = "ticket_price")]
    ticket_price: String,
    #[row_value(rename = "remark")]
    remark: Option<String>,
}

pub fn ticket_type() -> Result<Vec<TicketType>> {
    let sql = "
            SELECT
            stm.ticketModelCode,
            stm.ticketModelName,
            stm.priceSum ticketModelPrice,
            st.id ticketId,
            st.ticketFullName,
            st.ticketShortName,
            st.price,
            st.remark
            FROM
                Sys_TicketModel stm
                LEFT JOIN Sys_TicketModelDetail stmd ON stmd.ticketModelCode = stm.ticketModelCode
                LEFT JOIN Sys_Ticket st ON st.id = stmd.ticketId
            ORDER BY
                stm.ticketModelCode
            ";

    let rows = OracleDatabase::connection()?.query_as::<TicketType>(sql, &[])?;

    let mut ticket_type = vec![];
    for row in rows {
        ticket_type.push(row?);
    }

    Ok(ticket_type)
}
