use actix::prelude::*;

use actix_web::web::Payload;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use bytestring::ByteString;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use sunless_dawn_character::Character;

use log::info;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "chat")]
pub struct ChatMessage {
    sender: String,
    recipient: String,
    message: String,
    timestamp: chrono::DateTime<Utc>,
}

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

pub type ChatMessages = HashMap<String, ChatMessage>;

pub struct PlayerClient {}

#[derive(Clone)]
pub struct PlayerWS {
    addr: Option<Addr<PlayerWS>>,
    clients: Arc<Mutex<Vec<Arc<PlayerWS>>>>,
    //data: web::Data<Mutex<crate::ServerData>>,
    address: String,
    chat: ChatMessages,
}

impl PlayerWS {
    fn new(clients: Arc<Mutex<Vec<Arc<PlayerWS>>>>) -> Self {
        Self {
            addr: None,
            clients,
            address: String::from(""),
            chat: ChatMessages::new(),
        }
    }

    pub fn route_message(&mut self, message: &str, ctx: &mut <PlayerWS as Actor>::Context) {
        let result = serde_json::from_str::<Action>(message);
        match result {
            Ok(action) => match action {
                Action::AuthenticateRequest { address } => self.authenticate(&address),
                Action::ListPlayers {} => self.list_players(ctx),
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

    pub fn authenticate(&mut self, address: &String) {
        // TODO: check a signature
        self.address = address.clone();
    }

    pub fn list_players(&self, ctx: &mut <PlayerWS as Actor>::Context) {
        let clients = self.clients.lock().unwrap();

        for client in clients.iter() {
            client.do_send(ws::Message::Text(message.to_string())).unwrap();
        }
        /*let unlocked_data = self.data.lock().unwrap();

        let users: Vec<String> = unlocked_data.keys().cloned().collect();

        let json = serde_json::to_string(&users).unwrap();
        ctx.text(json);
        */
    }

    pub fn chat(&mut self, recipient: &String, message: &String) {
        /*let msg = ChatMessage {
            sender: self.address.clone(),
            recipient: recipient.clone(),
            message: message.clone(),
            timestamp: chrono::offset::Utc::now(),
        };

        // store a copy of this message in our chat history
        self.chat.insert(msg.recipient.clone(), msg.clone());
        let mut unlocked_data = self.data.lock().unwrap(); // TODO: error handling

        match unlocked_data.get_mut(recipient) {
            Some(player) => player.receive_chat(msg.clone()),
            None => (),
        }*/
    }

    pub fn receive_chat(&mut self, message: ChatMessage) {
        //self.chat.insert(message.sender.clone(), message.clone());
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

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        info!("PlayerWS connected {addr:?}");

        // Register the WebSocket actor's address in the shared state
        self.clients.lock().unwrap().push(addr.clone());

        // Set the WebSocket actor's address
        self.addr = Some(addr);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        if let Some(addr) = self.addr.take() {
            info!("PlayerWS disconnected {addr:?}");
            let mut clients = self.clients.lock().unwrap();
            if let Some(pos) = clients.iter().position(|client| *client.addr == addr) {
                clients.remove(pos);
            }
        }
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for PlayerWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(msg)) => (),
            Ok(ws::Message::Text(text)) => self.route_message(&text, ctx),
            Ok(ws::Message::Binary(_)) => ctx.close(Some(ws::CloseCode::Unsupported.into())),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            Ok(ws::Message::Continuation(_)) => ctx.close(Some(ws::CloseCode::Unsupported.into())),
            Ok(ws::Message::Nop) => (),
            Err(_) => ctx.close(Some(ws::CloseCode::Protocol.into())),
        }
    }
}

pub async fn player_ws_route(
    req: HttpRequest,
    stream: Payload,
    clients: web::Data<Arc<Mutex<Vec<Addr<PlayerWS>>>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let player_ws = PlayerWS::new(clients.get_ref().clone());
    ws::start(
        player_ws,
        &req,
        stream,
    )
}
