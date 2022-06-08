use database::mysql::{
    entity::{
        canyon_daily_sales_append as DailySalesAppend,
        canyon_offline_ticket_bill as OfflineTicketBill,
        canyon_online_ticket_bill as OnlineTicketBill,
    },
    query::{self, common::QueryTrait},
};
use poem::{
    handler,
    web::{Json, Query},
    IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::service::{
    error::Result,
    params::{DateTimeParams, ParseDateTimeParams},
    response::{Response, ResponseTrait},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketData {
    offline_tickets: Option<Vec<OfflineTicketBill::Model>>,
    online_tickets: Option<Vec<OnlineTicketBill::Model>>,
}

#[handler]
pub async fn upload_ticket_data(Json(params): Json<TicketData>) -> Result<impl IntoResponse> {
    let TicketData {
        offline_tickets,
        online_tickets,
    } = params;

    if let Some(offline_tickets) = offline_tickets {
        OfflineTicketBill::Entity::insert_many(offline_tickets).await?;
    }

    if let Some(online_tickets) = online_tickets {
        OnlineTicketBill::Entity::insert_many(online_tickets).await?;
    }

    Response::message("导入成功")
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceDailySalesAppend {
    append_data: Vec<DailySalesAppend::Model>,
}

#[handler]
pub async fn replace_daily_sales_append(
    Json(params): Json<ReplaceDailySalesAppend>,
) -> Result<impl IntoResponse> {
    let ReplaceDailySalesAppend { append_data } = params;

    DailySalesAppend::Entity::replace_many(append_data).await?;

    Response::message("录入成功")
}

#[handler]
pub async fn update_ticket_type_items() -> Result<impl IntoResponse> {
    let res = query::canyon::update_ticket_type_items().await?;

    Response::json(res)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketTypeParams {
    scope: Option<String>,
}

#[handler]
pub async fn ticket_types(Query(params): Query<TicketTypeParams>) -> Result<impl IntoResponse> {
    let TicketTypeParams { scope } = params;

    let res = query::canyon::ticket_types(scope.as_deref()).await?;

    Response::json(res)
}

#[derive(Deserialize)]
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

    let res = query::canyon::daily_sales(begin_time, end_time, &where_condition).await?;

    Response::json(res)
}

#[handler]
pub async fn daily_sales_appends(
    Query(params): Query<DateTimeParams>,
) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    let res = query::canyon::daily_sales_appends(begin_time, end_time).await?;

    Response::json(res)
}

#[handler]
pub async fn delete_ticket_bill(Query(params): Query<DateTimeParams>) -> Result<impl IntoResponse> {
    let (begin_time, end_time) = params.get_datetime_params()?;

    query::canyon::delete_ticket_bill(begin_time, end_time).await?;

    Response::message("删除成功")
}

#[handler]
pub async fn operators() -> Result<impl IntoResponse> {
    let res = query::canyon::operators().await?;

    Response::json(res)
}

#[handler]
pub async fn clients() -> Result<impl IntoResponse> {
    let res = query::canyon::clients().await?;

    Response::json(res)
}
