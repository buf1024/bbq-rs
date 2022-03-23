use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Stock,
    Fund,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Stock
    }
}

impl From<&str> for Kind {
    fn from(s: &str) -> Self {
        let ls = s.to_lowercase();
        if ls == "fund" {
            Self::Fund
        } else {
            Self::Stock
        }
    }
}
