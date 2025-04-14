use dashmap::DashMap;
use tokio::sync::oneshot;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub(crate) struct RequestState {
    client_maps: Arc<DashMap<String, DashMap<String, oneshot::Sender<String>>>>,
}

impl RequestState {
    pub(crate) fn new() -> Self {
        Self {
            client_maps: Arc::new(DashMap::new()),
        }
    }

    pub fn setup_channel(&self, client_id: String, request_id: String) -> oneshot::Receiver<String> {
        let (tx, rx) = oneshot::channel();

        let client_map = self.client_maps.entry(client_id).or_insert_with(DashMap::new);
        client_map.insert(request_id, tx);

        rx
    }

    pub fn send_response(&self, client_id: &str, request_id: &str, response: String) -> Result<(), String> {
        if let Some(client_map) = self.client_maps.get(client_id) {
            if let Some(mut tx) = client_map.remove(request_id) {
                if tx.1.send(response).is_err() {
                    return Err("Failed to send response: receiver dropped.".into());
                }
                Ok(())
            } else {
                Err("Request ID not found.".into())
            }
        } else {
            Err("Client ID not found.".into())
        }
    }
    pub fn cleanup_channel(&self, client_id: &str, request_id: &str) {
        if let Some(client_map) = self.client_maps.get(client_id) {
            client_map.remove(request_id);
        }
    }
}


