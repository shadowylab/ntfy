use std::net::TcpStream;

use tungstenite::client::connect;
use tungstenite::protocol::WebSocket;
use tungstenite::stream::MaybeTlsStream;
use url::Url;

use super::builder::SubscriberBuilder;
use super::request::get_request_builder;
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

    pub(crate) fn subscribe(&self, url: &Url, topic: &str) -> Result<MessageStream, Error> {
        let builder = get_request_builder(url, topic, &self.auth)?;

        // Create message iterator
        Ok(MessageStream {
            socket: connect(builder)?.0,
        })
    }
}

#[derive(Debug)]
pub struct MessageStream {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl Drop for MessageStream {
    fn drop(&mut self) {
        let _ = self.socket.close(None);
    }
}

impl Iterator for MessageStream {
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
