use anyhow::{Context, Result};

pub mod fetch;

pub mod trader;
pub use trader::*;

mod consts;
pub use consts::*;

mod proto;

pub mod data;

pub fn setup_log( file: &str, level: &str) -> Result<()> {
    use std::str::FromStr;

    let log_level = log::LevelFilter::from_str(level)
        .with_context(|| format!("invalid log level: {}", level))?;

    let log_path = fern::log_file(file).with_context(|| format!("invalid log path: {}", file))?;

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
    Ok(())
}
