use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use super::signal::Signal;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrustStatus {
    Init,
    Commit,
    Deal,
    PartDeal,
    Cancel,
}

impl Default for EntrustStatus {
    fn default() -> Self {
        Self::Init
    }
}

impl Display for EntrustStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EntrustStatus::Init => "初始化",
            EntrustStatus::Commit => "已提交",
            EntrustStatus::Deal => "已成交",
            EntrustStatus::PartDeal => "部分成交",
            EntrustStatus::Cancel => "已撤销",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrustType {
    Buy,
    Sell,
    Cancel,
}

impl Default for EntrustType {
    fn default() -> Self {
        Self::Cancel
    }
}

impl Display for EntrustType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EntrustType::Sell => "卖出",
            EntrustType::Buy => "买入",
            EntrustType::Cancel => "撤销",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Entrust {
    pub entrust_id: String,
    pub name: String,
    pub code: String,
    pub time: Option<NaiveDateTime>,

    pub entrust_type: EntrustType,
    pub status: EntrustStatus,

    pub price: f64,
    pub volume: u32,

    pub volume_deal: u32,
    pub volume_cancel: u32,

    pub desc: String,

    pub broker_entrust_id: Option<String>,
}

impl Entrust {
    pub fn new_from_signal(signal: &Signal) -> Self {
        Self {
            entrust_id: Uuid::new_v4().to_simple().to_string(),
            name: signal.name.clone(),
            code: signal.code.clone(),
            time: signal.time.clone(),
            entrust_type: match signal.signal {
                super::signal::SignalType::Sell => EntrustType::Sell,
                super::signal::SignalType::Buy => EntrustType::Buy,
                super::signal::SignalType::Cancel => EntrustType::Cancel,
            },
            status: EntrustStatus::Init,
            price: signal.price,
            volume: signal.volume,
            volume_deal: 0,
            volume_cancel: 0,
            desc: "".to_string(),
            broker_entrust_id: None,
        }
    }
}
