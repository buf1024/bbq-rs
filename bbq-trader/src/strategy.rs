use anyhow::{bail, Context, Result};
use bbq_core::{
    quot::QuotData,
    signal::{Signal, SignalSource},
    Account, Event,
};
use bbq_strategy::NewStrategyFunc;
use convert_case::{Case, Casing};
use log::{debug, error, info};
use pyo3::types::{PyModule, PyString};
use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{Arc, RwLock},
};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use uuid::Uuid;

pub struct Strategy {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<QuotData>,

    path: String,
    opts: Option<HashMap<String, String>>,

    shutdown: broadcast::Receiver<bool>,
    account: Arc<RwLock<Account>>,
}

impl Strategy {
    pub fn new(
        path: String,
        opts: Option<HashMap<String, String>>,
        shutdown: broadcast::Receiver<bool>,
        account: Arc<RwLock<Account>>,
    ) -> (Self, UnboundedSender<QuotData>, UnboundedReceiver<Event>) {
        let (tx, strategy_rx) = mpsc::unbounded_channel();
        let (quot_tx, rx) = mpsc::unbounded_channel();

        (
            Self {
                tx,
                rx,
                path,
                opts,
                shutdown,
                account,
            },
            quot_tx,
            strategy_rx,
        )
    }

    async fn run_py(&mut self) -> Result<()> {
        info!("run strategy python");

        let fname = Path::new(&self.path)
            .file_name()
            .with_context(|| format!("failed to get file name: {}", &self.path))?
            .to_str()
            .with_context(|| format!("failed to convert file name: {}", &self.path))?
            .to_string();

        let py_class_name = fname
            .split(".")
            .next()
            .with_context(|| format!("failed to get py class name: {}", &self.path))?
            .to_case(Case::Pascal);

        let py_code = fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read py strategy code, name: {}", &self.path))?;
        
        let py_opts = serde_json::to_string(&self.opts)
            .with_context(|| format!("failed to serialize opts, name: {}", &self.path))?;

        let (driver_tx, mut driver_rx) = mpsc::unbounded_channel();
        let (result_tx, mut result_rx) = mpsc::unbounded_channel();

        let account = self.account.clone();

        let (thread_tx, mut thread_rx) = broadcast::channel(1);
        let handler = std::thread::spawn(move || {
            let py_result = pyo3::Python::with_gil(|py| {
                let strategy = PyModule::from_code(py, py_code.as_str(), &fname, &fname)
                    .with_context(|| "failed to load python code")?;

                let py_class = strategy
                    .getattr(py_class_name.as_str())
                    .with_context(|| format!("failed to get strategy class: {}", &py_class_name))?;

                let py_inst = py_class
                    .call0()
                    .with_context(|| format!("failed to initial class: {}", &py_class_name))?;

                let py_name = py_inst
                    .getattr("name")
                    .with_context(|| format!("failed to get strategy name: {}", &py_class_name))?;

                let py_name_res = py_name.call0();
                if let Err(e) = &py_name_res {
                    error!("get strategy name err:{}", e);
                    bail!("get strategy name err:{}", e)
                }
                let name_res = py_name_res.unwrap().extract();
                if let Err(e) = &name_res {
                    error!("extract strategy name err:{}", e);
                    bail!("extract strategy name err:{}", e)
                }
                let name: String = name_res.unwrap();
                info!("strategy => {}:{}", &py_class_name, &name);

                let acct_js = {
                    let acct = account.read().unwrap();

                    serde_json::to_string(&acct.to_owned()).unwrap()
                };

                let init_res = py_inst.call1((&acct_js, "init", "", py_opts));
                if let Err(e) = &init_res {
                    error!("init strategy err:{}", e);
                    bail!("init strategy err:{}", e)
                }
                let init_res = init_res.unwrap().extract();
                if let Err(e) = &init_res {
                    error!("init extract err:{}", e);
                    bail!("init extract err:{}", e)
                }
                let is_init: bool = init_res.unwrap();

                if !is_init {
                    bail!("failed to init strategy, class: {}", &py_class_name);
                }

                loop {
                    let quot = driver_rx.blocking_recv();
                    if quot.is_none() {
                        info!("py outer channel close exit");
                        break;
                    }
                    let quot = quot.unwrap();

                    let js =
                        serde_json::to_string(&quot).with_context(|| "failed to convert json")?;
                    let acct_js = {
                        let acct = account.read().unwrap();

                        serde_json::to_string(&acct.to_owned()).unwrap()
                    };

                    let mut is_quot_end = false;
                    let rst = match quot {
                        QuotData::Quot(_) => py_inst.call1((&acct_js, "on_quot", "quot", js)),
                        QuotData::QuotStart(_) => {
                            py_inst.call1((&acct_js, "on_open", "quot_start", js))
                        }
                        QuotData::MorningStart(_) => {
                            py_inst.call1((&acct_js, "on_open", "morning_start", js))
                        }
                        QuotData::MorningEnd(_) => {
                            py_inst.call1((&acct_js, "on_close", "morning_end", js))
                        }
                        QuotData::NoonStart(_) => {
                            py_inst.call1((&acct_js, "on_open", "noon_open", js))
                        }
                        QuotData::NoonEnd(_) => {
                            py_inst.call1((&acct_js, "on_close", "noon_end", js))
                        }
                        QuotData::QuotEnd(_) => {
                            is_quot_end = true;
                            py_inst.call1((&acct_js, "on_close", "quot_end", js))
                        }
                    };

                    if let Err(e) = &rst {
                        error!("strategy quotation err:{}", e);
                        bail!("strategy quotation err:{}", e)
                    }

                    if is_quot_end {
                        if let Err(_) = py_inst.call1(("", "destroy", "", "")) {}
                        break;
                    }
                    let rst = rst.unwrap();
                    if !rst.is_none() {
                        let py_str = rst.downcast::<PyString>();
                        if py_str.is_ok() {
                            let py_str: String = py_str.unwrap().extract().unwrap();
                            let evt_list = serde_json::from_str(py_str.as_str());
                            if evt_list.is_ok() {
                                let evt_list: Vec<String> = evt_list.unwrap();
                                info!("strategy subscribe: {:?}", &evt_list);
                                result_tx
                                    .send(Event::Subscribe(evt_list))
                                    .with_context(|| "result_tx send subscribe error")?;
                            } else {
                                let evt_list = serde_json::from_str(py_str.as_str());
                                if evt_list.is_ok() {
                                    let evt_list: Vec<Signal> = evt_list.unwrap();
                                    for mut signal in evt_list {
                                        signal.source = SignalSource::Strategy(name.clone());
                                        signal.signal_id = Uuid::new_v4().to_simple().to_string();
                                        info!("strategy signal: {:?}", &signal);
                                        result_tx
                                            .send(Event::Signal(signal))
                                            .with_context(|| "result_tx send signal error")?;
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(())
            });
            if let Err(e) = py_result {
                error!("run python error: {}!", e);
                if let Err(_) = thread_tx.send(false) {}
            } else {
                if let Err(_) = thread_tx.send(true) {}
            }
        });
        let mut is_normal = true;
        let mut is_quot_end = false;
        let mut result_channel_dead = false;
        loop {
            tokio::select! {
                quot_data = self.rx.recv(), if !is_quot_end => {
                    if quot_data.is_none() {
                        error!("strategy recv quotation none");
                        is_quot_end = true;
                        continue;
                    }
                    let quot_data = quot_data.unwrap();

                    // debug!("strategy recv quot: {:?}", &quot_data);

                    match &quot_data {
                        QuotData::QuotEnd(_) => is_quot_end = true,
                        _ => {}
                    }
                    if let Err(e) = driver_tx.send(quot_data) {
                        error!("proxy transform quotation to py thread error: {}", e);
                    }
                },
                event = result_rx.recv(), if !result_channel_dead => {
                    if event.is_none() {
                        result_channel_dead = true;
                        continue;
                    }
                    let event = event.unwrap();

                    debug!("strategy event: {:?}", &event);
                    if let Err(e) = self.tx.send(event) {
                        error!("send strategy result error: {}", e);
                    }
                },
                _ = self.shutdown.recv() => {
                    drop(driver_tx);

                    break;
                },
                normal = thread_rx.recv() => {
                    if normal.is_ok() {
                        let normal = normal.unwrap();
                        info!("strategy py thread done, normal = {}", normal);
                        is_normal = normal;
                    }
                    break;
                },
            }
        }
        info!("strategy join py thread");
        handler.join().unwrap();
        info!("strategy task done");
        if !is_normal {
            bail!("py strategy exit with exception");
        }

        Ok(())
    }
    async fn run_dll(&mut self) -> Result<()> {
        info!("run rust dll strategy");

        let lib = unsafe { libloading::Library::new(&self.path) };
        if let Err(e) = lib {
            let msg = format!("failed to load strategy: {}, error: {}", &self.path, e);
            error!("{}", &msg);
            bail!(msg);
        }
        let lib = lib.unwrap();
        let symbol = unsafe { lib.get::<NewStrategyFunc>(b"new_strategy") };
        if let Err(e) = symbol {
            let msg = format!(
                "failed to load new_strategy func: {}, error: {}",
                &self.path, e
            );
            error!("{}", &msg);
            bail!(msg);
        }
        let new_strategy = symbol.unwrap();

        let mut strategy = new_strategy(self.account.clone(), self.opts.clone());

        let res = strategy
            .on_init()
            .await;

        if let Err(e) = res {
            let msg = format!("init strategy failed: {}, error: {}", &self.path, e);
            error!("{}", &msg);
            bail!(msg);
        }

        let mut is_quot_end = false;
        loop {
            tokio::select! {
                quot_data = self.rx.recv(), if !is_quot_end => {
                    if quot_data.is_none() {
                        error!("strategy recv quotation none");
                        is_quot_end = true;
                        continue;
                    }
                    let quot_data = quot_data.unwrap();

                    let res = match &quot_data {
                        QuotData::Quot(_) => {
                            strategy.on_quot(&quot_data).await
                        },
                        QuotData::QuotStart(_) | QuotData::MorningStart(_) | QuotData::NoonStart(_) => {
                            strategy.on_open(&quot_data).await
                        },
                        QuotData::MorningEnd(_) | QuotData::NoonEnd(_) => {
                            strategy.on_close(&quot_data).await
                        },
                        QuotData::QuotEnd(_) => {
                            is_quot_end = true;
                            strategy.on_close(&quot_data).await
                        }
                    };
                    if let Err(e) = res {
                        let msg = format!("call strategy failed: {}, error: {}", &self.path, e);
                        error!("{}", &msg);
                        bail!(msg);
                    }
                    // if is_quot_end {
                    //     if let Err(e) = strategy.on_destroy().await {
                    //         error!("strategy destroy failed: {}", e);
                    //     }
                    // }
                    if is_quot_end {
                        break;
                    }
                },
                _ = self.shutdown.recv() => {
                    break;
                },
            }
        }

        if let Err(e) = strategy.on_destroy().await {
            error!("strategy destroy failed: {}", e);
        }
        info!("done rust dll strategy");

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let p = Path::new(&self.path[..]);
        if !p.is_file() || !p.exists() {
            let msg = format!("strategy path={:?} does not exists!", p);
            error!("{}", &msg[..]);
            bail!(msg);
        }
        
        if self.path.ends_with(".py") {
            self.run_py().await?;
        } else {
            self.run_dll().await?;
        }
        Ok(())
    }
}
