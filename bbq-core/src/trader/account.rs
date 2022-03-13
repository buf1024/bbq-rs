use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use chrono::NaiveDateTime;
use crate::trader::deal::Deal;
use crate::trader::entrust::Entrust;
use crate::trader::position::Position;
use crate::trader::signal::Signal;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum AccountStatus {
    Running,
    Stop,
}

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Stop
    }
}

impl Display for AccountStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AccountStatus::Running => "运行中",
            AccountStatus::Stop => "已停止",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum AccountCategory {
    Fund,
    Stock,
}

impl Default for AccountCategory {
    fn default() -> Self {
        Self::Stock
    }
}

impl Display for AccountCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AccountCategory::Fund => "基金",
            AccountCategory::Stock => "股票",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum AccountKind {
    Backtest,
    Simulation,
    Real,
}

impl Default for AccountKind {
    fn default() -> Self {
        Self::Backtest
    }
}


impl Display for AccountKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AccountKind::Backtest => "回测",
            AccountKind::Simulation => "模拟",
            AccountKind::Real => "实盘",
        };
        write!(f, "{}", s)
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Account {
    pub account_id: String,
    pub status: AccountStatus,
    pub category: AccountCategory,
    pub kind: AccountKind,

    pub cash_init: f64,
    pub cash_available: f64,
    pub cash_frozen: f64,
    pub total_net_value: f64,

    pub total_hold_value: f64,
    // 持仓陈本
    pub cost: f64,
    // 持仓盈亏
    pub profit: f64,
    // 持仓盈比例
    pub profit_rate: f64,
    // 平仓盈亏
    pub close_profit: f64,
    // 总盈亏
    pub total_profit: f64,
    // 总盈亏比例
    pub total_profit_rate: f64,

    // 券商手续费
    pub broker_fee: f64,
    // 过户费
    pub transfer_fee: f64,
    // 印花税
    pub tax_fee: f64,

    // start_time: NaiveDateTime,
    // end_time: Option<NaiveDateTime>,

    // 持仓头寸
    pub position: HashMap<String, Position>,
    pub entrust: Vec<Entrust>,

    pub broker_name: String,
    pub broker_opts: HashMap<String, String>,
    pub strategy_name: String,
    pub strategy_opts: HashMap<String, String>,
    pub risk_name: String,
    pub brisk_opts: HashMap<String, String>,

    // # 成交 backtest
    pub deal: Vec<Deal>,
    pub signal: Vec<Signal>,
}
