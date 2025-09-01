#![feature(never_type)]
#![feature(stmt_expr_attributes)]


//! # traffic_cone API caller

use std::collections::HashMap;

use crate::prelude::*;
use reqwest::blocking::{
    Client as ReqwestClient, RequestBuilder as ReqwestBuilder, Response as ReqwestResponse,
};

pub use crate::app::ARGS;

pub(crate) type Json = String;
pub(crate) type Url = String;

type Body = String;

pub mod app;
pub mod handle;

pub mod downloads;
pub mod hosts;
pub mod settings;
pub mod streaming;
pub mod torrents;
pub mod traffic;
pub mod unrestrict;
pub mod user;
pub(crate) mod prelude {
    pub(crate) use crate::{HttpRequest::*, Json, send};
    pub(crate) use std::{fs::File, io::Read, process::exit, sync::LazyLock};
}

#[macro_export]
macro_rules! debug {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        if !*crate::ARGS.quiet() {
            let level = "DEBUG";
            if *crate::ARGS.no_color() {
                eprintln!("{level}:   {}", format!($($tt)*))
            } else {
                eprintln!("\x1b[38;5;12m{level}\x1b[0m:   {}", format!($($tt)*));
            }
        }
    };
}

#[macro_export]
macro_rules! warn {
    ($($tt:tt)*) => {
        if !*crate::ARGS.quiet() {
            let level = "WARNING";
            if *crate::ARGS.no_color() {
                eprintln!("{level}: {}", format!($($tt)*))
            } else {
                eprintln!("\x1b[38;5;11m{level}\x1b[0m: {}", format!($($tt)*))
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($tt:tt)*) => {
        if !*crate::ARGS.quiet() {
            let level = "ERROR";
            if *crate::ARGS.no_color() {
                eprintln!("{level}:   {}", format!($($tt)*))
            } else {
                eprintln!("\x1b[38;5;9m{level}\x1b[0m:   {}", format!($($tt)*))
            }
        }
    };
}

/// Memoization of the API Key
static API_KEY: LazyLock<String> = LazyLock::new(|| {
    let mut file = File::open(ARGS.api_key_path())
        .inspect_err(|e| {
            error!(
                "api key : could not locate API KEY `{}` : {e}",
                ARGS.api_key_path()
            );
            exit(1)
        })
        .unwrap();

    let mut api_key = String::new();

    file.read_to_string(&mut api_key)
        .inspect_err(|e| {
            error!(
                "api key : failed while reading contents of API KEY `{}` : {e}",
                ARGS.api_key_path()
            );
            exit(1)
        })
        .unwrap();

    api_key.lines().next().unwrap_or_default().to_string()
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

    pub(crate) fn send_to<'a>(self, url: impl Into<Url>) -> Option<ReqwestResponse> {
        let report_err = |response: Result<ReqwestResponse, reqwest::Error>| -> Result<ReqwestResponse, reqwest::Error> {
            return response
                .inspect_err(|e| error!("http_response : {e}"));
        };

        let debug_response = |response: Result<ReqwestResponse, reqwest::Error>| -> Result<ReqwestResponse, reqwest::Error> {
            #[allow(unused_variables)]
            response.inspect(|response| debug!("STATUS CODE: {}", response.status()))
        };

        let body = self.body();
        let request = default_headers(match self {
            Get(_) => HTTP_CLIENT.get(url.into()).body(self.body()),
            Post(_) => HTTP_CLIENT.post(url.into()).form(
                &serde_json::de::from_str::<HashMap<&str, &str>>(&body)
                    .inspect_err(|e| error!("derserialization : {e}"))
                    .unwrap_or_default(),
            ),
            Delete(_) => HTTP_CLIENT.delete(url.into()).body(self.body()),
            Put(_) => HTTP_CLIENT.put(url.into()).body(self.body()),
        });

        debug!("{request:?}");

        let response = request.send();

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
    request
        .header("Authorization", format!("Bearer {}", API_KEY.as_str()))
        .header("Content-Type", "application/x-www-form-urlencoded")
}
