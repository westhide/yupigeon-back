// @Author: westhide.yzw
// @Date: 2022-03-19 22:14:38
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:14:38

use poem::{
    get,
    middleware::{Compression, Tracing},
    EndpointExt, IntoEndpoint, Route,
};

use crate::api::{greet, ship_ticket_bill, user};

pub fn generate() -> impl IntoEndpoint {
    Route::new()
        .at("/greet/:name", get(greet::greet))
        .at("/user/:name", get(user::get))
        .at("/ship_ticket_bill/:name", get(ship_ticket_bill::get))
        .with(Tracing)
        .with(Compression)
}