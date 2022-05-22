//! Requests
use std::{error::Error, time::Duration};

use reqwest::{blocking::Response, header};
use serde::{Deserialize, Serialize};

pub mod app;
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

/// User info provided by platform
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub id: String,
    pub school_code: String,
    pub school_name: String,
    pub user_name: String,
    pub user_type: String,
    pub mobile_phone: String,
    pub job_no: String,
    pub user_idcard: String,
    pub sex: u8,
    pub user_class: String,
    pub bind_card_status: u8,
    pub test_account: u8,
    pub platform: String,
    pub third_openid: String,
}

/// Session handle
#[derive(Debug)]
pub struct Handler {
    pub session: Option<String>,
    pub client: reqwest::blocking::Client,
}

impl Handler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            session: None,
            client: init_default_client()?,
        })
    }

    fn check_response(res: &Response) -> Result<(), crate::error::Error> {
        if !res.status().is_success() {
            return Err(crate::error::Error {
                code: 1,
                msg: format!("remote server returned {} status", res.status()),
            });
        }

        Ok(())
    }
}
