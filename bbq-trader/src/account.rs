use crate::broker::Broker;
use crate::risk::Risk;
use crate::{quotation, strategy, TaskTarget};
use anyhow::Result;
use bbq_core::Event;
use bbq_core::{data::mongo::MongoDB, fetch::Sina, Account, AcctType, Entrust, QuotData, QuotOpts};
use log::{debug, error, info};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Barrier,
};
use tokio::task::JoinHandle;

pub struct AcctOpts {
    pub typ: AcctType,
    pub quot_opts: QuotOpts,
    pub db: Option<MongoDB>,

    pub broker_path: Option<String>,
    pub broker_opts: Option<HashMap<String, String>>,

    pub strategy_path: String,
    pub strategy_opts: Option<HashMap<String, String>>,

    pub risk_path: Option<String>,
    pub risk_opts: Option<HashMap<String, String>>,
}

pub async fn run(
    account: Arc<RwLock<Account>>,
    opts: AcctOpts,
    mut sd: broadcast::Receiver<bool>,
) -> Result<()> {
    let (account_id, acct_type) = {
        let acct = account.read().unwrap();
        (acct.account_id.clone(), acct.typ.clone())
    };
    info!(
        "start running account({:?}): {}",
        &acct_type,
        &account_id[..]
    );

    let mut handlers = vec![];

    let (shutdown, _) = broadcast::channel::<bool>(1);
    let barrier = Arc::new(Barrier::new(5));

    let (except_tx, mut except_rx) = broadcast::channel::<TaskTarget>(1);

    let (broker_entrust_tx, mut broker_push_rx, h) = run_broker(
        opts.broker_path.clone(),
        opts.broker_opts.clone(),
        shutdown.subscribe(),
        barrier.clone(),
        except_tx.clone(),
    );
    handlers.push(h);

    let (risk_quot_tx, mut risk_rx, h) = run_risk(
        opts.risk_path.clone(),
        opts.risk_opts.clone(),
        shutdown.subscribe(),
        account.clone(),
        barrier.clone(),
        except_tx.clone(),
    );
    handlers.push(h);

    let (strategy_quot_tx, mut strategy_rx, h) = run_strategy(
        opts.strategy_path.clone(),
        opts.strategy_opts.clone(),
        shutdown.subscribe(),
        account.clone(),
        barrier.clone(),
        except_tx.clone(),
    );
    handlers.push(h);

    let (quot_tx, mut quot_rx, h) = run_quotation(
        opts.typ.clone(),
        opts.quot_opts.clone(),
        opts.db.clone(),
        shutdown.subscribe(),
        barrier.clone(),
        except_tx.clone(),
    );
    handlers.push(h);

    barrier.wait().await;

    let mut is_except = false;

    let (mut is_quot_end, mut is_strategy_end, mut is_risk_end, mut is_broker_end) =
        (false, false, false, false);
    loop {
        if is_quot_end && is_strategy_end && is_risk_end {
            if !is_broker_end {
                if let Err(e) = broker_entrust_tx.send(Event::EventNone("QUIT".to_string())) {
                    error!(
                        "account: {}, notify broker task normal quit failed: {}",
                        &account_id[..],
                        e
                    );
                }
            } else {
                break;
            }
        }
        tokio::select! {
            ex = except_rx.recv() => {
                if ex.is_err() {
                    error!("all tasks is dead!");
                } else {
                    let ex = ex.unwrap();
                    error!("task {:?} is dead!", ex);
                }
                if let Err(e) = shutdown.send(true){
                    error!("task dead, account: {}, shutdown failed: {}", &account_id[..], e);
                }
                is_except = true;
                break;
            },
            quot_data = quot_rx.recv(), if !is_quot_end => {
                if quot_data.is_none() {
                   info!("account: {}, quotation task is end", &account_id[..]);
                   is_quot_end = true;
                   continue;
                }

                let quot_data = quot_data.unwrap();

                {
                    let mut acct = account.write().unwrap();
                    acct.update_account_quot(&quot_data);
                }
                match &quot_data {
                    QuotData::QuotEnd(_) => {
                        is_quot_end = true;
                    }
                    _ => {}
                }
                if let Err(e) = strategy_quot_tx.send(quot_data.clone()){
                    error!("account: {}, strategy fail to dispatch quot, error: {}", &account_id[..], e);
                }
                if let Err(e) = risk_quot_tx.send(quot_data){
                    error!("account: {}, risk fail to dispatch quot, error: {}", &account_id[..], e);
                }

                if is_quot_end {
                    info!("account: {} backtest quot end!", &account_id[..]);
                }
            },
            strategy_event = strategy_rx.recv(), if !is_strategy_end => {
                if strategy_event.is_none() {
                    info!("account: {}, strategy task is end", &account_id[..]);
                    is_strategy_end = true;
                    continue;
                }
                let strategy_event = strategy_event.unwrap();
                debug!("account strategy event: {:?}", &strategy_event);
                match &strategy_event {
                    Event::Signal(signal) => {
                        {
                            let mut acct = account.write().unwrap();
                            acct.update_account_signal(signal);
                        }
                        // test signal
                        let entrust = Entrust::new_from_signal(signal);

                        if let Err(e) = broker_entrust_tx.send(Event::Entrust(entrust)) {
                            error!("account: {}, strategy dispatch broker entrust failed: {}", &account_id[..], e);
                        }
                    },
                    Event::Subscribe(sub) => {
                        if !is_quot_end {
                            if let Err(e) = quot_tx.send(Event::Subscribe(sub.clone())) {
                                error!("account: {}, subscribe quotation error: {}", &account_id[..], e);
                            }
                        }
                    },
                    _ => {}
                }
            },
            risk_event = risk_rx.recv(), if !is_risk_end => {
                if risk_event.is_none() {
                    info!("account: {}, risk task is end", &account_id[..]);
                    is_risk_end = true;
                    continue;

                }
                let risk_event = risk_event.unwrap();

                match &risk_event {
                    Event::Signal(signal) => {
                        {
                            let mut acct = account.write().unwrap();
                            acct.update_account_signal(signal);
                        }
                        let entrust = Entrust::new_from_signal(signal);

                        if let Err(e) = broker_entrust_tx.send(Event::Entrust(entrust)) {
                            error!("account: {}, risk dispatch broker entrust failed: {}", &account_id[..], e);
                        }
                    },
                    _ => {}
                }

            },
            broker_push_event = broker_push_rx.recv(), if !is_broker_end => {
                if broker_push_event.is_none() {
                    info!("account: {}, broker task is end", &account_id[..]);
                    is_broker_end = true;
                    continue;
                }
                let push_event = broker_push_event.unwrap();
                match push_event {
                    Event::Broker(broker_event) => {
                        info!("broker push event: {:?}", broker_event);
                        let mut acct = account.write().unwrap();
                        acct.update_broker_push(&broker_event);
                    },
                    _ => {},
                }
            },
            _ = sd.recv() => {
                if let Err(e) = shutdown.send(true){
                    error!("account: {}, shutdown failed: {}", &account_id[..], e);
                }
                break;
            }

        }
    }
    info!("account: {}, waiting subtask end", &account_id[..]);
    for h in handlers {
        match h.await {
            _ => {}
        }
    }
    info!("end running account: {}", &account_id[..]);

    Ok(())
}

fn run_broker(
    path: Option<String>,
    opts: Option<HashMap<String, String>>,
    shutdown: broadcast::Receiver<bool>,
    barrier: Arc<Barrier>,
    except_tx: broadcast::Sender<TaskTarget>,
) -> (
    UnboundedSender<Event>,
    UnboundedReceiver<Event>,
    JoinHandle<()>,
) {
    info!("setting up broker");
    let (mut broker, broker_entrust_tx, broker_push_rx) = Broker::new(path, opts, shutdown);
    let h = tokio::spawn(async move {
        barrier.wait().await;
        let rs = broker.run().await;
        if let Err(e) = rs {
            error!("run broker failed: {}", e);
            if let Err(e) = except_tx.send(TaskTarget::Broker) {
                error!("send run broker failed result: {}", e);
            }
        }
    });
    (broker_entrust_tx, broker_push_rx, h)
}

fn run_risk(
    path: Option<String>,
    opts: Option<HashMap<String, String>>,
    shutdown: broadcast::Receiver<bool>,
    account: Arc<RwLock<Account>>,
    barrier: Arc<Barrier>,
    except_tx: broadcast::Sender<TaskTarget>,
) -> (
    UnboundedSender<QuotData>,
    UnboundedReceiver<Event>,
    JoinHandle<()>,
) {
    info!("setting up risk");
    let (mut risk, quot_tx, risk_rx) = Risk::new(path, opts, shutdown, account);
    let h = tokio::spawn(async move {
        barrier.wait().await;
        let rs = risk.run().await;
        if let Err(e) = rs {
            error!("run risk failed: {}", e);
            if let Err(e) = except_tx.send(TaskTarget::Risk) {
                error!("send run risk failed result: {}", e);
            }
        }
    });
    (quot_tx, risk_rx, h)
}

fn run_strategy(
    path: String,
    opts: Option<HashMap<String, String>>,
    shutdown: broadcast::Receiver<bool>,
    account: Arc<RwLock<Account>>,
    barrier: Arc<Barrier>,
    except_tx: broadcast::Sender<TaskTarget>,
) -> (
    UnboundedSender<QuotData>,
    UnboundedReceiver<Event>,
    JoinHandle<()>,
) {
    info!("setting up strategy");
    let (mut strategy, quot_tx, strategy_rx) =
        strategy::Strategy::new(path, opts, shutdown, account);
    let h = tokio::spawn(async move {
        barrier.wait().await;
        let rs = strategy.run().await;
        if let Err(e) = rs {
            error!("run strategy failed: {}", e);
            if let Err(e) = except_tx.send(TaskTarget::Strategy) {
                error!("send run strategy failed result: {}", e);
            }
        }
    });
    (quot_tx, strategy_rx, h)
}

fn run_quotation(
    acct_type: AcctType,
    opts: QuotOpts,
    db: Option<MongoDB>,
    shutdown: broadcast::Receiver<bool>,
    barrier: Arc<Barrier>,
    except_tx: broadcast::Sender<TaskTarget>,
) -> (
    UnboundedSender<Event>,
    UnboundedReceiver<QuotData>,
    JoinHandle<()>,
) {
    info!("setting up quotation");
    let (quot_tx, quot_rx) = mpsc::unbounded_channel();
    let (sub_tx, sub_rx) = mpsc::unbounded_channel();

    let (interval, quot) = if matches!(acct_type, AcctType::Backtest) {
        let quot = quotation::BacktestQuotation::new(opts, Box::new(Sina::new()), db);
        (
            Some(Duration::from_millis(50)),
            Box::new(quot) as Box<dyn quotation::Quotation>,
        )
    } else {
        let quot = quotation::RtQuotation::new(opts, Box::new(Sina::new()));
        (
            Some(Duration::from_secs(1)),
            Box::new(quot) as Box<dyn quotation::Quotation>,
        )
    };
    let h = tokio::spawn(async move {
        barrier.wait().await;
        let rs = quotation::run(quot, interval, quot_tx, sub_rx, shutdown).await;
        if let Err(e) = rs {
            error!("run quotation failed: {}", e);
            if let Err(e) = except_tx.send(TaskTarget::Quotation) {
                error!("send run quotation failed result: {}", e);
            }
        }
    });

    (sub_tx, quot_rx, h)
}

#[cfg(test)]
mod test_account {

    use bbq_core::{Deal, Entrust, Position, Signal};

    use super::Account;

    #[test]
    fn test_acct() {
        let mut acct = Account::default();
        acct.account_id = "test-account".to_string();
        acct.position
            .insert("sh001".to_string(), Position::default());
        acct.entrust.push(Entrust::default());
        acct.deal.push(Deal::default());
        acct.signal.push(Signal::default());

        let s = serde_json::to_string(&acct).unwrap();
        println!("{}", s);
    }
}
