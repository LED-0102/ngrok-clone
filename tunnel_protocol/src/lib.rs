pub enum MessageType {
    Connect,
    Disconnect,
    HTTPRequest,
    HTTPResponse,
    WebSocketRequest,
    WebSocketResponse,
    WebSocketMessage,
}

pub struct MessageProtocol {
    pub message_type: MessageType,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
