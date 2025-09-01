use crate::prelude::*;

const SETTINGS_URL: &str = "https://api.real-debrid.com/rest/1.0/settings";
const UPDATE_URL: &str = "https://api.real-debrid.com/rest/1.0/settings/update";
const CONVERT_POINTS_URL: &str = "https://api.real-debrid.com/rest/1.0/settings/convertPoints";
const CHANGE_PASSWORD_URL: &str = "https://api.real-debrid.com/rest/1.0/settings/changePassword";
const AVATAR_FILE_URL: &str = "https://api.real-debrid.com/rest/1.0/settings/avatarFile";
const AVATAR_DELETE_URL: &str = "https://api.real-debrid.com/rest/1.0/settings/avatarDelete";

pub fn get_settings() -> Json {
    send(Get(""), SETTINGS_URL)
}

pub fn update(setting_name: String, setting_value: String) -> Json {
    let body = format!(
        "{} 'setting_name': '{}', 'setting_value' {} {}",
        '{', setting_name, setting_value, '}'
    );

    send(Post(body), UPDATE_URL)
}

pub fn convert_points() -> Json {
    send(Post(""), CONVERT_POINTS_URL)
}

pub fn change_password() -> Json {
    send(Post(""), CHANGE_PASSWORD_URL)
}

pub fn avatar_file() -> Json {
    send(Put(""), AVATAR_FILE_URL)
}

pub fn avatar_delete() -> Json {
    send(Delete(""), AVATAR_DELETE_URL)
}

/* pub fn link(link: String) -> Json {
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
} */
