// @Author: westhide.yzw
// @Date: 2022-02-22 12:44:39
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:44:39

use poem::{handler, web::Path};

#[handler]
pub async fn greet(Path(path): Path<String>) -> String {
    format!("Hello: {}!", path)
}
