use oracle::{Result, RowValue};
use serde::{Deserialize, Serialize};

use crate::oracle::query::common::QueryTrait;

#[derive(RowValue, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketBill {
    #[row_value(rename = "tradeId")]
    trade_id: String,
    #[row_value(rename = "tradeChannel")]
    trade_channel: String,
    #[row_value(rename = "tradeType")]
    trade_type: String,
    #[row_value(rename = "tradeTypeName")]
    trade_type_name: String,
    #[row_value(rename = "tradeDate")]
    trade_date: String,
    #[row_value(rename = "clientType")]
    client_type: String,
    #[row_value(rename = "clientTypeName")]
    client_type_name: String,
    #[row_value(rename = "clientCode")]
    client_code: Option<String>,
    #[row_value(rename = "clientFullName")]
    client_full_name: String,
    #[row_value(rename = "payTypeName")]
    pay_type_name: Option<String>,
    #[row_value(rename = "optorCode")]
    operator_code: String,
    #[row_value(rename = "operatorName")]
    operator_name: String,
    #[row_value(rename = "ticketModelCode")]
    ticket_model_code: String,
    #[row_value(rename = "ticketModelName")]
    ticket_model_name: String,
    #[row_value(rename = "onlineTicketModelName")]
    online_ticket_model_name: Option<String>,
    #[row_value(rename = "ticketCount")]
    ticket_count: f32,
    #[row_value(rename = "ticketModelPrice")]
    ticket_model_price: f32,
    #[row_value(rename = "paySum")]
    pay_sum: f32,
    #[row_value(rename = "onlinePaySum")]
    online_pay_sum: Option<f32>,
    #[row_value(rename = "ticketModelGroupCode")]
    ticket_model_group_code: String,
    #[row_value(rename = "ticketModelGroupName")]
    ticket_model_group_name: String,
    #[row_value(rename = "printCount")]
    print_count: f32,
    #[row_value(rename = "onlineBillDate")]
    online_bill_date: Option<String>,
    #[row_value(rename = "billNo")]
    bill_no: Option<String>,
    #[row_value(rename = "onlineDetailNo")]
    online_detail_no: Option<String>,
    #[row_value(rename = "onlineClientName")]
    online_client_name: Option<String>,
    #[row_value(rename = "onlineBillType")]
    online_bill_type: Option<String>,
    #[row_value(rename = "onlineAppFrom")]
    online_app_from: Option<String>,
}

pub fn ticket_bill(
    datetime_from: &str,
    datetime_end: &str,
    condition: &str,
) -> Result<Vec<TicketBill>> {
    let sql = format!("
                WITH wb AS (
                    SELECT
                        wbd.billNo,
                        wbd.billDetailNo,
                        wbd.ticketModelCode,
                        wbd.ticketModelName,
                        wbd.paySum,
                        wbm.clientName,
                        wbm.billType,
                        wbm.app_from,
                        wbd.billDate
                    FROM
                        WEB_BillDetail wbd
                        LEFT JOIN WEB_BillMain wbm ON wbm.billNo = wbd.billNo
                    WHERE
                        wbm.billStatus = 1
                )
                SELECT
                    ttm.tradeID,
                    CASE
                        when ttm.billNo IS NULL then '??????'
                        else '??????'
                    end tradeChannel,
                    ttd.tradeType,
                    (
                        CASE
                            ttd.tradeType
                            when '01' then '??????'
                            when '02' then '??????'
                            when '03' then '??????'
                            when '04' then '?????????'
                            when '05' then '???????????????'
                            when '06' then '??????'
                            when '07' then '?????????'
                            else '?????????'
                        end
                    ) tradeTypeName,
                    ttm.tradeDate,
                    ttm.clientType,
                    ct.clientTypeName,
                    ttm.clientCode,
                    CASE
                        when ttm.clientType = '01' then '??????'
                        else nvl(ci.clientFullName, '')
                    end clientFullName,
                    ttp.payTypeName,
                    ttm.optorCode,
                    so.operatorName,
                    ttd.ticketModelCode,
                    stm.ticketModelName,
                    wb.ticketModelName onlineTicketModelName,
                    ttd.ticketCount,
                    ttd.ticketModelPrice,
                    ttd.paySum,
                    wb.paySum onlinePaySum,
                    stm.ticketModelGroupCode,
                    stmg.ticketModelGroupName,
                    ttd.printCount,
                    wb.billDate onlineBillDate,
                    ttm.billNo,
                    wb.billDetailNo onlineDetailNo,
                    wb.clientName onlineClientName,
                    wb.billType onlineBillType,
                    wb.app_from onlineAppFrom
                FROM
                    TKT_TradeMain ttm
                    LEFT JOIN TKT_TradeDetail ttd ON ttm.tradeId = ttd.tradeId
                    LEFT JOIN TKT_TradePayType ttp ON ttm.tradeId = ttp.tradeId
                    LEFT JOIN SYS_TicketModel stm ON ttd.ticketModelCode = stm.ticketModelCode
                    LEFT JOIN SYS_TicketModelGroup stmg ON stmg.ticketModelGroupCode = stm.ticketModelGroupCode
                    LEFT JOIN CLT_ClientType ct ON ct.clientTypeCode = ttm.clientType
                    LEFT JOIN CLT_ClientInfo ci ON ci.clientCode = ttm.clientCode
                    LEFT JOIN SYS_Operator so ON so.operatorCode = ttm.optorCode
                    LEFT JOIN wb ON wb.billNo = ttm.billNo
                    AND wb.ticketModelCode = ttd.ticketModelCode
                WHERE
                    ttm.tradeDate BETWEEN TO_DATE(:1, 'YYYY-MM-DD hh24:mi:ss')
                    AND TO_DATE(:2, 'YYYY-MM-DD hh24:mi:ss')
                    {}
                ORDER BY
                    ttm.tradeDate DESC
                    ",condition);

    TicketBill::query(&sql, &[&datetime_from, &datetime_end])
}
