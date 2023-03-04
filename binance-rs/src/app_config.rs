use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub secret_key: String,
    pub binance: AppConfigBinance,
}

#[derive(Debug, Deserialize)]
pub struct AppConfigBinance {
    pub endpoint: String,
    pub get_account: String,
    pub get_avg_price: String,
    pub get_all_orders: String,
    pub post_new_order: String,
}
