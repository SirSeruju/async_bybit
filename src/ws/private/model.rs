use serde::de::IntoDeserializer;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

fn empty_string_is_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_ref().map(String::as_str);
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum PositionIdx {
    Both = 0,
    Long = 1,
    Short = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Untriggered,
    Rejected,
    PartiallyFilledCanceled,
    Filled,
    Cancelled,
    Triggered,
    Deactivated,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
    PostOnly,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub enum TriggerPrice {
    LastPrice,
    MarkPrice,
    IndexPrice,
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

/// The pong response of private channels.
#[derive(Deserialize, Debug, Clone)]
pub struct PongResponse {
    pub req_id: Option<String>,
    pub op: String,
    pub args: [String; 1],
    pub conn_id: String,
}

/// The base response which contains common fields of private channels.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<Data> {
    /// Message ID.
    pub id: String,
    /// Topic name.
    pub topic: String,
    /// Data created timestamp (ms).
    pub creation_time: u64,
    /// The data vary on the topic.
    pub data: Data,
}

/// The position data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Product type.
    /// - Unified account: does not have this field.
    /// - Normal account: `linear`, `inverse`.
    pub category: Option<String>,
    /// Symbol name.
    pub symbol: String,
    /// Position side: `Buy`, `Sell`.
    pub side: Side,
    /// Position size.
    pub size: String,
    /// Used to identify positions in different position modes.
    /// - 0 one-way mode position.
    /// - 1 Buy side of hedge-mode position.
    /// - 2 Sell side of hedge-mode position.
    pub position_idx: PositionIdx,
    /// Trade mode. 0: cross margin, 1: isolated margin. Always 0 under unified margin account.
    pub trade_mode: u8,
    /// Position value.
    pub position_value: String,
    /// Risk limit ID.
    /// _Note_: for portfolio margin mode, it returns 0, which the risk limit value is invalid.
    pub risk_id: u16,
    /// Risk limit value corresponding to riskId.
    /// _Note_: for portfolio margin mode, it returns "", which the risk limit value is invalid.
    pub risk_limit_value: String,
    /// Entry price.
    pub entry_price: String,
    /// Mark price
    pub mark_price: String,
    /// Leverage.
    /// _Note_: for portfolio margin mode, it returns "", which the leverage value is invalid.
    pub leverage: String,
    /// Position margin. Unified account does not have this field.
    pub position_balance: Option<String>,
    /// Whether to add margin automatically. 0: false, 1: true. Unified account does not have this field.
    pub auto_add_margin: Option<u8>,
    /// Position maintenance margin.
    /// _Note_: for portfolio margin mode, it returns "".
    #[serde(alias = "positionMM")]
    pub position_mm: String,
    /// Position initial margin.
    /// _Note_: for portfolio margin mode, it returns "".
    #[serde(alias = "positionIM")]
    pub position_im: String,
    /// Est.liquidation price. "" for Unified trade(spot/linear/options).
    pub liq_price: String,
    /// Est.bankruptcy price. "" for Unified trade(spot/linear/options).
    pub bust_price: String,
    /// Tp/Sl mode: `Full`, `Partial`.
    pub tpsl_mode: String,
    /// Take profit price.
    pub take_profit: String,
    /// Stop loss price.
    pub stop_loss: String,
    /// Trailing stop.
    pub trailing_stop: String,
    /// Unrealised profit and loss.
    pub unrealised_pnl: String,
    /// Cumulative realised PnL.
    pub cum_realised_pnl: String,
    /// Position status.
    /// -`Normal`.
    /// - `Liq`: in the liquidation progress.
    /// - `Adl`: in the auto-deleverage progress.
    pub position_status: String,
    /// Position created timestamp (ms).
    pub created_time: String,
    /// Position data updated timestamp (ms).
    pub updated_time: String,
}

/// The execution data.
///
/// You may have multiple executions for one order in a single message.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    /// Product type.
    /// - Unified account: `spot`, `linear`, `option`.
    /// - Normal account: `linear`, `inverse`.
    pub category: String,
    /// Symbol name.
    pub symbol: String,
    /// Whether to borrow. Valid for `spot` only.
    /// - 0 (default): false.
    /// - 1: true.
    pub is_leverage: String,
    /// Order ID.
    pub order_id: String,
    /// User customized order ID.
    pub order_link_id: String,
    /// Side. `Buy`, `Sell`.
    pub side: Side,
    /// Order price.
    pub order_price: String,
    /// Order qty.
    pub order_qty: String,
    /// The remaining qty not executed.
    pub leaves_qty: String,
    /// Order type. `Market`, `Limit`.
    pub order_type: OrderType,
    /// Stop order type. If the order is not stop order, any type is not returned.
    pub stop_order_type: String,
    /// Executed trading fee.
    pub exec_fee: String,
    /// Execution ID.
    pub exec_id: String,
    /// Execution price.
    pub exec_price: String,
    /// Execution qty.
    pub exec_qty: String,
    /// Executed type.
    pub exec_type: String,
    /// Executed order value.
    pub exec_value: String,
    /// Executed timestamp (ms).
    pub exec_time: String,
    /// Is maker order. true: maker, false: taker.
    pub is_maker: bool,
    /// Trading fee rate.
    pub fee_rate: String,
    /// Implied volatility. Valid for option.
    pub trade_iv: String,
    /// Implied volatility of mark price. Valid for option.
    pub mark_iv: String,
    /// The mark price of the symbol when executing.
    pub mark_price: String,
    /// The index price of the symbol when executing.
    pub index_price: String,
    /// The underlying price of the symbol when executing. Valid for option.
    pub underlying_price: String,
    /// Paradigm block trade ID.
    pub block_trade_id: String,
}

/// The order data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    /// Product type.
    /// - Unified account: `spot`, `linear`, `option`.
    /// - Normal account: `linear`, `inverse`.
    pub category: String,
    /// Order ID.
    pub order_id: String,
    /// User customised order ID.
    pub order_link_id: String,
    /// Whether to borrow. `spot` returns this field only. 0 (default): false, 1: true.
    pub is_leverage: String,
    /// Block trade ID.
    pub block_trade_id: String,
    /// Symbol name.
    pub symbol: String,
    /// Order price.
    pub price: String,
    /// Order qty.
    pub qty: String,
    /// Side. `Buy`, `Sell`.
    pub side: Side,
    /// Position index. Used to identify positions in different position modes.
    pub position_idx: PositionIdx,
    /// Order status.
    pub order_status: OrderStatus,
    /// Cancel type.
    pub cancel_type: String,
    /// Reject reason.
    pub reject_reason: String,
    /// Average filled price. If unfilled, it is "".
    pub avg_price: String,
    /// The remaining qty not executed.
    pub leaves_qty: String,
    /// The remaining value not executed.
    pub leaves_value: String,
    /// Cumulative executed order qty.
    pub cum_exec_qty: String,
    /// Cumulative executed order value.
    pub cum_exec_value: String,
    /// Cumulative executed trading fee.
    pub cum_exec_fee: String,
    /// Time in force.
    pub time_in_force: TimeInForce,
    /// Order type. `Market`, `Limit`.
    pub order_type: OrderType,
    /// Stop order type.
    pub stop_order_type: String,
    /// Implied volatility.
    pub order_iv: String,
    /// Trigger price. If stopOrderType=TrailingStop, it is activate price. Otherwise, it is trigger price.
    pub trigger_price: String,
    /// Take profit price.
    pub take_profit: String,
    /// Stop loss price.
    pub stop_loss: String,
    /// The price type to trigger take profit.
    #[serde(deserialize_with = "empty_string_is_none")]
    pub tp_trigger_by: Option<TriggerPrice>,
    /// The price type to trigger stop loss.
    #[serde(deserialize_with = "empty_string_is_none")]
    pub sl_trigger_by: Option<TriggerPrice>,
    /// Trigger direction. 1: rise, 2: fall.
    pub trigger_direction: u8,
    /// The price type of trigger price.
    #[serde(deserialize_with = "empty_string_is_none")]
    pub trigger_by: Option<TriggerPrice>,
    /// Last price when place the order. For linear only.
    pub last_price_on_created: String,
    /// Reduce only. `true` means reduce position size.
    pub reduce_only: bool,
    /// Close on trigger.
    pub close_on_trigger: bool,
    /// Order created timestamp (ms).
    pub created_time: String,
    /// Order updated timestamp (ms).
    pub updated_time: String,
}

/// The wallet coin data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WalletCoin {
    /// Coin name, such as BTC, ETH, USDT, USDC.
    pub coin: String,
    /// Equity of current coin.
    pub equity: String,
    /// USD value of current coin. If this coin cannot be collateral, then it is 0.
    pub usd_value: String,
    /// Wallet balance of current coin.
    pub wallet_balance: String,
    /// Borrow amount of current coin.
    pub borrow_amount: String,
    /// Available amount to borrow of current coin.
    pub available_to_borrow: String,
    /// Available amount to withdraw of current coin.
    pub available_to_withdraw: String,
    /// Accrued interest.
    pub accrued_interest: String,
    /// Pre-occupied margin for order. For portfolio margin mode, it returns "".
    #[serde(alias = "totalOrderIM")]
    pub total_order_im: String,
    /// Sum of initial margin of all positions + Pre-occupied liquidation fee. For portfolio margin mode, it returns "".
    #[serde(alias = "totalPositionIM")]
    pub total_position_im: String,
    /// Sum of maintenance margin for all positions. For portfolio margin mode, it returns "".
    #[serde(alias = "totalPositionMM")]
    pub total_position_mm: String,
    /// Unrealised P&L.
    pub unrealised_pnl: String,
    /// Cumulative Realised P&L.
    pub cum_realised_pnl: String,
}

/// The wallet data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Account type.
    /// - Unified account: UNIFIED.
    /// - Normal account: CONTRACT.
    pub account_type: String,
    /// Initial Margin Rate: Account Total Initial Margin Base Coin / Account Margin Balance Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "accountIMRate")]
    pub account_im_rate: String,
    /// Maintenance Margin Rate: Account Total Maintenance Margin Base Coin / Account Margin Balance Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "accountMMRate")]
    pub account_mm_rate: String,
    /// Equity of account converted to usd：Account Margin Balance Base Coin + Account Option Value Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_equity: String,
    /// Wallet Balance of account converted to usd：∑ Asset Wallet Balance By USD value of each asset.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_wallet_balance: String,
    /// Margin Balance of account converted to usd：totalWalletBalance + totalPerpUPL.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_margin_balance: String,
    /// Available Balance of account converted to usd：Regular mode：totalMarginBalance - totalInitialMargin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_available_balance: String,
    /// Unrealised P&L of perpetuals of account converted to usd：∑ Each perp upl by base coin.
    /// In non-unified mode, the field will be returned as an empty string.
    #[serde(alias = "totalPerpUPL")]
    pub total_perp_upl: String,
    /// Initial Margin of account converted to usd：∑ Asset Total Initial Margin Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_initial_margin: String,
    /// Maintenance Margin of account converted to usd: ∑ Asset Total Maintenance Margin Base Coin.
    /// In non-unified mode, the field will be returned as an empty string.
    pub total_maintenance_margin: String,
    /// Coin.
    pub coin: Vec<WalletCoin>,
}

/// The greeks data.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Greek {
    /// Base coin.
    pub base_coin: String,
    /// Delta value.
    pub total_delta: String,
    /// Gamma value.
    pub total_gamma: String,
    /// Vega value.
    pub total_vega: String,
    /// Theta value.
    pub total_theta: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Response {
    Position(BaseResponse<Vec<Position>>),
    Execution(BaseResponse<Vec<Execution>>),
    Order(BaseResponse<Vec<Order>>),
    Wallet(BaseResponse<Vec<Wallet>>),
    Greek(BaseResponse<Vec<Greek>>),
    Pong(PongResponse),
    Op(OpResponse),
}

#[derive(Serialize)]
pub struct Op {
    pub req_id: Option<String>,
    pub op: String,
    pub args: Vec<String>,
}
