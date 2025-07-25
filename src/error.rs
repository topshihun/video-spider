
pub enum Error {
    Failed,
}

pub type Result<T> = std::result::Result<T, Error>;
