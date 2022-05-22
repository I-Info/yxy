//! Authorization APIs
use std::error::Error;

use super::url;

/// A constant value
const APPID: &'static str = "1810181825222034";

pub fn get_oauth_code(
    client: &reqwest::blocking::Client,
    id: &str,
) -> Result<String, Box<dyn Error>> {
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
    if response.status() != 200 {
        return Err(Box::new(crate::error::Error {
            code: 1,
            msg: "remote server returned a error status".into(),
        }));
    }

    let text = response.text()?;

    match extract_code(&text) {
        Some(t) => Ok(t),
        None => Err(Box::new(crate::error::Error {
            code: 2,
            msg: "no code find in response".into(),
        })),
    }
}

/// Extract code from HTML text
///
/// # Example match pattern
/// ``` js
/// var code = "d53b7c8c6d4cd6f0facf04ef3d776f64";
/// callbackUrl = "https://application.xiaofubao.com";
/// ```
/// If matched, return the code value
fn extract_code(text: &str) -> Option<String> {
    match text.find("var code = ") {
        Some(i) => Some(text[i + 12..i + 12 + 32].into()),
        None => None,
    }
}
