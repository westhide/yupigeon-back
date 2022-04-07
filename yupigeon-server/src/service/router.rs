// @Author: westhide.yzw
// @Date: 2022-03-19 22:14:38
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:14:38

use poem::{
    get,
    middleware::{Compression, Cors, Tracing},
    post, EndpointExt, IntoEndpoint, Route,
};

use super::auth::Auth;
use crate::api;

pub fn generate() -> impl IntoEndpoint {
    Route::new()
        .at("/greet/:name", get(api::greet::get))
        .at("/login", post(api::login::post))
        .at("/user", get(api::user::get))
        .at("/ship_ticket_bill", get(api::ship_ticket_bill::get))
        .at(
            "/ship_ticket_bill/clients",
            get(api::ship_ticket_bill::clients),
        )
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
            "/ship_ticket_bill/daily_receipt",
            get(api::ship_ticket_bill::daily_receipt),
        )
        .at(
            "/ship_ticket_bill/client_sales",
            get(api::ship_ticket_bill::client_sales),
        )
        .at(
            "/ship_ticket_bill/offline_conductor_daily_receipt",
            get(api::ship_ticket_bill::offline_conductor_daily_receipt),
        )
        .at(
            "/tenpay_bill/daily_receipt",
            get(api::tenpay_bill::daily_receipt),
        )
        .with(Auth)
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
