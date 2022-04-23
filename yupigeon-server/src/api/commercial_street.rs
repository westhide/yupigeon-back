use database::query;
use poem::{error::BadRequest, handler, web::Json, IntoResponse, Result};

#[handler]
pub async fn bill() -> Result<impl IntoResponse> {
    query::commercial_street::bill()
        .await
        .map_err(BadRequest)
        .map(Json)
}
