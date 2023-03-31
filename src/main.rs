extern crate env_logger;

use actix::prelude::*;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

mod battle;
mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!(
        "Starting {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let clients = Arc::new(Mutex::new(Vec::<Addr<battle::PlayerWS>>::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(clients.clone()))
            .wrap(Logger::default())
            .service(web::scope("/health").configure(health::config))
            .route("/ws/", web::get().to(battle::player_ws_route))
    })
    .bind(("127.0.0.1", 8080))? // TODO: add --bind and --port command line flags
    .run()
    .await
}
