use crate::prelude::*;

const TRAFFIC_URL: &str = "https://api.real-debrid.com/rest/1.0/traffic";
const DETAILS_URL: &str = "https://api.real-debrid.com/rest/1.0/traffic/details";

pub fn get_traffic() -> Json {
    send(Get(""), TRAFFIC_URL)
}

pub fn get_details() -> Json {
    send(Get(""), DETAILS_URL)
}
