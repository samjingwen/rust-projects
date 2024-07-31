use tungstenite::{connect, Message};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let (mut socket, _) = connect(url)?;
    println!("Connected to the server");

    socket.send(Message::Text(subscribe_message.into()))?;

    loop {
        let msg = socket.read()?;
        if let Message::Text(text) = msg {
            println!("Received: {}", text);
        }
    }

}
