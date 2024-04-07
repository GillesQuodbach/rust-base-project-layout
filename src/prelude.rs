//! Crate prelude
pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

// Generic Wrapper tuple struct for newtype pattern, mostly to implement external preference and type
pub struct  W<T>(pub T);

