// @Author: westhide.yzw
// @Date: 2022-03-19 22:44:07
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:44:07

mod api;
mod config;
mod service;

use poem::{listener::TcpListener, Server};

use crate::{config::GLOBAL_CONFIG, service::router};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    };
    tracing_subscriber::fmt::init();

    database::init_database().await;

    let bind_ip = GLOBAL_CONFIG
        .get::<String>("BIND_HOST")
        .unwrap_or_else(|_| "127.0.0.1".into());
    let bind_port = GLOBAL_CONFIG
        .get::<String>("BIND_PORT")
        .unwrap_or_else(|_| "9901".into());

    let address = format!("{bind_ip}:{bind_port}");
    let app = router::generate();
    Server::new(TcpListener::bind(address)).run(app).await
}
