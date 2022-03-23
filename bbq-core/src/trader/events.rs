use chrono::{NaiveDate, NaiveDateTime};
use crate::fetch::RtQuot;
use crate::trader::entrust::Entrust;
use crate::trader::quotation::QuotEvent;
use crate::trader::signal::Signal;

#[derive(Debug, Clone)]
pub enum Event {
    Entrust(Entrust),
    Signal(Signal),
    Broker(BrokerData),
    Quot(QuotEvent),
}

#[derive(Debug, Clone)]
pub struct BrokerData {

}
