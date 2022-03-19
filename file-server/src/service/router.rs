// @Author: westhide.yzw
// @Date: 2022-02-22 12:43:25
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:43:25

use poem::{
    get,
    middleware::{Compression, Tracing},
    EndpointExt, IntoEndpoint, Route,
};

use crate::api::{file, file_dir, greet};

pub fn generate() -> impl IntoEndpoint {
    Route::new()
        .at("/greet/:name", get(greet::greet))
        .nest("/file", get(file::file))
        .at("/file_dir", get(file_dir::file_dir))
        .with(Tracing)
        .with(Compression)
}
