use anyhow::{Result, bail};
use async_trait::async_trait;
use bbq_strategy::{Account, Event, Opts, QuotData, Strategy};
use log::info;
use std::sync::{Arc, RwLock};

pub struct Example {
    acct: Arc<RwLock<Account>>,
    opts: Opts,
}

impl Example {
    fn new(acct: Arc<RwLock<Account>>, opts: Opts) -> Self {
        Self { acct, opts }
    }
}

#[async_trait]
impl Strategy for Example {
    fn name(&self) -> String {
        "ExampleStrategy".to_string()
    }
    async fn on_init(&mut self) -> Result<()> {
        let account_id = {
            let acct = self.acct.read().unwrap();
            acct.account_id.clone()
        };
        info!("dll strategy: {} run account: {}", self.name(), &account_id);

        bail!("test err!");
        // Ok(())
    }
    async fn on_destroy(&self) -> Result<()> {
        info!("dll strategy on_destroy!");
        Ok(())
    }
    async fn on_open(&mut self, quot: &QuotData) -> Result<Option<Event>> {
        info!("dll strategy on_open: {:?}!", quot);
        Ok(None)
    }
    async fn on_close(&mut self, quot: &QuotData) -> Result<Option<Event>> {
        info!("dll strategy on_close: {:?}!", quot);
        Ok(None)
    }
    async fn on_quot(&mut self, quot: &QuotData) -> Result<Option<Event>> {
        info!("dll strategy on_quot: {:?}!", quot);
        Ok(None)
    }
}

#[no_mangle]
pub fn new_strategy(acct: Arc<RwLock<Account>>, opts: Opts) -> Box<dyn Strategy> {
    if let Err(e) = bbq_strategy::setup_strategy_log("example.log", &opts) {
        println!("setup strategy log error: {}!", e);
    }
    Box::new(Example::new(acct, opts))
}
