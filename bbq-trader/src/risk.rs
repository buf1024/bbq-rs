use anyhow::{bail, Context, Result};
use bbq_core::{Event, Signal, SignalSource};
use bbq_core::{trader::Account, QuotData};
use bbq_strategy::NewRiskFunc;
use convert_case::{Case, Casing};
use log::{debug, error, info};
use pyo3::types::{PyModule, PyString};
use uuid::Uuid;
use std::fs;
use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, RwLock},
};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

pub struct Risk {
    tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<QuotData>,

    path: Option<String>,
    opts: Option<HashMap<String, String>>,

    shutdown: broadcast::Receiver<bool>,

    account: Arc<RwLock<Account>>,
}

impl Risk {
    pub fn new(
        path: Option<String>,
        opts: Option<HashMap<String, String>>,
        shutdown: broadcast::Receiver<bool>,
        account: Arc<RwLock<Account>>,
    ) -> (Self, UnboundedSender<QuotData>, UnboundedReceiver<Event>) {
        let (tx, risk_rx) = mpsc::unbounded_channel();
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
            risk_rx,
        )
    }

    async fn run_py(&mut self) -> Result<()> {
        info!("run python risk");
        let path = self.path.as_ref().unwrap().clone();

        let fname = Path::new(&path)
            .file_name()
            .with_context(|| format!("failed to get file name: {}", &path))?
            .to_str()
            .with_context(|| format!("failed to convert file name: {}", &path))?
            .to_string();

        let py_class_name = fname
            .split(".")
            .next()
            .with_context(|| format!("failed to get py class name: {}", &path))?
            .to_case(Case::Pascal);

        let py_code = fs::read_to_string(&path)
            .with_context(|| format!("failed to read py risk code, name: {}", &path))?;

        let py_opts = serde_json::to_string(&self.opts)
            .with_context(|| format!("failed to serialize opts, name: {}", &path))?;

        let (driver_tx, mut driver_rx) = mpsc::unbounded_channel();
        let (result_tx, mut result_rx) = mpsc::unbounded_channel();

        let account = self.account.clone();

        let (thread_tx, mut thread_rx) = broadcast::channel(1);
        let handler = std::thread::spawn(move || {
            let py_result = pyo3::Python::with_gil(|py| {
                let risk = PyModule::from_code(py, py_code.as_str(), &fname, &fname)
                    .with_context(|| "failed to load python code")?;

                let py_class = risk
                    .getattr(py_class_name.as_str())
                    .with_context(|| format!("failed to get risk class: {}", &py_class_name))?;

                let py_inst = py_class
                    .call0()
                    .with_context(|| format!("failed to initial class: {}", &py_class_name))?;

                let py_name = py_inst
                    .getattr("name")
                    .with_context(|| format!("failed to get risk name: {}", &py_class_name))?;

                let py_name_res = py_name.call0();
                if let Err(e) = &py_name_res {
                    error!("get risk name err:{}", e);
                    bail!("get risk name err:{}", e)
                }
                let name_res = py_name_res.unwrap().extract();
                if let Err(e) = &name_res {
                    error!("extract risk name err:{}", e);
                    bail!("extract risk name err:{}", e)
                }
                let name: String = name_res.unwrap();
                info!("risk => {}:{}", &py_class_name, &name);

                let acct_js = {
                    let acct = account.read().unwrap();

                    serde_json::to_string(&acct.to_owned()).unwrap()
                };

                let init_res = py_inst.call1((&acct_js, "init", "", py_opts));
                if let Err(e) = &init_res {
                    error!("init risk err:{}", e);
                    bail!("init risk err:{}", e)
                }
                let init_res = init_res.unwrap().extract();
                if let Err(e) = &init_res {
                    error!("init extract err:{}", e);
                    bail!("init extract err:{}", e)
                }
                let is_init: bool = init_res.unwrap();

                if !is_init {
                    bail!("failed to init risk, class: {}", &py_class_name);
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
                        QuotData::Quot(_) => py_inst.call1((&acct_js, "on_risk", "quot", js)),
                        QuotData::QuotStart(_) => {
                            py_inst.call1((&acct_js, "on_risk", "quot_start", js))
                        }
                        QuotData::MorningStart(_) => {
                            py_inst.call1((&acct_js, "on_risk", "morning_start", js))
                        }
                        QuotData::MorningEnd(_) => {
                            py_inst.call1((&acct_js, "on_risk", "morning_end", js))
                        }
                        QuotData::NoonStart(_) => {
                            py_inst.call1((&acct_js, "on_risk", "noon_open", js))
                        }
                        QuotData::NoonEnd(_) => {
                            py_inst.call1((&acct_js, "on_risk", "noon_end", js))
                        }
                        QuotData::QuotEnd(_) => {
                            is_quot_end = true;
                            py_inst.call1((&acct_js, "on_risk", "quot_end", js))
                        }
                    };

                    if let Err(e) = &rst {
                        error!("risk quotation err:{}", e);
                        bail!("risk quotation err:{}", e)
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
                                let evt_list: Vec<Signal> = evt_list.unwrap();
                                for mut signal in evt_list {
                                    signal.source = SignalSource::Strategy(name.clone());
                                    signal.signal_id = Uuid::new_v4().to_simple().to_string();
                                    info!("risk signal: {:?}", &signal);
                                    result_tx
                                        .send(Event::Signal(signal))
                                        .with_context(|| "result_tx send signal error")?;
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
                        error!("risk recv quotation none");
                        is_quot_end = true;
                        continue;
                    }
                    let quot_data = quot_data.unwrap();

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
                        info!("risk py thread done, normal = {}", normal);
                        is_normal = normal;
                    }
                    break;
                },
            }
        }
        info!("risk join py thread");
        handler.join().unwrap();
        info!("risk task done");
        if !is_normal {
            bail!("py broker exit with exception");
        }

        Ok(())
    }
    async fn run_dll(&mut self) -> Result<()> {
        info!("run rust dll risk");
        let path = self.path.as_ref().unwrap().clone();

        let lib = unsafe { libloading::Library::new(&path) };
        if let Err(e) = lib {
            let msg = format!("failed to load risk: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }
        let lib = lib.unwrap();
        let symbol = unsafe { lib.get::<NewRiskFunc>(b"new_risk") };
        if let Err(e) = symbol {
            let msg = format!("failed to load risk func: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }
        let new_risk = symbol.unwrap();

        let mut risk = new_risk(self.account.clone(), self.tx.clone(), self.opts.clone());

        let res = risk.on_init().await;

        if let Err(e) = res {
            let msg = format!("init risk failed: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }

        let mut is_quot_end = false;

        loop {
            tokio::select! {
                quot_data = self.rx.recv(), if !is_quot_end => {
                    if quot_data.is_none() {
                        error!("risk recv quotation none");
                        is_quot_end = true;
                        continue;
                    }
                    debug!("dll risk on risk!");
                    let quot_data = quot_data.unwrap();

                    match quot_data {
                        QuotData::QuotEnd(_) => {
                            break;
                        },
                        _ => {
                            let is_trading = {
                                let acct = self.account.read().unwrap();
                                acct.is_trading
                            };
                            if is_trading {
                                let res = risk.on_risk().await;

                                if let Err(e) = res {
                                    let msg = format!("call risk failed: {}, error: {}", &path, e);
                                    error!("{}", &msg);
                                    bail!(msg);
                                }
                            }
                        }
                    }
                }
                _ = self.shutdown.recv() => {
                    break;
                },
            }
        }

        if let Err(e) = risk.on_destroy().await {
            error!("risk destroy failed: {}", e);
        }
        info!("done run rust dll risk");

        Ok(())
    }
    async fn run_default(&mut self) -> Result<()> {
        info!("run default risk(no risk control)");
        let mut is_quot_end = false;
        loop {
            tokio::select! {
                quot_data = self.rx.recv(), if !is_quot_end => {
                    if quot_data.is_none() {
                        error!("risk recv quotation none");
                        is_quot_end = true;
                        continue;
                    }
                    debug!("dll risk on risk!");
                    let quot_data = quot_data.unwrap();

                    match quot_data {
                        QuotData::QuotEnd(_) => {
                            break;
                        },
                        _ => {}
                    }
                }
                _ = self.shutdown.recv() => {
                    break;
                },
            }
        }
        info!("done default risk(no risk control)");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        match self.path {
            Some(ref path) => {
                let p = Path::new(path);
                if !p.is_file() || !p.exists() {
                    let msg = format!("risk path={:?} does not exists!", p);
                    error!("{}", &msg[..]);
                    bail!(msg);
                }
                if path.ends_with(".py") {
                    self.run_py().await?;
                } else {
                    self.run_dll().await?;
                }
            }
            None => {
                self.run_default().await?;
            }
        }
        Ok(())
    }
}
