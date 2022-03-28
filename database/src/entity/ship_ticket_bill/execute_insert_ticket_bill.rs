use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult, Statement,
};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn .execute(Statement::from_string(
            DatabaseBackend::MySql,
            r#"
                INSERT INTO ticket_bill ( serial_no, table_name, table_id, ticket_id, ticket_id_new, ticket_id_old, link_ticket_id, create_time )
                WITH
                    -- 截取指定期间，不含改签后、已取消状态的票记录
                    tid AS (
                    SELECT id FROM bt_ticket t
                    WHERE NOT t.ticket_status <=> -1 -- 剔除已取消记录
                    AND t.change_from_ticket_id IS NULL
                    AND t.create_time >= "2021-12-01"  -- ! 截取此日期后的记录
                    )
                    ,bt AS (
                        SELECT  'bt_ticket' table_name
                            ,t.id table_id
                            ,t.id ticket_id
                            ,t2.id ticket_id_new
                            ,t.change_from_ticket_id ticket_id_old
                            ,CONCAT_WS(",", t.change_from_ticket_id, t.id, t2.id) link_ticket_id
                            ,t.create_time
                        FROM bt_ticket t
                        LEFT JOIN bt_ticket t2
                        ON t.id = t2.change_from_ticket_id
                            -- 筛除ticket_status为0时, change_from_ticket_id重复项
                            AND NOT t2.ticket_status <=> 0
                            AND NOT t2.ticket_status <=> -1
                        WHERE t.id IN (SELECT id FROM tid)
                        OR t.change_from_ticket_id IN (SELECT id FROM tid)
                    )
                    ,cte1 AS (
                        -- 票主表.关键字段
                        SELECT * FROM bt
                        UNION ALL(
                        -- 升舱记录.关键字段
                        SELECT  'bt_seat_upgrade_history' table_name
                            ,up.id table_id
                            ,up.ticket_id
                            ,NULL ticket_id_new
                            ,NULL ticket_id_old
                            ,bt.link_ticket_id
                            ,up.create_time
                        FROM bt_seat_upgrade_history up
                        LEFT JOIN bt ON up.ticket_id = bt.ticket_id
                        WHERE up.STATUS = 2 AND bt.ticket_id IS NOT NULL
                        )
                        UNION ALL(
                        -- 补差记录.关键字段
                        SELECT  'bt_seat_compensation_history' table_name
                            ,comp.id table_id
                            ,comp.ticket_id
                            ,NULL ticket_id_new
                            ,NULL ticket_id_old
                            ,bt.link_ticket_id
                            ,comp.create_time
                        FROM bt_seat_compensation_history comp
                        LEFT JOIN bt ON comp.ticket_id = bt.ticket_id
                        WHERE comp.STATUS = 2 AND comp.history_type = 1
                        AND bt.ticket_id IS NOT NULL
                        )
                        UNION ALL(
                        -- 换船并班记录.关键字段
                        SELECT  'bt_hcbb_history_detail' table_name
                            ,hc.id table_id
                            ,hc.ticket_id
                            ,NULL ticket_id_new
                            ,NULL ticket_id_old
                            ,bt.link_ticket_id
                            ,hc.create_time
                        FROM bt_hcbb_history_detail hc
                        LEFT JOIN bt ON hc.ticket_id = bt.ticket_id
                        WHERE bt.ticket_id IS NOT NULL
                        )
                    )
                SELECT
                    -- 窗口函数 => 相同link_ticket_id按create_time序列化
                    ROW_NUMBER() over w1 AS serial_no
                    ,cte1.*
                FROM cte1
                WINDOW w1 AS ( PARTITION BY cte1.link_ticket_id ORDER BY cte1.create_time);
            "#
            .into(),
        ))
        .await
}
