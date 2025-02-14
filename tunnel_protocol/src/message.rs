use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg: String,
    pub id: String
}

impl actix::Message for Message {
    type Result = ();
}