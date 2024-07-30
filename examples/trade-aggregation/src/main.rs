use trade_aggregation::{candle_components::{Close, High, Low, Open}, *};
use trade_aggregation::candle_components::NumTrades;

#[derive(Debug, Default, Clone, Candle)]
struct SimpleCandle {
    open: Open,
    high: High,
    low: Low,
    close: Close,
    ticks: NumTrades<u32>,
}

fn main() {
    let trades = load_trades_from_csv("data/Bitmex_XBTUSD_1M.csv")
        .expect("could not load trades from file");

    let time_rule = TimeRule::new(M1, TimestampResolution::Millisecond);
    let mut aggregator = GenericAggregator::<SimpleCandle, TimeRule, Trade>::new(time_rule, false);

    for t in &trades {
        if let Some(candle) = aggregator.update(t) {
            println!(
                "candle created with open: {}, high: {}, low: {}, close: {}, ticks: {}, minute: {}",
                candle.open(),
                candle.high(),
                candle.low(),
                candle.close(),
                candle.ticks(),
                t.timestamp / 1000 / 60,
            );
        }
    }
}
