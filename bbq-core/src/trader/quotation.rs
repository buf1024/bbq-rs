use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use mongodb::Client;
use crate::{QResult, TrKind};
use async_trait::async_trait;
use chrono::NaiveDate;
use crate::fetch::{Fetcher, Quot, RtQuot};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuotationOpts {
    pub kind: TrKind,
    pub frequency: u32,
    pub codes: Vec<String>,
    pub index: Vec<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>
}



#[async_trait]
pub trait Quotation: Send + Sync  {
    async fn add_codes(&mut self, codes: &Vec<String>);
    async fn is_trading(&self) -> bool;
    async fn get_quot(&self) -> Option<RtQuot>;
}

pub struct RtQuotation {
    opts: QuotationOpts,
    client: Client,
    fetcher: Box<dyn Fetcher>
}


impl RtQuotation {
    async fn new(opts: QuotationOpts, client: Client, fetcher: Box<dyn Fetcher>) ->Self {

        Self{
            opts,
            client,
            fetcher
        }
    }
}

#[async_trait]
impl Quotation for RtQuotation {
    async fn add_codes(&mut self, codes: &Vec<String>) {
        todo!()
    }

    async fn is_trading(&self) -> bool {
        todo!()
    }

    async fn get_quot(&self) -> Option<RtQuot> {
        todo!()
    }
}

pub struct BacktestQuotation {

}


#[cfg(test)]
mod test {
    use crate::trader::quotation::RtQuotation;

    #[test]
    fn test_quotation() {
        use chrono::NaiveDate;

        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            // let rt_quot = RtQuotation::init()
        });
    }
}
