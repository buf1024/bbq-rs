use super::entrust::EntrustStatus;
use super::quot::QuotData;
use crate::trader::deal::Deal;
use crate::trader::entrust::Entrust;
use crate::trader::position::Position;
use crate::trader::signal::Signal;
use crate::{AcctStatus, AcctType, ActionType, BrokerEvent, Kind, EntrustType};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Account {
    pub account_id: String,
    pub status: AcctStatus,
    pub typ: AcctType,
    pub kind: Kind,

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

    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,

    // 持仓头寸
    pub position: HashMap<String, Position>,
    pub entrust: Vec<Entrust>,

    // # 成交 backtest
    pub deal: Vec<Deal>,
    pub signal: Vec<Signal>,

    #[serde(skip)]
    pub is_trading: bool,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            status: Default::default(),
            typ: Default::default(),
            kind: Default::default(),
            cash_init: Default::default(),
            cash_available: Default::default(),
            cash_frozen: Default::default(),
            total_net_value: Default::default(),
            total_hold_value: Default::default(),
            cost: Default::default(),
            profit: Default::default(),
            profit_rate: Default::default(),
            close_profit: Default::default(),
            total_profit: Default::default(),
            total_profit_rate: Default::default(),
            broker_fee: Default::default(),
            transfer_fee: Default::default(),
            tax_fee: Default::default(),
            start_time: Local::now().naive_local(),
            end_time: Default::default(),
            position: Default::default(),
            entrust: Default::default(),
            deal: Default::default(),
            signal: Default::default(),
            is_trading: false,
        }
    }
}

impl Account {
    pub fn new(account_id: String) -> Self {
        let mut acct = Self::default();
        acct.account_id = account_id;
        acct
    }
    pub fn get_fee(&self, typ: ActionType, code: &str, price: f64, volume: u32) -> f64 {
        let total = price * volume as f64;
        let mut broker_fee = total * self.broker_fee as f64;
        if broker_fee < 5.0 {
            broker_fee = 5.0;
        }
        let mut tax_fee = 0.0;
        if matches!(typ, ActionType::Buy) {
            if code.starts_with("sh6") {
                tax_fee = total * self.transfer_fee
            }
        } else if matches!(typ, ActionType::Sell) {
            if code.starts_with("sz") || code.starts_with("sh") {
                tax_fee = total * self.tax_fee
            }
        }
        broker_fee + tax_fee
    }

    pub fn get_cost(&self, typ: ActionType, code: &str, price: f64, volume: u32) -> f64 {
        let fee = self.get_fee(typ, code, price, volume);
        fee + price * volume as f64
    }
    pub fn update_account_quot(&mut self, quot: &QuotData) {
        match quot {
            QuotData::Quot(quot) => {
                self.profit = 0.0;
                self.cost = 0.0;
                self.total_hold_value = 0.0;
                for position in self.position.values_mut() {
                    if quot.contains_key(&position.code) {
                        position.on_update_quot(quot.get(&position.code).unwrap());
                    }
                    self.profit = self.profit + position.profit;
                    self.total_hold_value =
                        self.total_hold_value + (position.now_price * position.volume as f64);
                    self.cost =
                        self.cost + (position.price * position.volume as f64 + position.fee);
                }

                if self.cost > 0.0 {
                    self.profit_rate = self.profit / self.cost * 100.0;
                }
                self.total_net_value =
                    self.cash_available + self.cash_frozen + self.total_hold_value;
                self.total_profit = self.close_profit + self.profit;
                self.total_profit_rate = self.total_profit / self.cash_init * 100.0
            }
            QuotData::QuotStart(_) => {}
            QuotData::MorningStart(_) | QuotData::NoonStart(_) => self.is_trading = true,
            QuotData::MorningEnd(_) => self.is_trading = false,
            QuotData::NoonEnd(_) => {
                self.is_trading = false;
                for position in self.position.values_mut() {
                    if position.volume != position.volume_available {
                        position.volume_frozen = 0;
                        position.volume_available = position.volume;
                    }
                }

                for entrust in self.entrust.iter_mut() {
                    if matches!(entrust.status, EntrustStatus::Commit) {
                        entrust.status = EntrustStatus::Cancel;
                    }
                }
                self.cash_available += self.cash_frozen;

                if !matches!(self.typ, AcctType::Backtest) {
                    self.entrust.clear();
                    self.deal.clear();
                }
            }
            QuotData::QuotEnd(_) => self.end_time = Some(Local::now().naive_local()),
        }
    }
    pub fn update_account_signal(&mut self, signal: &Signal) {
        if matches!(self.typ, AcctType::Backtest) {
            self.signal.push(signal.clone());
        }
    }
    pub fn update_account_entrust(&mut self, entrust: &Entrust) {
        self.entrust.push(entrust.clone());
    }
    pub fn update_broker_push(&mut self, event: &BrokerEvent) {
        match event {
            BrokerEvent::Entrust(entrust) => {
                let found = self
                    .entrust
                    .iter()
                    .position(|e| e.entrust_id == *entrust.entrust_id);

                if let Some(index) = found {
                    let e = self.entrust.get_mut(index).unwrap();
                    e.status = entrust.status.clone();
                    e.broker_entrust_id = entrust.broker_entrust_id.clone();
                    match entrust.status {
                        EntrustStatus::Deal | EntrustStatus::PartDeal => {
                            e.volume_deal += entrust.volume_deal;
                            if matches!(entrust.status, EntrustStatus::PartDeal) {
                                if e.volume_deal == e.volume {
                                    e.status = EntrustStatus::Deal;
                                }
                            }
                            let mut deal = Deal::new_from_entrust(entrust);
                            match entrust.entrust_type {
                                EntrustType::Buy => {
                                    deal.fee = self.get_fee(
                                        ActionType::Buy,
                                        entrust.code.as_str(),
                                        entrust.price,
                                        entrust.volume_deal,
                                    );
                                },
                                EntrustType::Sell => {
                                    deal.fee = self.get_fee(
                                        ActionType::Sell,
                                        entrust.code.as_str(),
                                        entrust.price,
                                        entrust.volume_deal,
                                    );
                                },
                                EntrustType::Cancel => return,
                            }
                            
                            self.update_position(&deal);
                            self.deal.push(deal);
                        }
                        EntrustStatus::Cancel => {
                            e.volume_cancel = entrust.volume_cancel;
                        }
                        _ => {}
                    }
                }
            }
            BrokerEvent::FundSync((total, available, hold)) => {}
            BrokerEvent::Position(position) => {}
            _ => {}
        }
    }

    pub fn update_position(&mut self, deal: &Deal) {
        if self.position.contains_key(&deal.code) {
            let position = self.position.get_mut(&deal.code).unwrap();
        }
    }
}
