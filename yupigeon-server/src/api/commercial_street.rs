use database::query;
use poem::{error::BadRequest, handler, web::Json, IntoResponse, Result};

#[handler]
pub async fn bill() -> Result<impl IntoResponse> {
    query::commercial_street::bill()
        .await
        .map_err(BadRequest)
        .map(Json)
}

#[handler]
pub async fn rent_revenue() -> Result<impl IntoResponse> {
    query::commercial_street::rent_revenue()
        .await
        .map_err(BadRequest)
        .map(Json)
}
