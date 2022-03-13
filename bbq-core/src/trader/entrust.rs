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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Entrust {
    entrust_id: String,
    name: String,
    code: String,
    time: NaiveDateTime,

    entrust_type: EntrustType,
    status: EntrustStatus,

    price: f64,
    volume: u32,

    volume_deal: u32,
    volume_cancel: u32,

    desc: String

}
