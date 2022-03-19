// @Author: westhide.yzw
// @Date: 2022-02-22 12:43:02
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:43:02

mod api;
mod config;
mod service;
use crate::config::GLOBAL_CONFIG;
use crate::service::router;
use poem::{listener::TcpListener, Server};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    };
    tracing_subscriber::fmt::init();

    let bind_ip = GLOBAL_CONFIG
        .get::<String>("BIND_IP")
        .unwrap_or("127.0.0.1".into());
    let bind_port = GLOBAL_CONFIG
        .get::<String>("BIND_PORT")
        .unwrap_or("3301".into());

    let address = format!("{bind_ip}:{bind_port}");
    let app = router::generate();
    Server::new(TcpListener::bind(address)).run(app).await
}
