use crate::prelude::*;

const CHECK_URL: &str = "https://api.real-debrid.com/rest/1.0/unrestrict/check";
const LINK_URL: &str = "https://api.real-debrid.com/rest/1.0/unrestrict/link";
const FOLDER_URL: &str = "https://api.real-debrid.com/rest/1.0/unrestrict/folder";
const CONTAINER_FILE_URL: &str = "https://api.real-debrid.com/rest/1.0/unrestrict/containerFile";
const CONTAINER_LINK_URL: &str = "https://api.real-debrid.com/rest/1.0/unrestrict/containerLink";

pub fn check(link: String) -> Json {
    let body = format!("{} 'link': '{}' {}", '{', link, '}');

    send(Post(body), CHECK_URL)
}

pub fn link(link: String) -> Json {
    let body = format!("{} 'link': '{}' {}", '{', link, '}');

    send(Post(body), LINK_URL)
}

pub fn folder(link: String) -> Json {
    let body = format!("{} 'link': '{}' {}", '{', link, '}');

    send(Post(body), FOLDER_URL)
}

pub fn container_file() -> Json {
    send(Put(""), CONTAINER_FILE_URL)
}

pub fn container_link(link: String) -> Json {
    let body = format!("{} 'link': '{}' {}", '{', link, '}');

    send(Post(body), CONTAINER_LINK_URL)
}
