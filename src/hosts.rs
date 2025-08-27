//! # Hosts Module

use crate::prelude::*;

const HOSTS_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts";
const STATUS_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts/status";
const REGEX_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts/regex";
const REGEX_FOLDER_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts/regexFolder";
const DOMAINS_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts/domains";

pub fn get_hosts() -> Json {
    send(Get(""), HOSTS_URL)
}

pub fn get_status() -> Json {
    send(Get(""), STATUS_URL)
}

pub fn get_regex() -> Json {
    send(Get(""), REGEX_URL)
}

pub fn get_regex_folder() -> Json {
    send(Get(""), REGEX_FOLDER_URL)
}

pub fn get_domains() -> Json {
    send(Get(""), DOMAINS_URL)
}
