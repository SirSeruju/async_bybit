pub mod model;

pub const MAINNET_URL: &str = "https://api.bybit.com";
pub const TESTNET_URL: &str = "https://api-testnet.bybit.com";

use std::time::Duration;

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    util::{millis, sign},
    Credentials,
};

use self::model::{
    CancelAllOrderRequest, CancelOrderRequest, InstrumentsInfoRequest, PlaceOrderRequest,
};
use self::model::{
    CancelAllOrderResponse, CancelOrderResponse, InstrumentsInfoResponse, PlaceOrderResponse,
    Response,
};

#[macro_export]
macro_rules! handle {
    ($name:ident, $endpoint:expr, $method:ident, $params:expr, $request:ident, $response:ident) => {
        pub async fn $name(
            &self,
            request: $request,
        ) -> Result<Response<$response>, reqwest::Error> {
            self.request(
                $endpoint.to_string(),
                Method::$method,
                Some($params(request)),
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! handle_sig {
    ($name:ident, $endpoint:expr, $method:ident, $params:expr, $request:ident, $response:ident) => {
        pub async fn $name(
            &self,
            request: $request,
            recv_window: u64,
        ) -> Result<Response<$response>, reqwest::Error> {
            self.request_signed(
                $endpoint.to_string(),
                Method::$method,
                recv_window,
                Some($params(request)),
            )
            .await
        }
    };
}

pub enum Params<T> {
    Query(T),
    Body(T),
}

#[derive(Clone)]
pub struct Client {
    credentials: Credentials,
    inner: reqwest::Client,
    host: &'static str,
}

impl Client {
    pub fn new(credentials: Credentials, timeout: Option<u64>) -> Self {
        let mut builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        if let Some(timeout_secs) = timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs))
        }
        Client {
            credentials,
            inner: builder.build().unwrap(),
            host: MAINNET_URL,
        }
    }

    pub fn new_testnet(credentials: Credentials, timeout: Option<u64>) -> Self {
        let mut builder: reqwest::ClientBuilder = reqwest::ClientBuilder::new();
        if let Some(timeout_secs) = timeout {
            builder = builder.timeout(Duration::from_secs(timeout_secs))
        }
        Client {
            credentials,
            inner: builder.build().unwrap(),
            host: TESTNET_URL,
        }
    }

    handle_sig!(
        place_order,
        "/v5/order/create",
        POST,
        Params::Body,
        PlaceOrderRequest,
        PlaceOrderResponse
    );
    handle_sig!(
        cancel_order,
        "/v5/order/cancel",
        POST,
        Params::Body,
        CancelOrderRequest,
        CancelOrderResponse
    );
    handle_sig!(
        cancel_all_orders,
        "/v5/order/cancel-all",
        POST,
        Params::Body,
        CancelAllOrderRequest,
        CancelAllOrderResponse
    );
    handle!(
        get_instruments_info,
        "/v5/market/instruments-info",
        GET,
        Params::Query,
        InstrumentsInfoRequest,
        InstrumentsInfoResponse
    );

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
