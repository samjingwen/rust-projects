use std::sync::mpsc;
use std::thread;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::Error;
use serde_json::Value;
use tungstenite::{connect, Message};

pub const SUBSCRIBE_MESSAGE: &str = r#"
{
    "method": "subscribe",
    "params": {
        "channel": "book",
        "symbol": [
            "BTC/USD"
        ]
    }
}
    "#;


#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    #[serde(deserialize_with = "timestamp_to_i64")]
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub price: f64,
    pub qty: f64,
}

fn timestamp_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let datetime = chrono::DateTime::parse_from_rfc3339(&s)
        .map_err(serde::de::Error::custom)?;
    Ok(datetime.timestamp())
}

pub struct Kraken;

impl Kraken {
    pub fn connect() -> mpsc::Receiver<OrderBook> {
        let (sender, receiver) = mpsc::channel();

        let url = "wss://ws.kraken.com/v2";
        let (mut socket, _) = connect(url).expect("connection failed");
        log::info!("WebSocket handshake has been successfully completed");

        thread::spawn(move || {
            socket.send(Message::text(SUBSCRIBE_MESSAGE.to_string())).expect("request failed");

            loop {
                let message = socket.read();
                if let Ok(Message::Text(text)) = message {
                    if let Ok(order_book) = parse_order_book(&text) {
                        let _ = sender.send(order_book);
                    } else {
                        println!("not order book: {text}")
                    }
                }
            }
        });

        receiver
    }
}

fn parse_order_book(text: &str) -> Result<OrderBook, serde_json::Error> {
    let value: Value = serde_json::from_str(text)?;
    let data = value.get("data").ok_or_else(|| serde_json::Error::custom("Missing 'data' field"))?;
    let order_books: Vec<OrderBook> = serde_json::from_value(data.to_owned())?;
    order_books.into_iter().nth(0).ok_or_else(|| serde_json::Error::custom("No OrderBook found"))
}
