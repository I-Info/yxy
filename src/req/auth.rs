//! Authorization APIs
use std::collections::HashMap;

use reqwest::{blocking::Client, cookie::Cookie};
use serde::Deserialize;

use super::{check_response, url};
use crate::error::Error;

/// A constant value
const APPID: &'static str = "1810181825222034";
pub const SESSION_KEY: &'static str = "shiroJID";

/// Authorize API response definition
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    // status_code: i32,
    message: String,
    success: bool,
    data: Option<UserInfo>,
}

/// User info provided by platform
#[derive(Debug, Deserialize)]
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

/// Extract code from HTML text
///
/// # Example match pattern
/// ``` js
/// var code = "d53b7c8c6d4cd6f0facf04ef3d776f64";
/// callbackUrl = "https://xxx.com";
/// ```
/// If matched, return the code value
fn extract_code(text: &str) -> Option<String> {
    match text.find("var code = ") {
        Some(i) => {
            let pat = &text[i + 12..i + 12 + 32];
            if pat.contains("\"") {
                None
            } else {
                Some(pat.into())
            }
        }
        None => None,
    }
}

pub fn get_oauth_code(client: &Client, id: &str) -> Result<String, Error> {
    let response = client
        .get(url::auth::OAUTH_URL)
        .query(&[
            ("bindSkip", "1"),
            ("authType", "2"),
            ("appid", APPID),
            ("callbackUrl", url::application::BASE_URL),
            ("unionid", id),
        ])
        .send()?;
    check_response(&response)?;

    let text = response.text()?;

    match extract_code(&text) {
        Some(t) => Ok(t),
        None => Err(Error::Runtime("No OAuth code response".into())),
    }
}

/// Authorize the handler and fetch user infos
pub fn authorize(client: &Client, code: &str) -> Result<(String, UserInfo), Error> {
    // Form data
    let mut params = HashMap::new();
    params.insert("code", code);

    let response = client
        .post(url::application::GET_USER_FOR_AUTHORIZE)
        .form(&params)
        .send()?;
    check_response(&response)?;

    let cookies: Vec<Cookie> = response.cookies().collect();

    // get session
    match cookies.iter().find(|x| x.name() == SESSION_KEY) {
        Some(v) => {
            let session = v.value().to_string();
            let resp_ser: AuthResponse = response.json()?;
            if resp_ser.success == false {
                return Err(Error::Runtime(format!(
                    "Authorize failed: {}",
                    resp_ser.message
                )));
            }
            match resp_ser.data {
                Some(v) => Ok((session, v)),
                None => Err(Error::Runtime(format!(
                    "Authorize failed: {}",
                    resp_ser.message
                ))),
            }
        }
        None => {
            let resp_ser: AuthResponse = response.json()?;
            Err(Error::Runtime(format!(
                "Authorize failed: {}",
                resp_ser.message
            )))
        }
    }
}
