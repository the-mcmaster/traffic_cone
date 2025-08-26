use crate::prelude::*;

const USER_URL: &str = "https://api.real-debrid.com/rest/1.0/user";

pub fn get_user() -> Json {
    send(Get(""), USER_URL)
}