use bybit_async::ws::public::{model, FutureClient};

#[tokio::main]
async fn main() {
    let client = FutureClient::new();

    let (sender, mut receiver) = client.connect().await.unwrap();

    sender
        .send(model::Op {
            req_id: None,
            op: "subscribe".to_string(),
            args: vec!["publicTrade.BTCUSDT".to_string()],
        })
        .unwrap();

    while let Some(msg) = receiver.recv().await {
        println!("MSG: {:?}", msg);
    }
}
