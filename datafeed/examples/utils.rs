use datafeed::utils::protocol::{format_packet, parse_packet, Packet};
use std::any::type_name;

fn main() {
    tracing_subscriber::fmt::init();
    // let messages = r#"~m~566~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:GB10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:GB10Y","source-logoid":"provider/tvc","short_name":"GB10Y","provider_id":"refinitiv","pro_name":"TVC:GB10Y","pricescale":1000,"minmov":1,"logoid":"country/GB","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"United Kingdom 10 Year Government Bonds Yield","country_code":"GB","base_name":["TVC:GB10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:GB10Y"]}~m~559~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:DE10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:DE10Y","source-logoid":"provider/tvc","short_name":"DE10Y","provider_id":"refinitiv","pro_name":"TVC:DE10Y","pricescale":1000,"minmov":1,"logoid":"country/DE","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"Germany 10 Year Government Bonds Yield","country_code":"DE","base_name":["TVC:DE10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:DE10Y"]}~m~557~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:JP10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:JP10Y","source-logoid":"provider/tvc","short_name":"JP10Y","provider_id":"refinitiv","pro_name":"TVC:JP10Y","pricescale":1000,"minmov":1,"logoid":"country/JP","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"Japan 10 Year Government Bonds Yield","country_code":"JP","base_name":["TVC:JP10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:JP10Y"]}~m~556~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:EU10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:EU10Y","source-logoid":"provider/tvc","short_name":"EU10Y","provider_id":"refinitiv","pro_name":"TVC:EU10Y","pricescale":1000,"minmov":1,"logoid":"country/EU","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"Euro 10 Year Government Bonds Yield","country_code":"EU","base_name":["TVC:EU10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:EU10Y"]}~m~565~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:US10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:US10Y","source-logoid":"provider/tvc","short_name":"US10Y","provider_id":"refinitiv","pro_name":"TVC:US10Y","pricescale":1000,"minmov":1,"logoid":"country/US","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"United States 10 Year Government Bonds Yield","country_code":"US","base_name":["TVC:US10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:US10Y"]}~m~504~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CME:BTC1!","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"delayed_streaming_600","typespecs":["continuous","synthetic"],"type":"futures","source-logoid":"country/US","short_name":"BTC1!","provider_id":"ice","pro_name":"CME:BTC1!","pricescale":1,"minmov":5,"logoid":"crypto/XTVCBTC","listed_exchange":"CME","fractional":false,"exchange":"CME","description":"BITCOIN FUTURES","country_code":"US","base_name":["CME:BTC1!"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CME:BTC1!"]}~m~557~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:IN10Y","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["government","yield","benchmark"],"type":"bond","symbol-primaryname":"TVC:IN10Y","source-logoid":"provider/tvc","short_name":"IN10Y","provider_id":"refinitiv","pro_name":"TVC:IN10Y","pricescale":1000,"minmov":1,"logoid":"country/IN","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"India 10 Year Government Bonds Yield","country_code":"IN","base_name":["TVC:IN10Y"]}}]}~m~90~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:IN10Y"]}~m~524~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NYMEX:NG1!","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"delayed_streaming_600","typespecs":["continuous","synthetic"],"type":"futures","source-logoid":"country/US","short_name":"NG1!","provider_id":"ice","pro_name":"NYMEX:NG1!","pricescale":1000,"minmov":1,"logoid":"natural-gas","listed_exchange":"NYMEX","fractional":false,"exchange":"NYMEX","description":"HENRY HUB NATURAL GAS FUTURES","country_code":"US","base_name":["NYMEX:NG1!"]}}]}~m~91~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NYMEX:NG1!"]}~m~509~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NYMEX:CL1!","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"delayed_streaming_600","typespecs":["continuous","synthetic"],"type":"futures","source-logoid":"country/US","short_name":"CL1!","provider_id":"ice","pro_name":"NYMEX:CL1!","pricescale":100,"minmov":1,"logoid":"crude-oil","listed_exchange":"NYMEX","fractional":false,"exchange":"NYMEX","description":"CRUDE OIL FUTURES","country_code":"US","base_name":["NYMEX:CL1!"]}}]}~m~91~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NYMEX:CL1!"]}~m~504~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"COMEX:GC1!","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"delayed_streaming_600","typespecs":["continuous","synthetic"],"type":"futures","source-logoid":"country/US","short_name":"GC1!","provider_id":"ice","pro_name":"COMEX:GC1!","pricescale":10,"minmov":1,"logoid":"metal/gold","listed_exchange":"COMEX","fractional":false,"exchange":"COMEX","description":"GOLD FUTURES","country_code":"US","base_name":["COMEX:GC1!"]}}]}~m~91~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","COMEX:GC1!"]}~m~526~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"FX_IDC:AUDUSD","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":[],"type":"forex","source-logoid":"provider/ice","short_name":"AUDUSD","provider_id":"ice","pro_name":"FX_IDC:AUDUSD","pricescale":100000,"minmov":1,"listed_exchange":"FX_IDC","fractional":false,"exchange":"ICE","description":"AUSTRALIAN DOLLAR / U.S. DOLLAR","currency-logoid":"country/US","base_name":["FX_IDC:AUDUSD"],"base-currency-logoid":"country/AU"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","FX_IDC:AUDUSD"]}~m~519~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"FX_IDC:USDJPY","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":[],"type":"forex","source-logoid":"provider/ice","short_name":"USDJPY","provider_id":"ice","pro_name":"FX_IDC:USDJPY","pricescale":1000,"minmov":1,"listed_exchange":"FX_IDC","fractional":false,"exchange":"ICE","description":"U.S. DOLLAR / JAPANESE YEN","currency-logoid":"country/JP","base_name":["FX_IDC:USDJPY"],"base-currency-logoid":"country/US"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","FX_IDC:USDJPY"]}~m~513~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"FX_IDC:EURUSD","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":[],"type":"forex","source-logoid":"provider/ice","short_name":"EURUSD","provider_id":"ice","pro_name":"FX_IDC:EURUSD","pricescale":100000,"minmov":1,"listed_exchange":"FX_IDC","fractional":false,"exchange":"ICE","description":"EURO / U.S. DOLLAR","currency-logoid":"country/US","base_name":["FX_IDC:EURUSD"],"base-currency-logoid":"country/EU"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","FX_IDC:EURUSD"]}~m~518~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTOCAP:USDT.D","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["crypto","synthetic"],"type":"index","source-logoid":"provider/tvc","short_name":"USDT.D","provider_id":"tvc","pro_name":"CRYPTOCAP:USDT.D","pricescale":100,"minmov":1,"logoid":"crypto/XTVCUSDT","listed_exchange":"CRYPTOCAP","fractional":false,"exchange":"CRYPTOCAP","description":"Market Cap USDT Dominance, %","base_name":["CRYPTOCAP:USDT.D"]}}]}~m~97~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTOCAP:USDT.D"]}~m~512~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTOCAP:BTC.D","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["crypto","synthetic"],"type":"index","source-logoid":"provider/tvc","short_name":"BTC.D","provider_id":"tvc","pro_name":"CRYPTOCAP:BTC.D","pricescale":100,"minmov":1,"logoid":"crypto/XTVCBTC","listed_exchange":"CRYPTOCAP","fractional":false,"exchange":"CRYPTOCAP","description":"Market Cap BTC Dominance, %","base_name":["CRYPTOCAP:BTC.D"]}}]}~m~96~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTOCAP:BTC.D"]}~m~512~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTOCAP:ETH.D","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["crypto","synthetic"],"type":"index","source-logoid":"provider/tvc","short_name":"ETH.D","provider_id":"tvc","pro_name":"CRYPTOCAP:ETH.D","pricescale":100,"minmov":1,"logoid":"crypto/XTVCETH","listed_exchange":"CRYPTOCAP","fractional":false,"exchange":"CRYPTOCAP","description":"Market Cap ETH Dominance, %","base_name":["CRYPTOCAP:ETH.D"]}}]}~m~96~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTOCAP:ETH.D"]}~m~539~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTO:ETHUSD","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["crypto","cryptoasset","synthetic"],"type":"spot","source-logoid":"provider/tvc","short_name":"ETHUSD","provider_id":"tvc","pro_name":"CRYPTO:ETHUSD","pricescale":100,"minmov":1,"listed_exchange":"CRYPTO","fractional":false,"exchange":"CRYPTO","description":"Ethereum","currency-logoid":"country/US","base_name":["CRYPTO:ETHUSD"],"base-currency-logoid":"crypto/XTVCETH"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTO:ETHUSD"]}~m~538~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTO:BTCUSD","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["crypto","cryptoasset","synthetic"],"type":"spot","source-logoid":"provider/tvc","short_name":"BTCUSD","provider_id":"tvc","pro_name":"CRYPTO:BTCUSD","pricescale":100,"minmov":1,"listed_exchange":"CRYPTO","fractional":false,"exchange":"CRYPTO","description":"Bitcoin","currency-logoid":"country/US","base_name":["CRYPTO:BTCUSD"],"base-currency-logoid":"crypto/XTVCBTC"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTO:BTCUSD"]}~m~549~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CRYPTOCAP:TOTAL","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["crypto","synthetic"],"type":"index","source-logoid":"provider/tvc","short_name":"TOTAL","provider_id":"tvc","pro_name":"CRYPTOCAP:TOTAL","pricescale":1,"minmov":1,"logoid":"crypto-total-market-cap","listed_exchange":"CRYPTOCAP","fractional":false,"exchange":"CRYPTOCAP","description":"Crypto Total Market Cap, $","currency-logoid":"country/US","base_name":["CRYPTOCAP:TOTAL"]}}]}~m~96~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CRYPTOCAP:TOTAL"]}~m~527~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:META","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:META","source-logoid":"country/US","short_name":"META","provider_id":"ice","pro_name":"NASDAQ:META","pricescale":100,"minmov":1,"logoid":"meta-platforms","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Meta Platforms, Inc.","country_code":"US","base_name":["BATS:META"]}}]}~m~92~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:META"]}~m~513~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:NFLX","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:NFLX","source-logoid":"country/US","short_name":"NFLX","provider_id":"ice","pro_name":"NASDAQ:NFLX","pricescale":100,"minmov":1,"logoid":"netflix","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Netflix, Inc.","country_code":"US","base_name":["BATS:NFLX"]}}]}~m~92~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:NFLX"]}~m~509~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:TSLA","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:TSLA","source-logoid":"country/US","short_name":"TSLA","provider_id":"ice","pro_name":"NASDAQ:TSLA","pricescale":100,"minmov":1,"logoid":"tesla","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Tesla, Inc.","country_code":"US","base_name":["BATS:TSLA"]}}]}~m~92~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:TSLA"]}~m~538~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:AMD","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:AMD","source-logoid":"country/US","short_name":"AMD","provider_id":"ice","pro_name":"NASDAQ:AMD","pricescale":100,"minmov":1,"logoid":"advanced-micro-devices","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Advanced Micro Devices, Inc.","country_code":"US","base_name":["BATS:AMD"]}}]}~m~91~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:AMD"]}~m~515~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:AMZN","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:AMZN","source-logoid":"country/US","short_name":"AMZN","provider_id":"ice","pro_name":"NASDAQ:AMZN","pricescale":100,"minmov":1,"logoid":"amazon","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Amazon.com, Inc.","country_code":"US","base_name":["BATS:AMZN"]}}]}~m~92~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:AMZN"]}~m~508~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:AAPL","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["common"],"type":"stock","symbol-primaryname":"NASDAQ:AAPL","source-logoid":"country/US","short_name":"AAPL","provider_id":"ice","pro_name":"NASDAQ:AAPL","pricescale":100,"minmov":1,"logoid":"apple","listed_exchange":"NASDAQ","fractional":false,"exchange":"Cboe One","description":"Apple Inc.","country_code":"US","base_name":["BATS:AAPL"]}}]}~m~92~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:AAPL"]}~m~502~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:RUT","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["cfd"],"type":"index","symbol-primaryname":"TVC:RUT","source-logoid":"provider/tvc","short_name":"RUT","provider_id":"tvc","pro_name":"TVC:RUT","pricescale":10000,"minmov":1,"logoid":"indices/russell-2000","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"RUSSELL 2000","country_code":"US","base_name":["TVC:RUT"]}}]}~m~88~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:RUT"]}~m~510~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"DJ:DJI","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["main","cfd"],"type":"index","symbol-primaryname":"DJ:DJI","source-logoid":"country/US","short_name":"DJI","provider_id":"tvc","pro_name":"DJ:DJI","pricescale":100,"minmov":1,"logoid":"indices/dow-30","listed_exchange":"DJ","fractional":false,"exchange":"DJ","description":"Dow Jones Industrial Average","country_code":"US","base_name":["DJ:DJI"]}}]}~m~87~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","DJ:DJI"]}~m~519~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"TVC:DXY","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"streaming","typespecs":["cfd"],"type":"index","symbol-primaryname":"TVC:DXY","source-logoid":"provider/tvc","short_name":"DXY","provider_id":"ice","pro_name":"TVC:DXY","pricescale":1000,"minmov":1,"logoid":"indices/u-s-dollar-index","listed_exchange":"TVC","fractional":false,"exchange":"TVC","description":"U.S. DOLLAR CURRENCY INDEX","country_code":"US","base_name":["TVC:DXY"]}}]}~m~88~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","TVC:DXY"]}~m~540~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"CBOE:VIX","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"delayed_streaming_900","typespecs":["main"],"type":"index","symbol-primaryname":"CBOE:VIX","source-logoid":"country/US","short_name":"VIX","provider_id":"ice","pro_name":"CBOE:VIX","pricescale":100,"minmov":1,"logoid":"indices/volatility-s-and-p-500","listed_exchange":"CBOE","fractional":false,"exchange":"CBOE","description":"CBOE VOLATILITY INDEX","country_code":"US","base_name":["CBOE_DLY:VIX"]}}]}~m~89~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","CBOE:VIX"]}~m~529~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"NASDAQ:NDX","s":"ok","v":{"visible-plots-set":"ohlc","update_mode":"delayed_streaming_900","typespecs":["main"],"type":"index","symbol-primaryname":"NASDAQ:NDX","source-logoid":"country/US","short_name":"NDX","provider_id":"ice","pro_name":"NASDAQ:NDX","pricescale":100,"minmov":1,"logoid":"indices/nasdaq-100","listed_exchange":"NASDAQ","fractional":false,"exchange":"NASDAQ","description":"NASDAQ 100","country_code":"US","base_name":["NASDAQ_DLY:NDX"]}}]}~m~91~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","NASDAQ:NDX"]}~m~496~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"SP:SPX","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":["main","cfd"],"type":"index","symbol-primaryname":"CBOE:SPX","source-logoid":"country/US","short_name":"SPX","provider_id":"tvc","pro_name":"SP:SPX","pricescale":100,"minmov":1,"logoid":"indices/s-and-p-500","listed_exchange":"SP","fractional":false,"exchange":"SP","description":"S&P 500","country_code":"US","base_name":["SP:SPX"]}}]}~m~87~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","SP:SPX"]}~m~521~m~{"m":"qsd","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc",{"n":"FX_IDC:GBPUSD","s":"ok","v":{"visible-plots-set":"ohlcv","update_mode":"streaming","typespecs":[],"type":"forex","source-logoid":"provider/ice","short_name":"GBPUSD","provider_id":"ice","pro_name":"FX_IDC:GBPUSD","pricescale":10000,"minmov":1,"listed_exchange":"FX_IDC","fractional":false,"exchange":"ICE","description":"BRITISH POUND / U.S. DOLLAR","currency-logoid":"country/US","base_name":["FX_IDC:GBPUSD"],"base-currency-logoid":"country/GB"}}]}~m~94~m~{"m":"quote_completed","p":["qs_snapshoter_basic-symbol-quotes_2btz4hzMC5lc","FX_IDC:GBPUSD"]}"#;
    // let data = parse_packet(messages).unwrap();
    // for i in data {
    //     match i.p {
    //         serde_json::value::Value::Null => todo!(),
    //         serde_json::value::Value::Bool(_) => todo!(),
    //         serde_json::value::Value::Number(_) => todo!(),
    //         serde_json::value::Value::String(_) => todo!(),
    //         serde_json::value::Value::Array(_) => todo!(),
    //         serde_json::value::Value::Object(_) => todo!(),
    //     }
    // }
    // // println!("Parsed: {:?}", data);
    let test = Packet {
        p: serde_json::Value::from("test".to_string()),
        m: serde_json::Value::from(vec![1, 2, 3, 4, 5]),
    };
    // println!("{}", test.);
    let test1 = format_packet(test).unwrap();
    dbg!(test1);
}

fn print_type<T>(_: T) {
    println!("{}", type_name::<T>());
}
