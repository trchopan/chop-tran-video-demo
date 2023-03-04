use std::{fs, path::PathBuf, time::Duration};

use crate::{
    app_config::AppConfigBinance,
    domain::binance::{
        BinanceKeypair, BinanceSpotAccount, BinanceSpotOrder, BinanceSpotOrderRequest, Price,
    },
    infrastructure::reqwest_facade::ReqwestFacade,
};
use anyhow::{Context, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::header;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct BinanceRepo {
    cfg: AppConfigBinance,
    client: reqwest::blocking::Client,
    keypair: BinanceKeypair,
}

impl BinanceRepo {
    pub fn new(cfg: AppConfigBinance, keypair: BinanceKeypair) -> Self {
        let api_key_header =
            header::HeaderValue::from_str(&keypair.key).expect("cannot create header from api_key");
        let mut headers = header::HeaderMap::new();
        headers.insert("X-MBX-APIKEY", api_key_header);
        let client = reqwest::blocking::Client::builder()
            .https_only(true)
            .default_headers(headers)
            .timeout(Duration::from_secs(10))
            .build()
            .expect("cannot create reqwest client");
        Self {
            cfg,
            client,
            keypair,
        }
    }

    /// Create a HMAC SHA256 signature given a secret and raw message
    ///
    /// ```
    /// create_signature("my secret and secure key", "input message")
    /// // 97d2a569059bbcd8ead4444ff99071f4c01d005bcefe0d3567e1be628e5fdcd9
    /// ```
    fn create_signature(&self, raw: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.keypair.secret.as_bytes())
            .expect("HMAC can't take key of any size");
        mac.update(raw.as_bytes());
        let signature = mac.finalize().into_bytes();
        hex::encode(signature)
    }

    /// Compose the query for the request in the form of vector (key, value) pair.
    /// Add a timestamp field and produce a signature for the request.
    ///
    /// ```
    /// let query = compose_query("secret", vec![("symbol", "BTCUSDT")]);
    /// assert!(query, vec![("symbol", "BTCUSDT"), ("timestamp", "1675659775601"), ("signature", "0f1234abcdef")]);
    /// ```
    fn compose_query(&self, query: Vec<(String, String)>) -> Vec<(String, String)> {
        let timestamp = Utc::now().timestamp_millis();
        let mut query: Vec<(String, String)> = query;
        query.push(("timestamp".to_owned(), timestamp.to_string()));
        let raw_query: Vec<String> = query
            .iter()
            .map(|(key, value)| key.to_string() + "=" + value)
            .collect();
        let raw_query = raw_query.join("&");
        let signature = self.create_signature(&raw_query);
        query.push(("signature".to_owned(), signature));
        query
    }

    fn make_url(&self, path: &str) -> String {
        self.cfg.endpoint.to_owned() + path
    }

    /// Get the account information of binance SPOT Account
    pub fn get_account(&self) -> Result<BinanceSpotAccount> {
        let query = self.compose_query(vec![]);
        let resp = self
            .client
            .get(self.make_url(&self.cfg.get_account))
            .query(&query)
            .send()?;
        ReqwestFacade::handle_response_json::<BinanceSpotAccount>(resp)
    }

    /// Get price of given symbol.
    ///
    /// ```
    /// get_price("BTCUSDT")
    /// ```
    pub fn get_price(&self, symbol: &str) -> Result<Price> {
        let resp = self
            .client
            .get(self.make_url(&self.cfg.get_avg_price))
            .query(&[("symbol".to_owned(), symbol)])
            .send()?;
        ReqwestFacade::handle_response_json::<Price>(resp)
    }

    /// Get orders
    ///
    /// ```
    /// get_orders("BTCUSDT")
    /// ```
    pub fn get_orders(&self, symbol: &str) -> Result<Vec<BinanceSpotOrder>> {
        let query = vec![("symbol".to_owned(), symbol.to_owned())];
        let query = self.compose_query(query);
        let resp = self
            .client
            .get(self.make_url(&self.cfg.get_all_orders))
            .query(&query)
            .send()?;
        ReqwestFacade::handle_response_json::<Vec<BinanceSpotOrder>>(resp)
    }

    /// Request Binance API to create the order
    /// Use [read_order_from_file] to construct the order request
    pub fn make_spot_order(&self, order: BinanceSpotOrderRequest) -> Result<BinanceSpotOrder> {
        let query = ReqwestFacade::object_to_query::<BinanceSpotOrderRequest>(order);
        let query = self.compose_query(query);
        let resp = self
            .client
            .post(self.make_url(&self.cfg.post_new_order))
            .query(&query)
            .send()?;
        ReqwestFacade::handle_response_json::<BinanceSpotOrder>(resp)
    }

    /// Given a toml file_path read and parse the file to <BinanceSpotOrderRequest>
    pub fn read_order_from_file(file_path: PathBuf) -> Result<BinanceSpotOrderRequest> {
        let content = fs::read_to_string(file_path).context("cannot read order from file")?;
        let order = toml::from_str::<BinanceSpotOrderRequest>(&content)
            .context("cannot parse order from json")?;
        Ok(order)
    }
}
