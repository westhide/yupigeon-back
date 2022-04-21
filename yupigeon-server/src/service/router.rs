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
        .at("/ship_ticket/bill", get(api::ship_ticket::bill))
        .at(
            "/ship_ticket/clients",
            get(api::ship_ticket::clients),
        )
        .at(
            "/ship_ticket/conductors",
            get(api::ship_ticket::conductors),
        )
        .at(
            "/ship_ticket/refresh_status",
            get(api::ship_ticket::refresh_status),
        )
        .at(
            "/ship_ticket/refresh",
            post(api::ship_ticket::refresh),
        )
        .at(
            "/ship_ticket/daily_sales",
            get(api::ship_ticket::daily_sales),
        )
        .at(
            "/ship_ticket/daily_receipt",
            get(api::ship_ticket::daily_receipt),
        )
        .at(
            "/ship_ticket/client_sales",
            post(api::ship_ticket::client_sales),
        )
        .at(
            "/ship_ticket/conductor_daily_receipt",
            post(api::ship_ticket::conductor_daily_receipt),
        )
        .at(
            "/ship_ticket/voucher_revenue",
            get(api::ship_ticket::voucher_revenue),
        )
        .at(
            "/tenpay/daily_receipt",
            get(api::tenpay::daily_receipt),
        )
        .at(
            "/finance_account/finance_accounts",
            get(api::finance_account::finance_accounts),
        )
        .at(
            "/finance_account/finance_account_info",
            get(api::finance_account::finance_account_info),
        )
        .at(
            "/finance_subsidiary/update_items",
            get(api::finance_subsidiary::update_items),
        )
        .at(
            "/finance_subsidiary/subsidiary_account",
            get(api::finance_subsidiary::subsidiary_account),
        )
        .at(
            "/finance_subsidiary/subsidiary_group_info",
            get(api::finance_subsidiary::subsidiary_group_info),
        )
        .at(
            "/finance_voucher/voucher_template",
            get(api::finance_voucher::voucher_template),
        )
        .at(
            "/finance_voucher/voucher_template_info",
            get(api::finance_voucher::voucher_template_info),
        )
        .at(
            "/finance_voucher/voucher_template_group",
            get(api::finance_voucher::voucher_template_group),
        )
        .with(Auth)
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
