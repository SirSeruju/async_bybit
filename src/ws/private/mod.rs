pub mod model;

pub const MAINNET_PRIVATE: &str = "wss://stream.bybit.com/v5/private";
pub const TESTNET_PRIVATE: &str = "wss://stream-testnet.bybit.com/v5/private";

use std::mem;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures_util::sink::SinkExt;
use futures_util::StreamExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{self, protocol::Message};

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

fn send_subscribers<E: Clone>(subscribers: Arc<Mutex<Vec<UnboundedSender<E>>>>, event: E) {
    let mut old_subscribers = subscribers.lock().unwrap();
    let mut subscribers = Vec::with_capacity(old_subscribers.len());
    for subscriber in old_subscribers.drain(..) {
        if subscriber.send(event.clone()).is_ok() {
            subscribers.push(subscriber)
        }
    }
    mem::swap(&mut subscribers, &mut old_subscribers);
}

pub struct Client {
    sender: UnboundedSender<Message>,
    subscribers: Arc<Mutex<Vec<UnboundedSender<model::Response>>>>,
}

impl Client {
    pub async fn new(credentials: Credentials) -> Result<Self, tungstenite::error::Error> {
        let (sx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let (mut sender, mut receiver) = connect_async(MAINNET_PRIVATE).await?.0.split();
        let subscribers = Arc::new(Mutex::new(Vec::new()));

        let subscribers_c = subscribers.clone();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    pck = receiver.next() => {
                        match pck {
                            Some(pck) => match pck {
                                Ok(pck) => match pck {
                                    Message::Text(msg) => {send_subscribers(
                                        subscribers_c.clone(),
                                        serde_json::from_str::<model::Response>(&msg).unwrap()
                                    )},
                                    _ => {},
                                },
                                Err(e) => eprintln!("Error: {:?}", e),
                            },
                            None => {},
                        }

                    }
                    pck = rx.recv() => {
                        match pck {
                            Some(m) => {sender.send(m).await.unwrap();},
                            None => {},
                        }
                    }
                }
            }
        });

        let sx_c = sx.clone();
        tokio::spawn(async move {
            loop {
                sx_c.send(Message::Text(
                    serde_json::to_string(&model::Op {
                        req_id: None,
                        op: "ping".to_string(),
                        args: vec![],
                    })
                    .unwrap(),
                ))
                .unwrap();
                tokio::time::sleep(Duration::from_secs(20)).await;
            }
        });

        sx.send(Message::Text(auth_req(&credentials))).unwrap();
        Ok(Client {
            sender: sx,
            subscribers,
        })
    }

    pub fn subscribe(&self, sender: UnboundedSender<model::Response>) {
        self.subscribers.lock().unwrap().push(sender);
    }

    pub async fn send_op(&self, op: model::Op) {
        self.sender
            .send(Message::Text(serde_json::to_string(&op).unwrap()))
            .unwrap()
    }
}
