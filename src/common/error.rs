use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)] Generic(#[from] Box<dyn std::error::Error>),
}

pub type Result<T> = std::result::Result<T, Error>;
