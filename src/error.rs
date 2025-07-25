
pub enum Error {
    PlayFailed(String),
    Failed,
}

pub type Result<T> = std::result::Result<T, Error>;
