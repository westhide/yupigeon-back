// @Author: westhide.yzw
// @Date: 2022-03-19 22:44:07
// @Last Modified by:   westhide.yzw
// @Last Modified time: 2022-03-19 22:44:07

mod api;
mod config;
mod service;

use poem::{listener::TcpListener, Server};

use crate::{
    config::get_config,
    service::{
        error::{Result, WrapError},
        global_data, router,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug,sea_orm=debug");
    };
    tracing_subscriber::fmt::init();

    global_data::init_global_data()?;

    database::mysql::Database::init().await?;

    database::mongo::MongoPool::init().await?;

    database::oracle::base::test_oracle().await?;

    let bind_ip = get_config("BIND_HOST")?;
    let bind_port = get_config("BIND_PORT")?;

    let address = format!("{}:{}", bind_ip, bind_port);
    let app = router::generate();
    Server::new(TcpListener::bind(address))
        .run(app)
        .await
        .map_err(|e| WrapError::Message(e.to_string()))
}
