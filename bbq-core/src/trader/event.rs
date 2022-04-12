use crate::{Signal, Entrust, Position};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum Event {
    /// 交易信号, 发出: strategy/risk
    Signal(Signal),
    /// 行情订阅, 发出: strategy
    Subscribe(Vec<String>),
    /// 委托事件, 发往: broker
    Entrust(Entrust),
    /// 券商推送/同步, 发出: broker
    Broker(BrokerEvent),

    ///
    EventNone(String),
}

impl Default for Event {
    fn default() -> Self {
        Self::EventNone("QUIT".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum BrokerEvent {
    Entrust(Entrust),
    /// 总资金，可用资金，持仓市值
    FundSync((f64, f64, f64)),
    Position(Vec<Position>),
    ///
    EventNone,
}

impl Default for BrokerEvent {
    fn default() -> Self {
        Self::EventNone
    }
}
