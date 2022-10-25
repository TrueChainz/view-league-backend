use std::error;

use serde::{Deserialize, Serialize};

pub type FetchResult<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Deserialize, Serialize, Debug)]
struct StatusBody {
    message: String,
    status_code: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Status {
    status: StatusBody,
}