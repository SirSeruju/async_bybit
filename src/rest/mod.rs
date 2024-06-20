pub mod model;

use std::time::Duration;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    util::{millis, sign},
    Credentials,
};

use self::model::{CancelAllOrderRequest, CancelOrderRequest, InstrumentsInfoRequest};
use self::model::{CancelAllOrderResponse, CancelOrderResponse, InstrumentsInfoResponse, Response};

pub enum Params<T> {
    Query(T),
    Body(T),
}

#[derive(Clone)]
pub struct Client {
    credentials: Credentials,
    inner: reqwest::Client,
    host: String,
}

impl Client {
    pub fn new(credentials: Credentials, host: String, timeout: Option<u64>) -> Self {
        let mut builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        if let Some(timeout_secs) = timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs))
        }
        Client {
            credentials,
            inner: builder.build().unwrap(),
            host,
        }
    }

    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
        recv_window: u64,
    ) -> Result<Response<CancelOrderResponse>, reqwest::Error> {
        self.request_signed(
            "/v5/order/cancel".to_string(),
            Method::POST,
            recv_window,
            Some(Params::Body(request)),
        )
        .await
    }

    pub async fn cancel_all_order(
        &self,
        request: CancelAllOrderRequest,
        recv_window: u64,
    ) -> Result<Response<CancelAllOrderResponse>, reqwest::Error> {
        self.request_signed(
            "/v5/order/cancel-all".to_string(),
            Method::POST,
            recv_window,
            Some(Params::Body(request)),
        )
        .await
    }

    pub async fn get_instruments_info(
        &self,
        request: InstrumentsInfoRequest,
    ) -> Result<Response<InstrumentsInfoResponse>, reqwest::Error> {
        self.request(
            "/v5/market/instruments-info".to_string(),
            Method::GET,
            Some(Params::Query(request)),
        )
        .await
    }

    async fn request<P: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: String,
        method: Method,
        params: Option<Params<P>>,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.host, endpoint);

        let builder = self.inner.request(method, url);

        let request = match params {
            Some(Params::Body(b)) => {
                let msg = serde_json::to_string(&b).unwrap();
                builder.body(msg).build()?
            }
            Some(Params::Query(q)) => {
                let msg = serde_qs::to_string(&q).unwrap();
                let mut req = builder.build()?;
                req.url_mut().set_query(Some(&msg));
                req
            }
            None => builder.build()?,
        };
        self.inner.execute(request).await?.json::<R>().await
    }

    async fn request_signed<P: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: String,
        method: Method,
        recv_window: u64,
        params: Option<Params<P>>,
    ) -> Result<R, reqwest::Error> {
        let url = format!("{}{}", self.host, endpoint);
        let timestamp = millis().to_string();
        let api_key = self.credentials.api_key.clone();
        let recv_window = recv_window.to_string();

        let builder = self
            .inner
            .request(method, url)
            .header("X-BAPI-API-KEY", self.credentials.api_key.clone())
            .header("X-BAPI-TIMESTAMP", timestamp.clone())
            .header("X-BAPI-RECV-WINDOW", recv_window.to_string());

        let request = match params {
            Some(Params::Body(b)) => {
                let msg = serde_json::to_string(&b).unwrap();
                let signature = sign(
                    &self.credentials.secret,
                    &format!("{}{}{}{}", timestamp, api_key, recv_window, msg),
                );
                builder.header("X-BAPI-SIGN", signature).body(msg).build()?
            }
            Some(Params::Query(q)) => {
                let msg = serde_qs::to_string(&q).unwrap();
                let signature = sign(
                    &self.credentials.secret,
                    &format!("{}{}{}{}", timestamp, api_key, recv_window, msg),
                );
                let mut req = builder.header("X-BAPI-SIGN", signature).build()?;
                req.url_mut().set_query(Some(&msg));
                req
            }
            None => {
                let signature = sign(
                    &self.credentials.secret,
                    &format!("{}{}{}{}", timestamp, api_key, recv_window, ""),
                );
                builder.header("X-BAPI-SIGN", signature).build()?
            }
        };
        self.inner.execute(request).await?.json::<R>().await
    }
}
