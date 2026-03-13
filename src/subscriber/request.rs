use url::Url;

use tungstenite::client::ClientRequestBuilder;

use crate::auth::Auth;

pub(crate) fn get_request_builder(
    url: &Url,
    topic: String,
    auth: &Option<Auth>,
) -> ClientRequestBuilder {
    let mut ws_url = url.clone();
    let _ = match url.scheme() {
        "https" => ws_url.set_scheme("wss"),
        _ => ws_url.set_scheme("ws"),
    };
    ws_url.set_path(&format!("{topic}/ws"));
    let uri = ws_url.to_string().parse::<http::Uri>().unwrap();

    match auth {
        Some(auth) => ClientRequestBuilder::new(uri)
            .with_header("Authorization", auth.clone().to_header_value()),
        None => ClientRequestBuilder::new(uri),
    }
}
