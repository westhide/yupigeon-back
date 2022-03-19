// @Author: westhide.yzw
// @Date: 2022-02-22 12:43:02
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:43:02

mod api;
mod config;
mod service;

use poem::{listener::TcpListener, Server};

use crate::service::router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    };
    tracing_subscriber::fmt::init();

    let config = config::config().unwrap();
    let bind_ip: String = config.get("BIND_IP").unwrap();
    let bind_port: String = config.get("BIND_PORT").unwrap();
    let address = format!("{bind_ip}:{bind_port}");
    let app = router::generate();
    Server::new(TcpListener::bind(address)).run(app).await
}
