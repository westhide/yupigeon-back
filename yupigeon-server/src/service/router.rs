// @Author: westhide.yzw
// @Date: 2022-03-19 22:14:38
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:14:38

use poem::{
    get,
    middleware::{Compression, Cors, Tracing},
    post, EndpointExt, IntoEndpoint, Route,
};

use crate::api;

pub fn generate() -> impl IntoEndpoint {
    Route::new()
        .at("/greet/:name", get(api::greet::get))
        .at("/user", get(api::user::get))
        .at("/ship_ticket_bill", get(api::ship_ticket_bill::get))
        .at(
            "/ship_ticket_bill/refresh_status",
            get(api::ship_ticket_bill::refresh_status),
        )
        .at(
            "/ship_ticket_bill/refresh",
            post(api::ship_ticket_bill::refresh),
        )
        .at(
            "/ship_ticket_bill/daily_sales",
            get(api::ship_ticket_bill::daily_sales),
        )
        .at(
            "/ship_ticket_bill/client_sales",
            get(api::ship_ticket_bill::client_sales),
        )
        .at(
            "/ship_ticket_bill/offline_conductor_daily_receipt",
            get(api::ship_ticket_bill::offline_conductor_daily_receipt),
        )
        .at("/login", get(api::login::get))
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
