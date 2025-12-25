mod config_lua;
pub mod error;
pub mod search;
pub mod series;
mod utils;

pub use error::Error;
pub use error::Result;
pub use search::get_detail;
pub use search::search;

pub use config_lua::output::Output;
pub use series::Episode;
pub use series::Series;
