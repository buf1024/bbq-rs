use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use anyhow::Result;
use tokio::sync::mpsc::UnboundedSender;

pub mod strategy;
pub use strategy::*;

pub mod risk;
pub use risk::*;

pub mod broker;
pub use broker::*;

pub use bbq_core::*;

pub type Opts = Option<HashMap<String, String>>;
pub type NewStrategyFunc = fn(Arc<RwLock<Account>>, Opts) -> Box<dyn Strategy>;
pub type NewRiskFunc = fn(Arc<RwLock<Account>>, UnboundedSender<Event>, Opts) -> Box<dyn Risk>;
pub type NewBrokerFunc = fn(UnboundedSender<Event>, Opts) -> Box<dyn Broker>;

fn setup_my_log(typ: &str, file: &str, opts: &Opts) -> Result<()> {
    if let Some(opt) = opts {
        let k_path = "log_path".to_string();
        let k_level = "log_level".to_string();

        if opt.contains_key(&k_path) && opt.contains_key(&k_level) {
            let v_path = format!("{}/{}", opt.get(&k_path).unwrap(), typ);
            std::fs::create_dir_all(&v_path)?;
            let v_path = format!("{}/{}", v_path, file);
            let v_level = opt.get(&k_level).unwrap();

            bbq_core::setup_log(&v_path, &v_level)?;
        }
    }
    Ok(())
}

pub fn setup_strategy_log(file: &str, opts: &Opts) -> Result<()> {
    setup_my_log("strategy", file, opts)
}
pub fn setup_risk_log(file: &str, opts: &Opts) -> Result<()> {
    setup_my_log("risk", file, opts)
}
pub fn setup_broker_log(file: &str, opts: &Opts) -> Result<()> {
    setup_my_log("broker", file, opts)
}

pub fn get_id() -> String {
    uuid::Uuid::new_v4().to_simple().to_string()
}