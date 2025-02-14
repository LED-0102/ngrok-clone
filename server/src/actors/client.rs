use actix::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use actix_web::web;
use actix_web_actors::ws;
use crate::actors::user::UserActor;
use crate::server::{ChatServer, Connect, Disconnect};
use tunnel_protocol::message::Message;
use tunnel_protocol::MessageProtocol;
use crate::GLOBAL_REQUEST_STATE;
use crate::request_manager::RequestState;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);


#[derive(Debug)]
pub struct ClientActor {
    pub hb: Instant,
    pub client_id: String,
    pub addr: Addr<ChatServer>,
    pub user_sessions: HashMap<String, Addr<UserActor>>,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct ConnectUser {
    pub user_id: String,
    pub addr: Addr<UserActor>,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct DisconnectUser {
    pub user_id: String,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct UserMessage {
    pub user_id: String,
    pub message: String,
}

impl ClientActor {
    pub fn new(client_id: String, addr: Addr<ChatServer>) -> Self {
        ClientActor {
            hb: Instant::now(),
            client_id,
            addr,
            user_sessions: HashMap::new(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Client heartbeat failed, disconnecting!");
                act.addr.do_send(Disconnect {
                    username: act.client_id.clone(),
                });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl Actor for ClientActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                username: self.client_id.clone(),
                addr,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                if res.is_err() {
                    ctx.stop();
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect {
            username: self.client_id.clone(),
        });
        Running::Stop
    }
}

impl Handler<ConnectUser> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: ConnectUser, _: &mut Self::Context) {
        println!("User connected to client: {}", msg.user_id);
        self.user_sessions.insert(msg.user_id, msg.addr);
    }
}

impl Handler<DisconnectUser> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: DisconnectUser, _: &mut Self::Context) {
        println!("User disconnected from client: {}", msg.user_id);
        self.user_sessions.remove(&msg.user_id);
    }
}

impl Handler<UserMessage> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: UserMessage, _: &mut Self::Context) {
        println!("Message from user {}: {}", msg.user_id, msg.message);
        // Forward or process the user message as needed
    }
}

impl Handler<Message> for ClientActor {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        let msg = serde_json::to_string(&msg).unwrap();
        ctx.text(msg);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ClientActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                println!("protocol error {}", e.to_string());
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let msg: Message = match serde_json::from_str(&text) {
                    Ok(msg) => msg,
                    Err(e) => {
                        println!("Failed to parse message: {}", e.to_string());
                        return;
                    }
                };
                let client_id = self.client_id.clone();
                println!("Client sent: {}", &msg.msg);
                request_handler(msg, &client_id);
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

pub fn request_handler (msg: Message, client_id: &str) {
    let msg_protocol: MessageProtocol = serde_json::from_str(&msg.msg).unwrap();
    let id = msg.id;

    match msg_protocol {
        MessageProtocol::HTTPRequest(_) => {}
        MessageProtocol::HTTPResponse(res) => {
            GLOBAL_REQUEST_STATE.send_response(client_id, &id, serde_json::to_string(&res).unwrap()).unwrap_or_else(|e| {
                eprintln!("Failed to send response: {}", e);
            });
        }
        MessageProtocol::WebSocketMessage => {}
        MessageProtocol::WebSocketConnect => {}
    }
}
