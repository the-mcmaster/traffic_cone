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

use std::{io::Read, process::exit};

use derive_getters::Getters;
use log::{error, warn};
use serde::{Deserialize, Serialize};

use crate::{HTTP_CLIENT, default_headers};

pub(crate) type Id = String;
pub(crate) type FileName = String;
pub(crate) type MimeType = String;
pub(crate) type FileSize = u64;
pub(crate) type Link = String;
pub(crate) type Host = String;
pub(crate) type Chunks = u64;
pub(crate) type DownloadLink = String;
pub(crate) type Generated = String;

pub(crate) type Downloads = Vec<Download>;

/// A download entry.
///
/// This contains the serialized information
/// from the download request.
#[derive(Serialize, Deserialize)]
#[derive(Getters)]
#[derive(Debug)]
pub struct Download {
    id: Id,
    filename: FileName,
    mimeType: MimeType,
    filesize: FileSize,
    link: Link,
    host: Host,
    chunks: Chunks,
    download: DownloadLink,
    generated: Generated,
}

const DOWNLOAD_URL: &str = "https://api.real-debrid.com/rest/1.0/downloads";
const DELETE_DOWNLOAD_URL: &str = "https://api.real-debrid.com/rest/1.0/downloads/delete/";

/// Get the downloads in json form.
pub fn get_downloads_json() -> String {
    let mut body = String::new();

    default_headers(HTTP_CLIENT.get(DOWNLOAD_URL))
        .send()
        .unwrap()
        .read_to_string(&mut body)
        .unwrap();

    body
}

/// Get a deserialized list of the Downloads.
///
/// See `get_downloads_json()`
pub fn get_downloads() -> Downloads {
    serde_json::from_str(&get_downloads_json()).unwrap()
}

/// Delete a specific download by its id.
pub fn delete_download(id: Id) -> String {
    let mut body = String::new();

    let mut response =
        match default_headers(HTTP_CLIENT.delete(format!("{DELETE_DOWNLOAD_URL}{id}"))).send() {
            Ok(response) => response,
            Err(err) => {
                warn!("http_response : {err:?}");
                return String::new();
            }
        };

    if response.status().is_success() {
        response
            .read_to_string(&mut body)
            .inspect_err(|_| {
                warn!(
                    "{} : {}",
                    "utf8 error", "delete download reponse was not utf8 encoded; skipping"
                )
            })
            .unwrap_or(0);
    } else {
        warn!("bad response : {}", response.status())
    }

    response
        .read_to_string(&mut body)
        .inspect_err(|_| {
            error!(
                "{} : {}",
                "utf8 error", "delete download request was not utf8 encoded"
            );
            exit(1)
        })
        .unwrap();

    body
}

/// Downloads from the link.
pub fn download(link: Link) -> String {
    let mut body = Vec::new();

    let mut response = match default_headers(HTTP_CLIENT.get(format!("{link}"))).send() {
        Ok(response) => response,
        Err(err) => {
            warn!("http_response : {err:?}");
            return String::new();
        }
    };

    if response.status().is_success() {
        response
            .read_to_end(&mut body)
            .inspect_err(|_| {
                warn!(
                    "{} : {}",
                    "read error", "download reponse could not be read successfully; skipping"
                )
            })
            .unwrap_or(0);
    } else {
        warn!("bad response : {}", response.status())
    }

    String::from_utf8(body.clone()).unwrap()
}
