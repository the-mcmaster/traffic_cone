#![feature(never_type)]
#![allow(non_snake_case)]

//! # traffic_cone API caller

use crate::prelude::*;
use reqwest::blocking::{
    Client as ReqwestClient, RequestBuilder as ReqwestBuilder, Response as ReqwestResponse,
};

pub use crate::app::ARGS;

pub(crate) type Json = String;
pub(crate) type Url = String;

type Body = String;

pub mod app;
pub mod download;
pub mod hosts;
pub mod torrents;
pub(crate) mod prelude {
    pub(crate) use crate::{HttpRequest::*, Json, send};
    pub(crate) use log::{error, warn};
    pub(crate) use std::{fs::File, io::Read, process::exit, sync::LazyLock};
}

/// Memoization of the API Key
static API_KEY: LazyLock<String> = LazyLock::new(|| {
    let mut file = File::open(ARGS.api_key_path())
        .inspect_err(|e| {
            error!("api key : could not locate API KEY : {e}");
            exit(1)
        })
        .unwrap();

    let mut api_key = String::new();

    file.read_to_string(&mut api_key)
        .inspect_err(|e| {
            error!("api key : failed while reading contents of API KEY : {e}");
            exit(1)
        })
        .unwrap();

    api_key
        .lines()
        .next()
        .unwrap_or_default()
        .to_string()
});

/// Memoization of the reqwest `Client`.
static HTTP_CLIENT: LazyLock<ReqwestClient> = LazyLock::new(|| ReqwestClient::new());

#[allow(dead_code)]
pub(crate) enum HttpRequest<T: Into<Body>> {
    Get(T),
    Post(T),
    Delete(T),
    Put(T),
}
impl<T> HttpRequest<T>
where
    T: Into<String> + Clone,
{
    pub(crate) fn body(&self) -> Body {
        match self {
            Get(body) | Post(body) | Delete(body) | Put(body) => body.clone().into(),
        }
    }

    pub(crate) fn send_to(self, url: impl Into<Url>) -> Option<ReqwestResponse> {
        let report_err = |response: Result<ReqwestResponse, reqwest::Error>| -> Result<ReqwestResponse, reqwest::Error> {
            #[cfg(debug_assertions)]
            return response
                .inspect_err(|e| warn!("http_response : {e}"));

            #[cfg(not(debug_assertions))]
            return response
                .inspect_err(|e| {
                    error!("http_response : {e}");
                });
        };

        let debug_response = |response: Result<ReqwestResponse, reqwest::Error>| -> Result<ReqwestResponse, reqwest::Error> {
            #[cfg(debug_assertions)]
            return response.inspect(|respons| {
                eprintln!("STATUS CODE: {}", respons.status());
            });
            #[cfg(not(debug_assertions))]
            return response;
        };

        let request = match self {
            Get(_) => HTTP_CLIENT.get(url.into()),
            Post(_) => HTTP_CLIENT.post(url.into()),
            Delete(_) => HTTP_CLIENT.delete(url.into()),
            Put(_) => HTTP_CLIENT.put(url.into()),
        };

        eprintln!("DEBUG {:#?}", request);
        let response = default_headers(request).body(self.body()).send();

        eprintln!("DEBUG {:#?}", response);

        debug_response(report_err(response)).ok()
    }
}

fn send<B: Into<Body> + Clone, Link: Into<Url>>(request: HttpRequest<B>, to: Link) -> String {
    let report_read_error = |response: std::io::Result<usize>| -> usize {
        response.inspect_err(|e| warn!("io read: {e}")).unwrap_or(0)
    };

    let mut response_json = String::new();
    if let Some(mut response) = request.send_to(to) {
        report_read_error(response.read_to_string(&mut response_json));
    }

    response_json
}

/// Extends the request with default header information.
fn default_headers(request: ReqwestBuilder) -> ReqwestBuilder {
    request.header("Authorization", format!("Bearer {}", API_KEY.as_str()))
}
