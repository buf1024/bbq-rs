use anyhow::{Context, Result};
use async_trait::async_trait;
use bbq_strategy::{
    get_id, Account, Event, Opts, Position, Risk, Signal, SignalSource, SignalType
};
use chrono::Local;
use log::{error, info};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::UnboundedSender;

pub struct SimpleStop {
    acct: Arc<RwLock<Account>>,
    tx: UnboundedSender<Event>,
    opts: Opts,
    // 止损正数
    lost: Option<f64>,
    lost_rate: Option<f64>,
    // 止盈正数
    profit: Option<f64>,
    profit_rate: Option<f64>,
}

impl SimpleStop {
    fn new(acct: Arc<RwLock<Account>>, tx: UnboundedSender<Event>, opts: Opts) -> Self {
        Self {
            acct,
            tx,
            opts,
            lost: None,
            lost_rate: None,
            profit: None,
            profit_rate: None,
        }
    }
    fn emit(&mut self, position: &Position) -> Result<()> {
        let signal = Signal {
            signal_id: get_id(),
            source: SignalSource::Risk(self.name()),
            signal: SignalType::Sell,
            name: position.name.clone(),
            code: position.code.clone(),
            time: Some(Local::now().naive_local()),
            price: position.price,
            volume: position.volume_available,
            desc: format!("{} risk signal", self.name()),
            entrust_id: None,
        };
        self.tx.send(Event::Signal(signal))?;

        Ok(())
    }
}

#[async_trait]
impl Risk for SimpleStop {
    fn name(&self) -> String {
        "SimpleStop".to_string()
    }
    async fn on_init(&mut self) -> Result<()> {
        if let Some(o) = &self.opts {
            if let Some(value) = o.get(&"profit".to_string()) {
                let value: f64 = value.parse().with_context(|| "parse profit to f64 error")?;
                self.profit = Some(value);
            }
            if let Some(value) = o.get(&"profit_rate".to_string()) {
                let value: f64 = value
                    .parse()
                    .with_context(|| "parse profit_rate to f64 error")?;
                self.profit_rate = Some(value);
            }
            if let Some(value) = o.get(&"lost".to_string()) {
                let value: f64 = value.parse().with_context(|| "parse lost to f64 error")?;
                self.lost = Some(value);
            }
            if let Some(value) = o.get(&"lost_rate".to_string()) {
                let value: f64 = value
                    .parse()
                    .with_context(|| "parse lost_rate to f64 error")?;
                self.lost_rate = Some(value);
            }
        }

        let account_id = {
            let acct = self.acct.read().unwrap();
            acct.account_id.clone()
        };

        info!(
            "dll risk: {} run account: {}, opts={:?}",
            self.name().clone(),
            &account_id,
            &self.opts
        );

        Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        info!("dll risk on_destroy!");
        Ok(())
    }
    async fn on_risk(&mut self) -> Result<()> {
        if self.profit.is_some()
            || self.profit_rate.is_some()
            || self.lost.is_some()
            || self.lost_rate.is_some()
        {
            let positions = {
                let acct = self.acct.read().unwrap();
                acct.position.clone()
            };
            for position in positions.values() {
                if position.volume_available <= 0 {
                    continue;
                }
                if let Some(profit) = self.profit {
                    // 止盈
                    if position.profit > 0.0 && profit > 0.0 && position.profit > profit {
                        if let Err(e) = self.emit(position) {
                            error!("emit risk signal error: {}", e);
                        }
                        continue;
                    }
                }
                if let Some(profit_rate) = self.profit_rate {
                    // 比例止盈
                    if position.profit_rate > 0.0
                        && profit_rate > 0.0
                        && position.profit_rate > profit_rate
                    {
                        if let Err(e) = self.emit(position) {
                            error!("emit risk signal error: {}", e);
                        }
                        continue;
                    }
                }
                if let Some(lost) = self.lost {
                    // 止损
                    if position.profit < 0.0 && position.profit < -lost.abs() {
                        if let Err(e) = self.emit(position) {
                            error!("emit risk signal error: {}", e);
                        }
                        continue;
                    }
                }
                if let Some(lost_rate) = self.lost_rate {
                    // 比例止损
                    if position.profit_rate < 0.0 && position.profit_rate < -lost_rate.abs() {
                        if let Err(e) = self.emit(position) {
                            error!("emit risk signal error: {}", e);
                        }
                        continue;
                    }
                }
            }
        }

        Ok(())
    }
}

#[no_mangle]
pub fn new_risk(
    acct: Arc<RwLock<Account>>,
    tx: UnboundedSender<Event>,
    opts: Opts,
) -> Box<dyn Risk> {
    if let Err(e) = bbq_strategy::setup_risk_log("simple_stop.log", &opts) {
        println!("setup broker log error: {}!", e);
    }

    Box::new(SimpleStop::new(acct, tx, opts))
}
