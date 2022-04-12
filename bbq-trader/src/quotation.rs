use anyhow::{Context, Result};
use async_trait::async_trait;
use bbq_core::{
    data::mongo::{IndexDaily, MongoDB, StockDaily},
    fetch::{is_index, is_trade_date, Fetcher, Quot, RtQuot, StockBar},
    QuotBar, QuotData, QuotOpts, QuotStatus, RtQuotBar, FREQ_15M, FREQ_1D, FREQ_1M, FREQ_30M,
    FREQ_5M, FREQ_60M,
};
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use futures::TryStreamExt;
use log::{debug, error, info};
use mongodb::{bson::doc, options::FindOptions};
use std::collections::{BTreeMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::time::Duration;
use tokio::sync::{
    broadcast,
    mpsc::{UnboundedReceiver, UnboundedSender},
};

use bbq_core::Event;

#[async_trait]
pub trait Quotation: Sync + Send {
    async fn add_codes(&mut self, codes: &Vec<String>) -> Result<()>;
    async fn get_quot(&mut self) -> Result<Option<QuotData>>;
}

pub struct MyQuotation {
    opts: QuotOpts,
    fetcher: Box<dyn Fetcher>,

    is_start: bool,
    is_end: bool,

    trade_date: Option<NaiveDate>,
    base_event: [bool; 4],
}

impl MyQuotation {
    fn is_trading(&self) -> bool {
        self.trade_date.is_some()
            && ((self.base_event[0] && !self.base_event[1])
                || (self.base_event[2] && !self.base_event[3]))
    }

    fn _base_event(&mut self, idx: usize, n: &NaiveDateTime) -> Result<Option<QuotData>> {
        for i in 0..=idx {
            if !self.base_event[i] {
                self.base_event[i] = true;
                return match i {
                    0 => Ok(Some(QuotData::MorningStart(QuotStatus {
                        opts: self.opts.clone(),
                        time: n.clone(),
                    }))),
                    1 => Ok(Some(QuotData::MorningEnd(QuotStatus {
                        opts: self.opts.clone(),
                        time: n.clone(),
                    }))),
                    2 => Ok(Some(QuotData::NoonStart(QuotStatus {
                        opts: self.opts.clone(),
                        time: n.clone(),
                    }))),
                    3 => Ok(Some(QuotData::NoonEnd(QuotStatus {
                        opts: self.opts.clone(),
                        time: n.clone(),
                    }))),
                    _ => Ok(None),
                };
            }
        }
        Ok(None)
    }
    async fn get_base_event(&mut self, n: &NaiveDateTime) -> Result<Option<QuotData>> {
        if self.trade_date.is_some() && self.trade_date.unwrap() != n.date() {
            for i in 0..4 {
                if !self.base_event[i] {
                    let date = self.trade_date.unwrap().format("%Y-%m-%d").to_string();
                    let time = match i {
                        0 => "09:30:00",
                        1 => "11:30:00",
                        2 => "13:00:00",
                        3 => "15:00:00",
                        _ => "",
                    };
                    let dt = NaiveDateTime::parse_from_str(
                        format!("{} {}", date, time).as_str(),
                        "%Y-%m-%d %H:%M:%S",
                    )
                    .unwrap();

                    return self._base_event(i, &dt);
                }
            }
        }

        if self.trade_date.is_none() || self.trade_date.unwrap() != n.date() {
            for i in 0..4 {
                self.base_event[i] = false;
            }
            self.trade_date = None;
            if !is_trade_date(&n.date()) {
                return Ok(None);
            }
            self.trade_date = Some(n.date());
        }
        let m_start = NaiveDate::from_ymd(n.year(), n.month(), n.day()).and_hms(9, 15, 0);
        let m_end = NaiveDate::from_ymd(n.year(), n.month(), n.day()).and_hms(11, 30, 0);
        let n_start = NaiveDate::from_ymd(n.year(), n.month(), n.day()).and_hms(13, 0, 0);
        let n_end = NaiveDate::from_ymd(n.year(), n.month(), n.day()).and_hms(15, 0, 0);

        let idx = if *n > m_start && *n <= m_end {
            0
        } else if *n > m_end && *n <= n_start {
            1
        } else if *n > n_start && *n <= n_end {
            2
        } else {
            3
        };

        self._base_event(idx, n)
    }
    fn add_quot_codes(&mut self, codes: &Vec<String>) {
        for code in codes.iter() {
            if !self.opts.codes.contains(code) {
                self.opts.codes.push(code.clone());
            }
        }
    }
}

pub struct RtQuotation {
    quotation: MyQuotation,
    bar: Option<RtQuotBar>,
}

impl RtQuotation {
    pub fn new(opts: QuotOpts, fetcher: Box<dyn Fetcher>) -> Self {
        Self {
            quotation: MyQuotation {
                opts,
                fetcher,
                is_start: false,
                is_end: false,
                trade_date: None,
                base_event: [false; 4],
            },
            bar: None,
        }
    }
    fn quot2bar(&mut self, quot: RtQuot) -> Option<RtQuotBar> {
        if self.bar.is_none() {
            self.bar = Some(RtQuotBar::new());
        }
        let n = Local::now().naive_local();
        let n_str = n.format("%Y-%m-%d %H:%M:%S").to_string();

        let (mut is_ready, mut is_test) = (false, false);
        let frequency = self.opts.frequency;
        let bar = self.bar.as_mut().unwrap();
        for (code, q) in quot.iter() {
            if !bar.contains_key(code) {
                let qb = QuotBar {
                    frequency,
                    open: q.now,
                    high: q.now,
                    low: q.now,
                    close: q.now,
                    start: n_str.clone(),
                    end: n_str.clone(),
                    quot: q.clone(),
                };
                bar.insert(code.clone(), qb);
            }

            let mut qb = bar.get_mut(code).unwrap();
            if qb.high < q.now {
                qb.high = q.now;
            }
            if qb.low > q.now {
                qb.low = q.now;
            }
            qb.close = q.now;
            qb.end = q.time.clone();

            if !is_test {
                is_test = true;
                let start = NaiveDateTime::parse_from_str(qb.start.as_str(), "%Y-%m-%d %H:%M:%S")
                    .unwrap()
                    .timestamp();
                let end = n.timestamp();
                if end - start >= qb.frequency as i64 {
                    is_ready = true;
                }
            }
        }

        if is_ready {
            return self.bar.take();
        }

        None
    }
}

#[async_trait]
impl Quotation for RtQuotation {
    async fn add_codes(&mut self, codes: &Vec<String>) -> Result<()> {
        self.add_quot_codes(codes);
        Ok(())
    }
    async fn get_quot(&mut self) -> Result<Option<QuotData>> {
        let n = Local::now().naive_local();

        if !self.is_start {
            self.is_start = true;
            return Ok(Some(QuotData::QuotStart(QuotStatus {
                opts: self.opts.clone(),
                time: n,
            })));
        }
        let be = self.get_base_event(&n).await?;
        if be.is_some() {
            return Ok(be);
        }
        if !self.is_trading() || self.opts.codes.is_empty() {
            return Ok(None);
        }

        let bar = self
            .quot2bar(self.fetcher.fetch_rt_quot(&self.opts.codes).await?)
            .map(|v| QuotData::Quot(v));

        Ok(bar)
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

pub struct BacktestQuotation {
    quotation: MyQuotation,
    bar_list: BTreeMap<u64, RtQuotBar>,
    index: usize,
    iter_vec: Vec<u64>,
    freq: Vec<u32>,
    codes: HashSet<String>,
    db: Option<MongoDB>,
}

impl BacktestQuotation {
    pub fn new(opts: QuotOpts, fetcher: Box<dyn Fetcher>, db: Option<MongoDB>) -> Self {
        Self {
            quotation: MyQuotation {
                opts,
                fetcher,
                is_start: false,
                is_end: false,
                trade_date: None,
                base_event: [false; 4],
            },
            bar_list: BTreeMap::new(),
            index: 0,
            freq: vec![FREQ_1M, FREQ_5M, FREQ_15M, FREQ_30M, FREQ_60M, FREQ_1D],
            codes: HashSet::new(),
            iter_vec: vec![],
            db,
        }
    }
}
impl Deref for BacktestQuotation {
    type Target = MyQuotation;

    fn deref(&self) -> &Self::Target {
        &self.quotation
    }
}

impl DerefMut for BacktestQuotation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.quotation
    }
}

#[async_trait]
impl Quotation for BacktestQuotation {
    async fn add_codes(&mut self, codes: &Vec<String>) -> Result<()> {
        let frequency = self.opts.frequency;
        let m = frequency;
        if self.freq.contains(&m) {
            for code in codes {
                if !self.opts.codes.contains(code) {
                    self.opts.codes.push(code.clone());
                }
            }
            // 借用好垃圾啊
            let codes: Vec<String> = self.opts.codes.iter().map(|s| s.clone()).collect();
            for code in codes {
                if self.codes.contains(&code[..]) {
                    continue;
                }
                self.codes.insert(code.clone());
                let r = if m < FREQ_1D {
                    self.fetcher
                        .fetch_stock_minute(code.as_str(), m / 60)
                        .await
                        .with_context(|| "fetch stock minute error")?
                } else {
                    let mut q_data = Vec::new();
                    if self.db.is_some() {
                        let db = self.db.as_ref().unwrap();
                        let filter = match (self.opts.start_date, self.opts.end_date) {
                            (None, None) => doc! {"code": code.as_str()},
                            (None, Some(end)) => {
                                let s = end.and_hms(0, 0, 0);
                                let s = Local.from_local_datetime(&s).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$lte": s}
                                }
                            }
                            (Some(start), None) => {
                                let s = start.and_hms(0, 0, 0);
                                let s = Local.from_local_datetime(&s).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$gte": s}
                                }
                            }
                            (Some(start), Some(end)) => {
                                let s = start.and_hms(0, 0, 0);
                                let s = Utc.from_local_datetime(&s).unwrap();

                                let e = end.and_hms(0, 0, 0);
                                let e = Utc.from_local_datetime(&e).unwrap();
                                doc! {
                                   "code": code.as_str(),
                                   "trade_date": {"$gte": s, "$lte": e},
                                }
                            }
                        };
                        let opts = FindOptions::builder().sort(doc! {"trade_date": 1}).build();
                        if is_index(code.as_str()) {
                            let mut cursor = db
                                .get_coll::<IndexDaily>("index_daily")?
                                .find(filter, opts)
                                .await
                                .with_context(|| "query index_daily failed")?;
                            while let Some(item) = cursor
                                .try_next()
                                .await
                                .with_context(|| "query index_daily failed")?
                            {
                                let time = item
                                    .trade_date
                                    .to_chrono()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string();
                                let s_bar = StockBar {
                                    time,
                                    open: item.open,
                                    high: item.high,
                                    low: item.low,
                                    close: item.close,
                                    vol: item.volume,
                                };
                                q_data.push(s_bar)
                            }
                        } else {
                            let mut cursor = db
                                .get_coll::<StockDaily>("stock_daily")?
                                .find(filter, opts)
                                .await
                                .with_context(|| "query stock_daily failed")?;
                            while let Some(item) = cursor
                                .try_next()
                                .await
                                .with_context(|| "query stock_daily failed")?
                            {
                                let time = item
                                    .trade_date
                                    .to_chrono()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string();
                                let s_bar = StockBar {
                                    time,
                                    open: item.open,
                                    high: item.high,
                                    low: item.low,
                                    close: item.close,
                                    vol: item.volume as u64,
                                };
                                q_data.push(s_bar)
                            }
                        }
                    }
                    q_data
                };
                for bar in r.iter() {
                    let t = NaiveDateTime::parse_from_str(bar.time.as_str(), "%Y-%m-%d %H:%M:%S")
                        .with_context(|| "parse time error")?;
                    let t = t.timestamp();
                    if !self.bar_list.contains_key(&(t as u64)) {
                        self.bar_list.insert(t as u64, RtQuotBar::new());
                    }
                    let q_bar = self.bar_list.get_mut(&(t as u64)).unwrap();

                    if !q_bar.contains_key(&code) {
                        let rt_q = QuotBar {
                            frequency,
                            open: bar.open,
                            high: bar.high,
                            low: bar.close,
                            close: bar.high,
                            start: NaiveDateTime::from_timestamp(t - (frequency as i64), 0)
                                .format("%Y-%m-%d %H:%M:%S")
                                .to_string(),
                            end: bar.time.clone(),
                            quot: Quot {
                                code: code.clone(),
                                open: bar.open,
                                now: bar.close,
                                high: bar.high,
                                low: bar.low,
                                buy: bar.close,
                                sell: bar.close,
                                vol: bar.vol,
                                date: NaiveDateTime::parse_from_str(
                                    &bar.time[..],
                                    "%Y-%m-%d %H:%M:%S",
                                )
                                .unwrap()
                                .date()
                                .format("%Y-%m-%d")
                                .to_string(),
                                time: bar.time.clone(),
                                ..Default::default()
                            },
                        };
                        q_bar.insert(code.clone(), rt_q);
                    }
                }
            }
            self.iter_vec.clear();
            self.iter_vec.extend(self.bar_list.keys());
        }
        Ok(())
    }
    async fn get_quot(&mut self) -> Result<Option<QuotData>> {
        if self.bar_list.is_empty() && !self.opts.codes.is_empty() {
            self.add_codes(&self.opts.codes.clone()).await?;
        }
        let n = Local::now().naive_local();
        let index = self.index;

        if !self.is_start {
            self.is_start = true;
            return Ok(Some(QuotData::QuotStart(QuotStatus {
                opts: self.opts.clone(),
                time: n,
            })));
        }
        if index >= self.iter_vec.len() && self.iter_vec.len() > 0 {
            let t = *self
                .iter_vec
                .get(self.iter_vec.len() - 1)
                .with_context(|| "index not found")?;

            let mut adj_t = t;
            if self.opts.frequency != FREQ_1D {
                adj_t += 15 * 60 * 60;
            }
            let n = NaiveDateTime::from_timestamp(adj_t as i64, 0);

            let be = self.get_base_event(&n).await?;
            if be.is_some() {
                return Ok(be);
            }
            if !self.is_end {
                self.is_end = true;
                return Ok(Some(QuotData::QuotEnd(QuotStatus {
                    opts: self.opts.clone(),
                    time: n,
                })));
            }
            return Ok(None);
        }
        let t = *self
            .iter_vec
            .get(index)
            .with_context(|| "index not found")?;

        let mut adj_t = t;
        if self.opts.frequency == FREQ_1D {
            adj_t += 9 * 60 * 60 + 30 * 60;
        }
        let n = NaiveDateTime::from_timestamp(adj_t as i64, 0);
        let be = self.get_base_event(&n).await?;
        if be.is_some() {
            return Ok(be);
        }
        if !self.is_trading() || self.opts.codes.is_empty() {
            return Ok(None);
        }

        let bar = self
            .bar_list
            .get(&t)
            .map(|q| q.clone())
            .with_context(|| "key not found")?;

        self.index += 1;
        Ok(Some(QuotData::Quot(bar)))
    }
}

pub async fn run(
    mut quot: Box<dyn Quotation>,
    interval: Option<Duration>,
    tx: UnboundedSender<QuotData>,
    mut rx: UnboundedReceiver<Event>,
    mut shutdown: broadcast::Receiver<bool>,
) -> Result<()> {
    let mut event_tx_is_close = false;
    loop {
        tokio::select! {
            qrs = quot.get_quot() => {
                if let Ok(qrs) = qrs {
                    if let Some(q_data)  = qrs {
                        let is_end = match &q_data {
                            QuotData::QuotEnd(_) => true,
                            _ => false
                        };
                        debug!(" quot desc: {:?}", &q_data);
                        let rs = tx.send(q_data);
                        if let Err(e) = rs {
                            error!("quotation task dispatch quot failed(no receiver): {}", e);
                        }



                        if is_end {
                            break;
                        }

                    }
                }

            },
            event = rx.recv(), if !event_tx_is_close => {
                if event.is_none() {
                    error!("quotation recv none event(all tx close)!");
                    event_tx_is_close = true;
                    continue;
                }
                let event = event.unwrap();

                match &event {
                    Event::Subscribe(codes) => {
                        if let Err(e) = quot.add_codes(codes).await {
                            error!("quotation add new codes: {:?} failed: {}", codes, e);
                        }
                    },
                    _ => {}
                }


            },
            _ = shutdown.recv() => break,
        }
        if let Some(it) = interval {
            tokio::select! {
                _ = tokio::time::sleep(it) => {},
                _ = shutdown.recv() => break,
            }
        }
    }
    info!("quotation tasks end!");
    Ok(())
}

#[cfg(test)]
mod test_quotation {
    use crate::quotation::{BacktestQuotation, Quotation, RtQuotation};
    use bbq_core::fetch::Sina;
    use bbq_core::{data::mongo::MongoDB, QuotOpts};
    use chrono::NaiveDate;
    use mongodb::options::ClientOptions;
    use mongodb::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    use super::{FREQ_1D, FREQ_1M};

    async fn rt_quot() -> RtQuotation {
        let quot_opts = QuotOpts {
            frequency: FREQ_1M,
            kind: Default::default(),
            codes: vec![
                "sh600063".to_string(),
                "sh601456".to_string(),
                "sh000001".to_string(),
            ],
            start_date: None,
            end_date: None,
        };

        let sina = Sina::new();

        RtQuotation::new(quot_opts, Box::new(sina))
    }

    async fn bt_quot() -> BacktestQuotation {
        let quot_opts = QuotOpts {
            frequency: FREQ_1D,
            kind: Default::default(),
            codes: vec![
                "sh600063".to_string(),
                "sh601456".to_string(),
                "sh000001".to_string(),
            ],
            start_date: Some(NaiveDate::parse_from_str("2022-03-01", "%Y-%m-%d").unwrap()),
            end_date: Some(NaiveDate::parse_from_str("2022-03-01", "%Y-%m-%d").unwrap()),
        };

        let sina = Sina::new();

        let mut clt_opts = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        clt_opts.app_name = Some("bbq".into());
        clt_opts.connect_timeout = Some(Duration::from_secs(3));

        let client = Client::with_options(clt_opts).unwrap();

        let s = MongoDB::new(client);

        BacktestQuotation::new(quot_opts, Box::new(sina), Some(s))
    }

    #[test]
    fn test_bt_quotation() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let mut quot = bt_quot().await;
            loop {
                let q = quot.get_quot().await.unwrap();

                if q.is_some() {
                    println!("q={:?}", serde_json::to_string(&q));
                }

                // sleep(Duration::from_secs(1)).await;
                if q.is_none() {
                    break;
                }

                println!("sleep");
            }
        });
    }

    #[test]
    fn test_rt_quotation() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async move {
            let mut quot = rt_quot().await;
            loop {
                let q = quot.get_quot().await.unwrap();

                println!("q={:?}", q);

                sleep(Duration::from_secs(1)).await;

                println!("sleep");
            }
        });
    }
}
