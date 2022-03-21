use database::entity;
use poem::{
    error::BadRequest,
    handler,
    web::{Json, Path},
    IntoResponse, Result,
};

#[handler]
pub async fn get(Path(_username): Path<String>) -> Result<impl IntoResponse> {
    let user = entity::user::get().await.map_err(BadRequest)?;
    Ok(Json(user))
}
