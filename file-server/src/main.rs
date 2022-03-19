// @Author: westhide.yzw
// @Date: 2022-02-22 12:43:02
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-02-22 12:43:02

mod api;
mod service;

use poem::{listener::TcpListener, Server};

use crate::service::router;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    };
    tracing_subscriber::fmt::init();
    std::env::set_var("BIND_ADDRESS", "poem=debug");
    println!("{:?}",std::env::var_os("BIND_ADDRESS"));
    let app = router::generate();
    Server::new(TcpListener::bind("0.0.0.0:3301"))
        .run(app)
        .await
}
