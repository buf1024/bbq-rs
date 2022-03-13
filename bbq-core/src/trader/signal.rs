use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Signal {
    signal_id: String,
    // 信号源
    source: SignalSource,
    signal: SignalType,

    // 股票名称
    name: String,
    // 股票代码
    code: String,
    // 信号时间
    time: String,

    // 价格
    price: f64,
    // 量
    volume: f64,
    // 描述
    desc: String,
}
