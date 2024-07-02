use std::time::Duration;

use rand::{distributions::Alphanumeric, Rng};

use bybit_async::rest::model::{
    CancelAllOrderRequest, Category, OrderType, PlaceOrderRequest, PositionIdx, Side,
};
use bybit_async::rest::Client;
use bybit_async::ws::private::{model, Client as ClientWS};
use bybit_async::Credentials;

#[tokio::main]
async fn main() {
    let credentials = Credentials {
        api_key: "".to_owned(), // Testnet keys
        secret: "".to_owned(),  // Testnet keys
    };
    let client = Client::new_testnet(credentials.clone(), None);
    let client_ws = ClientWS::new_testnet(credentials);

    let (sender, mut receiver) = client_ws.connect().await.unwrap();
    sender
        .send(model::Op {
            req_id: None,
            op: "subscribe".to_string(),
            args: vec!["order".to_string()],
        })
        .unwrap();
    tokio::spawn(async move {
        while let Some(res) = receiver.recv().await {
            println!("WS response: {:?}", res);
        }
    });

    let suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let order_link_id = format!("test_order_{}", suffix);

    tokio::time::sleep(Duration::from_secs(2)).await;
    let res = client
        .place_order(
            PlaceOrderRequest {
                category: Category::Linear,
                symbol: "BTCUSDT".to_string(),
                side: Side::Buy,
                order_type: OrderType::Limit,
                qty: "0.001".to_string(),
                price: Some("40000".to_string()),
                position_idx: Some(PositionIdx::Long),
                order_link_id: Some(order_link_id),
                ..PlaceOrderRequest::default()
            },
            5_000,
        )
        .await;
    println!("CL: {:?}", res);

    tokio::time::sleep(Duration::from_secs(2)).await;
    let res = client
        .cancel_all_orders(
            CancelAllOrderRequest {
                category: Category::Linear,
                symbol: Some("BTCUSDT".to_string()),
                base_coin: None,
                settle_coin: None,
                order_filter: None,
                stop_order_type: None,
            },
            5_000,
        )
        .await;
    println!("CL: {:?}", res);

    tokio::time::sleep(Duration::from_secs(10)).await;
}
