use std::collections::HashMap;
use std::{error::Error, fmt::Display};

use anyhow::{Context, Result};
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct ResponseError(String);

impl Error for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub struct ReqwestFacade {}

impl ReqwestFacade {
    /// Handle a response from reqwest using anyhow
    /// Check if it is Success then try to serialize it to json
    pub fn handle_response_json<T>(resp: Response) -> Result<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        if !resp.status().is_success() {
            return Err(anyhow::Error::from(ResponseError(format!(
                "request - status {} - {}",
                resp.status(),
                resp.text().unwrap_or("<empty response>".to_owned())
            ))));
        }
        let text = resp.text().unwrap_or("".to_owned());
        log::debug!("reqwest response text: {}", text);
        let obj = serde_json::from_str::<T>(&text).context("failed json")?;

        Ok(obj)
    }

    /// Use serde_json to serialize the object into vector (key, value) to be used with reqwest client
    ///
    /// ```
    /// let request = BinanceSpotOrderRequest {
    ///     symbol: "BTCUSDT"
    ///     order_type: Limit,
    ///     side: Buy,
    ///     quantity: "0.001",
    /// };
    /// let query = object_to_query(request);
    /// assert!(query, vec![("symbol", "BTCUSDT"), ("type", "LIMIT"), ("side", "BUY"), ("quantity", "0.001")]);
    ///
    /// ```
    pub fn object_to_query<T>(obj: T) -> Vec<(String, String)>
    where
        T: for<'a> Deserialize<'a> + Serialize,
    {
        let lookup: HashMap<String, Value> =
            serde_json::from_value(serde_json::to_value::<T>(obj).unwrap()).unwrap();
        lookup
            .iter()
            .filter(|(_, val)| !val.is_null())
            .map(|(key, val)| {
                (
                    key.to_owned(),
                    serde_json::to_string(val).unwrap().replace("\"", ""),
                )
            })
            .collect::<Vec<(String, String)>>()
    }
}
