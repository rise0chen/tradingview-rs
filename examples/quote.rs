use std::env;

use serde_json::Value;
use tracing::{debug, info};
use tradingview_rs::quote::{websocket::QuoteSocket, QuoteEvent, QuoteValue};
use tradingview_rs::socket::DataServer;
use tradingview_rs::user::User;
type Result<T> = std::result::Result<T, tradingview_rs::error::Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let session = env::var("TV_SESSION").unwrap();
    let signature = env::var("TV_SIGNATURE").unwrap();

    let user = User::new()
        .session(&session, &signature)
        .build()
        .await
        .unwrap();

    let mut binding = QuoteSocket::new(DataServer::Data, event_handler);
    let mut socket = binding.auth_token(user.auth_token).build().await.unwrap();

    socket
        .quote_add_symbols(vec![
            "BINANCE:BTCUSDT".to_string(),
            // "BINANCE:ETHUSDT".to_string(),
            // "BITSTAMP:ETHUSD".to_string(),
            // "NASDAQ:TSLA".to_string(),
        ])
        .await
        .unwrap();

    socket.event_loop().await;
}

fn event_handler(event: QuoteEvent, data: Value) -> Result<()> {
    match event {
        QuoteEvent::Data => {
            // debug!("data: {:#?}", &data);
            // let data2 = data.get("v").unwrap();
            // let quote: QuoteValue = serde_json::from_value(data2.clone()).unwrap();
            // info!("quote: {:#?}", quote);
        }
        QuoteEvent::Loaded => {
            info!("loaded: {:#?}", data);
        }
        QuoteEvent::Error(_) => todo!(),
    }
    Ok(())
}
