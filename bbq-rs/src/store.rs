use std::collections::{BTreeSet, HashMap};
use std::sync::{RwLock};
use bbq_core::account::Account;


#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Store {
    pub module: Module,
    pub settings: Settings,
    pub data: DataStore,
    pub trade: Trade,
    pub backtest: Trade,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Module {
    Data,
    Analyse,
    Backtest,
    Trade,
    Setting,
}

impl Default for Module {
    fn default() -> Self {
        Module::Data
    }
}

// settings
#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct EmailPush {
    pub smtp_host: String,
    pub user: String,
    pub secret: String,
    pub notify: String,
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct WechatPush {
    pub token: String,
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Settings {
    pub show: bool,
    pub open_windows: BTreeSet<String>,

    pub db_url: String,
    pub db_is_testing: bool,
    pub db_is_valid: bool,

    pub email_push: EmailPush,
    pub wechat_push: WechatPush,

    pub broker_path: Vec<String>,
    pub strategy_path: Vec<String>,
    pub risk_path: Vec<String>,
}

// trade
#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct TreePath {
    pub name: String,
    pub sub_path: Vec<TreePath>,
}

impl TreePath {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            sub_path: vec![],
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Trade {
    pub strategy_show: bool,
    pub strategy_is_build: bool,
    pub strategy: TreePath,
    pub strategy_running: HashMap<String, String>,
    pub strategy_selected: String,

    pub account: Account,
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CollInfo {
    pub coll_name: String,
    pub last_sync: String,
    pub is_latest: bool,
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct DataStore {
    pub stock_coll: Vec<CollInfo>,
    pub stock_auto_sync: String,
    pub stock_log: Vec<String>,
    pub stock_log_show: bool,

    pub fund_coll: Vec<CollInfo>,
    pub fund_auto_sync: String,
    pub fund_log: Vec<String>,
    pub fund_log_show: bool,
}
