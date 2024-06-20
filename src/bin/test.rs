use std::time::Duration;

use async_bybit::rest::model::InstrumentsInfoRequest;
use async_bybit::rest::Client;
use async_bybit::ws::private::{model, Client as ClientWS};
use async_bybit::Credentials;

const URL: &str = "https://api.bybit.com";

#[tokio::main]
async fn main() {
    let credentials = Credentials {
        api_key: "ykd3WyNknCn1mqTjD1".to_owned(),
        secret: "5UjnYErTJycxv9ZL4pg5v4Mqv7HS2tlA7CC8".to_owned(),
    };
    let client = Client::new(credentials.clone(), URL.to_string(), None);
    // println!("CL: {:?}", client.get_signed("https://api.bybit.com/v5/position/list".to_string(), ""));
    let res = client
        .get_instruments_info(InstrumentsInfoRequest {
            category: "linear".to_string(),
            symbol: None,
            status: None,
            base_coin: None,
            limit: None,
            cursor: None,
        })
        .await;
    println!("CL: {:?}", res);
    let client_ws = ClientWS::new(credentials).await.unwrap();

    let (sx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    client_ws.subscribe(sx);
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("MSG: {:?}", msg);
        }
    });
    tokio::time::sleep(Duration::from_secs(2)).await;
    client_ws
        .send_op(model::Op {
            req_id: None,
            op: "subscribe".to_string(),
            args: vec!["order".to_string()],
        })
        .await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    tokio::time::sleep(Duration::from_secs(60)).await;
}
