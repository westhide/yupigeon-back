use database::{
    entity::{canyon_daily_sales_append, canyon_offline_ticket_bill, canyon_online_ticket_bill},
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailySalesAppend {
    append_data: Vec<canyon_daily_sales_append::Model>,
}

#[handler]
pub async fn upload_daily_sales_append(
    Json(params): Json<DailySalesAppend>,
) -> Result<impl IntoResponse> {
    let DailySalesAppend { append_data } = params;

    query::canyon::insert_many::<
        canyon_daily_sales_append::Entity,
        canyon_daily_sales_append::ActiveModel,
    >(append_data)
    .await
    .map_err(BadRequest)?;

    Response::<String>::new(None, "录入成功")
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

    query::canyon::ticket_types(scope.as_deref())
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
        .unwrap_or_else(|| "".into());

    query::canyon::daily_sales(begin_time, end_time, &where_condition)
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn daily_sales_appends(
    Query(params): Query<DateTimeParams>,
) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::canyon::daily_sales_appends(begin_time, end_time)
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

#[handler]
pub async fn clients() -> Result<impl IntoResponse> {
    query::canyon::clients().await.map_err(BadRequest).map(Json)
}
