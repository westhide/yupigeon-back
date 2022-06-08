use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_link_ticket_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    ticket_type_id: u32,
    ticket_type_item_id: u32,
    #[sea_orm(default_value = 1)]
    ticket_item_num: i32,
}

#[derive(Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::canyon_ticket_type::Entity",
        from = "Column::TicketTypeId",
        to = "super::canyon_ticket_type::Column::Id"
    )]
    TicketType,
    #[sea_orm(
        belongs_to = "super::canyon_ticket_type_item::Entity",
        from = "Column::TicketTypeItemId",
        to = "super::canyon_ticket_type_item::Column::Id"
    )]
    TicketTypeItem,
}
