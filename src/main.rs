extern crate env_logger;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use std::sync::Mutex;
use std::collections::HashMap;

mod battle;
mod health;

pub type ServerData = HashMap<String, battle::PlayerWS>;
/*
pub struct ServerData {
    pub users
    pub users: []String,
    pub
}
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!(
        "Starting {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let data = Data::new(Mutex::new(ServerData::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::clone(&data))
            .wrap(Logger::default())
            .service(web::scope("/health").configure(health::config))
            .route("/ws/", web::get().to(battle::player_ws_route))
    })
    .bind(("127.0.0.1", 8080))? // TODO: add --bind and --port command line flags
    .run()
    .await
}
