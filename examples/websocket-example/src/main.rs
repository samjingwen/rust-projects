mod client;

fn main() {
    let receiver = client::Kraken::connect();
    while let Ok(text) = receiver.recv() {
        println!("Received: {:?}", text);
    }
}