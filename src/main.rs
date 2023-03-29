extern crate env_logger;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use log::info;
//use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use sunless_dawn_character::{Character, Class, EyeColor, HairColor, Sex, SkinColor};

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

    // just suppressing unused import warnings
    {
        let _ = Data::new(Mutex::new(Character::new("Name", Class::Mercenary, Sex::Male, HairColor::Black, EyeColor::Brown, SkinColor::Dark)));
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/health").configure(health::config))
            .route("/ws/", web::get().to(battle::player_ws_route))
    })
    .bind(("127.0.0.1", 8080))? // TODO: add --bind and --port command line flags
    .run()
    .await
}
