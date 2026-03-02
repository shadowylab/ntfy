use tungstenite::client::ClientRequestBuilder;
use url::Url;

use crate::Error;
use crate::auth::Auth;

pub(crate) fn get_request_builder(
    url: &Url,
    topic: &str,
    auth: &Option<Auth>,
) -> Result<ClientRequestBuilder, Error> {
    let path: String = format!("{topic}/ws");
    let url: Url = url.join(&path)?;

    let uri = url.to_string().parse::<http::Uri>().unwrap();

    let mut builder = ClientRequestBuilder::new(uri);

    if let Some(auth) = auth {
        builder = builder.with_header("Authorization", auth.header_value());
    }

    Ok(builder)
}
