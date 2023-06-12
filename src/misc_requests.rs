use reqwest::{Client, Error};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Indicator {
    #[serde(rename = "scriptName")]
    pub name: String,

    #[serde(rename = "scriptIdPart")]
    pub id: String,

    pub version: String,

    #[serde(flatten, rename = "extra")]
    pub info: HashMap<String, Value>,
}

#[tracing::instrument]
async fn fetch_scan_data(
    tickers: Vec<String>,
    scan_type: &str,
    columns: Vec<String>,
) -> Result<serde_json::Value, Error> {
    let client = Client::new();
    let url = format!("https://scanner.tradingview.com/{}/scan", scan_type);
    let body = serde_json::json!({
        "symbols": {
            "tickers": tickers
        },
        "columns": columns
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let data = response.text().await?;

    if !data.starts_with('{') {
        Err("Wrong screener or symbol").unwrap()
    }

    let parsed_data = serde_json::from_str(&data).unwrap();

    Ok(parsed_data)
}

#[tracing::instrument]
pub async fn get_builtin_indicators() -> Result<Vec<Indicator>, Box<dyn std::error::Error>> {
    let indicator_types = vec!["standard", "candlestick", "fundamental"];
    let mut indicators: Vec<Indicator> = vec![];
    for indicator_type in indicator_types {
        let url = format!(
            "https://pine-facade.tradingview.com/pine-facade/list/?filter={}",
            indicator_type
        );

        let mut data = crate::utils::client::get_request(&url, None)
            .await?
            .json::<Vec<Indicator>>()
            .await?;

        indicators.append(&mut data);
    }
    Ok(indicators)
}

#[tracing::instrument]
pub async fn get_private_indicators(
    session: &str,
    signature: &str,
) -> Result<Vec<Indicator>, Box<dyn std::error::Error>> {
    let data = crate::utils::client::get_request(
        "https://pine-facade.tradingview.com/pine-facade/list?filter=saved",
        Some(format!(
            "sessionid={}; sessionid_sign={};",
            session, signature
        )),
    )
    .await?
    .json::<Vec<Indicator>>()
    .await?;
    Ok(data)
}

#[tracing::instrument]
pub fn get_screener(exchange: &str) -> String {
    let e = exchange.to_uppercase();
    if ["NASDAQ", "NYSE", "NYSE ARCA", "OTC"].contains(&e.as_str()) {
        return String::from("america");
    }
    if ["ASX"].contains(&e.as_str()) {
        return String::from("australia");
    }
    if ["TSX", "TSXV", "CSE", "NEO"].contains(&e.as_str()) {
        return String::from("canada");
    }
    if ["EGX"].contains(&e.as_str()) {
        return String::from("egypt");
    }
    if ["FWB", "SWB", "XETR"].contains(&e.as_str()) {
        return String::from("germany");
    }
    if ["BSE", "NSE"].contains(&e.as_str()) {
        return String::from("india");
    }
    if ["TASE"].contains(&e.as_str()) {
        return String::from("israel");
    }
    if ["MIL", "MILSEDEX"].contains(&e.as_str()) {
        return String::from("italy");
    }
    if ["LUXSE"].contains(&e.as_str()) {
        return String::from("luxembourg");
    }
    if ["NEWCONNECT"].contains(&e.as_str()) {
        return String::from("poland");
    }
    if ["NGM"].contains(&e.as_str()) {
        return String::from("sweden");
    }
    if ["BIST"].contains(&e.as_str()) {
        return String::from("turkey");
    }
    if ["LSE", "LSIN"].contains(&e.as_str()) {
        return String::from("uk");
    }
    if ["HNX", "HOSE", "UPCOM"].contains(&e.as_str()) {
        return String::from("vietnam");
    }
    return exchange.to_lowercase();
}
