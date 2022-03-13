use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum QError {

    #[error("GetTradeDay error: {0}")]
    GetTradeDay(String),

    #[error("Fetch error: {0}")]
    Fetch(String),
}
