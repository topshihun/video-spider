use mlua::prelude::LuaError;
use std::{error, fmt::Display};

#[derive(Debug)]
pub enum Error {
    PlayFailed(String),
    LuaFailed(LuaError),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PlayFailed(err) => {
                write!(f, "{}", err)
            }
            Self::LuaFailed(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::PlayFailed(_) => None,
            Self::LuaFailed(err) => Some(err),
        }
    }
}
