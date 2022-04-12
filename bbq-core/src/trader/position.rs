use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::quot::QuotBar;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Position {
    pub position_id: String,

    // 股票名称
    pub name: String,
    // 股票代码
    pub code: String,
    // 首次建仓时间
    pub time: Option<NaiveDateTime>,

    // 持仓量
    pub volume: u32,
    // 可用持仓量
    pub volume_available: u32,
    // 可用持仓量
    pub volume_frozen: u32,
    // 持仓费用
    pub fee: f64,
    // 平均持仓价
    pub price: f64,
    // 最新价
    pub now_price: f64,
    // 最高价
    pub max_price: f64,
    // 最低价
    pub min_price: f64,
    // 盈利比例
    pub profit_rate: f64,
    // 最大盈利比例
    pub max_profit_rate: f64,
    // 最小盈利比例
    pub min_profit_rate: f64,

    // 盈利
    pub profit: f64,
    // 最大盈利
    pub max_profit: f64,
    // 最小盈利
    pub min_profit: f64,

    // 最大盈利时间
    pub max_profit_time: Option<NaiveDateTime>,
    // 最小盈利时间
    pub min_profit_time: Option<NaiveDateTime>,
}

impl Position {
    pub fn on_update_quot(&mut self, quot_bar: &QuotBar) {
        self.now_price = quot_bar.close;
        if self.max_price < self.now_price {
            self.max_price = self.now_price;
        }
        if self.min_price > self.now_price {
            self.min_price = self.now_price
        }
        self.profit = (self.now_price - self.price) * self.volume as f64 - self.fee;
        self.profit_rate = self.profit / (self.price * self.volume as f64 + self.fee);
        let t = NaiveDateTime::parse_from_str(quot_bar.quot.time.as_str(), "%Y-%m-%d %H:%M:%S");
        if self.profit > self.max_profit {
            self.max_profit = self.profit;
            if let Ok(time) = &t {
                self.max_profit_time = Some(time.clone());
            }

            self.max_profit_rate = self.profit_rate;
        }

        if self.profit < self.min_profit {
            self.min_profit = self.profit;
            if let Ok(time) = &t {
                self.min_profit_time = Some(time.clone());
            }
            self.min_profit_rate = self.profit_rate;
        }
    }
}
