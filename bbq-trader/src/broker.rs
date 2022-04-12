use anyhow::{bail, Context, Ok, Result};
use bbq_core::{trader::entrust::EntrustStatus, EntrustType};
use bbq_core::{BrokerEvent, Event};
use bbq_strategy::NewBrokerFunc;
use convert_case::{Case, Casing};
use log::{debug, error, info};
use pyo3::types::{PyModule, PyString};
use std::fs;
use std::time::Duration;
use std::{collections::HashMap, path::Path};
use tokio::sync::{
    broadcast,
    mpsc::{self, UnboundedReceiver, UnboundedSender},
};

pub const POLL_SECONDS: u64 = 3;

pub struct Broker {
    rx: UnboundedReceiver<Event>,
    tx: UnboundedSender<Event>,

    path: Option<String>,
    opts: Option<HashMap<String, String>>,

    shutdown: broadcast::Receiver<bool>,
}

impl Broker {
    pub fn new(
        path: Option<String>,
        opts: Option<HashMap<String, String>>,
        shutdown: broadcast::Receiver<bool>,
    ) -> (Self, UnboundedSender<Event>, UnboundedReceiver<Event>) {
        let (broker_entrust_tx, rx) = mpsc::unbounded_channel();
        let (tx, broker_push_rx) = mpsc::unbounded_channel();

        (
            Self {
                rx,
                tx,
                path,
                opts,
                shutdown,
            },
            broker_entrust_tx,
            broker_push_rx,
        )
    }

    async fn run_py(&mut self) -> Result<()> {
        info!("run python broker");

        // let (broker_entrust_tx, rx) = mpsc::unbounded_channel();
        let (tx, mut broker_push_rx) = mpsc::unbounded_channel();

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
            .with_context(|| format!("failed to read py broker code, name: {}", &path))?;

        let py_opts = serde_json::to_string(&self.opts)
            .with_context(|| format!("failed to serialize opts, name: {}", &path))?;

        let (driver_tx, mut driver_rx) = mpsc::unbounded_channel();
        // let (result_tx, mut result_rx) = mpsc::unbounded_channel();

        let (thread_tx, mut thread_rx) = broadcast::channel(1);
        let handler = std::thread::spawn(move || {
            let py_result = pyo3::Python::with_gil(|py| {
                let broker = PyModule::from_code(py, py_code.as_str(), &fname, &fname)
                    .with_context(|| "failed to load python code")?;

                let py_class = broker
                    .getattr(py_class_name.as_str())
                    .with_context(|| format!("failed to get broker class: {}", &py_class_name))?;

                let py_inst = py_class
                    .call0()
                    .with_context(|| format!("failed to initial class: {}", &py_class_name))?;

                let py_name = py_inst
                    .getattr("name")
                    .with_context(|| format!("failed to get broker name: {}", &py_class_name))?;

                let py_name_res = py_name.call0();
                if let Err(e) = &py_name_res {
                    error!("get broker name err:{}", e);
                    bail!("get broker name err:{}", e)
                }
                let name_res = py_name_res.unwrap().extract();
                if let Err(e) = &name_res {
                    error!("extract broker name err:{}", e);
                    bail!("extract broker name err:{}", e)
                }
                let name: String = name_res.unwrap();
                info!("broker => {}:{}", &py_class_name, &name);

                let init_res = py_inst.call1(("", "init", "", py_opts));
                if let Err(e) = &init_res {
                    error!("init broker err:{}", e);
                    bail!("init broker err:{}", e)
                }
                let init_res = init_res.unwrap().extract();
                if let Err(e) = &init_res {
                    error!("init extract err:{}", e);
                    bail!("init extract err:{}", e)
                }
                let is_init: bool = init_res.unwrap();

                if !is_init {
                    bail!("failed to init broker, class: {}", &py_class_name);
                }

                loop {
                    let event = driver_rx.blocking_recv();
                    if event.is_none() {
                        info!("py outer channel close exit");
                        break;
                    }
                    let mut is_broker_end = false;
                    let event = event.unwrap();
                    match &event {
                        Event::Entrust(entrust) => {
                            let js = serde_json::to_string(entrust)
                                .with_context(|| "failed to convert json")?;
                            let typ = match entrust.entrust_type {
                                EntrustType::Buy => "buy",
                                EntrustType::Sell => "sell",
                                EntrustType::Cancel => "cancel",
                            };
                            let rst = py_inst.call1(("", "on_entrust", typ, js));

                            if let Err(e) = &rst {
                                error!("broker entrust err:{}", e);
                                bail!("broker entrust err:{}", e)
                            }
                        }
                        Event::EventNone(cmd) => {
                            if *cmd == "QUIT".to_string() {
                                is_broker_end = true;
                            } else if *cmd == "POLL".to_string() {
                                debug!("py poll broker");
                                let rst = py_inst.call1(("", "on_poll", "", ""));

                                if let Err(e) = &rst {
                                    error!("broker poll err:{}", e);
                                    bail!("broker poll err:{}", e)
                                }
                                let rst = rst.unwrap();
                                if !rst.is_none() {
                                    let py_str = rst.downcast::<PyString>();
                                    if py_str.is_ok() {
                                        let py_str: String = py_str.unwrap().extract().unwrap();

                                        let evt_list = serde_json::from_str(py_str.as_str());
                                        if evt_list.is_ok() {
                                            let evt_list: Vec<BrokerEvent> = evt_list.unwrap();
                                            for event in evt_list {
                                                info!("broker push event: {:?}", &event);
                                                tx.send(Event::Broker(event)).with_context(
                                                    || "py push_tx send event error",
                                                )?;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }

                    if is_broker_end {
                        if let Err(_) = py_inst.call1(("", "destroy", "", "")) {}
                        break;
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

        let mut is_entrust_tx_end = false;
        let mut is_push_tx_end = false;
        let mut is_normal = true;

        let poll_duration = Duration::from_secs(POLL_SECONDS);
        loop {
            tokio::select! {
                event = self.rx.recv(), if !is_entrust_tx_end => {
                    if event.is_none() {
                        info!("py broker receive none event(all sender die)");
                        is_entrust_tx_end = true;
                        continue;
                    }
                    let event = event.unwrap();

                    if let Err(e) = driver_tx.send(event) {
                        error!("proxy transform broker to py thread error: {}", e);
                    }
                },
                event = broker_push_rx.recv(), if !is_push_tx_end => {
                    if event.is_none() {
                        info!("py broker push receive none event(all sender die)");
                        is_push_tx_end = true;
                        continue;
                    }
                    let event = event.unwrap();
                    if let Err(e) = self.tx.send(event) {
                        error!("broker proxy push event error: {}", e);
                    }
                },
                _ = self.shutdown.recv() => {
                    drop(driver_tx);

                    break;
                },
                normal = thread_rx.recv() => {
                    if normal.is_ok() {
                        let normal = normal.unwrap();
                        info!("broker py thread done, normal = {}", normal);
                        is_normal = normal;
                    }
                    break;
                },
                _ = tokio::time::sleep(poll_duration), if !is_entrust_tx_end => {
                    let event = Event::EventNone("POLL".to_string());
                    if let Err(e) = driver_tx.send(event) {
                        error!("proxy transform broker poll to py thread error: {}", e);
                    }
                }
            }
        }
        info!("broker join py thread");
        handler.join().unwrap();
        info!("broker task done");
        if !is_normal {
            bail!("py broker exit with exception");
        }
        Ok(())
    }
    async fn run_dll(&mut self) -> Result<()> {
        info!("run rust dll broker");
        let path = self.path.as_ref().unwrap().clone();

        let lib = unsafe { libloading::Library::new(&path) };
        if let Err(e) = lib {
            let msg = format!("failed to load broker: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }
        let lib = lib.unwrap();
        let symbol = unsafe { lib.get::<NewBrokerFunc>(b"new_broker") };
        if let Err(e) = symbol {
            let msg = format!("failed to load broker func: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }
        let new_broker = symbol.unwrap();

        let mut broker = new_broker(self.tx.clone(), self.opts.clone());

        let res = broker.on_init().await;

        if let Err(e) = res {
            let msg = format!("init broker failed: {}, error: {}", &path, e);
            error!("{}", &msg);
            bail!(msg);
        }

        let mut is_entrust_tx_end = false;

        loop {
            tokio::select! {
                event = self.rx.recv(), if !is_entrust_tx_end => {
                    if event.is_none() {
                        info!("dll broker receive none event(all sender die)");
                        is_entrust_tx_end = true;
                        continue;
                    }
                    let event = event.unwrap();
                    match event {
                        Event::Entrust(entrust) => {
                            if let Err(e) = broker.on_entrust(&entrust).await {
                                error!("broker dll entrust failed: {}!", e);
                            }
                        },
                        Event::EventNone(_) => {
                            break;
                        },
                        _ => {},
                    }
                },
                _ = self.shutdown.recv() => {
                    break;
                }
            }
        }

        if let Err(e) = broker.on_destroy().await {
            error!("broker destroy failed: {}", e);
        }
        info!("done run rust dll broker");

        Ok(())
    }
    async fn run_default(&mut self) -> Result<()> {
        info!("run default broker(simulation broker)");
        let mut is_entrust_tx_end = false;
        loop {
            tokio::select! {
                event = self.rx.recv(), if !is_entrust_tx_end => {
                    if event.is_none() {
                        info!("broker receive none event(all sender die)");
                        is_entrust_tx_end = true;
                        continue;
                    }
                    let event = event.unwrap();
                    match event {
                        Event::Entrust(e) => {
                            info!("broker recv: {:?}", &e);
                            let mut e = e.clone();
                            e.desc = format!("default simulation broker handled! {}", &e.desc[..]);
                            match &e.entrust_type {
                                EntrustType::Buy | EntrustType::Sell  => {
                                    e.status = EntrustStatus::Deal;
                                    e.volume_deal = e.volume;
                                    e.volume_cancel = 0;
                                },
                                EntrustType::Cancel => {
                                    e.status = EntrustStatus::Cancel;
                                    e.volume_deal = 0;
                                    e.volume_cancel = e.volume;
                                },
                            }

                            if let Err(e) = self.tx.send(Event::Broker(BrokerEvent::Entrust(e))) {
                                error!("broker feedback entrust event failed: {}!", e);
                            }
                        },
                        Event::EventNone(_) => {
                            break;
                        },
                        _ => {},
                    }
                },
                _ = self.shutdown.recv() => {
                    break;
                }
            }
        }
        info!("done default broker(simulation broker)");
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        match self.path {
            Some(ref path) => {
                let p = Path::new(path);
                if !p.is_file() || !p.exists() {
                    let msg = format!("broker path={:?} does not exists!", p);
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
