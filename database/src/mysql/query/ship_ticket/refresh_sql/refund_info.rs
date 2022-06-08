pub const DROP_TABLE: &str = r#"
    DROP TABLE IF EXISTS ship_ticket_refund_bill;
"#;

pub const CREATE_TABLE: &str = r#"
    CREATE TABLE ship_ticket_refund_bill (
        id INT ( 11 ) NOT NULL AUTO_INCREMENT,
        table_name VARCHAR ( 50 ) NOT NULL COMMENT '数据来源表名',
        table_id BIGINT ( 20 ) NOT NULL COMMENT '数据来源表id',
        tb_id INT ( 11 ) COMMENT 'ticket_bill.id',
        ticket_id BIGINT ( 20 ) COMMENT '票id',
        link_ticket_id VARCHAR ( 255 ) NOT NULL COMMENT '关联票务系统id<CONCAT_WS(",",ticket_id_old,ticket_id_new)>',
        ticket_no BIGINT ( 20 ) COMMENT '票号',
        refund_type VARCHAR ( 50 ) COMMENT '退款类型<改签|补差|零退|换船>',
        channel_id VARCHAR ( 255 ) COMMENT '退款渠道id',
        channel_name VARCHAR ( 255 ) COMMENT '退款渠道',
        user_id BIGINT ( 30 ) COMMENT '用户id <sys_user.code>',
        user_type VARCHAR ( 50 ) COMMENT '客户类型 <bt_channel.category_id>',
        user_name VARCHAR ( 50 ) COMMENT '用户名称',
        refund_method VARCHAR ( 255 ) COMMENT '退款方式',
        u8_refund_method VARCHAR ( 255 ) COMMENT 'laiu8退款方式',
        refund_finish_time datetime COMMENT '退款时间(实际退款成功)',
        trade_no VARCHAR ( 255 ) COMMENT '交易号',
        refund_no VARCHAR ( 255 ) COMMENT 'out_refund_no',
        refund_id VARCHAR ( 255 ) COMMENT '退款流水号(实际退款成功)',
        refund_amount DECIMAL ( 11, 3 ) DEFAULT '0.000' COMMENT '实际退款金额(实际退款成功)',
        fee DECIMAL ( 11, 3 ) DEFAULT '0.000' COMMENT '手续费',
        detail_id INT ( 11 ) COMMENT 'common表id',
        order_id BIGINT ( 20 ) COMMENT '票务单号',
        related_type INT ( 11 ) COMMENT '产品类型',

        PRIMARY KEY ( id ),
        KEY ix_table_id ( table_id ),
        KEY ix_tb_id ( tb_id ),
        KEY ix_ticket_id ( ticket_id ),
        KEY ix_ticket_no ( ticket_no ),
        KEY ix_order_id ( order_id ),
        KEY ix_detail_id ( detail_id ),
        KEY ix_related_type ( related_type ),
        KEY ix_refund_id ( refund_id ),
        KEY ix_refund_finish_time ( refund_finish_time )
    ) ENGINE = INNODB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;
"#;

pub const DELETE_TABLE: &str = r#"
    DELETE FROM ship_ticket_refund_bill;
"#;

pub const INSERT_REFUND_RECORD: &str = r#"
    INSERT INTO ship_ticket_refund_bill(table_name, table_id, tb_id, ticket_id, link_ticket_id, ticket_no, refund_type, channel_id, channel_name, user_id, user_type, user_name, refund_method, refund_finish_time, refund_amount, fee, order_id, related_type)
    WITH rs AS (
        SELECT  ROW_NUMBER() over w1 AS serial_no ,r.id
        FROM bt_ticket_refund_history r
        LEFT JOIN ticket_bill tb ON r.link_ticket_id=tb.ticket_id
            AND r.ticket_no=tb.ticket_no AND r.link_order_id=tb.link_order_id
        WHERE tb.ticket_status="已退款"
        WINDOW w1 AS ( PARTITION BY tb.id ORDER BY r.ticket_refund_finish_time)
    )
    SELECT  'bt_ticket_refund_history' table_name
        ,r.id table_id
        ,tb.id tb_id
        ,tb.ticket_id
        ,tb.link_ticket_id
        ,r.ticket_no
        ,tb.ticket_status refund_type
        ,c.id channel_id
        ,c.name channel_name
        ,r.ticket_refund_finish_user_code user_id
        ,cc.name user_type
        ,u.user_name
        ,r.ticket_refund_method refund_method
        ,r.ticket_refund_finish_time refund_finish_time
        ,r.ticket_refund refund_amount
        ,IF(rs.serial_no=1,r.ticket_price-r.ticket_refund,-r.ticket_refund) fee
        ,tb.link_order_id order_id
        ,1 related_type
    FROM bt_ticket_refund_history r
    LEFT JOIN rs ON rs.id=r.id
    LEFT JOIN ticket_bill tb ON r.link_ticket_id=tb.ticket_id
        AND r.ticket_no=tb.ticket_no AND r.link_order_id=tb.link_order_id
    LEFT JOIN sys_user u ON r.ticket_refund_finish_user_code=u.code
    LEFT JOIN bt_channel c ON u.channel_id=c.id
    LEFT JOIN bt_channel_category cc ON cc.id=c.category_id
    WHERE tb.ticket_status="已退款"
    ;
"#;

pub const INSERT_OTHER_RECORD: &str = r#"
    INSERT INTO ship_ticket_refund_bill(table_name, table_id, tb_id, ticket_id, link_ticket_id, ticket_no, refund_type, channel_id, channel_name, user_id, user_type, user_name, refund_method, refund_finish_time, refund_amount, fee, order_id, related_type)
    WITH cte AS (
        SELECT
        'ticket_bill' table_name
        ,tb.id table_id
        ,tb.id tb_id
        ,tb.ticket_id
        ,tb.link_ticket_id
        ,tb.ticket_no
        ,tb.ticket_status refund_type
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2c.id
                        ELSE tb2.channel_id
                    END) channel_id
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2c.name
                        ELSE tb2.channel_name
                    END) channel_name
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2u.code
                        ELSE tb2.user_id
                    END) user_id
                ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2cc.name
                        ELSE tb2.user_type
                    END) user_type
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2u.user_name
                        ELSE tb2.user_name
                    END) user_name
        -- TODO:退款方式暂用支付方式
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2.ticket_price_diff_refund_method
                        ELSE IFNULL(tb2.payment_method,tb.payment_method)
                    END) refund_method
        -- TODO:来游吧退款时间暂用create_time
        ,(CASE tb2.table_name
                        WHEN 'bt_hcbb_history_detail' THEN hc2.ticket_price_diff_refund_time
                        ELSE tb2.create_time
                    END) refund_finish_time

        ,(CASE tb2.table_name
                WHEN 'bt_seat_upgrade_history' THEN
                    IF(tb2.user_type='线下',NULL,up2.old_ticket_price)
                WHEN 'bt_seat_compensation_history' THEN (
                    CASE
                    WHEN comp2.ticket_price_new-comp2.ticket_price_old>0 THEN
                        IF(tb2.user_type='线下',NULL,comp2.ticket_price_old)
                    WHEN comp2.ticket_price_new-comp2.ticket_price_old<0 THEN
                        comp2.ticket_price_old-comp2.ticket_price_new
                    END
                )
                WHEN 'bt_hcbb_history_detail' THEN
                    IF(hc2.from_ticket_price>hc2.to_ticket_price,
                    hc2.from_ticket_price-hc2.to_ticket_price,
                    NULL)
                WHEN 'bt_ticket' THEN
                    (CASE
                    WHEN cgo2.price_difference>0 THEN
                        IF(tb2.user_type='线下',
                        NULL,
                        cg2.ticket_price_old - cg2.ticket_change_fee)
                    WHEN cgo2.price_difference<0 THEN
                            cg2.ticket_price_old - cg2.ticket_price_new - cg2.ticket_change_fee
                    END
                    )
            END) refund_amount

            ,(CASE tb2.table_name
                WHEN 'bt_ticket' THEN IF(cg2.ticket_change_fee>0,cg2.ticket_change_fee,NULL)
                ELSE NULL
            END) fee

            ,tb2.link_order_id order_id
            ,1 related_type

        FROM ticket_bill tb
        INNER JOIN ticket_bill tb2 ON tb2.link_ticket_id = tb.link_ticket_id
            AND tb2.serial_no = tb.serial_no+1

        LEFT JOIN bt_seat_upgrade_history up2 ON tb2.table_id = up2.id
            AND tb2.table_name = "bt_seat_upgrade_history"

        LEFT JOIN bt_seat_compensation_history comp2 ON tb2.table_id = comp2.id
            AND tb2.table_name = "bt_seat_compensation_history"

        LEFT JOIN bt_hcbb_history_detail hc2 ON tb2.table_id = hc2.id
            AND tb2.table_name = "bt_hcbb_history_detail"
        LEFT JOIN sys_user hc2u ON hc2u.code=hc2.ticket_price_diff_refund_user_code
        LEFT JOIN bt_channel hc2c ON hc2c.id=hc2u.channel_id
        LEFT JOIN bt_channel_category hc2cc ON hc2cc.id=hc2c.category_id

        LEFT JOIN bt_ticket_change_history cg2 ON tb2.ticket_no = cg2.ticket_no_new
            AND cg2.link_order_id=tb2.link_order_id
                    -- !ticket_no有重复,用order_id加强join唯一性,防止join重复陷阱
        LEFT JOIN bt_ticket_change_order cgo2 ON cg2.change_order_id = cgo2.id
    )
    SELECT * FROM cte
    WHERE cte.refund_amount IS NOT NULL
        OR cte.fee is NOT NULL
    ;
"#;

pub const UPDATE_RELATED_INFO: &str = r#"
    UPDATE ship_ticket_refund_bill trb
    SET trb.refund_method = (CASE trb.refund_method
            WHEN "1" THEN 'POS'
            WHEN "2" THEN '微信'
            WHEN "3" THEN '支付宝'
            WHEN "4" THEN '现金'
            WHEN "5" THEN '转账'
            WHEN "6" THEN '银联支付'
            WHEN "7" THEN '来游吧在线支付'
            WHEN "8" THEN '来游吧OTA预存款'
            WHEN "9" THEN '预存款'
            ELSE trb.refund_method
        END)
        ,trb.refund_amount = IFNULL(refund_amount,0)
        ,trb.fee = IFNULL(fee,0)
    ;
"#;

pub const UPDATE_TICKET_REFUND_INFO: &str = r#"
    WITH trb AS (
        SELECT  tb_id
            ,SUM( refund_amount ) refund_amount
            ,SUM( fee ) fee
        FROM ship_ticket_refund_bill
        GROUP BY  tb_id
    )
    UPDATE ticket_bill tb
    INNER JOIN trb
    ON trb.tb_id = tb.id

    SET
        tb.refund_amount=trb.refund_amount
        , tb.fee=trb.fee
    ;
"#;
