use database::{
    entity::{canyon_offline_ticket_bill, canyon_online_ticket_bill},
    query,
};
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Query},
    IntoResponse, Result,
};
use serde::Deserialize;

use crate::service::utils::{DateTimeParams, ParseDateTimeParams, Response};

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

#[handler]
pub async fn update_ticket_type_items() -> Result<impl IntoResponse> {
    query::canyon::update_ticket_type_items()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketTypeParams {
    scope: Option<String>,
}

#[handler]
pub async fn ticket_types(Query(params): Query<TicketTypeParams>) -> Result<impl IntoResponse> {
    let TicketTypeParams { scope } = params;

    query::canyon::ticket_types(scope.as_ref().map(String::as_str))
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySalesParams {
    #[serde(flatten)]
    datetime_params: DateTimeParams,
    where_condition: Option<String>,
}

#[handler]
pub async fn daily_sales(Json(params): Json<DailySalesParams>) -> Result<impl IntoResponse> {
    let DailySalesParams {
        datetime_params,
        where_condition,
    } = params;

    let (begin_time, end_time) = datetime_params.get_datetime_params()?;

    let where_condition = where_condition
        .map(|s| format!(" AND {}", s))
        .unwrap_or("".to_string());

    query::canyon::daily_sales(begin_time, end_time, &where_condition)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn operators() -> Result<impl IntoResponse> {
    query::canyon::operators()
        .await
        .map_err(BadRequest)
        .map(Json)
}
