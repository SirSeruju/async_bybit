use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}

/// The pong/subscription response.
#[derive(Deserialize, Debug, Clone)]
pub struct OpResponse {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub req_id: Option<String>,
    pub op: String,
}

/// The option pong response of public channels.
#[derive(Deserialize, Debug, Clone)]
pub struct OptionPongResponse {
    pub args: [String; 1],
    pub op: String,
}

/// The data in option subscription response.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionSubscriptionData {
    pub fail_topics: Vec<String>,
    pub success_topics: Vec<String>,
}

/// The option subscription response.
#[derive(Deserialize, Debug, Clone)]
pub struct OptionSubscriptionResponse {
    pub success: bool,
    pub conn_id: String,
    pub data: OptionSubscriptionData,
    #[serde(alias = "type")]
    pub type_: String,
}

/// The base response which contains common fields of public channels.
#[derive(Deserialize, Debug, Clone)]
pub struct BaseResponse<Data> {
    /// Topic name.
    pub topic: String,
    /// Data type. `snapshot`, `delta`.
    #[serde(alias = "type")]
    pub type_: String,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The base ticker response which contains common fields.
#[derive(Deserialize, Debug, Clone)]
pub struct BaseTickerResponse<Data> {
    /// Topic name.
    pub topic: String,
    /// Data type. `snapshot`, `delta`.
    #[serde(alias = "type")]
    pub type_: String,
    /// Cross sequence.
    pub cs: u64,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The spot/future ticker data.
    pub data: Data,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BaseOptionResponse<Data> {
    /// message ID
    pub id: String,
    /// Topic name.
    pub topic: String,
    #[serde(alias = "type")]
    /// Data type. `snapshot`.
    pub type_: String,
    /// The timestamp (ms) that the system generates the data.
    pub ts: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The (price, size) pair of orderbook.
#[derive(Deserialize, Debug, Clone)]
pub struct OrderbookItem(pub String, pub String);

/// The orderbook data.
#[derive(Deserialize, Debug, Clone)]
pub struct Orderbook {
    /// Symbol name.
    pub s: String,
    /// Bids. For `snapshot` stream, the element is sorted by price in descending order.
    pub b: Vec<OrderbookItem>,
    /// Asks. For `snapshot` stream, the element is sorted by price in ascending order.
    pub a: Vec<OrderbookItem>,
    /// Update ID. Is a sequence.
    /// Occasionally, you'll receive "u"=1, which is a snapshot data due to the restart of the service.
    /// So please overwrite your local orderbook.
    pub u: u64,
    /// Cross sequence. Option does not have this field.
    pub seq: Option<u64>,
}

/// The trade data.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    /// The timestamp (ms) that the order is filled.
    pub T: u64,
    /// Symbol name.
    pub s: String,
    /// Side. `Buy`, `Sell`.
    pub S: Side,
    /// Trade size.
    pub v: String,
    /// Trade price.
    pub p: String,
    /// Direction of price change. Unique field for future.
    pub L: Option<String>,
    /// Trade ID.
    pub i: String,
    /// Whether it is a block trade order or not.
    pub BT: bool,
}

/// The spot ticker data. (`snapshot` only)
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpotTicker {
    /// Symbol name.
    pub symbol: String,
    /// Last price.
    pub last_price: String,
    /// The highest price in the last 24 hours.
    pub high_price_24h: String,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: String,
    /// Percentage change of market price relative to 24h.
    pub prev_price_24h: String,
    /// Volume for 24h.
    pub volume_24h: String,
    /// Turnover for 24h.
    pub turnover_24h: String,
    /// Percentage change of market price relative to 24h.
    pub price_24h_pcnt: String,
    /// USD index price. It can be empty.
    pub usd_index_price: String,
}

/// The option ticker data. (`snapshot` only)
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OptionTicker {
    /// Symbol name.
    pub symbol: String,
    /// Best bid price.
    pub bid_price: String,
    /// Best bid size.
    pub bid_size: String,
    /// Best bid iv.
    pub bid_iv: String,
    /// Best ask price.
    pub ask_price: String,
    /// Best ask size.
    pub ask_size: String,
    /// Best ask iv.
    pub ask_iv: String,
    /// Last price.
    pub last_price: String,
    /// The highest price in the last 24 hours.
    pub high_price_24h: String,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: String,
    /// Market price.
    pub mark_price: String,
    /// Index price.
    pub index_price: String,
    /// Mark price iv.
    pub mark_price_iv: String,
    /// Underlying price.
    pub underlying_price: String,
    /// Open interest size.
    pub open_interest: String,
    /// Turnover for 24h.
    pub turnover_24h: String,
    /// Volume for 24h.
    pub volume_24h: String,
    /// Total volume.
    pub total_volume: String,
    /// Total turnover.
    pub total_turnover: String,
    /// Delta.
    pub delta: String,
    /// Gamma.
    pub gamma: String,
    /// Vega.
    pub vega: String,
    /// Theta.
    pub theta: String,
    /// Predicated delivery price. It has value when 30 min before delivery.
    pub predicted_delivery_price: String,
    /// The change in the last 24 hous.
    pub change_24h: String,
}

/// The future ticker data.
///
/// This data utilises the snapshot field and delta field. `None` means field value has not changed.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FutureTicker {
    /// Symbol name.
    pub symbol: String,
    /// Tick direction.
    pub tick_direction: Option<String>,
    /// Percentage change of market price in the last 24 hours.
    pub price_24h_pcnt: Option<String>,
    /// Last price.
    pub last_price: Option<String>,
    /// Market price 24 hours ago.
    pub prev_price_24h: Option<String>,
    /// The highest price in the last 24 hours.
    pub high_price_24h: Option<String>,
    /// The lowest price in the last 24 hours.
    pub low_price_24h: Option<String>,
    /// Market price an hour ago.
    pub prev_price_1h: Option<String>,
    /// Mark price.
    pub mark_price: Option<String>,
    /// Index price.
    pub index_price: Option<String>,
    /// Open interest size.
    pub open_interest: Option<String>,
    /// Open interest value.
    pub open_interest_value: Option<String>,
    /// Turnover for 24h.
    pub turnover_24h: Option<String>,
    /// Volume for 24h.
    pub volume_24h: Option<String>,
    /// Next funding timestamp (ms).
    pub next_funding_time: Option<String>,
    /// Funding rate.
    pub funding_rate: Option<String>,
    /// Best bid price.
    pub bid1_price: Option<String>,
    /// Best bid size.
    pub bid1_size: Option<String>,
    /// Best ask price.
    pub ask1_price: Option<String>,
    /// Best ask size.
    pub ask1_size: Option<String>,
    /// Delivery date time (UTC+0). Unique field for inverse futures.
    pub delivery_time: Option<String>,
    /// Delivery fee rate. Unique field for inverse futures.
    pub basis_rate: Option<String>,
    /// Delivery fee rate. Unique field for inverse futures.
    pub delivery_fee_rate: Option<String>,
    /// Predicated delivery price. Unique field for inverse futures.
    pub predicted_delivery_price: Option<String>,
}

/// The (leveraged token) kline data.
#[derive(Deserialize, Debug, Clone)]
pub struct Kline {
    /// The start timestamp (ms)
    pub start: u64,
    /// The end timestamp (ms). It is current timestamp if it does not reach to the end time of candle.
    pub end: u64,
    /// Kline interval.
    pub interval: String,
    /// Open price.
    pub open: String,
    /// Close price.
    pub close: String,
    /// Highest price.
    pub high: String,
    /// Lowest price.
    pub low: String,
    /// Trade volume. Leveraged token does not have this field.
    pub volume: Option<String>,
    /// Turnover. Leveraged token does not have this field.
    pub turnover: Option<String>,
    /// Weather the tick is ended or not.
    pub confirm: bool,
    /// The timestamp (ms) of the last matched order in the candle.
    pub timestamp: u64,
}

/// The liquidation data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Liquidation {
    /// The updated timestamp (ms).
    pub updated_time: u64,
    /// Symbol name.
    pub symbol: String,
    /// Order side. `Buy`, `Sell`.
    pub side: Side,
    /// Executed size.
    pub size: String,
    /// Executed price.
    pub price: String,
}

// The leveraged token ticker data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LtTicker {
    /// Symbol name.
    pub symbol: String,
    /// Market price change percentage in the past 24 hours.
    pub price_24h_pcnt: String,
    /// The last price.
    pub last_price: String,
    /// Market price 24 hours ago.
    pub prev_price_24h: String,
    /// Highest price in the past 24 hours.
    pub high_price_24h: String,
    /// Lowest price in the past 24 hours.
    pub low_price24h: String,
}

/// The leveraged token nav data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LtNav {
    /// The generated timestamp of nav.
    pub time: u64,
    /// Symbol name.
    pub symbol: String,
    /// Net asset value.
    pub nav: String,
    /// Total position value = basket value * total circulation.
    pub basket_position: String,
    /// Leverage.
    pub leverage: String,
    /// Basket loan.
    pub basket_loan: String,
    /// Circulation.
    pub circulation: String,
    /// Basket.
    pub basket: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SpotResponse {
    Orderbook(BaseResponse<Orderbook>),
    Trade(BaseResponse<Vec<Trade>>),
    Ticker(BaseTickerResponse<SpotTicker>),
    Kline(BaseResponse<Vec<Kline>>),
    LtTicker(BaseResponse<LtTicker>),
    LtNav(BaseResponse<LtNav>),
    Op(OpResponse),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum FutureResponse {
    Orderbook(BaseResponse<Orderbook>),
    Trade(BaseResponse<Vec<Trade>>),
    // Box because large size difference between variants
    Ticker(Box<BaseTickerResponse<FutureTicker>>),
    Kline(BaseResponse<Vec<Kline>>),
    Liquidation(BaseResponse<Liquidation>),
    Op(OpResponse),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum OptionResponse {
    Orderbook(BaseOptionResponse<Orderbook>),
    Trade(BaseOptionResponse<Vec<Trade>>),
    // Box because large size difference between variants
    Ticker(Box<BaseOptionResponse<OptionTicker>>),
    Pong(OptionPongResponse),
    Subscription(OptionSubscriptionResponse),
}

#[derive(Serialize, Clone)]
pub struct Op {
    pub req_id: Option<String>,
    pub op: String,
    pub args: Vec<String>,
}
