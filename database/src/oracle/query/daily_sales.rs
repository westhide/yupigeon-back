use oracle::{Result, RowValue};
use serde::{Deserialize, Serialize};

use crate::oracle::query::common::QueryTrait;

#[derive(RowValue, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySalesChart {
    date: String,
    times: f32,
    sales: f32,
}

pub fn daily_sales_chart(datetime_from: &str, datetime_end: &str) -> Result<Vec<DailySalesChart>> {
    let sql = r#"
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
            ),
            tb AS (
                SELECT
                    ttm.tradeID,
                    CASE
                        when ttm.billNo IS NULL then '窗口'
                        else '线上'
                    end tradeChannel,
                    ttd.tradeType,
                    (
                        CASE
                            ttd.tradeType
                            when '01' then '售票'
                            when '02' then '退票'
                            when '03' then '换票'
                            when '04' then '预售票'
                            when '05' then '退票原记录'
                            when '06' then '半退'
                            when '07' then '已核销'
                            else '手工票'
                        end
                    ) tradeTypeName,
                    ttm.tradeDate,
                    ttm.clientType,
                    ct.clientTypeName,
                    ttm.clientCode,
                    CASE
                        when ttm.clientType = '01' then '散客'
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
                ORDER BY
                    tradeChannel,
                    ttm.tradeDate
            )
            SELECT
                TO_CHAR(tradeDate, 'YYYY-MM-DD') "date",
                SUM(ticketCount) times,
                SUM(paySum) sales
            FROM
                tb
            GROUP BY
                TO_CHAR(tradeDate, 'YYYY-MM-DD')
            ORDER BY
                "date"
            "#;

    DailySalesChart::query(sql, &[&datetime_from, &datetime_end])
}
