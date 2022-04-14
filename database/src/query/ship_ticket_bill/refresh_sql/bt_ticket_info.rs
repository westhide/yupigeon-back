pub const DROP_TABLE: &str = r#"
    -- TODO: 重命名表为ship_ticket_bill
    DROP TABLE IF EXISTS ticket_bill;
"#;

pub const CREATE_TABLE: &str = r#"
    CREATE TABLE ticket_bill (
        id INT ( 11 ) NOT NULL AUTO_INCREMENT,
        table_name VARCHAR ( 50 ) NOT NULL COMMENT '数据来源表名',
        u8_table_name VARCHAR ( 50 ) COMMENT 'laiu8关联数据表名',
        table_id BIGINT ( 20 ) NOT NULL COMMENT '数据来源表id',
        u8_table_id INT ( 20 ) COMMENT 'laiu8关联数据id',
        serial_no TINYINT ( 2 ) COMMENT '票变更序列号',
        link_order_id BIGINT ( 20 ) COMMENT '关联票务订单号',
        order_id BIGINT ( 20 ) COMMENT '票务订单号',
        u8_order_key VARCHAR ( 20 ) COMMENT 'laiu8关联票务订单号=>统一字段类型 <u8_order_key=order_id>',
        link_ticket_id VARCHAR ( 255 ) NOT NULL COMMENT '关联票务系统id <CONCAT_WS(",",ticket_id_old,ticket_id_new)>',
        link_id INT (11) COMMENT 'ticket关联ID',
        ticket_id_old BIGINT ( 20 ) COMMENT '关联旧票id',
        u8_ticket_id_old VARCHAR ( 255 ) COMMENT 'laiu8关联旧票id=>统一字段类型',
        ticket_id BIGINT ( 20 ) NOT NULL COMMENT '票务系统票id',
        u8_ticket_num VARCHAR ( 255 ) COMMENT 'laiu8关联票务系统id=>统一字段类型 <u8_ticket_num=ticket_id>',
        ticket_id_new BIGINT ( 20 ) COMMENT '关联新票id',
        ticket_no BIGINT ( 20 ) COMMENT '票号',
        u8_ticket_key VARCHAR ( 20 ) COMMENT 'laiu8关联票号 <IF(ticket_status=已换船,ticket_no_new,ticket_no)>',
        ticket_no_new BIGINT ( 20 ) COMMENT '新订单的票号 退改升操作后新订单的票号',
        channel_id BIGINT ( 20 ) COMMENT '渠道id',
        u8_channel_id BIGINT ( 20 ) COMMENT 'laiu8渠道id',
        channel_name VARCHAR ( 50 ) COMMENT '订票渠道',
        u8_channel_name VARCHAR ( 50 ) COMMENT 'laiu8订票渠道',
        user_id BIGINT ( 30 ) COMMENT '用户id <sys_user.code>',
        u8_user_id BIGINT ( 30 ) COMMENT 'laiu8用户id',
        user_type VARCHAR ( 50 ) COMMENT '客户类型 <bt_channel.category_id>',
        u8_user_type VARCHAR ( 50 ) COMMENT 'laiu8用户类型',
        user_name VARCHAR ( 50 ) COMMENT '用户名称',
        u8_user_name VARCHAR ( 20 ) COMMENT 'laiu8用户名称',
        u8_nickname VARCHAR ( 60 ) COMMENT 'laiu8用户昵称',
        u8_vip_pact VARCHAR ( 50 ) COMMENT 'laiu8签约客户',
        mobile VARCHAR ( 50 ) COMMENT '用户手机号',
        ticket_type_name VARCHAR ( 50 ) COMMENT '票型',
        product_type VARCHAR ( 50 ) COMMENT '产品类型',
        change_order_id BIGINT ( 20 ) COMMENT '改签订单id <bt_ticket_change_order.id>',
        change_type VARCHAR ( 50 ) COMMENT '<改签|升舱|补差|换船>操作',
        ticket_status VARCHAR ( 50 ) COMMENT '船票状态',
        flight_id BIGINT ( 20 ) COMMENT '航班id',
        departure_datetime datetime COMMENT '航班时间',
        line_code VARCHAR ( 50 ) COMMENT '航线code',
        line_name VARCHAR ( 50 ) COMMENT '航线名称',
        ship_name VARCHAR ( 50 ) COMMENT '船舶',
        cabin_id BIGINT ( 20 ) COMMENT '舱位id',
        cabin_name VARCHAR ( 50 ) COMMENT '舱位',
        seat_memo VARCHAR ( 50 ) COMMENT '座位号',
        passenger_name VARCHAR ( 50 ) COMMENT '乘船人',
        passenger_id_no VARCHAR ( 50 ) COMMENT '证件号',
        full_ticket_price DECIMAL ( 10, 3 ) COMMENT '原价',
        discount_price DECIMAL ( 10, 3 ) COMMENT '折扣金额',
        ticket_price DECIMAL ( 10, 3 ) COMMENT '票价',
        get_voucher DECIMAL ( 10, 3 ) COMMENT '产生积分',
        use_voucher DECIMAL ( 10, 3 ) COMMENT '使用积分',
        pay_amount DECIMAL ( 10, 3 ) COMMENT '支付金额',
        real_price DECIMAL ( 10, 3 ) COMMENT '实际金额',
        payment_time datetime COMMENT '支付时间',
        payment_method VARCHAR ( 50 ) COMMENT '支付方式',
        u8_payment_method VARCHAR ( 50 ) COMMENT 'laiu8支付方式',
        order_no VARCHAR ( 50 ) COMMENT '平台订单号 <laiu8.order_no>',
        trade_no VARCHAR ( 40 ) COMMENT '商户订单号 <laiu8.trade_no>',
        pay_id VARCHAR ( 50 ) COMMENT '支付号 <laiu8.callback_trade_no>',
        refund_amount DECIMAL ( 10, 3 ) COMMENT '退票总额',
        fee DECIMAL ( 10, 3 ) COMMENT '手续费',
        is_lock TINYINT ( 1 ) DEFAULT '0' COMMENT '锁定状态<0=否,1=是>',
        create_time datetime NOT NULL COMMENT '创建时间',
        update_time datetime COMMENT '修改时间',
        PRIMARY KEY ( id ),
        KEY ix_table_id ( table_id ),
        KEY ix_ticket_id ( ticket_id ),
        KEY ix_link_order_id ( link_order_id ),
        KEY ix_link_ticket_id ( link_ticket_id ),
        KEY ix_link_id ( link_id ),
        KEY ix_ticket_id_new ( ticket_id_new ),
        KEY ix_ticket_id_old ( ticket_id_old ),
        KEY ix_order_id ( order_id ),
        KEY ix_ticket_no ( ticket_no ),
        KEY ix_u8_ticket_key ( u8_ticket_key ),
        KEY ix_change_order_id ( change_order_id ),
        KEY ix_flight_id ( flight_id ),
        KEY ix_departure_datetime ( departure_datetime ),
        KEY ix_trade_no ( trade_no ),
        KEY ix_pay_id ( pay_id ),
        KEY ix_payment_time ( payment_time ),
        KEY ix_create_time ( create_time ),
        KEY ix_u8_order_key ( u8_order_key ),
        KEY ix_u8_ticket_num ( u8_ticket_num ),
        KEY ix_u8_ticket_id_old ( u8_ticket_id_old )
    ) ENGINE = INNODB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci
    ;
"#;

pub const INSERT_KEY_RECORD: &str = r#"
    INSERT INTO ticket_bill ( serial_no, table_name, table_id, ticket_id, ticket_id_new, ticket_id_old, link_ticket_id, create_time )
    WITH
        -- 截取指定期间,不含改签后、已取消状态的票记录
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
    WINDOW w1 AS ( PARTITION BY cte1.link_ticket_id ORDER BY cte1.create_time)
    ;
"#;

pub const UPDATE_LINK_ID: &str = r#"
    WITH lid AS (
        SELECT
            id
            ,link_ticket_id
        FROM ticket_bill
        WHERE serial_no =1
    )
    UPDATE ticket_bill tb
    LEFT JOIN lid ON tb.link_ticket_id=lid.link_ticket_id
    LEFT JOIN bt_ticket t ON tb.ticket_id=t.id
    SET tb.link_order_id = t.order_id
        ,tb.link_id = lid.id
    ;
"#;

pub const UPDATE_ORDER_INFO: &str = r#"
    UPDATE ticket_bill tb
    LEFT JOIN bt_ticket t ON tb.ticket_id = t.id
    LEFT JOIN bt_order o ON t.order_id = o.id
    LEFT JOIN bt_channel c ON o.from_channel_id = c.id
    LEFT JOIN sys_user u ON o.create_user_code = u.code

    SET tb.order_id = o.id
        , tb.channel_id = c.id
        , tb.channel_name = c.name
        , tb.user_id = u.code	-- ! sys_user.id非主键,存在null值;用sys_user.code代替主键
        , tb.user_type = c.category_id
        , tb.user_name = u.user_name
        , tb.payment_time = o.payment_time
        , tb.payment_method = o.payment_method
    WHERE tb.serial_no = 1
    ;
"#;

pub const UPDATE_TICKET_INFO: &str = r#"
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

    LEFT JOIN bt_ticket_change_history cg ON tb.ticket_no = cg.ticket_no_new AND cg.link_order_id=tb.link_order_id	-- !ticket_no有重复,用order_id加强join唯一性,防止join重复陷阱
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
"#;

pub const UPDATE_PAY_AMOUNT: &str = r#"
    UPDATE ticket_bill tb
        SET tb.pay_amount = tb.ticket_price
        WHERE tb.serial_no = 1 AND payment_time IS NOT NULL
    ;
"#;

pub const UPDATE_DEPARTURE_INFO: &str = r#"
    UPDATE ticket_bill tb
    INNER JOIN bt_flight f ON tb.flight_id = f.id
    -- TODO:航班时间取数字段需确定
    SET tb.departure_datetime = f.planned_departure_datetime
        ,tb.line_code = f.line_code
        ,tb.ship_name = f.ship_name
    ;
"#;

pub const UPDATE_RELATED_INFO: &str = r#"
    UPDATE ticket_bill tb
    LEFT JOIN bt_channel_category cc ON cc.id=tb.user_type
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
        ,tb.user_type = cc.name
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
"#;

pub const UPDATE_U8_TICKET_KEY: &str = r#"
    WITH RECURSIVE
    tbcg( serial_no, id, ticket_no, ticket_id) AS (
        SELECT  tb1.serial_no
            ,tb1.id
            ,tb1.ticket_no
            ,tb1.ticket_id
        FROM ticket_bill tb1
        WHERE tb1.ticket_status IN ('已换船','已补差')
        -- 截取换船补差前的票记录
        AND tb1.serial_no =(SELECT  MIN(tb2.serial_no)
                            FROM ticket_bill tb2
                            WHERE tb2.ticket_id=tb1.ticket_id
                            AND tb2.ticket_status IN ('已换船','已补差')
                            GROUP BY  tb2.ticket_id)
        UNION ALL
        SELECT  tb1.serial_no
            ,tbcg.id
            ,tb1.ticket_no
            ,tb1.ticket_id
        FROM ticket_bill tb1
        INNER JOIN tbcg ON tb1.ticket_id = tbcg.ticket_id
        AND tb1.serial_no = tbcg.serial_no + 1
        WHERE tb1.table_name IN ('bt_hcbb_history_detail','bt_seat_compensation_history')
    )
    , tbcglast AS (
        SELECT  *
        FROM tbcg tbcg1
        -- 换船补差后最终的票记录
        WHERE tbcg1.serial_no=(SELECT  MAX(tbcg2.serial_no)
                                FROM tbcg tbcg2
                                WHERE tbcg1.id=tbcg2.id
                                GROUP BY  tbcg2.id)
    )
    UPDATE ticket_bill tb
    LEFT JOIN tbcglast ON tb.id = tbcglast.id
    SET tb.u8_ticket_key = IFNULL(tbcglast.ticket_no, tb.ticket_no)
    ;
"#;
