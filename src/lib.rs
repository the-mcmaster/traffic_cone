#![feature(never_type)]
#![allow(non_snake_case)]

//! # traffic_cone API caller

use crate::prelude::*;
use reqwest::blocking::{Client as ReqwestClient, RequestBuilder as ReqwestBuilder};

pub use crate::app::ARGS;

pub mod app;
pub mod download;
pub mod hosts;
pub(crate) mod prelude {
    pub(crate) use std::{fs::File, io::Read, process::exit, sync::LazyLock};
    pub(crate) use log::{warn, error};
    pub(crate) use crate::{HTTP_CLIENT, default_headers};
}

/// Memoization of the API Key
static API_KEY: LazyLock<String> = LazyLock::new(|| {
    let mut file = File::open(ARGS.api_key_path())
        .inspect_err(|e| {
            error!("api key : could not locate API KEY : {e:?}");
            exit(1)
        })
        .unwrap();

    let mut api_key = String::new();

    file.read_to_string(&mut api_key)
        .inspect_err(|e| {
            error!("api key : failed while reading contents of API KEY : {e:?}");
            exit(1)
        })
        .unwrap();

    api_key
});

/// Memoization of the reqwest `Client`.
static HTTP_CLIENT: LazyLock<ReqwestClient> = LazyLock::new(|| ReqwestClient::new());

/// Extends the request with default header information.
fn default_headers(request: ReqwestBuilder) -> ReqwestBuilder {
    request.header("Authorization", format!("Bearer {}", API_KEY.as_str()))
}
