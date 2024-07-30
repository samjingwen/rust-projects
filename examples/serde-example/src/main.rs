mod messages;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::de::Error;
use serde_json::{Value};
use serde_with::{serde_as, DisplayFromStr};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let message = serde_json::from_str::<Value>(messages::UPDATE)?;

    let order_book = message.get("data")
        .ok_or_else(|| serde_json::Error::missing_field("`data` field is missing"))
        .and_then(|data| serde_json::from_value::<Vec<OrderBook>>(data.to_owned()));

    println!("{order_book:?}");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct OrderBookLevel {
    price: f64,
    qty: f64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct OrderBook {
    symbol: String,
    bids: Vec<OrderBookLevel>,
    asks: Vec<OrderBookLevel>,
    #[serde_as(as = "DisplayFromStr")]
    timestamp: DateTime<Utc>,
}