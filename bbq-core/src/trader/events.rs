use chrono::{NaiveDate, NaiveDateTime};
use crate::fetch::RtQuot;
use crate::trader::entrust::Entrust;
use crate::trader::signal::Signal;

pub enum Events {
    Entrust(Entrust),
    Signal(Signal),
    Broker(BrokerData),
    Quot(RtQuot),
    Period(TradePeriod),
}

#[derive(Debug, Clone)]
struct TradePeriod {
    period: QuotPeriod,
    quot_freq: u32,
    trade_date: NaiveDate,
    day_time: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum QuotPeriod {
    QuotStart,
    TradeMorningStart,
    TradeMorningEnd,
    TradeNoonStart,
    TradeNoonEnd,
    QuotEnd
}

struct BrokerData {

}
