use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use mongodb::{bson::Document, bson::doc};
use crate::{QResult, Kind};
use async_trait::async_trait;
use chrono::{Datelike, DateTime, Local, NaiveDate, NaiveDateTime, Timelike};
use crate::fetch::{Fetcher, is_trade_date, Quot, RtQuot};
use crate::data::mongo::{IndexInfo, MongoDB, StockInfo};

// #[derive(Debug, Clone)]
// pub struct Bar {
//     pub code: String,
//     pub name: String,
//     pub open: f64,
//     pub high: f64,
//     pub low: f64,
//     pub close: f64,
//     pub frequency: u32,
//     pub start: NaiveDateTime,
//     pub end: NaiveDateTime,
// }

// #[derive(Debug, Clone)]
// pub struct MyQuot {
//     // 合成的bar
//     pub bar: Bar,
//     // 实时行情
//     pub quot: Quot,
// }

// pub type MyRtQuot = HashMap<String, MyQuot>;

#[derive(Debug, Clone)]
pub struct QuotEventData {
    pub opts: QuotOpts,
    pub time: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub enum QuotEvent {
    Quot(RtQuot),
    QuotStart(QuotEventData),
    MorningStart(QuotEventData),
    MorningEnd(QuotEventData),
    NoonStart(QuotEventData),
    NoonEnd(QuotEventData),
    QuotEnd(QuotEventData),
}

#[derive(Debug, Clone)]
pub struct QuotOpts {
    pub kind: Kind,
    pub codes: Vec<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[async_trait]
pub trait Quotation: Sync + Send {
    async fn add_codes(&mut self, codes: &Vec<String>) -> QResult<()>;
    async fn get_quot(&mut self) -> QResult<Option<QuotEvent>>;
}

pub struct MyQuotation {
    opts: QuotOpts,
    db: MongoDB,
    fetcher: Box<dyn Fetcher>,

    is_start: bool,

    trade_date: Option<NaiveDate>,
    base_event: [bool; 4],

    // bar: BTreeMap<String, Bar>,

}

impl MyQuotation {
    fn is_trading(&self) -> bool {
        self.trade_date.is_some() &&
            ((self.base_event[0] && !self.base_event[1]) || (self.base_event[2] && !self.base_event[3]))
    }

    fn _base_event(&mut self, idx: usize, n: &NaiveDateTime) -> QResult<Option<QuotEvent>> {
        for i in 0..=idx {
            if !self.base_event[i] {
                self.base_event[i] = true;
                return match i {
                    0 => Ok(Some(QuotEvent::MorningStart(
                        QuotEventData { opts: self.opts.clone(), time: n.clone() }))),
                    1 => Ok(Some(QuotEvent::MorningEnd(
                        QuotEventData { opts: self.opts.clone(), time: n.clone() }))),
                    2 => Ok(Some(QuotEvent::NoonStart(
                        QuotEventData { opts: self.opts.clone(), time: n.clone() }))),
                    3 => Ok(Some(QuotEvent::NoonEnd(
                        QuotEventData { opts: self.opts.clone(), time: n.clone() }))),
                    _ => Ok(None)
                };
            }
        }
        Ok(None)
    }
    async fn get_base_event(&mut self, n: &NaiveDateTime) -> QResult<Option<QuotEvent>> {
        if self.trade_date.is_none() || self.trade_date.unwrap() != n.date() {
            for i in 0..4 {
                self.base_event[i] = false;
            }
            self.trade_date = None;
            if !is_trade_date(&n.date(), None) {
                return Ok(None);
            }
            self.trade_date = Some(n.date());
        }
        let m_start = NaiveDate::from_ymd(n.year(), n.hour(), n.day())
            .and_hms(9, 15, 0);
        let m_end = NaiveDate::from_ymd(n.year(), n.hour(), n.day())
            .and_hms(11, 30, 0);
        let n_start = NaiveDate::from_ymd(n.year(), n.hour(), n.day())
            .and_hms(13, 0, 0);
        let n_end = NaiveDate::from_ymd(n.year(), n.hour(), n.day())
            .and_hms(15, 0, 0);

        let idx = if m_start > *n && *n <= m_end {
            0
        } else if m_end > *n && *n <= n_start {
            1
        } else if n_start < *n && *n <= n_end {
            2
        } else {
            3
        };

        self._base_event(idx, n)
    }
    // fn trans_quot(&mut self, n: &NaiveDateTime, quots: &RtQuot) -> Option<MyRtQuot> {
    //     let (mut is_check, mut is_pub) = (false, false);
    //     let mut my_quot = HashMap::new();
    //     for (code, quot) in quots.iter() {
    //         if self.bar.contains_key(code.as_str()) {
    //             let bar = self.bar.get_mut(code.as_str()).unwrap();
    //             bar.close = quot.now;
    //             if quot.now > bar.high {
    //                 bar.high = quot.now;
    //             }
    //             if quot.now < bar.low {
    //                 bar.low = quot.now;
    //             }
    //             bar.end = n.clone();
    //         } else {
    //             let bar = Bar {
    //                 code: quot.code.clone(),
    //                 name: quot.name.clone(),
    //                 open: quot.now,
    //                 high: quot.now,
    //                 low: quot.now,
    //                 close: quot.now,
    //                 frequency: self.opts.frequency,
    //                 start: n.clone(),
    //                 end: n.clone(),
    //             };
    //             self.bar.insert(code.clone(), bar);
    //         };
    //         let bar = self.bar.get(code.as_str()).unwrap();
    //         if !is_check {
    //             let frequency = bar.end.second() - bar.start.second();
    //             if frequency >= bar.frequency {
    //                 is_pub = true
    //             }
    //             is_check = true;
    //         }
    //         if is_pub {
    //             let q = MyQuot { bar: bar.clone(), quot: quot.clone() };
    //             my_quot.insert(quot.code.clone(), q);
    //         }
    //     }
    //     if is_pub {
    //         self.bar.clear();
    //         return Some(my_quot);
    //     }
    //     None
    // }
}


pub struct RtQuotation {
    quotation: MyQuotation,
}

impl RtQuotation {
    pub fn new(opts: QuotOpts, db: MongoDB, fetcher: Box<dyn Fetcher>) -> Self {
        Self {
            quotation: MyQuotation {
                opts,
                db,
                fetcher,
                is_start: false,
                trade_date: None,
                base_event: [false; 4],
            }
        }
    }

}

#[async_trait]
impl Quotation for RtQuotation {
    async fn add_codes(&mut self, codes: &Vec<String>) -> QResult<()> {
        let filter = doc! {"code": {"$in": codes}};
        let codes_db: Vec<StockInfo> = self.db.find("stock_info", filter.clone(), None).await?;
        for info in codes_db.iter() {
            if !self.opts.codes.contains(&info.code) {
                self.opts.codes.push(info.code.clone());
            }
        }
        if codes_db.len() != codes.len() {
            let index_db: Vec<IndexInfo> = self.db.find("index_info", filter, None).await?;
            for info in index_db.iter() {
                if !self.opts.codes.contains(&info.code) {
                    self.opts.codes.push(info.code.clone());
                }
            }
        }

        Ok(())
    }

    async fn get_quot(&mut self) -> QResult<Option<QuotEvent>> {
        let n = Local::now().naive_local();
        
        if !self.is_start {
            self.is_start = true;
            return Ok(Some(QuotEvent::QuotStart(QuotEventData { opts: self.opts.clone(), time: n })));
        }
        let be = self.get_base_event(&n).await?;
        if be.is_some() {
            return Ok(be);
        }
        if !self.is_trading() {
            return Ok(None);
        }

        let quot = self.fetcher.fetch_rt_quot(&self.opts.codes).await?;
        Ok(Some(QuotEvent::Quot(quot)))
    }
}

impl Deref for RtQuotation {
    type Target = MyQuotation;

    fn deref(&self) -> &Self::Target {
        &self.quotation
    }
}

impl DerefMut for RtQuotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quotation
    }
}

pub struct BacktestQuotation {}


#[cfg(test)]
mod test {
    use std::time::Duration;
    use mongodb::Client;
    use mongodb::options::ClientOptions;
    use tokio::time::sleep;
    use crate::data::mongo::MongoDB;
    use crate::trader::quotation::{Quotation, QuotOpts, RtQuotation};
    use crate::fetch::Sina;

    #[test]
    fn test_quotation() {
        use chrono::NaiveDate;

        let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
        rt.block_on(async move {
            let mut clt_opts = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
            clt_opts.app_name = Some("bbq".into());
            clt_opts.connect_timeout = Some(Duration::from_secs(3));

            let client = Client::with_options(clt_opts).unwrap();
            let db = MongoDB::new(client);

            let quot_opts = QuotOpts {
                kind: Default::default(),
                codes: vec![],
                start_date: None,
                end_date: None
            };

            let sina = Sina::new();

            let mut rt_quot = RtQuotation::new(quot_opts, db,
                                               Box::new(sina));
            loop {
                let q = rt_quot.get_quot().await.unwrap();
                println!("q={:?}", q);

                sleep(Duration::from_secs(1)).await;

                println!("sleep");
            }

        });
    }
}
