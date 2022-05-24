//! Requests
use std::{sync::Arc, time::Duration};

use reqwest::{blocking::Response, cookie::Jar, header};

use crate::error::Error;

pub mod app;
pub mod auth;
pub mod login;
pub mod url;

const APP_VER: &'static str = "410";
const APP_VER_NAME: &'static str = "4.1.0";
const USER_AGENT: &'static str = const_format::formatcp!(
    "\
Mozilla/5.0 (Linux; Android 11; Android for arm64; wv) \
AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 \
Chrome/66.0.3359.158 Mobile Safari/537.36 \
ZJYXYwebviewbroswer ZJYXYAndroid tourCustomer/yunmaapp.NET/{}/",
    APP_VER_NAME
);

/// Define default headers.
fn get_default_headers() -> header::HeaderMap {
    let mut headers: header::HeaderMap = header::HeaderMap::new();
    {
        headers.insert(
            "X-Requested-With",
            header::HeaderValue::from_static("cn.com.yunma.school.app"),
        );
    }

    headers
}

/// Init default reqwest (blocking) client.
pub fn init_default_client() -> Result<reqwest::blocking::Client, Error> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();
    let result: reqwest::blocking::Client = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .default_headers(get_default_headers())
        .build()?;

    Ok(result)
}

/// Session handle
#[derive(Debug)]
pub struct Handler {
    client: reqwest::blocking::Client,
}

impl Handler {
    pub fn new(session: &str) -> Result<Self, Error> {
        Ok(Self {
            client: {
                let jar = Jar::default();
                jar.add_cookie_str(
                    &format!("{}={}", auth::SESSION_KEY, session),
                    &reqwest::Url::parse(url::application::BASE_URL).unwrap(),
                );
                reqwest::blocking::Client::builder()
                    .connect_timeout(Duration::new(5, 0))
                    .user_agent(USER_AGENT)
                    .default_headers(get_default_headers())
                    .cookie_provider(Arc::new(jar))
                    .build()?
            },
        })
    }
}

fn check_response(res: &Response) -> Result<(), Error> {
    if !res.status().is_success() {
        return Err(Error::Runtime(format!("bad response: {}", res.status())));
    }

    Ok(())
}
