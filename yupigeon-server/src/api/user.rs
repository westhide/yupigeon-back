use poem::{handler, web::Path};

#[handler]
pub async fn user(Path(_path): Path<String>) -> String {
    // let user = database::user::select("1").await.unwrap();
    // format!("Hello: {:?}!", user)
    format!("Hello")
}
