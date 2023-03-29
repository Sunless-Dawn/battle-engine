use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Payload;
use actix_web_actors::ws;
//use sunless_dawn_character::Character;

/*
pub struct Battle {
  pub id:u64,
  pub player1:Character,
  pub player2:Character,
}
*/

struct PlayerWS;

impl PlayerWS {
    pub fn route_message(&mut self, message:&str, ctx: &mut <PlayerWS as Actor>::Context) {
        match message {
            "/list\n" => ctx.text("list of stuff"),
            "/help\n" => ctx.text("help command"),
            _ => ctx.text("unknown command"),
        }
    }
}

impl Actor for PlayerWS {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => self.route_message(&text, ctx),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

pub async fn player_ws_route(
    req: HttpRequest,
    stream: Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(PlayerWS {}, &req, stream)
}
