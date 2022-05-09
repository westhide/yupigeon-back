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
            "/ship_ticket/refund_bill",
            post(api::ship_ticket::refund_bill),
        )
        .at("/ship_ticket/clients", get(api::ship_ticket::clients))
        .at("/ship_ticket/conductors", get(api::ship_ticket::conductors))
        .at(
            "/ship_ticket/refresh_status",
            get(api::ship_ticket::refresh_status),
        )
        .at("/ship_ticket/refresh", post(api::ship_ticket::refresh))
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
            "/ship_ticket/ticket_revenue",
            get(api::ship_ticket::ticket_revenue),
        )
        .at(
            "/ship_ticket/fee_revenue",
            get(api::ship_ticket::fee_revenue),
        )
        .at("/tenpay/daily_receipt", get(api::tenpay::daily_receipt))
        .at(
            "/finance/finance_accounts",
            get(api::finance::finance_accounts),
        )
        .at(
            "/finance/finance_account_info",
            get(api::finance::finance_account_info),
        )
        .at(
            "/finance/update_subsidiary_account_items",
            get(api::finance::update_subsidiary_account_items),
        )
        .at(
            "/finance/subsidiary_account",
            get(api::finance::subsidiary_account),
        )
        .at(
            "/finance/subsidiary_group_info",
            get(api::finance::subsidiary_group_info),
        )
        .at(
            "/finance/voucher_template",
            get(api::finance::voucher_template),
        )
        .at(
            "/finance/voucher_template_info",
            get(api::finance::voucher_template_info),
        )
        .at(
            "/finance/voucher_template_group",
            get(api::finance::voucher_template_group),
        )
        .at("/mapper/domain_value", get(api::mapper::domain_value))
        .at("/commercial_street/bill", get(api::commercial_street::bill))
        .at(
            "/commercial_street/rent_revenue",
            get(api::commercial_street::rent_revenue),
        )
        .at(
            "/canyon/update_ticket_type_items",
            get(api::canyon::update_ticket_type_items),
        )
        .at("/canyon/ticket_types", get(api::canyon::ticket_types))
        .at(
            "/canyon/upload_ticket_data",
            post(api::canyon::upload_ticket_data),
        )
        .at(
            "/canyon/replace_daily_sales_append",
            post(api::canyon::replace_daily_sales_append),
        )
        .at("/canyon/daily_sales", post(api::canyon::daily_sales))
        .at(
            "/canyon/daily_sales_appends",
            get(api::canyon::daily_sales_appends),
        )
        .at(
            "/canyon/delete_ticket_bill",
            get(api::canyon::delete_ticket_bill),
        )
        .at("/canyon/operators", get(api::canyon::operators))
        .at("/canyon/clients", get(api::canyon::clients))
        .at("/mongo/collection_names", get(api::mongo::collection_names))
        .at(
            "/mongo/organization/insert_organization_company",
            post(api::mongo_organization::insert_organization_company),
        )
        .at(
            "/mongo/organization/insert_organization_group",
            post(api::mongo_organization::insert_organization_group),
        )
        .at(
            "/mongo/organization/organization_company",
            get(api::mongo_organization::organization_company),
        )
        .at(
            "/mongo/finance/update_assist_account_items",
            post(api::mongo_finance::update_assist_account_items),
        )
        .at(
            "/mongo/finance/assist_account_info",
            get(api::mongo_finance::assist_account_info),
        )
        .at(
            "/mongo/finance/assist_account_group_info",
            get(api::mongo_finance::assist_account_group_info),
        )
        .at(
            "/mongo/finance/finance_account_info",
            get(api::mongo_finance::finance_account_info),
        )
        .at(
            "/mongo/finance/insert_finance_voucher_template",
            post(api::mongo_finance::insert_finance_voucher_template),
        )
        .at(
            "/mongo/finance/voucher_template_info",
            get(api::mongo_finance::voucher_template_info),
        )
        .with(Auth)
        .with(Auth)
        .with(Tracing)
        .with(Compression)
        .with(Cors::new())
}
