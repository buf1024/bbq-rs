use std::fmt::{Display, Formatter};

use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalType {
    Sell,
    Buy,
    Cancel,
}

impl Default for SignalType {
    fn default() -> Self {
        SignalType::Cancel
    }
}

impl Display for SignalType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SignalType::Sell => "卖出",
            SignalType::Buy => "买入",
            SignalType::Cancel => "撤销",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalSource {
    Risk(String),
    Strategy(String),
    Broker(String),
    Robot(String),
}

impl Default for SignalSource {
    fn default() -> Self {
        SignalSource::Robot("Unknown".to_string())
    }
}

impl Display for SignalSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SignalSource::Risk(v) => format!("Risk: {}", v),
            SignalSource::Strategy(v) => format!("Strategy: {}", v),
            SignalSource::Broker(v) => format!("Broker: {}", v),
            SignalSource::Robot(v) => format!("Robot: {}", v),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Signal {
    pub signal_id: String,
    // 信号源
    pub source: SignalSource,
    pub signal: SignalType,

    // 股票名称
    pub name: String,
    // 股票代码
    pub code: String,
    // 信号时间
    pub time: Option<NaiveDateTime>,

    // 价格
    pub price: f64,
    // 量
    pub volume: u32,
    // 描述
    pub desc: String,

    pub entrust_id: Option<String>,
}
