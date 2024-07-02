pub mod model;

use std::time::Duration;

use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::error::Result;
use tokio_tungstenite::tungstenite::protocol::Message;

#[macro_export]
macro_rules! define_client {
    ($name:ident, $response:ident, $mainnet_url:ident, $testnet_url:ident) => {
        pub struct $name {
            url: &'static str,
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl $name {
            pub fn new() -> Self {
                $name { url: $mainnet_url }
            }

            pub fn new_testnet() -> Self {
                $name { url: $testnet_url }
            }

            pub async fn connect(
                &self,
            ) -> Result<(UnboundedSender<model::Op>, UnboundedReceiver<$response>)> {
                let (op_sender, mut op_receiver) = tokio::sync::mpsc::unbounded_channel();
                let (res_sender, res_receiver) = tokio::sync::mpsc::unbounded_channel();

                let (mut sender, mut receiver) = connect_async(self.url).await?.0.split();

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
                                let data = serde_json::from_str::<$response>(&text);
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
    };
}

pub const SPOT_MAINNET_URL: &str = "wss://stream.bybit.com/v5/public/spot";
pub const SPOT_TESTNET_URL: &str = "wss://stream-testnet.bybit.com/v5/public/spot";
pub const FUTURES_MAINNET_URL: &str = "wss://stream.bybit.com/v5/public/linear";
pub const FUTURES_TESTNET_URL: &str = "wss://stream-testnet.bybit.com/v5/public/linear";
pub const OPTION_MAINNET_URL: &str = "wss://stream.bybit.com/v5/public/option";
pub const OPTION_TESTNET_URL: &str = "wss://stream-testnet.bybit.com/v5/public/option";

use model::{FutureResponse, OptionResponse, SpotResponse};

define_client!(SpotClient, SpotResponse, SPOT_MAINNET_URL, SPOT_TESTNET_URL);
define_client!(
    FutureClient,
    FutureResponse,
    FUTURES_MAINNET_URL,
    FUTURES_TESTNET_URL
);
define_client!(
    OptionClient,
    OptionResponse,
    OPTION_MAINNET_URL,
    OPTION_TESTNET_URL
);
