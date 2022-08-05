pub type ResultWithError<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    FnError(String),
    #[error("{0}")]
    ParseConfigurationError(String),
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Error::ParseConfigurationError(e.to_string())
    }
}