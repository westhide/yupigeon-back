use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel, DeriveActiveModelBehavior,
)]
#[serde(rename_all = "camelCase")]
#[sea_orm(table_name = "canyon_ticket_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    id: u32,
    #[sea_orm(unique)]
    name: String,
    price: Decimal,
    items: Option<Json>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Debug)]
pub struct Link2TicketTypeItem;

use super::canyon_link_ticket_type as link;

impl Linked for Link2TicketTypeItem {
    type FromEntity = Entity;
    type ToEntity = super::canyon_ticket_type_item::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            link::Relation::TicketType.def().rev(),
            link::Relation::TicketTypeItem.def(),
        ]
    }
}
