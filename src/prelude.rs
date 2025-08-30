pub use crate::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub struct Wrapper<T>(pub T);