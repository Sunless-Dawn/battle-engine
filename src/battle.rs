use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::Payload;
use actix_web_actors::ws;
use futures::StreamExt;
use sunless_dawn_character::{Character, Class, EyeColor, HairColor, Sex, SkinColor};

pub struct Battle {
  pub id:u64,
  pub player1:Character,
  pub player2:Character,
}

struct Echo;

impl Actor for Echo {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Echo {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

pub async fn websocket_route(
    req: HttpRequest,
    stream: Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(Echo {}, &req, stream)
}
