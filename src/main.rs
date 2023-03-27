extern crate env_logger;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use sunless_dawn_character::{Character, Class, EyeColor, HairColor, Sex, SkinColor};

mod health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!(
        "Starting {} version {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/health").configure(health::config))
    })
    .bind(("127.0.0.1", 8080))? // TODO: add --bind and --port command line flags
    .run()
    .await
}
