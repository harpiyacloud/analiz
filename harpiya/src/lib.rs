use tonic::{Status, Response};

mod crypto;
pub mod config;
mod encoding;
mod error;
mod header;
mod database;
pub mod services;
mod utils;

pub type ServiceResult<T> = Result<Response<T>, Status>;