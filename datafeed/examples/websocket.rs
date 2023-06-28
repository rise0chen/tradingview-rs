use datafeed::utils::protocol::{format_packet, parse_packet, Packet};
use tracing::debug;
use tungstenite::{client::IntoClientRequest, connect, Message};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt::init();
    let mut client = datafeed::client::Socket::new(true);

    client.read_message();
    // let raw = r#"~m~559~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:DE10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:DE10Y","source-logoid":"provider/tvc","short_name":"DE10Y","provider_id":"refinitiv","pro_name":"TVC:DE10Y","pricescale":1000,"minmov":1,"logoid":"country/DE","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"Germany 10 Year Government Bonds Yield","country_code":"DE","base_name":["TVC:DE10Y"]}}]}"#;
    // let packets = parse_packet(raw).unwrap();
    // let msg = format_packet(packets[0].clone());
    // debug!("{:?}", msg.to_string());
}
