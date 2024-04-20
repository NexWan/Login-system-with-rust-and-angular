use std::sync::Mutex;
use serde::{Deserialize, Serialize};

//This struct is used to create a USER type which deserialize the JSON data coming from the front end
#[derive(Deserialize, Debug)]
pub struct User {
    pub username: Mutex<String>,
    pub password: Mutex<String>,
}

#[derive(Serialize, Debug)]
pub struct Response {
    pub status: String,
    pub message: String,
}