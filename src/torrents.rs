use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

const TORRENTS_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents";
const TORRENT_INFO_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/info/";
const ACTIVE_COUNT_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/activeCount";
const AVAILABLE_HOSTS_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/availableHosts";
const ADD_TORRENT_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/addTorrent";
const ADD_MAGNET_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/addMagnet";
const SELECT_FILES_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/selectFiles/";
const DELETE_URL: &str = "https://api.real-debrid.com/rest/1.0/torrents/delete/";

pub fn get_torrents() -> Json {
    send(Get("{ {offset} : {8} }"), TORRENTS_URL)
}

type Id = String;
pub fn get_torrent_info(id: Id) -> Json {
    send(Get(""), format!("{TORRENT_INFO_URL}{id}"))
}

pub fn get_active_count() -> Json {
    send(Get(""), ACTIVE_COUNT_URL)
}

pub fn get_available_hosts() -> Json {
    send(Get(""), AVAILABLE_HOSTS_URL)
}

type Host = String;
#[derive(Serialize, Deserialize, Getters, Debug)]
struct AddTorrent {
    host: Host,
}
pub fn add_torrent(host: Host) -> Json {
    let body = serde_json::ser::to_string_pretty(&AddTorrent { host }).unwrap();

    send(Get(body), ADD_TORRENT_URL)
}

type Link = String;
pub fn add_magnet(link: Link) -> Json {
    let body = format!("{} 'magnet': '{}' {}", '{', link, '}');

    send(Post(body), ADD_MAGNET_URL)
}

type Files = String;
pub fn select_files(id: Id, files: Files) -> Json {
    let body = format!("{} 'files': '{}' {}", '{', files, '}');

    send(Post(body), format!("{SELECT_FILES_URL}{id}"))
}

pub fn delete(id: Id) -> Json {
    send(Delete(""), format!("{DELETE_URL}{id}"))
}
