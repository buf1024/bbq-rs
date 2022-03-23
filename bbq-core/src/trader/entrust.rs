use std::fmt::{Display, Formatter};
use chrono::NaiveDateTime;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum EntrustStatus {
    Init,
    Commit,
    Deal,
    PartDeal,
    Cancel
}

impl Default for EntrustStatus {
    fn default() -> Self {
        Self::Cancel
    }
}

impl Display for EntrustStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EntrustStatus::Init => "初始化",
            EntrustStatus::Commit => "已提交",
            EntrustStatus::Deal => "已成交",
            EntrustStatus::PartDeal => "部分成交",
            EntrustStatus::Cancel => "撤销"
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum EntrustType {
    Buy,
    Sell,
    Cancel
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

#[derive(Debug, Clone)]
pub struct Entrust {
    pub entrust_id: String,
    pub name: String,
    pub code: String,
    pub time: NaiveDateTime,

    pub entrust_type: EntrustType,
    pub status: EntrustStatus,

    pub price: f64,
    pub volume: u32,

    pub volume_deal: u32,
    pub volume_cancel: u32,

    pub desc: String
}
