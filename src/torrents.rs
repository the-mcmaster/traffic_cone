use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub(crate) type Link = String;

const TORRENTS_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents";
const ADD_MAGNET_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/addMagnet";

pub fn get_torrents() -> Json {
    send(Get(""), TORRENTS_URL)
}

#[derive(Serialize, Deserialize, Getters, Debug)]
pub struct Magnet {
    magnet: Link,
}
pub fn add_magnet(link: Link) -> Json {
    #[cfg(not(debug_assertions))]
    todo!("this function needs some debugging first");

    let body = serde_json::ser::to_string_pretty(&Magnet { magnet: link }).unwrap();

    send(Post(body), ADD_MAGNET_URL)
}
