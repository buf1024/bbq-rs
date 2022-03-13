mod trade_date;
mod sina;

pub use trade_date::is_trade_date;

use async_trait::async_trait;
use std::collections::HashMap;
use chrono::{NaiveDate, NaiveTime, DateTime, Utc, Local, Datelike};
use crate::QResult;

pub type RtQuot = HashMap<String, Quot>;

#[derive(Clone, Debug)]
pub struct Quot {
    pub code: String,
    pub name: String,
    pub open: f64,
    pub pre_open: f64,
    pub now: f64,
    pub high: f64,
    pub low: f64,
    pub buy: f64,
    pub sell: f64,
    pub vol: u32,
    pub amount: f64,
    pub bid: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    pub ask: ((u32, f64), (u32, f64), (u32, f64), (u32, f64), (u32, f64)),
    pub date: NaiveDate,
    pub time: NaiveTime,
}

#[async_trait]
pub trait Fetcher: Send + Sync {
    async fn fetch_rt_quot(&self, codes: &Vec<String>) -> QResult<RtQuot>;
}
