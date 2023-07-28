pub mod websocket;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_QUOTE_FIELDS: Vec<String> = vec![
        "base-currency-logoid".to_string(),
        "ch".to_string(),
        "chp".to_string(),
        "currency-logoid".to_string(),
        "currency_code".to_string(),
        "current_session".to_string(),
        "description".to_string(),
        "exchange".to_string(),
        "format".to_string(),
        "fractional".to_string(),
        "is_tradable".to_string(),
        "language".to_string(),
        "local_description".to_string(),
        "logoid".to_string(),
        "lp".to_string(),
        "lp_time".to_string(),
        "minmov".to_string(),
        "minmove2".to_string(),
        "original_name".to_string(),
        "pricescale".to_string(),
        "pro_name".to_string(),
        "short_name".to_string(),
        "type".to_string(),
        "update_mode".to_string(),
        "volume".to_string(),
        "ask".to_string(),
        "bid".to_string(),
        "fundamentals".to_string(),
        "high_price".to_string(),
        "low_price".to_string(),
        "open_price".to_string(),
        "prev_close_price".to_string(),
        "rch".to_string(),
        "rchp".to_string(),
        "rtc".to_string(),
        "rtc_time".to_string(),
        "status".to_string(),
        "industry".to_string(),
        "basic_eps_net_income".to_string(),
        "beta_1_year".to_string(),
        "market_cap_basic".to_string(),
        "earnings_per_share_basic_ttm".to_string(),
        "price_earnings_ttm".to_string(),
        "sector".to_string(),
        "dividends_yield".to_string(),
        "timezone".to_string(),
        "country_code".to_string(),
        "provider_id".to_string(),
    ];
}

#[derive(Debug, PartialEq, Clone)]
pub enum QuoteSocketEvent {
    Data,
    Loaded,
    Error,
}

impl std::fmt::Display for QuoteSocketEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            QuoteSocketEvent::Data => write!(f, "qsd"),
            QuoteSocketEvent::Loaded => write!(f, "quote_completed"),
            QuoteSocketEvent::Error => write!(f, "error"),
        }
    }
}
