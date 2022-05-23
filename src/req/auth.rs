//! Authorization APIs
use std::{collections::HashMap, error::Error};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::{check_response, url, UserInfo};

/// A constant value
const APPID: &'static str = "1810181825222034";
pub const SESSION_KEY: &'static str = "shiroJID";

/// Authorize API response definition
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthResponse {
    status_code: u32,
    message: String,
    success: bool,
    data: Option<UserInfo>,
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
        Some(i) => Some(text[i + 12..i + 12 + 32].into()),
        None => None,
    }
}

pub fn get_oauth_code(client: &Client, id: &str) -> Result<String, Box<dyn Error>> {
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
        None => Err(Box::new(crate::error::Error {
            code: 2,
            msg: "no code find in response".into(),
        })),
    }
}

/// Authorize the handler and fetch user infos
pub fn authorize(client: &Client, code: &str) -> Result<(String, UserInfo), Box<dyn Error>> {
    // Form data
    let mut params = HashMap::new();
    params.insert("code", code);

    let response = client
        .post(url::application::GET_USER_FOR_AUTHORIZE)
        .form(&params)
        .send()?;
    check_response(&response)?;

    // get session
    let session = match response.cookies().find(|x| x.name() == SESSION_KEY) {
        Some(v) => (v.value().to_string()),
        None => {
            return Err(Box::new(crate::error::Error {
                code: 3,
                msg: "Authorize failed: no cookie returned".into(),
            }))
        }
    };

    let resp_ser: AuthResponse = response.json()?;
    // Set user info
    if let Some(v) = resp_ser.data {
        Ok((session, v))
    } else {
        return Err(Box::new(crate::error::Error {
            code: 4,
            msg: format!("Authorize failed: {}", resp_ser.message),
        }));
    }
}
