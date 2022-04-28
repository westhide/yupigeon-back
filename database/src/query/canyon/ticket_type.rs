use sea_orm::{entity::prelude::*, Set};

use crate::entity::{canyon_ticket_type as TicketType, canyon_ticket_type::Link2TicketTypeItem};

pub async fn update_ticket_type_items() -> Result<Vec<TicketType::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;
    let ticket_types = TicketType::Entity::find().all(&txn).await?;

    let mut results = vec![];

    for ticket_type in ticket_types {
        let items = ticket_type
            .find_linked(Link2TicketTypeItem)
            .all(&txn)
            .await?;
        let items_json = serde_json::json!(items);

        let mut ticket_type_active: TicketType::ActiveModel = ticket_type.into();
        ticket_type_active.items = Set(items_json.into());

        let result = ticket_type_active.update(&txn).await?;
        results.push(result);
    }

    txn.commit().await?;
    Ok(results)
}

pub async fn ticket_types() -> Result<Vec<TicketType::Model>, DbErr> {
    let txn = crate::Database::new("default").await?.txn;

    TicketType::Entity::find().all(&txn).await
}
