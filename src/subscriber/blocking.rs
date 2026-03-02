use tungstenite::{
    client::connect, client::ClientRequestBuilder, protocol::WebSocket, stream::MaybeTlsStream,
};
use url::Url;

use super::builder::SubscriberBuilder;

use crate::auth::Auth;
use crate::error::Error;
use crate::payload::ReceivedPayload;

/// Blocking subscriber
#[derive(Debug, Clone)]
pub struct Blocking {
    auth: Option<Auth>,
}

impl Blocking {
    #[inline]
    pub(crate) fn new(builder: SubscriberBuilder) -> Result<Self, Error> {
        Ok(Self { auth: builder.auth })
    }

    pub(crate) fn subscribe(&self, url: &Url, topic: String) -> Result<MessageIterator, Error> {
        let mut ws_url = url.clone();
        let _ = match url.scheme() {
            "https" => ws_url.set_scheme("wss"),
            _ => ws_url.set_scheme("ws"),
        };
        ws_url.set_path(&format!("{topic}/ws"));
        let uri = ws_url.to_string().parse::<http::Uri>().unwrap();

        // Build request
        let builder = match &self.auth {
            Some(auth) => ClientRequestBuilder::new(uri)
                .with_header("Authorization", auth.clone().to_header_value()),
            None => ClientRequestBuilder::new(uri),
        };

        // Create message iterator
        Ok(MessageIterator {
            socket: connect(builder)?.0,
        })
    }
}

#[derive(Debug)]
pub struct MessageIterator {
    socket: WebSocket<MaybeTlsStream<std::net::TcpStream>>,
}

impl Drop for MessageIterator {
    fn drop(&mut self) {
        let _ = self.socket.close(None);
    }
}

impl Iterator for MessageIterator {
    type Item = Result<ReceivedPayload, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.socket.can_read() {
            return None;
        }

        let message = match self.socket.read() {
            Ok(message) => message,
            Err(error) => return Some(Err(Error::from(error))),
        };

        let text_message = match message.to_text() {
            Ok(text_message) => text_message,
            Err(error) => return Some(Err(Error::from(error))),
        };

        match serde_json::from_str(text_message) {
            Ok(received_message) => Some(Ok(received_message)),
            Err(error) => Some(Err(Error::from(error))),
        }
    }
}
