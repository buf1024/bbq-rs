use tokio::sync::{mpsc, broadcast, oneshot, };
use crate::Account;
use crate::trader::config::Config;

pub struct Channel {
    pub tx: mpsc::UnboundedSender<()>,
    pub rx: mpsc::UnboundedReceiver<()>,
}

pub struct Trader {
    channel: Option<Channel>,
    accounts: Vec<Account>,
    cfg: Config,
    codes: Vec<String>
}

impl Trader {
    pub fn new(cfg: Config) -> Self {
        todo!()
    }
    pub fn channel(&mut self) {
        todo!()
    }
    pub async fn run(&mut self, shutdown: broadcast::Receiver<bool>) {
        todo!()
    }
}
