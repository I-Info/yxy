//! Requests
use reqwest::header;
use std::{error::Error, time::Duration};

pub mod auth;
pub mod url;

const USER_AGENT: &'static str = "\
Mozilla/5.0 (Linux; Android 11; Android for arm64; wv) \
AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 \
Chrome/66.0.3359.158 Mobile Safari/537.36 \
ZJYXYwebviewbroswer ZJYXYAndroid tourCustomer/yunmaapp.NET/4.1.0/";

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
pub fn init_default_client() -> Result<reqwest::blocking::Client, Box<dyn Error>> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();
    let result: reqwest::blocking::Client = builder
        .connect_timeout(Duration::new(5, 0))
        .user_agent(USER_AGENT)
        .default_headers(get_default_headers())
        .build()?;

    Ok(result)
}
