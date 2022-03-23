use serde::{Serialize, Deserialize};
use crate::{QError, QResult, Kind};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default, rename_all="kebab-case")]
pub struct Config {
    pub init_cash: f64,
    pub kind: Kind,
    pub fee: Fee,
    pub log: Log,
    pub push: Push,
    pub strategy: Strategy,
}

impl Config {
    pub fn from_str(s: &str) -> QResult<Self> {
        let cfg: Config = toml::from_str(s).map_err(|e|QError::Config(e))?;
        Ok(cfg)
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
}

impl Default for Log {
    fn default() -> Self {
        Self {
            level: "info".to_string()
        }
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Push {
    pub email: Email,
    pub wechat: Wechat,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
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
    pub monitor: Vec<String>,
    pub strategy: Vec<String>,
    pub broker: Vec<String>,
    pub risk: Vec<String>,
}


#[cfg(test)]
mod test {
    use crate::trader::config::Config;

    #[test]
    fn test_config() {
        let path = std::path::Path::new("./res/config.toml");
        let s = std::fs::read_to_string(path).unwrap();
        let cfg = Config::from_str(s.as_str()).unwrap();
        println!("cfg: {:?}", cfg);
    }
}
