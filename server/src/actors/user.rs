use crate::client::{ClientActor, ConnectUser, DisconnectUser, UserMessage};
use actix::prelude::*;
use actix_web_actors::ws;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const USER_TIMEOUT: Duration = Duration::from_secs(10);

pub struct UserActor {
    pub hb: Instant,
    pub user_id: String,
    pub client: Addr<ClientActor>,
}

impl UserActor {
    pub fn new(user_id: String, client: Addr<ClientActor>) -> Self {
        UserActor {
            hb: Instant::now(),
            user_id,
            client,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > USER_TIMEOUT {
                println!("User heartbeat failed, disconnecting!");

                act.client.do_send(DisconnectUser {
                    user_id: act.user_id.clone(),
                });

                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for UserActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.client
            .send(ConnectUser {
                user_id: self.user_id.clone(),
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
        self.client.do_send(DisconnectUser {
            user_id: self.user_id.clone(),
        });
        Running::Stop
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for UserActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
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
                println!("User sent: {}", text);
                self.client.do_send(UserMessage {
                    user_id: self.user_id.clone(),
                    message: text.to_string(),
                });
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
