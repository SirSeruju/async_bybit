use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

pub fn deserialize_empty_object<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(
        untagged,
        deny_unknown_fields,
        expecting = "object, empty object or null"
    )]
    enum Helper<T> {
        Data(T),
        Empty {},
        Null,
    }
    match Helper::deserialize(deserializer) {
        Ok(Helper::Data(data)) => Ok(Some(data)),
        Ok(_) => Ok(None),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response<T: DeserializeOwned> {
    ret_code: u64,
    ret_msg: String,
    #[serde(deserialize_with = "deserialize_empty_object")]
    result: Option<T>,
    ret_ext_info: HashMap<String, String>,
    time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: String,
    pub symbol: String,
    pub order_id: Option<String>,
    pub order_link_id: Option<String>,
    pub order_filter: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub order_filter: Option<String>,
    pub stop_order_type: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderResponse {
    pub list: Vec<CancelOrderResponse>,
    pub success: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoRequest {
    pub category: String,
    pub symbol: Option<String>,
    pub status: Option<String>,
    pub base_coin: Option<String>,
    pub limit: Option<u64>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResponse {
    pub category: String,
    pub list: Vec<InstrumentInfo>,
    pub next_page_cursor: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    pub symbol: String,
    pub contract_type: String,
    pub status: String,
    pub base_coin: String,
    pub quote_coin: String,
    pub launch_time: String,
    pub delivery_time: String,
    pub delivery_fee_rate: String,
    pub price_scale: String,
    pub leverage_filter: LeverageFilter,
    pub price_filter: PriceFilter,
    pub lot_size_filter: LotSizeFilter,
    pub unified_margin_trade: bool,
    pub funding_interval: u64,
    pub settle_coin: String,
    pub copy_trading: String,
    pub upper_funding_rate: String,
    pub lower_funding_rate: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    pub min_leverage: String,
    pub max_leverage: String,
    pub leverage_step: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    pub max_order_qty: String,
    pub max_mkt_order_qty: String,
    pub min_order_qty: String,
    pub qty_step: String,
    pub post_only_max_order_qty: String,
    pub min_notional_value: String,
}
