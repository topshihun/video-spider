
#[derive(Debug)]
pub enum Error {
    PlayFailed(String),
    LuaFailed(String),
}

pub type Result<T> = std::result::Result<T, Error>;
