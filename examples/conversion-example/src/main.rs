use serde::{Deserialize, Serialize};
use trade_aggregation::Trade;

#[derive(Serialize, Deserialize, Debug)]
struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    pub price: f64,
    pub qty: f64,
}

impl From<OrderBook> for Vec<Trade> {
    fn from(order_book: OrderBook) -> Self {
        let mut output: Vec<Trade> = Vec::new();

        let bids = order_book.bids;
        let asks = order_book.asks;

        for bid in bids {
            output.push(Trade {
                timestamp: order_book.timestamp,
                price: bid.price,
                size: bid.qty,
            });
        }

        for ask in asks {
            output.push(Trade {
                timestamp: order_book.timestamp,
                price: ask.price,
                size: -ask.qty,
            })
        }

        output
    }
}

impl FromIterator<OrderBook> for Vec<Trade> {
    fn from_iter<T: IntoIterator<Item=OrderBook>>(iter: T) -> Self {
        iter.into_iter().flat_map(|x| Vec::from(x)).collect()
    }
}


fn main() {
    let order_books: Vec<OrderBook> = vec![
        OrderBook {
            bids: vec![
                Order {
                    price: 69711.4,
                    qty: 0.48330000,
                }
            ],
            asks: vec![
                Order {
                    price: 69712.4,
                    qty: 0.10330000,
                }
            ],
            timestamp: 1514764808289,
        },
    ];

    let trades: Vec<Trade> = order_books.into_iter().collect();
    println!("{trades:?}")
}
