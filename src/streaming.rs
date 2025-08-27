use crate::prelude::*;

const TRANSCODE_URL: &str = "https://api.real-debrid.com/rest/1.0/streaming/transcode/";
const MEDIA_INFOS_URL: &str = "https://api.real-debrid.com/rest/1.0/streaming/mediInfos/";

type Id = String;
pub fn transcode(id: Id) -> Json {
    send(Get(""), format!("{TRANSCODE_URL}{id}"))
}

pub fn media_infos(id: Id) -> Json {
    send(Get(""), format!("{MEDIA_INFOS_URL}{id}"))
}
