use crate::{
    payload,
    quote::ALL_QUOTE_FIELDS,
    socket::{ Socket, SocketMessageDe, SocketSession, TradingViewDataEvent },
    utils::gen_session_id,
    Result,
    subscriber::Subscriber,
};
use async_trait::async_trait;
use serde_json::Value;

// #[derive(Default)]
// pub struct WebSocketsBuilder {
//     server: Option<DataServer>,
//     auth_token: Option<String>,
//     quote_fields: Option<Vec<String>>,
//     socket: Option<SocketSession>,
// }

pub struct WebSocket {
    subscriber: Subscriber,
    socket: SocketSession,
    quote_session: String,
    quote_fields: Vec<Value>,
    // prev_quotes: HashMap<String, QuoteValue>,
}

impl WebSocket {
    pub fn new(subscriber: Subscriber, socket: SocketSession) -> Self {
        let quote_session = gen_session_id("qs");

        let mut quote_fields = payload![quote_session.clone().to_string()];
        quote_fields.extend(ALL_QUOTE_FIELDS.clone().into_iter().map(Value::from));

        Self {
            subscriber,
            socket,
            quote_session,
            quote_fields,
            // prev_quotes:,
        }
    }

    pub async fn create_session(&mut self) -> Result<()> {
        self.socket.send("quote_create_session", &payload!(self.quote_session.clone())).await?;
        Ok(())
    }

    pub async fn delete_session(&mut self) -> Result<()> {
        self.socket.send("quote_delete_session", &payload!(self.quote_session.clone())).await?;
        Ok(())
    }

    pub async fn update_session(&mut self) -> Result<()> {
        self.delete_session().await?;
        self.quote_session = gen_session_id("qs");
        self.socket.send("quote_create_session", &payload!(self.quote_session.clone())).await?;
        Ok(())
    }

    pub async fn set_fields(&mut self) -> Result<()> {
        self.socket.send("quote_set_fields", &self.quote_fields.clone()).await?;
        Ok(())
    }

    pub async fn add_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        self.socket.send("quote_add_symbols", &payloads).await?;
        Ok(())
    }

    pub async fn update_auth_token(&mut self, auth_token: &str) -> Result<()> {
        self.socket.update_auth_token(auth_token).await?;
        Ok(())
    }

    pub async fn fast_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        self.socket.send("quote_fast_symbols", &payloads).await?;
        Ok(())
    }

    pub async fn remove_symbols(&mut self, symbols: Vec<&str>) -> Result<()> {
        let mut payloads = payload![self.quote_session.clone()];
        payloads.extend(symbols.into_iter().map(Value::from));
        self.socket.send("quote_remove_symbols", &payloads).await?;
        Ok(())
    }

    pub async fn subscribe(&mut self) {
        self.event_loop(&mut self.socket.clone()).await;
    }
}

#[async_trait]
impl Socket for WebSocket {
    async fn handle_message_data(&mut self, message: SocketMessageDe) -> Result<()> {
        let event = TradingViewDataEvent::from(message.m.clone());
        self.subscriber.event_handler(event, &message.p, None).await;
        // match TradingViewDataEvent::from(message.m.clone()) {
        //     TradingViewDataEvent::OnQuoteData => {
        //         trace!("received OnQuoteData: {:#?}", message);
        //         let qsd = serde_json::from_value::<QuoteData>(message.p[1].clone())?;
        //         if qsd.status == "ok" {
        //             if let Some(prev_quote) = self.prev_quotes.get_mut(&qsd.name) {
        //                 *prev_quote = merge_quotes(prev_quote, &qsd.value);
        //             } else {
        //                 self.prev_quotes.insert(qsd.name.clone(), qsd.value.clone());
        //             }
        //             tokio::spawn((self.callbacks.data)(self.prev_quotes.clone()));
        //         } else {
        //             tokio::spawn(
        //                 (self.callbacks.error)(
        //                     Error::TradingViewError(TradingViewError::QuoteDataStatusError)
        //                 )
        //             );
        //         }
        //     }
        //     TradingViewDataEvent::OnQuoteCompleted => {
        //         tokio::spawn((self.callbacks.loaded)(message.p.clone()));
        //     }
        //     TradingViewDataEvent::OnError(e) => {
        //         self.handle_error(Error::TradingViewError(e)).await;
        //     }
        //     _ => {
        //         debug!("unhandled event on this session: {:?}", message);
        //     }
        // }
        Ok(())
    }
}
