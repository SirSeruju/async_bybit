use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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
    pub ret_code: u64,
    pub ret_msg: String,
    #[serde(deserialize_with = "deserialize_empty_object")]
    pub result: Option<T>,
    pub ret_ext_info: HashMap<String, String>,
    pub time: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PositionIdx {
    Both = 0,
    Long = 1,
    Short = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerPrice {
    LastPrice,
    MarkPrice,
    IndexPrice,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TpslMode {
    Full,
    Partial,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    PostOnly,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    pub category: Category,
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
    pub category: Category,
    pub symbol: Option<String>,
    pub base_coin: Option<String>,
    pub settle_coin: Option<String>,
    pub order_filter: Option<String>,
    pub stop_order_type: Option<OrderType>,
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
    pub category: Category,
    pub symbol: Option<String>,
    pub status: Option<String>,
    pub base_coin: Option<String>,
    pub limit: Option<u64>,
    pub cursor: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentsInfoResponse {
    pub category: Category,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub category: Category,
    pub symbol: String,
    pub is_leverage: Option<u8>,
    pub side: Side,
    pub order_type: OrderType,
    pub qty: String,
    pub market_unit: Option<String>,
    pub price: Option<String>,
    pub trigger_direction: Option<u8>,
    pub order_filter: Option<String>,
    pub trigger_price: Option<String>,
    pub trigger_by: Option<TriggerPrice>,
    pub order_iv: Option<String>,
    pub time_in_force: Option<TimeInForce>,
    pub position_idx: Option<PositionIdx>,
    pub order_link_id: Option<String>,
    pub take_profit: Option<String>,
    pub stop_loss: Option<String>,
    pub tp_trigger_by: Option<TriggerPrice>,
    pub sl_trigger_by: Option<TriggerPrice>,
    pub reduce_only: Option<bool>,
    pub close_on_trigger: Option<bool>,
    pub smp_type: Option<String>,
    pub mmp: Option<bool>,
    pub tpsl_mode: Option<TpslMode>,
    pub tp_limit_price: Option<String>,
    pub sl_limit_price: Option<String>,
    pub tp_order_type: Option<OrderType>,
    pub sl_order_type: Option<OrderType>,
}

impl Default for PlaceOrderRequest {
    fn default() -> Self {
        PlaceOrderRequest {
            category: Category::Spot,
            symbol: "".to_string(),
            is_leverage: None,
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "".to_string(),
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub order_link_id: String,
}
