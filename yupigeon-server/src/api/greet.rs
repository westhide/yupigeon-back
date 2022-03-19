// @Author: westhide.yzw
// @Date: 2022-03-19 22:14:51
// @Last Modified by: westhide.yzw
// @Last Modified time: 2022-03-19 22:41:44

use poem::{handler, web::Path};

#[handler]
pub async fn greet(Path(path): Path<String>) -> String {
    format!("Hello: {path}!")
}
