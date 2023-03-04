use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct BinanceKeypair {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceSpotAccount {
    #[serde(rename = "makerCommission")]
    pub maker_commission: i64,

    #[serde(rename = "takerCommission")]
    pub taker_commission: i64,

    #[serde(rename = "buyerCommission")]
    pub buyer_commission: i64,

    #[serde(rename = "sellerCommission")]
    pub seller_commission: i64,

    #[serde(rename = "commissionRates")]
    pub commission_rates: CommissionRates,

    #[serde(rename = "canTrade")]
    pub can_trade: bool,

    #[serde(rename = "canWithdraw")]
    pub can_withdraw: bool,

    #[serde(rename = "canDeposit")]
    pub can_deposit: bool,

    #[serde(rename = "brokered")]
    pub brokered: bool,

    #[serde(rename = "requireSelfTradePrevention")]
    pub require_self_trade_prevention: bool,

    #[serde(rename = "updateTime")]
    pub update_time: i64,

    #[serde(rename = "accountType")]
    pub account_type: String,

    #[serde(rename = "balances")]
    pub balances: Vec<Balance>,

    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Balance {
    #[serde(rename = "asset")]
    pub asset: String,

    #[serde(rename = "free")]
    pub free: F64String,

    #[serde(rename = "locked")]
    pub locked: F64String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionRates {
    #[serde(rename = "maker")]
    pub maker: F64String,

    #[serde(rename = "taker")]
    pub taker: F64String,

    #[serde(rename = "buyer")]
    pub buyer: F64String,

    #[serde(rename = "seller")]
    pub seller: F64String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    pub mins: u32,
    pub price: F64String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(try_from = "String")]
pub struct F64String(f64);

impl From<f64> for F64String {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for F64String {
    type Error = String;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        let value = v
            .parse::<f64>()
            .map_err(|e| format!("cannot parse {} to f64. {:?}", v, e))?;
        Ok(Self(value))
    }
}

impl Display for F64String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BinanceOrderSide {
    #[serde(rename = "BUY")]
    Buy,

    #[serde(rename = "SELL")]
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinanceOrderType {
    #[serde(rename = "LIMIT")]
    Limit,

    #[serde(rename = "MARKET")]
    Market,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceSpotOrderRequest {
    #[serde(rename = "symbol")]
    pub symbol: String,

    #[serde(rename = "side")]
    pub side: BinanceOrderSide,

    #[serde(rename = "type")]
    pub order_type: BinanceOrderType,

    #[serde(rename = "quantity")]
    pub quantity: F64String,

    #[serde(rename = "price")]
    pub price: Option<F64String>,

    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinanceSpotOrder {
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,

    #[serde(rename = "orderId")]
    pub order_id: Option<i64>,

    #[serde(rename = "orderListId")]
    pub order_list_id: Option<i64>,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: Option<String>,

    #[serde(rename = "price")]
    pub price: Option<String>,

    #[serde(rename = "origQty")]
    pub orig_qty: Option<String>,

    #[serde(rename = "executedQty")]
    pub executed_qty: Option<String>,

    #[serde(rename = "cummulativeQuoteQty")]
    pub cummulative_quote_qty: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,

    #[serde(rename = "timeInForce")]
    pub time_in_force: Option<String>,

    #[serde(rename = "type")]
    pub binance_spot_order_type: Option<String>,

    #[serde(rename = "side")]
    pub side: Option<String>,

    #[serde(rename = "stopPrice")]
    pub stop_price: Option<String>,

    #[serde(rename = "icebergQty")]
    pub iceberg_qty: Option<String>,

    #[serde(rename = "time")]
    pub time: Option<i64>,

    #[serde(rename = "updateTime")]
    pub update_time: Option<i64>,

    #[serde(rename = "isWorking")]
    pub is_working: Option<bool>,

    #[serde(rename = "workingTime")]
    pub working_time: Option<i64>,

    #[serde(rename = "origQuoteOrderQty")]
    pub orig_quote_order_qty: Option<String>,

    #[serde(rename = "selfTradePreventionMode")]
    pub self_trade_prevention_mode: Option<String>,
}
