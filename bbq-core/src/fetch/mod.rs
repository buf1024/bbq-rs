use anyhow::Result;
use async_trait::async_trait;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

mod trade_date;
pub use trade_date::is_trade_date;

mod sina;
pub use sina::Sina;

pub type RtQuot = HashMap<String, Quot>;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Quot {
    pub code: String,
    pub name: String,
    pub open: f64,
    pub pre_close: f64,
    pub now: f64,
    pub high: f64,
    pub low: f64,
    pub buy: f64,
    pub sell: f64,
    pub vol: u64,
    pub amount: f64,
    pub bid: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    pub ask: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    pub date: String,
    pub time: String,
}

pub type StockBarList = Vec<StockBar>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct StockBar {
    #[serde(rename = "day")]
    pub time: String,
    #[serde(deserialize_with = "from_str2f64")]
    pub open: f64,
    #[serde(deserialize_with = "from_str2f64")]
    pub high: f64,
    #[serde(deserialize_with = "from_str2f64")]
    pub low: f64,
    #[serde(deserialize_with = "from_str2f64")]
    pub close: f64,
    #[serde(rename = "volume", deserialize_with = "from_str2u64")]
    pub vol: u64,
}

fn from_str2f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(D::Error::custom)
}

fn from_str2u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(D::Error::custom)
}

#[async_trait]
pub trait Fetcher: Send + Sync {
    async fn fetch_stock_minute(&self, code: &str, min: u32) -> Result<StockBarList>;
    async fn fetch_rt_quot(&self, codes: &Vec<String>) -> Result<RtQuot>;
}

pub fn is_index(code: &str) -> bool {
    static INDEXES: [&str; 12] = [
        "sh000001", "sz399001", "sz399006", "sz399102", "sz399005", "sh000300", "sh000688",
        "sz399673", "sz399550", "sz399678", "sz399007", "sz399008",
    ];
    INDEXES.contains(&code)
}
