//! # Hosts Module

use std::{collections::HashMap, io::Read};

use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::{default_headers, HTTP_CLIENT};

pub(crate) type Id = String;
pub(crate) type Name = String;
pub(crate) type Url = String;

pub(crate) type HostKey = String;
pub(crate) type Hosts = HashMap<HostKey, Host>;

const HOSTS_URL: &str = "https://api.real-debrid.com/rest/1.0/hosts";

/// A host entry.
///
/// This contains the serialized information
/// from the hosts request.
#[derive(Serialize, Deserialize)]
#[derive(Getters)]
#[derive(Debug)]
pub struct Host {
    id: Id,
    name: Name,
    image: Url
}


pub fn get_hosts_json() -> String {
    let mut body = String::new();

    default_headers(HTTP_CLIENT.get(HOSTS_URL))
        .send()
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();

    body
}

pub fn get_hosts() -> Hosts {
    serde_json::de::from_str(&get_hosts_json()).unwrap()
}