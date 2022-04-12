use crate::fetch::{Quot};
use crate::Kind;
use chrono::{NaiveDate, NaiveDateTime};
use mongodb::bson::doc;

use serde::{Deserialize, Serialize};
use std::collections::{HashMap};
use std::fmt::Debug;

pub const FREQ_1M: u32 = 60;
pub const FREQ_5M: u32 = 5 * 60;
pub const FREQ_15M: u32 = 15 * 60;
pub const FREQ_30M: u32 = 30 * 60;
pub const FREQ_60M: u32 = 60 * 60;
pub const FREQ_1D: u32 = 24 * 60 * 60;

pub type RtQuotBar = HashMap<String, QuotBar>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuotBar {
    pub frequency: u32,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub start: String,
    pub end: String,

    pub quot: Quot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuotStatus {
    pub opts: QuotOpts,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotData {
    Quot(RtQuotBar),
    QuotStart(QuotStatus),
    MorningStart(QuotStatus),
    MorningEnd(QuotStatus),
    NoonStart(QuotStatus),
    NoonEnd(QuotStatus),
    QuotEnd(QuotStatus),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuotOpts {
    pub kind: Kind,
    /// 1, 5, 15, 30, 60min
    pub frequency: u32,
    pub codes: Vec<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

