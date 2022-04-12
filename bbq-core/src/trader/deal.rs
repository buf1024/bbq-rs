use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{EntrustType, Entrust};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Deal {
    pub deal_id: String,
    pub entrust_id: String,

    // 股票名称
    pub name: String,
    // 股票代码
    pub code: String,
    // 成交时间
    pub time: Option<NaiveDateTime>,

    // 成交类型
    pub deal_type: EntrustType,
    // 成交价格
    pub price: f64,
    // 成交量
    pub volume: u32,
    // 盈利额
    pub profit: f64,
    // 手续费
    pub fee: f64,
}

impl Deal {
    pub fn new_from_entrust(entrust: &Entrust) -> Self {
        Self {
            deal_id: Uuid::new_v4().to_simple().to_string(),
            entrust_id: entrust.entrust_id.clone(),
            name: entrust.name.clone(),
            code: entrust.code.clone(),
            time: entrust.time.clone(),
            deal_type: entrust.entrust_type.clone(),
            price: entrust.price,
            volume: entrust.volume_deal,
            profit: 0.0,
            fee: 0.0,
        }
    }
}