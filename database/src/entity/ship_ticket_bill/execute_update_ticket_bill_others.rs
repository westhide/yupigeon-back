use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, ExecResult, Statement};

use crate::get_db;

pub async fn execute() -> Result<ExecResult, DbErr> {
    update_pay_amount().await?;
    update_departure_info().await?;
    update_other_info().await
}

async fn update_pay_amount() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                UPDATE ticket_bill tb
                SET tb.pay_amount = tb.ticket_price
                WHERE tb.serial_no = 1 AND payment_time IS NOT NULL;
            "#
            .into(),
        ))
        .await
}

async fn update_departure_info() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                UPDATE ticket_bill tb
                INNER JOIN bt_flight f ON tb.flight_id = f.id
                -- TODO:航班时间取数字段需确定
                SET tb.departure_datetime = f.planned_departure_datetime
                    ,tb.line_code = f.line_code
                    ,tb.ship_name = f.ship_name
            ;
            "#
            .into(),
        ))
        .await
}

async fn update_other_info() -> Result<ExecResult, DbErr> {
    get_db("laiu8")
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                UPDATE ticket_bill tb
                SET tb.ticket_status = (CASE tb.ticket_status
                        WHEN "0" THEN '待出票'
                        WHEN "1" THEN '出票成功'
                        WHEN "-1" THEN '已取消'
                        WHEN "100" THEN '一检'
                        WHEN "101" THEN '二检'
                        WHEN "200" THEN '已申退'
                        WHEN "210" THEN '退款中'
                        WHEN "220" THEN '退成功未结款'
                        WHEN "230" THEN '已退款'
                        WHEN "240" THEN '退款失败'
                        WHEN "300" THEN '已废票'
                        WHEN "410" THEN '改签废票'
                        ELSE tb.ticket_status
                    END)
                    ,tb.payment_method = (CASE tb.payment_method
                        WHEN "1" THEN 'POS'
                        WHEN "2" THEN '微信'
                        WHEN "3" THEN '支付宝'
                        WHEN "4" THEN '现金'
                        WHEN "5" THEN '转账'
                        WHEN "6" THEN '银联支付'
                        WHEN "7" THEN '来游吧在线支付'
                        WHEN "8" THEN '来游吧OTA预存款'
                        WHEN "9" THEN '预存款'
                        ELSE tb.payment_method
                    END)
                    ,tb.line_name = (CASE tb.line_code
                        WHEN "BW" THEN '北海-涠洲'
                        WHEN "BH" THEN '北海-海口'
                        WHEN "WB" THEN '涠洲-北海'
                        ELSE tb.line_code
                    END)
                    ,tb.u8_order_key = tb.link_order_id
                    ,tb.u8_ticket_id_old = tb.ticket_id_old
                    ,tb.u8_ticket_num = tb.ticket_id
                ;
            "#
            .into(),
        ))
        .await
}
