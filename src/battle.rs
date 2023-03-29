use actix::{Actor, StreamHandler};
use actix_web::web::Payload;
use actix_web::{HttpRequest, HttpResponse};
use actix_web_actors::ws;
use bytestring::ByteString;
use serde::{Deserialize, Serialize};
use sunless_dawn_character::Character;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "params")]
pub enum Action {
    AuthenticateRequest { address: String },
    AuthenticateChallengeResponse { signature: String },
    ListPlayers {},
    ChallengePlayer { name: String },
    ListChallenges {},
    AcceptChallenge { name: String },
    ListBattles {},
    Chat { recipient: String, message: String },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "response", content = "data")]
pub enum Response {
    AuthenticateChallenge { message: String },
}

struct PlayerWS;

impl PlayerWS {
    pub fn route_message(&mut self, message: &str, ctx: &mut <PlayerWS as Actor>::Context) {
        let action_result = serde_json::from_str::<Action>(message);
        match action_result {
            Ok(action) => match action {
                Action::Chat { recipient, message } => self.chat(&recipient, &message),
                _ => ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Unsupported,
                    description: Some(String::from("Not Implemented")),
                })),
            },
            Err(e) => ctx.close(Some(ws::CloseReason {
                code: ws::CloseCode::Invalid,
                description: Some(e.to_string()),
            })),
        }
    }

    pub fn chat(&self, recipient: &String, message: &String) {
        //self.server.
    }

    pub fn new_character(&mut self, ctx: &mut <PlayerWS as Actor>::Context) {
        let c = Character::random("New character");
        let json = serde_json::to_string(&c).unwrap(); // TODO: actually handle the error
        let jsonb: ByteString = json.into();
        ctx.text(jsonb);
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
            Ok(ws::Message::Binary(bin)) => ctx.close(Some(ws::CloseCode::Unsupported.into())),
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
