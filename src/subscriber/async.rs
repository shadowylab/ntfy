use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::stream::{FusedStream, Stream, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

use super::builder::SubscriberBuilder;
use super::request::get_request_builder;
use crate::auth::Auth;
use crate::error::Error;
use crate::payload::ReceivedPayload;

/// Async subscriber
#[derive(Debug, Clone)]
pub struct Async {
    auth: Option<Auth>,
}

impl Async {
    #[inline]
    pub(crate) fn new(builder: SubscriberBuilder) -> Result<Self, Error> {
        Ok(Self { auth: builder.auth })
    }

    pub(crate) async fn subscribe(&self, url: &Url, topic: &str) -> Result<MessageStream, Error> {
        let builder = get_request_builder(url, topic, &self.auth)?;

        // Create message iterator
        Ok(MessageStream {
            socket: connect_async(builder).await?.0,
        })
    }
}

pub struct MessageStream {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Stream for MessageStream {
    type Item = Result<ReceivedPayload, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.socket.is_terminated() {
            return Poll::Ready(None);
        }

        let message = match self.socket.poll_next_unpin(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Some(Ok(message))) => message,
            Poll::Ready(Some(Err(error))) => return Poll::Ready(Some(Err(Error::from(error)))),
            Poll::Ready(None) => return Poll::Ready(None),
        };

        let text_message = match message.to_text() {
            Ok(text_message) => text_message,
            Err(error) => return Poll::Ready(Some(Err(Error::from(error)))),
        };

        match serde_json::from_str(text_message) {
            Ok(received_message) => Poll::Ready(Some(Ok(received_message))),
            Err(error) => Poll::Ready(Some(Err(Error::from(error)))),
        }
    }
}
