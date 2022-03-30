// @Author: westhide.yzw
// @Date: 2022-03-19 22:44:07
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:44:07

mod api;
mod config;
mod service;

pub use global_data::GLOBAL_DATA;
use poem::{listener::TcpListener, Server};

use crate::{
    config::GLOBAL_CONFIG,
    service::{global_data, router},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,sea_orm=debug");
    };
    tracing_subscriber::fmt::init();

    global_data::init_global_data();

    database::init_database()
        .await
        .expect("Database init failed");

    let bind_ip = GLOBAL_CONFIG
        .get::<String>("BIND_HOST")
        .unwrap_or_else(|_| "127.0.0.1".into());
    let bind_port = GLOBAL_CONFIG
        .get::<String>("BIND_PORT")
        .unwrap_or_else(|_| "9901".into());

    let address = format!("{}:{}", bind_ip, bind_port);
    let app = router::generate();
    Server::new(TcpListener::bind(address)).run(app).await
}
