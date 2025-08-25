//! # Downloads Module
//!
//! This module provides two api calls: `get_downloads()` and `delete_download(id)`.
//!
//! ## `get_downloads()`
//!
//! This requests the downloads list from the API.
//!
//! This returns a serialized list of the `Download`s.
//!
//! ## `delete_download(id)`
//!
//! This requests that a specific download be deleted.
//! The `id` is a string received as the "id" from `get_downloads()`

use crate::prelude::*;

const DOWNLOAD_URL: &str = "https://api.real-debrid.com/rest/1.0/downloads";
const DELETE_DOWNLOAD_URL: &str = "https://api.real-debrid.com/rest/1.0/downloads/delete/";

/// Get the downloads in json form.
pub fn get_downloads() -> Json {
    send(Get(""), DOWNLOAD_URL)
}

type Id = String;

/// Delete a specific download by its id.
pub fn delete_download(id: Id) -> Json {
    send(Delete(""), format!("{DELETE_DOWNLOAD_URL}{id}"))
}
