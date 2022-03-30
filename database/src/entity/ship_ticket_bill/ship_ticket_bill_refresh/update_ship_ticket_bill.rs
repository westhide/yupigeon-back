use async_recursion::async_recursion;
use sea_orm::{DatabaseTransaction, DbErr, ExecResult};

pub async fn execute(txn: &DatabaseTransaction) -> Result<(), DbErr> {
    for _ in 0..4 {
        recursion_execute(txn).await?;
    }
    Ok(())
}

#[async_recursion]
pub async fn recursion_execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    crate::execute_sql(txn,r#"
            WITH
            -- TODO: 暂时用create_user关联sys_user,获取改签渠道信息;可能会关联不到
            ul AS (
                SELECT  any_value(code) code
                    ,any_value(channel_id) channel_id
                    ,CONCAT( user_name,"(",employee_id,")" ) user
                FROM sys_user
                WHERE channel_id IS NOT NULL
                -- 防止concat()后的user重复
                GROUP BY user HAVING( COUNT(*)= 1 )
            ),
            comp2 AS (
                SELECT * FROM bt_seat_compensation_history
                WHERE status = 2 AND history_type = 0
            )

            UPDATE ticket_bill tb
            LEFT JOIN ticket_bill tb2 ON tb.link_ticket_id = tb2.link_ticket_id AND tb.serial_no+1 = tb2.serial_no
            LEFT JOIN bt_ticket t ON tb.ticket_id = t.id

            LEFT JOIN bt_seat_upgrade_history up ON tb.table_id = up.id AND tb.table_name = "bt_seat_upgrade_history"
            LEFT JOIN bt_channel up_c ON up.operator_channel_id = up_c.id
            LEFT JOIN ul up_ul ON up.create_user = up_ul.user
            LEFT JOIN sys_user up_u ON up_ul.code = up_u.code

            LEFT JOIN bt_seat_upgrade_history up2 ON tb2.table_id = up2.id AND tb2.table_name = "bt_seat_upgrade_history"

            LEFT JOIN bt_seat_compensation_history comp ON tb.table_id = comp.id AND tb.table_name = "bt_seat_compensation_history"
            LEFT JOIN bt_channel comp_c ON comp.operator_channel_id = comp_c.id
            LEFT JOIN ul comp_ul ON comp.create_user = comp_ul.user
            LEFT JOIN sys_user comp_u ON comp_ul.code = comp_u.code

            LEFT JOIN comp2 ON tb2.ticket_id = comp2.ticket_id AND tb2.table_name = "bt_seat_compensation_history"

            LEFT JOIN bt_hcbb_history_detail hc ON tb.table_id = hc.id AND tb.table_name = "bt_hcbb_history_detail"
            LEFT JOIN bt_hcbb_history hch ON hc.history_id = hch.id
            LEFT JOIN ul hc_ul ON hc.create_user = hc_ul.user
            LEFT JOIN sys_user hc_u ON hc_ul.code = hc_u.code
            LEFT JOIN bt_channel hc_c ON hc_ul.channel_id = hc_c.id

            LEFT JOIN bt_hcbb_history_detail hc2 ON tb2.table_id = hc2.id AND tb2.table_name = "bt_hcbb_history_detail"
            LEFT JOIN bt_hcbb_history hch2 ON hc2.history_id = hch2.id

            LEFT JOIN bt_ticket_change_history cg ON tb.ticket_no = cg.ticket_no_new AND cg.link_order_id=tb.link_order_id	-- !ticket_no有重复,用order_id加强join唯一性，防止join重复陷阱
            LEFT JOIN ul cg_ul ON cg.create_user = cg_ul.user
            LEFT JOIN sys_user cg_u ON cg_ul.code = cg_u.code
            LEFT JOIN bt_channel cg_c ON t.channel_id = cg_c.id
            LEFT JOIN bt_ticket_change_order cgo ON cg.change_order_id = cgo.id

            LEFT JOIN bt_ticket_change_history cg2 ON tb2.ticket_no = cg2.ticket_no_new AND cg2.link_order_id=tb2.link_order_id

            SET
                tb.channel_id = IFNULL(tb.channel_id,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up_c.id
                        WHEN 'bt_seat_compensation_history' THEN comp_c.id
                        WHEN 'bt_hcbb_history_detail' THEN hc_c.id
                        WHEN 'bt_ticket' THEN cg_c.id
                    END)
                )
                ,tb.channel_name = IFNULL(tb.channel_name,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up_c.name
                        WHEN 'bt_seat_compensation_history' THEN comp_c.name
                        WHEN 'bt_hcbb_history_detail' THEN hc_c.name
                        WHEN 'bt_ticket' THEN cg_c.name
                    END)
                )
                ,tb.user_id = IFNULL(tb.user_id,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up_u.code
                        WHEN 'bt_seat_compensation_history' THEN comp_u.code
                        WHEN 'bt_hcbb_history_detail' THEN hc_u.code
                        WHEN 'bt_ticket' THEN cg_u.code
                    END)
                )
                ,tb.user_type = IFNULL(tb.user_type,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up_c.category_id
                        WHEN 'bt_seat_compensation_history' THEN comp_c.category_id
                        WHEN 'bt_hcbb_history_detail' THEN hc_c.category_id
                        WHEN 'bt_ticket' THEN cg_c.category_id
                    END)
                )
                ,tb.user_name = IFNULL(tb.user_name,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up_u.user_name
                        WHEN 'bt_seat_compensation_history' THEN comp_u.user_name
                        WHEN 'bt_hcbb_history_detail' THEN hc_u.user_name
                        WHEN 'bt_ticket' THEN cg_u.user_name
                    END)
                )
                ,tb.change_order_id = IFNULL(tb.change_order_id,cgo.id)
                ,tb.update_time = IFNULL(tb.update_time,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up.update_time
                        WHEN 'bt_seat_compensation_history' THEN comp.update_time
                        WHEN 'bt_hcbb_history_detail' THEN hc.update_time
                        WHEN 'bt_ticket' THEN t.update_time
                    END)
                )

                ,tb.ticket_no = IFNULL(tb.ticket_no,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.ticket_no
                        WHEN 'bt_seat_compensation_history' THEN comp2.ticket_no
                        WHEN 'bt_hcbb_history_detail' THEN hc2.ticket_no
                        WHEN 'bt_ticket' THEN IFNULL(cg2.ticket_no_old, t.ticket_no)
                        ELSE t.ticket_no
                    END)
                )
                ,tb.ticket_type_name = IFNULL(tb.ticket_type_name,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.ticket_type_name
                        WHEN 'bt_seat_compensation_history' THEN comp2.ticket_type_name
                        WHEN 'bt_hcbb_history_detail' THEN tb2.ticket_type_name
                        WHEN 'bt_ticket' THEN t.ticket_type_name
                        ELSE t.ticket_type_name
                    END)
                )
                ,tb.ticket_status = IFNULL(tb.ticket_status,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN '已升舱'
                        WHEN 'bt_seat_compensation_history' THEN '已补差'
                        WHEN 'bt_hcbb_history_detail' THEN '已换船'
                        WHEN 'bt_ticket' THEN t.ticket_status
                        ELSE t.ticket_status
                    END)
                )
                ,tb.flight_id = IFNULL(tb.flight_id,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN tb2.flight_id
                        WHEN 'bt_seat_compensation_history' THEN comp2.link_flight_id
                        WHEN 'bt_hcbb_history_detail' THEN hch2.from_flight_id
                        WHEN 'bt_ticket' THEN IFNULL(cg2.link_flight_id_old, t.link_flight_id)
                        ELSE t.link_flight_id
                    END)
                )
                ,tb.cabin_id = IFNULL(tb.cabin_id,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.old_cabin_id
                        WHEN 'bt_seat_compensation_history' THEN comp2.link_cabin_id
                        WHEN 'bt_hcbb_history_detail' THEN hch2.from_cabin_id
                        WHEN 'bt_ticket' THEN IFNULL(cg2.link_cabin_id_old, t.link_cabin_id)
                        ELSE t.link_cabin_id
                    END)
                )
                ,tb.cabin_name = IFNULL(tb.cabin_name,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.old_cabin_name
                        WHEN 'bt_seat_compensation_history' THEN comp2.cabin_name
                        WHEN 'bt_hcbb_history_detail' THEN hch2.from_cabin_name
                        WHEN 'bt_ticket' THEN t.cabin_name
                        ELSE t.cabin_name
                    END)
                )
                ,tb.seat_memo = IFNULL(tb.seat_memo,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.old_seat_memo
                        WHEN 'bt_seat_compensation_history' THEN tb2.seat_memo
                        WHEN 'bt_hcbb_history_detail' THEN hc2.from_seat_memo
                        WHEN 'bt_ticket' THEN t.seat_memo
                        ELSE t.seat_memo
                    END)
                )

                ,tb.passenger_name = t.passenger_name
                ,tb.passenger_id_no = t.passenger_id_no

                ,tb.ticket_price = IFNULL(tb.ticket_price,
                    (CASE tb2.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up2.old_ticket_price
                        WHEN 'bt_seat_compensation_history' THEN comp2.ticket_price_old
                        WHEN 'bt_hcbb_history_detail' THEN hc2.from_ticket_price
                        WHEN 'bt_ticket' THEN IFNULL(cg2.ticket_price_old, t.ticket_price)
                        ELSE t.ticket_price
                    END)
                )

                ,tb.payment_time = IFNULL(tb.payment_time,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up.create_time
                        WHEN 'bt_seat_compensation_history' THEN
                            IF(comp.ticket_price_new-comp.ticket_price_old>0,
                            comp.create_time,
                            NULL)
                        WHEN 'bt_hcbb_history_detail' THEN NULL
                        WHEN 'bt_ticket' THEN IF(cgo.price_difference>0, cgo.create_time, NULL)
                    END)
                )
                ,tb.payment_method = IFNULL(tb.payment_method,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN up.payment_method
                        WHEN 'bt_seat_compensation_history' THEN
                            IF(comp.ticket_price_new-comp.ticket_price_old>0,
                            comp.payment_method,
                            NULL)
                        WHEN 'bt_hcbb_history_detail' THEN NULL
                        WHEN 'bt_ticket' THEN IF(cgo.price_difference>0, cgo.payment_method, NULL)
                    END)
                )
                ,tb.pay_amount = IFNULL(tb.pay_amount,
                    (CASE tb.table_name
                        WHEN 'bt_seat_upgrade_history' THEN
                            IF(up_c.category_id=2,
                            up.new_ticket_price-up.old_ticket_price,
                            up.new_ticket_price)
                        WHEN 'bt_seat_compensation_history' THEN
                            IF(comp.ticket_price_new-comp.ticket_price_old>0,
                                IF(comp_c.category_id=2,
                                comp.ticket_price_new-comp.ticket_price_old,
                                comp.ticket_price_new),
                                NULL)
                        WHEN 'bt_hcbb_history_detail' THEN NULL
                        WHEN 'bt_ticket' THEN
                            IF(cgo.price_difference>0,
                                -- ! 窗口改签订单重新支付只取差额
                                IF(cg_c.category_id=2,
                                cg.ticket_price_diff + cg.ticket_change_fee,
                                cg.ticket_price_new),
                                NULL)
                    END)
                )
            ;
        "#
        )
        .await
    // if  {
    // recursion_execute().await?;
    // };
}
