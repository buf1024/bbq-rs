use crate::{
    account::{self, AcctOpts},
    config::Config,
};
use anyhow::{Context, Result};
use backoff::{backoff::Backoff, ExponentialBackoff};
use bbq_core::{data::mongo::MongoDB, Account, AcctType, Kind, QuotOpts};
use chrono::NaiveDate;
use log::{error, info};
use mongodb::{options::ClientOptions, Client};
use std::collections::HashMap;
use std::{
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{net::TcpListener, sync::broadcast, time};

pub struct Trader {
    cfg: Config,
    shutdown: broadcast::Receiver<bool>,
    accounts: HashMap<String, Arc<RwLock<Account>>>,
    db: Option<MongoDB>,
}

impl Trader {
    pub fn new(cfg: Config, shutdown: broadcast::Receiver<bool>) -> Self {
        Self {
            cfg,
            shutdown,
            accounts: HashMap::new(),
            db: None,
        }
    }

    async fn connect_mongo(&mut self) -> Result<()> {
        if let Some(db_uri) = &self.cfg.mongodb {
            let mut clt_opts = ClientOptions::parse(db_uri.as_str())
                .await
                .with_context(|| format!("invalid mongodb uri: {}", db_uri))?;
            clt_opts.app_name = Some("bbq-trader".into());
            clt_opts.connect_timeout = Some(Duration::from_secs(3));
            let client = Client::with_options(clt_opts).unwrap();
            info!("connecting to data db...");
            client
                .list_databases(None, None)
                .await
                .with_context(|| format!("failed to connect to database: {}", db_uri))?;
            self.db = Some(MongoDB::new(client));
        }
        Ok(())
    }

    pub async fn init(&mut self) -> Result<()> {
        fdlimit::raise_fd_limit();

        self.connect_mongo().await?;

        self.load_strategy().await?;
        Ok(())
    }

    async fn load_strategy(&mut self) -> Result<()> {
        Ok(())
    }

    async fn load_acct(&mut self) -> Result<()> {
        // sled::open(path)
        Ok(())
    }

    async fn bind(&self) -> Result<TcpListener> {
        let port = self.cfg.listen.port;
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        info!("listen bind 127.0.0.1:{}", port);
        Ok(listener)
    }

    pub async fn run(&mut self) -> Result<()> {
        let listener = self.bind().await?;

        self.load_acct().await?;

        let mut backoff = ExponentialBackoff {
            max_interval: Duration::from_millis(100),
            max_elapsed_time: None,
            ..Default::default()
        };

        let (s, _) = broadcast::channel(1);

        loop {
            tokio::select! {
                res = listener.accept() => {
                    match res {
                        Err(err)  => {
                            if let Some(d) = backoff.next_backoff() {
                                error!("failed to accept: {:#}. Retry in {:?}...", err, d);
                                time::sleep(d).await;
                            } else {
                                error!("too many retries. aborting...");
                                break;
                            }
                        },
                        Ok((conn, addr)) => {
                            backoff.reset();

                            info!("accept connection: {}", addr);

                            let mut acct = Account::default();
                            acct.account_id = "TestAccount".to_string();
                            self.accounts.insert("test".to_string(), Arc::new(RwLock::new(acct)));

                            let acct = self.accounts.get(&"test".to_string())
                                .with_context(||"aa")?
                                .clone();

                            let mut log_opts = HashMap::new();
                            log_opts.insert("log_path".to_string(), self.cfg.log.path.clone());
                            log_opts.insert("log_level".to_string(), self.cfg.log.level.clone());


                            let opts = AcctOpts {
                                // typ: AcctType::Simulation,
                                typ: AcctType::Backtest,
                                quot_opts: QuotOpts {
                                    // frequency: 3,
                                    frequency: bbq_core::FREQ_1D,
                                    kind: Kind::Stock,
                                    codes: vec![
                                        "sh600063".to_string(),
                                        "sh601456".to_string(),
                                        "sh000001".to_string(),
                                    ],
                                    start_date: Some(NaiveDate::parse_from_str("2022-03-01", "%Y-%m-%d")?),
                                    end_date: Some(NaiveDate::parse_from_str("2022-03-01", "%Y-%m-%d")?),
                                },
                                db: self.db.clone(),
                                // broker_path: Some("/Users/luoguochun/privt/proj/bbq-rs/target/debug/libbroker.dylib".to_string()),
                                // broker_path: None,
                                broker_path: Some("/Users/luoguochun/privt/proj/bbq-rs/bbq-strategy-py/example/example_broker.py".to_string()),
                                broker_opts: Some(log_opts.clone()),
                                strategy_path: "/Users/luoguochun/privt/proj/bbq-rs/bbq-strategy-py/example/example_strategy.py".to_string(),
                                // strategy_path: "/Users/luoguochun/privt/proj/bbq-rs/target/debug/libstrategy.dylib".to_string(),
                                strategy_opts: Some(log_opts.clone()),
                                // risk_path: Some("/Users/luoguochun/privt/proj/bbq-rs/bbq-strategy-py/example/example_risk.py".to_string()),
                                // risk_path: Some("/Users/luoguochun/privt/proj/bbq-rs/target/debug/librisk.dylib".to_string()),
                                risk_path: None,
                                risk_opts: Some(log_opts.clone()),
                            };
                            // let handler = Handler { cfg: self.cfg.clone() };
                            let s = s.subscribe();
                            tokio::spawn(async move {
                                account::run(acct, opts, s).await.unwrap();
                            });

                        }
                    }
                },
                _ = self.shutdown.recv() => {
                    info!("shuting down...");
                    break;
                }
            }
        }

        Ok(())
    }
}

struct Handler {
    cfg: Arc<RwLock<Config>>,
}

impl Handler {}
