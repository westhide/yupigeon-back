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
        .at("/ship_ticket_bill/bill", get(api::ship_ticket_bill::bill))
        .at(
            "/ship_ticket_bill/clients",
            get(api::ship_ticket_bill::clients),
        )
        .at(
            "/ship_ticket_bill/conductors",
            get(api::ship_ticket_bill::conductors),
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
            post(api::ship_ticket_bill::client_sales),
        )
        .at(
            "/ship_ticket_bill/conductor_daily_receipt",
            post(api::ship_ticket_bill::conductor_daily_receipt),
        )
        .at(
            "/tenpay_bill/daily_receipt",
            get(api::tenpay_bill::daily_receipt),
        )
        .at("/finance_account", get(api::finance_account::get))
        .at(
            "/finance_subsidiary/update_items",
            get(api::finance_subsidiary::update_items),
        )
        .at(
            "/finance_subsidiary/subsidiary_account",
            get(api::finance_subsidiary::subsidiary_account),
        )
        .at(
            "/finance_subsidiary/subsidiary_group",
            get(api::finance_subsidiary::subsidiary_group),
        )
        .at(
            "/finance_voucher/voucher_template",
            get(api::finance_voucher::voucher_template),
        )
        .with(Auth)
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
