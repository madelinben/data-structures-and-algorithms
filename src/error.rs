#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Input Error: {0}")]
    Input(String),
    
    #[error("Validation Error: {0}")]
    Validation(String),
    
    #[error("Not Found Error: {0}")]
    NotFound(String),
    
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
}

impl Error {
    pub fn input(msg: impl Into<String>) -> Self {
        Error::Input(msg.into())
    }
    
    pub fn validation(msg: impl Into<String>) -> Self {
        Error::Validation(msg.into())
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        Error::NotFound(msg.into())
    }
    
    pub fn generic(msg: impl Into<String>) -> Self {
        Error::Generic(msg.into())
    }
}