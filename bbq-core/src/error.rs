use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum QError {

    #[error("GetTradeDay error: {0}")]
    GetTradeDay(String),

    #[error("Fetch error: {0}")]
    Fetch(String),

    #[error("MongDB error")]
    MongoDB(#[from] mongodb::error::Error),

    #[error("MongoDB Customer Error")]
    MongoDBError(String),

    #[error("MongDB error")]
    Config(#[from] toml::de::Error),
}

