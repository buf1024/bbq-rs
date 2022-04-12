use std::fmt::Display;

use serde::{Serialize, Deserialize};


/// 交易品种
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Stock,
    Fund,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Stock
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            Kind::Stock => "股票",
            Kind::Fund => "基金",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcctType {
    Backtest,
    Simulation,
    Real,
}

impl Default for AcctType {
    fn default() -> Self {
        Self::Backtest
    }
}

impl Display for AcctType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            AcctType::Backtest => "回测",
            AcctType::Simulation => "模拟",
            AcctType::Real => "实盘",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcctStatus {
    Running,
    Stop,
}

impl Default for AcctStatus {
    fn default() -> Self {
        Self::Stop
    }
}

impl Display for AcctStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            AcctStatus::Running => "运行中",
            AcctStatus::Stop => "已停止",
        };
        write!(f, "{}", s)
    }
}


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Buy,
    Sell,
}

impl Default for ActionType {
    fn default() -> Self {
        Self::Sell
    }
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self {
            ActionType::Buy => "买入",
            ActionType::Sell => "卖出",
        };
        write!(f, "{}", s)
    }
}
