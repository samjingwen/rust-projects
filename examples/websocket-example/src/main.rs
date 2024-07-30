use futures_util::{pin_mut, SinkExt, StreamExt};

use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "wss://ws.kraken.com/v2";
    let subscribe_message: &str = r#"
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

    let (ws_stream, _) = connect_async(url).await?;

    let (mut write, read) = ws_stream.split();
    write.send(Message::text(subscribe_message.to_string())).await?;

    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();

            tokio::io::stdout().write_all(&data).await.unwrap();
            tokio::io::stdout().write_all(b"\n").await.unwrap();
        })
    };

    pin_mut!(ws_to_stdout);

    ws_to_stdout.await;

    Ok(())
}
