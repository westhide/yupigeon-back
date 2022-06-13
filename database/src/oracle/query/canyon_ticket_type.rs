use oracle::{Result, RowValue};
use serde::{Deserialize, Serialize};

use crate::oracle::query::common::QueryTrait;

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
    #[row_value(rename = "ticketPrice")]
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
            st.price ticketPrice,
            st.remark
            FROM
                Sys_TicketModel stm
                LEFT JOIN Sys_TicketModelDetail stmd ON stmd.ticketModelCode = stm.ticketModelCode
                LEFT JOIN Sys_Ticket st ON st.id = stmd.ticketId
            ORDER BY
                stm.ticketModelCode
            ";

    TicketType::query(sql, &[])
}
