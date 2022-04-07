use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "ticket_bill")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing, skip_serializing)]
    id: i32,
    #[sea_orm(column_type = "BigInteger")]
    link_order_id: i64,
    channel_name: String,
    #[sea_orm(column_type = "TinyInteger")]
    serial_no: i8,
    ticket_status: String,
    line_name: String,
    create_time: DateTime,
    departure_datetime: DateTime,
    ship_name: String,
    ticket_type_name: String,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))")]
    ticket_price: Decimal,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    pay_amount: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    refund_amount: Option<Decimal>,
    #[sea_orm(column_type = "Decimal(Some((10, 3)))", nullable)]
    fee: Option<Decimal>,
    cabin_name: String,
    seat_memo: String,
    passenger_name: String,
    passenger_id_no: String,
    user_type: String,
    user_name: String,
    u8_user_type: Option<String>,
    u8_user_name: Option<String>,
    u8_nickname: Option<String>,
    u8_vip_pact: Option<String>,
    payment_method: Option<String>,
    payment_time: Option<DateTime>,
    pay_id: Option<String>,
    #[sea_orm(column_type = "BigInteger")]
    ticket_id: i64,
    link_ticket_id: String,
    #[sea_orm(column_type = "BigInteger")]
    ticket_no: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub async fn get(datetime_from: DateTime, datetime_end: DateTime) -> Result<Vec<Model>, DbErr> {
    let txn = crate::Database::new("laiu8").await?.txn;
    Entity::find()
        .filter(Column::DepartureDatetime.gte(datetime_from))
        .filter(Column::DepartureDatetime.lte(datetime_end))
        .all(&txn)
        .await
}

pub async fn refresh() -> Result<(), DbErr> {
    let database = crate::Database::new("laiu8").await?;
    use super::refresh_sql::{bt_ticket_info, laiu8_info, refund_info};
    database
        .execute_multi_sql(vec![
            bt_ticket_info::DROP_TABLE,
            bt_ticket_info::CREATE_TABLE,
            bt_ticket_info::INSERT_KEY_RECORD,
            bt_ticket_info::UPDATE_LINK_ID,
            bt_ticket_info::UPDATE_ORDER_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_TICKET_INFO,
            bt_ticket_info::UPDATE_PAY_AMOUNT,
            bt_ticket_info::UPDATE_DEPARTURE_INFO,
            bt_ticket_info::UPDATE_RELATED_INFO,
            bt_ticket_info::UPDATE_U8_TICKET_KEY,
            laiu8_info::DROP_TEMP_TABLE,
            laiu8_info::CREATE_TEMP_TABLE,
            laiu8_info::CREATE_INDEX_ID,
            laiu8_info::CREATE_INDEX_OLD_ID,
            laiu8_info::UPDATE_LAIU8_INFO,
            laiu8_info::UPDATE_EDGE_CASE,
            laiu8_info::UPDATE_RELATED_INFO,
            laiu8_info::UPDATE_MINI_PROGRAM_INFO,
            laiu8_info::UPDATE_MINI_PROGRAM_PAY_ID,
            refund_info::DELETE_TABLE,
            refund_info::INSERT_REFUND_RECORD,
            refund_info::INSERT_OTHER_RECORD,
            refund_info::UPDATE_RELATED_INFO,
            refund_info::UPDATE_TICKET_REFUND_INFO,
        ])
        .await?;
    database.txn.commit().await
}
