mod fetch;

mod error;
mod trader;

pub use error::QError;

pub use trader::account::Account;
pub use trader::entrust::Entrust;
pub use trader::deal::Deal;
pub use trader::position::Position;
pub use trader::signal::Signal;

pub type QResult<T> = Result<T, QError>;


#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum TrKind {
    Fund,
    Stock,
}
