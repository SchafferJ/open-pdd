use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error occurred")]
    Reqwest(#[from] reqwest::Error),

    #[error("IO error occurred")]
    IO(#[from] io::Error),

    #[error("serde json error occurred")]
    Json(#[from] serde_json::Error),
}
