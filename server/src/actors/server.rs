use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::actors::client::ClientActor;
use crate::request_manager::RequestState;
use actix::prelude::*;
use actix_web::web;
use rand::rngs::ThreadRng;
use serde::Serialize;
use tokio::sync::oneshot;

#[derive(actix::Message, Serialize)]
#[rtype(result = "()")]
pub struct Message {
    pub msg: String,
    pub id: String
}

#[derive(actix::Message)]
#[rtype(usize)]
pub struct Connect {
    pub username: String,
    pub addr: Addr<ClientActor>,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub username: String,
}

#[derive(actix::Message)]
#[rtype(bool)]
pub struct CheckKey {
    pub key: String,
}

pub struct ClientAddr {
    pub client_id: String,
}

impl actix::Message for ClientAddr {
    type Result = Option<Addr<ClientActor>>;
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub client_id: String,
    pub message: String,
    pub request_id: String,
}

pub struct AddRequest {
    pub request_id: String,
    pub client_id: String,
    pub request_state: web::Data<RequestState>,
}

impl actix::Message for AddRequest {
    type Result = Option<oneshot::Receiver<String>>;
}

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<String, Addr<ClientActor>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {

        ChatServer {
            sessions: HashMap::new(),
            rng: rand::rng(),
            visitor_count,
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        let id = msg.username;
        self.sessions.insert(id, msg.addr);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);

        // send id back
        count
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");

        // remove address
        self.sessions.remove(&msg.username);
    }
}

impl Handler<CheckKey> for ChatServer {
    type Result = bool;

    fn handle(&mut self, msg: CheckKey, _: &mut Self::Context) -> Self::Result {
        let key = msg.key;
        let result = self.sessions.contains_key(&key);

        result
    }
}

impl Handler<ClientAddr> for ChatServer {
    type Result = Option<Addr<ClientActor>>;

    fn handle(&mut self, msg: ClientAddr, _: &mut Self::Context) -> Self::Result {
        let addr = self.sessions.get(&msg.client_id);

        match addr {
            Some(addr) => Some(addr.clone()),
            None => None,
        }
    }
}

impl Handler<AddRequest> for ChatServer {
    type Result = Option<oneshot::Receiver<String>>;

    fn handle(&mut self, msg: AddRequest, _: &mut Self::Context) -> Self::Result {
        if self.sessions.contains_key(&msg.client_id) {
            let rx = msg.request_state.setup_channel(msg.client_id.clone(), msg.request_id.clone());

            Some(rx)
        } else {
            None
        }
    }
}

impl Handler<SendMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(addr) = self.sessions.get(&msg.client_id) {
            addr.do_send(Message {
                msg: msg.message,
                id: msg.request_id,
            });
        }
    }
}