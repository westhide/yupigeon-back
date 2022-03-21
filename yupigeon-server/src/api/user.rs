use database::entity;
use poem::{
    handler,
    web::{Json, Path},
};

#[handler]
pub async fn get(Path(_username): Path<String>) -> Json<Option<entity::user::Model>> {
    let user = entity::user::get().await;
    Json(user)
}
