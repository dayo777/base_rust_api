// You can create more custom error ENUMs as you deem fit.
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Custom error 1")]
    Error1,
    // #[error("Custom error 2")]
    // Error2,
    // #[error("Custom error 3")]
    // Error3,
}