#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Deal {
    deal_id: String,
    entrust_id: String,

    // 股票名称
    name: String,
    // 股票代码
    code: String,
    // 成交时间
    time: String,

    // deal_type = '' # 成交类型
    // 成交价格
    price: f64,
    // 成交量
    volume: u32,
    // 盈利额
    profit: f64,
    // 手续费
    fee: f64,
}
