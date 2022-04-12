use std::{fs, path::Path};
use anyhow::{Context, Ok, Result};
use bbq_core::Kind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all = "snake_case")]
pub struct Config {
    pub init_cash: f64,
    pub data_path: String,
    pub mongodb: Option<String>,
    pub kind: Kind,
    pub fee: Fee,
    pub log: Log,
    pub listen: Listen,
    pub push: Push,
    pub strategy: Strategy,
}

impl Config {
    pub fn from_str(s: &str) -> Result<Self> {
        let cfg: Config = toml::from_str(s).with_context(|| "load cfg failed")?;
        std::env::set_var(
            "BBQ_TRADE_DATE",
            format!("{}/trade_date.txt", cfg.data_path.as_str()).as_str(),
        );
        Ok(cfg)
    }
    pub fn write<P: AsRef<Path>>(&self, path: Option<P>) -> Result<()> {
        let cfg = if path.is_none() {
            let dir = directories::BaseDirs::new().with_context(|| "failed to get base dirs")?;
            dir.home_dir()
                .join(".config")
                .join("bbq-trader")
                .join("config.toml")
                .to_str()
                .with_context(|| "failed to get default path")?
                .to_string()
        } else {
            path.unwrap()
                .as_ref()
                .to_str()
                .with_context(|| "failed to get default path")?
                .to_string()
        };

        let cfg_str = toml::to_string(&self).unwrap();
        fs::write(cfg, cfg_str).with_context(|| "failed to write file")?;

        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        let def = Self {
            init_cash: Default::default(),
            data_path: Default::default(),
            kind: Default::default(),
            fee: Default::default(),
            log: Default::default(),
            push: Default::default(),
            strategy: Default::default(),
            listen: Default::default(),
            mongodb: Some("mongodb://localhost:27017".to_string()),
        };
        if let Some(dir) = directories::BaseDirs::new() {
            let h = dir.home_dir();

            let h = h.join(".config").join("bbq-trader");
            let log = h.join("logs");
            let strategy = h.join("strategy").join("strategy");
            let broker = h.join("strategy").join("broker");
            let risk = h.join("strategy").join("risk");

            for p in vec![&h, &log, &strategy, &broker, &risk] {
                if let Err(_) = fs::create_dir_all(p) {
                    return def;
                }
            }

            let data_path = h.to_str().map(|s| String::from(s)).unwrap();

            std::env::set_var(
                "BBQ_TRADE_DATE",
                format!("{}/trade_date.txt", data_path.as_str()).as_str(),
            );

            let log_path = log.to_str().map(|s| String::from(s)).unwrap();

            let strategy = vec![strategy.to_str().map(|s| String::from(s)).unwrap()];

            let broker = vec![broker.to_str().map(|s| String::from(s)).unwrap()];

            let risk = vec![risk.to_str().map(|s| String::from(s)).unwrap()];

            let def = Self {
                init_cash: 10_000.0,
                data_path,
                kind: Default::default(),
                fee: Default::default(),
                log: Log {
                    level: "debug".to_string(),
                    path: log_path,
                },
                push: Default::default(),
                strategy: Strategy {
                    strategy,
                    broker,
                    risk,
                },
                listen: Default::default(),
                mongodb: Some("mongodb://localhost:27017".to_string()),
            };
            return def;
        }
        return def;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fee {
    pub broker: f64,
    pub transfer: f64,
    pub tax: f64,
}

impl Default for Fee {
    fn default() -> Self {
        Self {
            broker: 0.00025,
            transfer: 0.00002,
            tax: 0.001,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
    pub level: String,
    pub path: String,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            #[cfg(debug_assertions)]
            level: "debug".to_string(),
            #[cfg(not(debug_assertions))]
            level: "info".to_string(),
            path: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listen {
    pub port: i32,
}

impl Default for Listen {
    fn default() -> Self {
        Self { port: 9527 }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Push {
    pub email: Option<Email>,
    pub wechat: Option<Wechat>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Email {
    pub smtp_host: String,
    pub smtp_port: i32,
    pub user: String,
    pub token: String,
    pub notify: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Wechat {
    pub token: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub strategy: Vec<String>,
    pub broker: Vec<String>,
    pub risk: Vec<String>,
}

#[cfg(test)]
mod test {
    use crate::config::Config;

    #[test]
    fn test_config() {
        let cfg = Config::default();
        println!("cfg: {:?}", cfg);

        cfg.write::<String>(None).unwrap();
    }
}
