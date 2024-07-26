mod a;

use a::CandleStick;
use dotenv::dotenv;
use minifb::{Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::borrow::{Borrow, BorrowMut};
use std::time::Duration;
use tokio::sync::mpsc;
use tradingview::{
    data_loader::DataLoader,
    quote::{self, models::QuoteValue},
    socket::{DataServer, SocketSession},
};

const W: usize = 800;
const H: usize = 600;

struct BufferWrapper(Vec<u32>);
impl Borrow<[u8]> for BufferWrapper {
    fn borrow(&self) -> &[u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *const u8, self.0.len() * 4) }
    }
}
impl BorrowMut<[u8]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u8] {
        // Safe for alignment: align_of(u8) <= align_of(u32)
        // Safe for cast: u32 can be thought of as being transparent over [u8; 4]
        unsafe { std::slice::from_raw_parts_mut(self.0.as_mut_ptr() as *mut u8, self.0.len() * 4) }
    }
}
impl Borrow<[u32]> for BufferWrapper {
    fn borrow(&self) -> &[u32] {
        self.0.as_slice()
    }
}
impl BorrowMut<[u32]> for BufferWrapper {
    fn borrow_mut(&mut self) -> &mut [u32] {
        self.0.as_mut_slice()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let socket = SocketSession::new(DataServer::ProData, String::from("unauthorized_user_token"))
        .await
        .unwrap();
    let mut publisher: DataLoader = DataLoader::default();
    let (dtx, mut drx) = mpsc::channel(64);
    std::thread::spawn(move || {
        let mut window = Window::new("title", W, H, WindowOptions::default()).unwrap();
        let v = QuoteValue {
            ask: None,
            ask_size: None,
            bid: None,
            bid_size: None,
            change: None,
            change_percent: None,
            open: None,
            high: None,
            low: None,
            prev_close: None,
            price: None,
            timestamp: None,
            volume: None,
            currency: None,
            symbol: None,
            exchange: None,
            market_type: None,
        };
        let vi: QuoteValue = drx.blocking_recv().unwrap();
        let price_range = vi.bid.unwrap() * 0.995..vi.ask.unwrap() * 1.005;
        let mut datas_spot = [
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
        ];
        let mut datas_future = [
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
            v.clone(),
        ];
        fn set_data(datas: &mut [QuoteValue], v: QuoteValue) {
            let i = match v.exchange.as_ref().unwrap().as_str() {
                "BINANCE" => 1,
                "OKX" => 2,
                "MEXC" => 3,
                "BYBIT" => 4,
                "BITGET" => 5,
                "HTX" => 6,
                _ => {
                    tracing::info!("{:?}", v.exchange);
                    0
                }
            };
            datas[i as usize] = v;
        }
        loop {
            let v: QuoteValue = drx.blocking_recv().unwrap();
            let datas = if v.market_type.as_deref() == Some("spot") {
                &mut datas_spot
            } else {
                &mut datas_future
            };
            set_data(datas, v);
            while let Ok(v) = drx.try_recv() {
                let datas = if v.market_type.as_deref() == Some("spot") {
                    &mut datas_spot
                } else {
                    &mut datas_future
                };
                set_data(datas, v);
            }
            let mut buf = BufferWrapper(vec![0u32; W * H]);
            {
                let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                    buf.borrow_mut(),
                    (W as u32, H as u32),
                )
                .unwrap()
                .into_drawing_area();
                root.fill(&WHITE).unwrap();

                let mut chart = ChartBuilder::on(&root)
                    .margin(10)
                    .set_all_label_area_size(30)
                    .caption("MSFT Stock Price", ("sans-serif", 50.0).into_font())
                    .build_cartesian_2d(0..3, price_range.clone())
                    .unwrap();

                chart
                    .configure_mesh()
                    .label_style(("sans-serif", 15).into_font().color(&GREEN))
                    .axis_style(&GREEN)
                    .draw()
                    .unwrap();
                chart
                    .draw_series(datas_spot.iter().filter_map(|v| {
                        Some(CandleStick::new(
                            if v.market_type.as_deref() == Some("spot") {
                                1
                            } else {
                                2
                            },
                            v.ask?,
                            v.ask?,
                            v.bid?,
                            v.bid?,
                            15,
                            v.exchange.as_ref()?.to_string(),
                        ))
                    }))
                    .unwrap();
                chart
                    .draw_series(datas_future.iter().filter_map(|v| {
                        Some(CandleStick::new(
                            if v.market_type.as_deref() == Some("spot") {
                                1
                            } else {
                                2
                            },
                            v.ask?,
                            v.ask?,
                            v.bid?,
                            v.bid?,
                            15,
                            v.exchange.as_ref()?.to_string(),
                        ))
                    }))
                    .unwrap();
                root.present().unwrap();
            }
            window.update_with_buffer(buf.borrow(), W, H).unwrap();
        }
    });
    publisher.callbacks.on_quote_data(move |v| {
        let dtx = dtx.clone();
        async move {
            if v.ask.is_none() || v.bid.is_none() {
                return;
            }
            if v.exchange.is_none() || v.market_type.is_none() {
                return;
            }
            println!("{:?}",v);
            let _ = dtx.send(v).await;
        }
    });

    let fields = ["ask", "bid", "exchange", "type"];
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
            .set_fields(None)
            .await
            .unwrap();
        let _ape = [
            "BINANCE:APEUSDT.P",
            "OKX:APEUSDT.P",
            "MEXC:APEUSDT.P",
            "BYBIT:APEUSDT.P",
            "BINANCE:APEUSDT",
            "OKX:APEUSDT",
            "MEXC:APEUSDT",
            "BYBIT:APEUSDT",
        ];
        let _core = [
            "BYBIT:COREUSDT.P",
            "OKX:COREUSDT.P",
            "MEXC:COREUSDT.P",
            "BYBIT:COREUSDT",
            "OKX:COREUSDT",
            "MEXC:COREUSDT",
        ];
        let _test = [
            "BYBIT:AEVOUSDT.P",
            "OKX:AEVOUSDT.P",
            "MEXC:AEVOUSDT.P",
            "BITGET:AEVOUSDT.P",
        ];
        let symbols = _core;
        quote.add_symbols(&symbols).await.unwrap();
        quote.fast_symbols(&symbols).await.unwrap();
        quote.subscribe().await;
        tracing::error!("quote had closed");
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
