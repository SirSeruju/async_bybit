pub mod model;

pub const MAINNET_URL: &str = "wss://stream.bybit.com/v5/private";
pub const TESTNET_URL: &str = "wss://stream-testnet.bybit.com/v5/private";

use std::time::Duration;

use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::error::Result;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::util::{millis, sign};
use crate::Credentials;

fn auth_req(credentials: &Credentials) -> String {
    let expires = millis() + 10000;
    let val = format!("GET/realtime{}", expires);
    let signature = sign(&credentials.secret, &val);
    let auth_req = model::Op {
        req_id: None,
        op: "auth".to_string(),
        args: vec![credentials.api_key.clone(), expires.to_string(), signature],
    };
    serde_json::to_string(&auth_req).unwrap()
}

pub struct Client {
    credentials: Credentials,
    url: &'static str,
}

impl Client {
    pub fn new(credentials: Credentials) -> Self {
        Client {
            credentials,
            url: MAINNET_URL,
        }
    }

    pub fn new_testnet(credentials: Credentials) -> Self {
        Client {
            credentials,
            url: TESTNET_URL,
        }
    }

    pub async fn connect(
        &self,
    ) -> Result<(
        UnboundedSender<model::Op>,
        UnboundedReceiver<model::Response>,
    )> {
        let (op_sender, mut op_receiver) = tokio::sync::mpsc::unbounded_channel();
        let (res_sender, res_receiver) = tokio::sync::mpsc::unbounded_channel();

        let (mut sender, mut receiver) = connect_async(self.url).await?.0.split();
        sender
            .send(Message::Text(auth_req(&self.credentials)))
            .await?;

        tokio::spawn(async move {
            while let Some(op) = op_receiver.recv().await {
                let op_text = serde_json::to_string(&op).unwrap();
                if sender.send(Message::Text(op_text)).await.is_err() {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        let data = serde_json::from_str::<model::Response>(&text);
                        let data = match data {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error: {:?} with {:?}", e, text);
                                continue;
                            }
                        };
                        if res_sender.send(data).is_err() {
                            break;
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        break;
                    }
                }
            }
        });

        let op_sender_c = op_sender.clone();
        tokio::spawn(async move {
            loop {
                if op_sender_c
                    .send(model::Op {
                        req_id: None,
                        op: "ping".to_string(),
                        args: vec![],
                    })
                    .is_err()
                {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        });

        Ok((op_sender, res_receiver))
    }
}
