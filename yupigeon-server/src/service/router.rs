// @Author: westhide.yzw
// @Date: 2022-03-19 22:14:38
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:14:38

use poem::{
    get,
    middleware::{Compression, Cors, Tracing},
    EndpointExt, IntoEndpoint, Route,
};

use crate::api;

pub fn generate() -> impl IntoEndpoint {
    Route::new()
        .at("/greet/:name", get(api::greet::get))
        .at("/user", get(api::user::get))
        .at("/ship_ticket_bill", get(api::ship_ticket_bill::get))
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
