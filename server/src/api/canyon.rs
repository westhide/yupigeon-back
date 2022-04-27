use database::{
    entity::{canyon_offline_ticket_bill, canyon_online_ticket_bill},
    query,
};
use poem::{error::BadRequest, handler, web::Json, IntoResponse, Result};
use serde::Deserialize;

use crate::service::utils::Response;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketData {
    offline_tickets: Option<Vec<canyon_offline_ticket_bill::Model>>,
    online_tickets: Option<Vec<canyon_online_ticket_bill::Model>>,
}

#[handler]
pub async fn upload_ticket_data(Json(params): Json<TicketData>) -> Result<impl IntoResponse> {
    let TicketData {
        offline_tickets,
        online_tickets,
    } = params;

    if let Some(offline_tickets) = offline_tickets {
        query::canyon::insert_many::<
            canyon_offline_ticket_bill::Entity,
            canyon_offline_ticket_bill::ActiveModel,
        >(offline_tickets)
        .await
        .map_err(BadRequest)?;
    }

    if let Some(online_tickets) = online_tickets {
        query::canyon::insert_many::<
            canyon_online_ticket_bill::Entity,
            canyon_online_ticket_bill::ActiveModel,
        >(online_tickets)
        .await
        .map_err(BadRequest)?;
    }

    Response::<String>::new(None, "导入成功")
}
