use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseTransaction, DbErr, ExecResult, Statement,
};

pub async fn execute(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    drop_temp_table(txn).await?;
    create_temp_table(txn).await?;
    create_temp_table_index(txn).await?;
    update_laiu8_info(txn).await?;
    update_laiu8_payment_method(txn).await?;
    update_mini_program_table_info(txn).await?;
    update_mini_program_pay_info(txn).await
}

async fn drop_temp_table(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
            DROP TABLE IF EXISTS ticket_bill2;
        "#
        .into(),
    ))
    .await
}

async fn create_temp_table(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                CREATE TABLE ticket_bill2 AS
                WITH u8ot AS (
                SELECT *
                FROM u8_order_ship_tickets
                WHERE ticket_num!=''
                AND ticket_status NOT LIKE '取消%'
                -- AND ticket_status NOT IN ('已补差')
                AND create_time>UNIX_TIMESTAMP('2021-12-01')
                ),
                otid AS (
                    SELECT u8ot1.id,tb.ticket_id_old,u8ot2.id old_ticket_id
                    FROM u8ot u8ot1
                    LEFT JOIN ticket_bill tb ON u8ot1.ticket_num = tb.u8_ticket_num
                    LEFT JOIN u8ot u8ot2 ON u8ot2.ticket_num = tb.u8_ticket_id_old
                        AND u8ot2.old_ticket_id=0
                    WHERE tb.table_name='bt_ticket'
                        AND tb.u8_ticket_id_old IS NOT NULL
                        AND u8ot1.old_ticket_id=0
                        AND u8ot1.id!=u8ot2.id
                )
                SELECT
                u8ot.id,
                IFNULL(otid.old_ticket_id,u8ot.old_ticket_id) old_ticket_id
                FROM u8ot
                LEFT JOIN otid ON otid.id=u8ot.id
            "#
        .into(),
    ))
    .await
}

async fn create_temp_table_index(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                CREATE INDEX ix_id ON ticket_bill2(id);
            "#
        .into(),
    ))
    .await?;

    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                CREATE INDEX ix_old_ticket_id ON ticket_bill2(old_ticket_id);
            "#
        .into(),
    ))
    .await
}

async fn update_laiu8_info(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                WITH RECURSIVE
                u8ot AS (
                    SELECT *
                    FROM u8_order_ship_tickets
                    WHERE ticket_num!=''
                    AND ticket_status NOT LIKE '取消%'
                    -- AND ticket_status NOT IN ('已补差')
                    AND create_time>UNIX_TIMESTAMP('2021-11-01')
                ),
                otid AS (
                    SELECT u8ot1.id,tb.ticket_id_old,u8ot2.id old_ticket_id
                    FROM u8ot u8ot1
                    LEFT JOIN ticket_bill tb ON u8ot1.ticket_num = tb.u8_ticket_num
                    LEFT JOIN u8ot u8ot2 ON u8ot2.ticket_num = tb.u8_ticket_id_old
                        AND u8ot2.old_ticket_id=0
                    WHERE tb.table_name='bt_ticket'
                        AND tb.u8_ticket_id_old IS NOT NULL
                        AND u8ot1.old_ticket_id=0
                        AND u8ot1.id!=u8ot2.id
                )
                ,u8t1 AS (
                    -- TODO: 重命名表为temp_u8ot_id
                    SELECT  * FROM ticket_bill2
                )
                ,u8t2 (serial_no, orgin_id, old_ticket_id, id) AS (
                    SELECT  1
                        ,u8t1.id AS orgin_id
                        ,u8t1.old_ticket_id
                        ,u8t1.id
                    FROM u8t1
                    WHERE u8t1.old_ticket_id=0
                    UNION ALL
                    SELECT  serial_no+1
                        ,u8t2.orgin_id
                        ,u8t1.old_ticket_id
                        ,u8t1.id
                    FROM u8t1 INNER JOIN u8t2 ON u8t1.old_ticket_id = u8t2.id
                )
                ,u8t AS (
                    SELECT  u8t2.serial_no
                        ,u8t2.orgin_id
                        ,u8ot.*
                    FROM u8t2
                    LEFT JOIN u8ot ON u8t2.id=u8ot.id
                )
                ,sn AS (
                    SELECT  ROW_NUMBER() over w1 AS u8_serial_no
                        ,id
                    FROM ticket_bill
                    WHERE table_name !='bt_seat_compensation_history'
                    AND table_name!='bt_hcbb_history_detail'
                    WINDOW w1 AS ( PARTITION BY link_ticket_id ORDER BY serial_no)
                )

            UPDATE ticket_bill tb
            LEFT JOIN sn ON sn.id=tb.id
            LEFT JOIN u8t ON u8t.order_key = tb.u8_order_key
                AND u8t.ticket_num=tb.u8_ticket_num
                AND u8t.ticket_key=tb.u8_ticket_key
                AND u8t.serial_no=sn.u8_serial_no
            LEFT JOIN u8_order u8o ON u8o.id = u8t.order_id
            LEFT JOIN u8_user u8u ON u8u.id = u8t.user_id
            LEFT JOIN u8_tickets_orgsign u8org ON u8org.id = u8t.ota_id
            LEFT JOIN u8_order_payment u8p ON u8p.trade_no = u8o.trade_no
                AND FIND_IN_SET(u8o.id, u8p.orders)

            SET
                tb.u8_table_name='u8_order_ship_tickets'
                ,tb.u8_table_id=u8t.id
                ,tb.trade_no = u8o.trade_no
                ,tb.order_no = u8o.order_no
                ,tb.u8_channel_id = u8org.id
                ,tb.u8_channel_name = u8org.name
                ,tb.u8_payment_method = IF(tb.pay_amount IS NULL,NULL,u8p.pay_type)
                ,tb.pay_id = IF(
                    tb.pay_amount IS NULL
                    OR u8p.pay_type IN (10,23) -- 10='阶梯预存款',23='内部结算',
                    OR u8p.callback_trade_no=''
                    , NULL
                    , u8p.callback_trade_no)
            WHERE u8t.ticket_key IS NOT NULL
            AND tb.table_name != 'bt_hcbb_history_detail';
            "#
        .into(),
    ))
    .await
}

async fn update_laiu8_payment_method(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                UPDATE ticket_bill tb
                SET tb.u8_payment_method = ( CASE u8_payment_method
                        WHEN '0' THEN '未支付'
                        WHEN '1' THEN '支付宝'
                        WHEN '2' THEN '微信'
                        WHEN '3' THEN '哆啦宝支付'
                        WHEN '4' THEN '哆啦宝微信支付'
                        WHEN '5' THEN '预付款'
                        WHEN '9' THEN '积分'
                        WHEN '10' THEN '内部结算'
                        WHEN '11' THEN '红水河现金'
                        WHEN '12' THEN '红水河线下微信'
                        WHEN '13' THEN '红水河线下支付宝'
                        WHEN '14' THEN '红水河OTA在线收款'
                        WHEN '15' THEN '来游吧OTA在线收款'
                        WHEN '16' THEN '后付'
                        WHEN '17' THEN '零元付'
                        WHEN '18' THEN '来游吧预存款'
                        WHEN '19' THEN '桂林银行'
                        WHEN '20' THEN '测试付'
                        WHEN '21' THEN '飞猪支付宝'
                        WHEN '22' THEN '现金'
                        WHEN '23' THEN '阶梯预存款'
                        ELSE tb.u8_payment_method
                    END)
                ;
            "#
        .into(),
    ))
    .await
}

async fn update_mini_program_table_info(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                UPDATE ticket_bill tb
                LEFT JOIN b_orderinfo_yg_1 ygt ON tb.u8_order_key=ygt.FormalOrderId
                LEFT JOIN b_orderinfo_zd_1 zdt ON tb.u8_order_key=zdt.FormalOrderId
                LEFT JOIN u8_pay_api_log ygp ON ygt.OrderNo=ygp.order_id
                LEFT JOIN u8_pay_api_log zdp ON zdt.OrderNo=zdp.order_id

                SET tb.order_no = IF(ygt.id IS NULL, zdt.OrderNo, ygt.OrderNO)
                    ,tb.u8_table_name = IF(ygt.id IS NULL, 'b_orderinfo_zd_1', 'b_orderinfo_yg_1')
                    ,tb.u8_table_id = IF(ygt.id IS NULL, zdt.id, ygt.id)
                WHERE ygt.id IS NOT NULL OR zdt.id IS NOT NULL;
            "#
        .into(),
    ))
    .await
}

async fn update_mini_program_pay_info(txn: &DatabaseTransaction) -> Result<ExecResult, DbErr> {
    txn.execute(Statement::from_string(
        DatabaseBackend::MySql,
        r#"
                -- 更新小程序 pay_id
                UPDATE ticket_bill tb
                LEFT JOIN b_orderinfo_yg_1 ygt ON tb.u8_order_key=ygt.FormalOrderId
                LEFT JOIN b_orderinfo_zd_1 zdt ON tb.u8_order_key=zdt.FormalOrderId
                LEFT JOIN u8_pay_api_log ygp ON ygt.OrderNo=ygp.order_id
                LEFT JOIN u8_pay_api_log zdp ON zdt.OrderNo=zdp.order_id

                SET tb.pay_id = IF(ygp.pay_id IS NULL, zdp.pay_id, ygp.pay_id)
                    ,tb.u8_payment_method = IF(ygp.pay_id IS NULL
                        ,(CASE zdp.pay_type WHEN 0 THEN '微信' WHEN 1 THEN '支付宝' END)
                        ,(CASE ygp.pay_type WHEN 0 THEN '微信' WHEN 1 THEN '支付宝' END)
                    )
                WHERE tb.pay_amount IS NOT NULL
                AND (ygp.pay_id IS NOT NULL OR zdp.pay_id IS NOT NULL);
            "#
        .into(),
    ))
    .await
}
