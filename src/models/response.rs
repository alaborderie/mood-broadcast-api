use rocket::serde::json::Value;
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub message: String,
    pub data: Value,
}

#[derive(Debug)]
pub struct ResponseWithStatus {
    pub status_code: u16,
    pub response: Response
}
