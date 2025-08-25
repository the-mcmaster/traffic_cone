//! # Hosts Module

use crate::prelude::*;

const HOSTS_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts";

pub fn get_hosts() -> Json {
    send(Get(""), HOSTS_URL)
}
