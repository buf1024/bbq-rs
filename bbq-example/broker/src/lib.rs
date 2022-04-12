use anyhow::Result;
use async_trait::async_trait;
use bbq_strategy::{Broker, Entrust, EntrustStatus, EntrustType, Event, Opts, BrokerEvent};
use log::{error, info};

use tokio::sync::mpsc::UnboundedSender;

#[allow(dead_code)]
pub struct Example {
    tx: UnboundedSender<Event>,
    opts: Opts,
}

impl Example {
    fn new(tx: UnboundedSender<Event>, opts: Opts) -> Self {
        Self { tx, opts }
    }
}

#[async_trait]
impl Broker for Example {
    fn name(&self) -> String {
        "ExampleBroker".to_string()
    }
    async fn on_init(&mut self) -> Result<()> {
        info!("on_init dll broker: {}", self.name());

        Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        info!("on_destroy dll broker on_destroy!");
        Ok(())
    }
    async fn on_entrust(&mut self, entrust: &Entrust) -> Result<()> {
        info!("on_entrust dll broker on_entrust: {:?}!", entrust);
        let mut e = entrust.clone();
        e.desc = format!("example dll broker handled! {}", &e.desc[..]);
        match &e.entrust_type {
            EntrustType::Buy | EntrustType::Sell => {
                e.status = EntrustStatus::Deal;
                e.volume_deal = e.volume;
                e.volume_cancel = 0;
            }
            EntrustType::Cancel => {
                e.status = EntrustStatus::Cancel;
                e.volume_deal = 0;
                e.volume_cancel = e.volume;
            }
        }

        if let Err(e) = self.tx.send(Event::Broker(BrokerEvent::Entrust(e))) {
            error!("broker feedback entrust event failed: {}!", e);
        }

        Ok(())
    }
}

#[no_mangle]
pub fn new_broker(tx: UnboundedSender<Event>, opts: Opts) -> Box<dyn Broker> {
    if let Err(e) = bbq_strategy::setup_broker_log("example.log", &opts) {
        println!("setup broker log error: {}!", e);
    }
    Box::new(Example::new(tx, opts))
}
