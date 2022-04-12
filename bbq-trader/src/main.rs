use std::str::FromStr;
use bbq_trader::{config::Config, trader::Trader};
use chrono::Local;
use log::debug;
use tokio::{signal, sync::broadcast};

use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::default();

    let log_level = log::LevelFilter::from_str(cfg.log.level.to_uppercase().as_str())
        .with_context(|| format!("invalid log level: {}", cfg.log.level.clone()))?;
        
    let log_file = format!("{}/trader.log", cfg.log.path);
    let log_path = fern::log_file(log_file)
        .with_context(|| format!("invalid log path: {}", cfg.log.path.clone()))?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S%.f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .chain(log_path)
        .apply()
        .with_context(|| "failed to setup logger")?;

    log::info!("logger setup success!");

    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    tokio::spawn(async move {
        let mut ls = 0;
        const PRESS_INTERVAL: i64 = 3;

        loop {
            if let Err(_) = signal::ctrl_c().await {
                panic!("failed to listen ctrl-c");
            }
            let ns = Local::now().timestamp();
            if ns - ls < PRESS_INTERVAL {
                if let Err(e) = shutdown_tx.send(true) {
                    panic!("failed to send shutdown signal: {:?}", e);
                }
                break;
            }
            println!("press once more to exit");
            ls = ns;
        }
    });

    let mut trader = Trader::new(cfg, shutdown_rx);

    trader.init().await?;

    debug!("start engine");
    trader.run().await?;

    Ok(())
}
