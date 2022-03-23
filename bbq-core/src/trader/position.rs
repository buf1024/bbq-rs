use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct Position {
    position_id: String,

    // 股票名称
    name: String,
    // 股票代码
    code: String,
    // 首次建仓时间
    time: NaiveDateTime,

    // 持仓量
    volume: u32,
    // 可用持仓量
    volume_available: u32,
    // 可用持仓量
    volume_frozen: u32,
    // 持仓费用
    fee: f64,
    // 平均持仓价
    price: f64,
    // 最新价
    now_price: f64,
    // 最高价
    max_price: f64,
    // 最低价
    min_price: f64,
    // 盈利比例
    profit_rate: f64,
    // 最大盈利比例
    max_profit_rate: f64,
    // 最小盈利比例
    min_profit_rate: f64,

    // 盈利
    profit: f64,
    // 最大盈利
    max_profit: f64,
    // 最小盈利
    min_profit: f64,

    // 最大盈利时间
    max_profit_time: NaiveDateTime,
    // 最小盈利时间
    min_profit_time: NaiveDateTime,
}
