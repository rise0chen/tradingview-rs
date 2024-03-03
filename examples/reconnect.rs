use dotenv::dotenv;
use std::{env, time::Duration};

use tradingview::{
    chart,
    chart::ChartOptions,
    data_loader::DataLoader,
    models::{pine_indicator::ScriptType, Interval},
    quote,
    socket::{DataServer, SocketSession},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let socket = SocketSession::new(DataServer::ProData, String::from("unauthorized_user_token"))
        .await
        .unwrap();
    let mut publisher: DataLoader = DataLoader::default();
    publisher.callbacks.on_quote_data(move |v| async move {
        tracing::info!("{}:{:?}:{}", v.bid.unwrap(), v.price, v.ask.unwrap());
    });
    loop {
        let publisher = publisher.clone();
        let mut socket = socket.clone();
        let _ = socket.reconnect().await;
        let mut socket_ctrl = socket.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            //let _ = socket_ctrl.close().await;
        });
        let mut quote = quote::session::WebSocket::new(publisher, socket);
        quote
            .create_session()
            .await
            .unwrap()
            .set_fields()
            .await
            .unwrap();
        quote.add_symbols(vec!["BINANCE:APEUSDT.P"]).await.unwrap();
        quote.subscribe().await;
        tracing::error!("quote had closed");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
