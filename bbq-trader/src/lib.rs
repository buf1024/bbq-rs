pub mod account;

pub mod quotation;

pub mod trader;

pub mod config;

pub mod broker;
pub mod risk;
pub mod strategy;

#[derive(Debug, Clone)]
pub(crate) enum TaskTarget {
    Quotation,
    Broker,
    Risk,
    Strategy,
}
