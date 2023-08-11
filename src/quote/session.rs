use crate::{
    error::TradingViewError,
    payload,
    prelude::*,
    quote::{QuoteData, QuoteValue, ALL_QUOTE_FIELDS},
    socket::{DataServer, Events, Socket, SocketSession},
    utils::gen_session_id,
};
use async_trait::async_trait;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tracing::{debug, error, info, trace, warn};

#[derive(Default)]
pub struct WebSocketsBuilder {
    server: Option<DataServer>,
    auth_token: Option<String>,
    quote_fields: Option<Vec<String>>,
}

pub struct WebSocket {
    socket: SocketSession,
    quote_session_id: String,
    quote_fields: Vec<Value>,
    prev_quotes: HashMap<String, QuoteValue>,
    auth_token: String,
    callbacks: QuoteCallbackFn,
}

fn merge_quotes(quote_old: &QuoteValue, quote_new: &QuoteValue) -> QuoteValue {
    QuoteValue {
        ask: quote_new.ask.or(quote_old.ask),
        ask_size: quote_new.ask_size.or(quote_old.ask_size),
        bid: quote_new.bid.or(quote_old.bid),
        bid_size: quote_new.bid_size.or(quote_old.bid_size),
        price_change: quote_new.price_change.or(quote_old.price_change),
        price_change_percent: quote_new
            .price_change_percent
            .or(quote_old.price_change_percent),
        open: quote_new.open.or(quote_old.open),
        high: quote_new.high.or(quote_old.high),
        low: quote_new.low.or(quote_old.low),
        prev_close: quote_new.prev_close.or(quote_old.prev_close),
        price: quote_new.price.or(quote_old.price),
        timestamp: quote_new.timestamp.or(quote_old.timestamp),
        volume: quote_new.volume.or(quote_old.volume),
    }
}

pub struct QuoteCallbackFn {
    pub data: Box<dyn FnMut(HashMap<String, QuoteValue>) -> Result<()> + Send + Sync>,
    pub loaded: Box<dyn FnMut(Vec<Value>) -> Result<()> + Send + Sync>,
    pub error: Box<dyn FnMut(TradingViewError) -> Result<()> + Send + Sync>,
}

impl WebSocketsBuilder {
    pub fn server(&mut self, server: DataServer) -> &mut Self {
        self.server = Some(server);
        self
    }

    pub fn auth_token(&mut self, auth_token: String) -> &mut Self {
        self.auth_token = Some(auth_token);
        self
    }

    pub fn quote_fields(&mut self, quote_fields: Vec<String>) -> &mut Self {
        self.quote_fields = Some(quote_fields);
        self
    }

    pub async fn connect(&self, callback: QuoteCallbackFn) -> Result<WebSocket> {
        let auth_token = self
            .auth_token
            .clone()
            .unwrap_or("unauthorized_user_token".to_string());

        let server = self.server.clone().unwrap_or_default();

        let session = gen_session_id("qs");

        let quote_fields: Vec<Value> = match self.quote_fields.clone() {
            Some(fields) => {
                let mut quote_fields = payload![session.clone().to_string()];
                quote_fields.extend(fields.into_iter().map(Value::from));
                quote_fields
            }
            None => {
                let mut quote_fields = payload![session.clone().to_string()];
                quote_fields.extend(ALL_QUOTE_FIELDS.clone().into_iter().map(Value::from));
                quote_fields
            }
        };

        let socket = SocketSession::new(auth_token.clone(), server).await?;

        Ok(WebSocket {
            socket: socket,
            quote_session_id: session,
            quote_fields,
            auth_token,
            prev_quotes: HashMap::new(),
            callbacks: callback,
        })
    }
}

impl WebSocket {
    pub fn build() -> WebSocketsBuilder {
        WebSocketsBuilder::default()
    }

    pub async fn create_session(&mut self) -> Result<()> {
        SocketSession::send(
            &mut self.socket,
            "quote_create_session",
            &payload!(self.quote_session_id.clone()),
        )
        .await?;
        Ok(())
    }

    pub async fn delete_session(&mut self) -> Result<()> {
        SocketSession::send(
            &mut self.socket,
            "quote_delete_session",
            &payload!(self.quote_session_id.clone()),
        )
        .await?;
        Ok(())
    }

    pub async fn update_session(&mut self) -> Result<()> {
        self.delete_session().await?;
        self.quote_session_id = gen_session_id("qs");
        SocketSession::send(
            &mut self.socket,
            "quote_create_session",
            &payload!(self.quote_session_id.clone()),
        )
        .await?;
        Ok(())
    }

    pub async fn set_fields(&mut self) -> Result<()> {
        SocketSession::send(
            &mut self.socket,
            "quote_set_fields",
            &self.quote_fields.clone(),
        )
        .await?;
        Ok(())
    }

    pub async fn add_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session_id.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        SocketSession::send(&mut self.socket, "quote_add_symbols", &payloads).await?;
        Ok(())
    }

    pub async fn update_auth_token(&mut self, auth_token: &str) -> Result<()> {
        self.auth_token = auth_token.to_owned();
        SocketSession::send(&mut self.socket, "set_auth_token", &payload!(auth_token)).await?;
        Ok(())
    }

    pub async fn fast_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session_id.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        SocketSession::send(&mut self.socket, "quote_fast_symbols", &payloads).await?;
        Ok(())
    }

    pub async fn remove_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session_id.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        SocketSession::send(&mut self.socket, "quote_remove_symbols", &payloads).await?;
        Ok(())
    }
}

#[async_trait]
impl Socket for WebSocket {
    async fn load() {
        todo!()
    }
}
