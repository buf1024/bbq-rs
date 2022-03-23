mod fetch;

mod error;
mod trader;
mod data;
mod consts;
pub use consts::*;

pub use error::QError;

pub use trader::account::Account;
pub use trader::entrust::Entrust;
pub use trader::deal::Deal;
pub use trader::position::Position;
pub use trader::signal::Signal;

pub type QResult<T> = Result<T, QError>;
