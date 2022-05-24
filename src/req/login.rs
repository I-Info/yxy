/// Simulate app login requests
use std::collections::HashMap;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::{check_response, url};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicResponse<T> {
    pub status_code: u32,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityToken {
    pub level: u8,
    pub security_token: String,
}

pub struct LoginHandler {
    pub phone_num: String,
    pub device_id: String,
    client: Client,
}

impl LoginHandler {
    pub fn new(phone_num: String) -> Result<Self, Error> {
        let device_id = Self::gen_device_id();

        Ok(Self {
            phone_num,
            client: init_app_sim_client(&device_id)?,
            device_id,
        })
    }

    /// Random device id generator
    pub fn gen_device_id() -> String {
        let mut uuid = uuid::Uuid::new_v4().to_string();
        uuid.retain(|c| c != '-');
        uuid.insert_str(0, "yunma");

        uuid
    }

    /// Init general request body
    pub fn get_basic_request_body(&self) -> HashMap<&str, &str> {
        let mut result = HashMap::new();
        result.insert("appVersion", super::APP_VER);
        result.insert("deviceId", &self.device_id);
        result.insert("platform", "YUNMA_APP");
        result.insert("testAccount", "1");

        result
    }

    /// Return security token & level
    pub fn get_security_token(&self) -> Result<SecurityToken, Error> {
        let mut body = self.get_basic_request_body();
        body.insert("sceneCode", "app_user_login");
        let resp = self
            .client
            .post(url::app::GET_SECURITY_TOKEN)
            .json(&body)
            .send()?;
        check_response(&resp)?;

        let resp_ser: BasicResponse<SecurityToken> = resp.json()?;
        if resp_ser.success == false {
            return Err(Error::Runtime(format!(
                "Get security token failed: {}",
                resp_ser.message
            )));
        }

        match resp_ser.data {
            Some(v) => Ok(v),
            None => Err(Error::EmptyResp),
        }
    }

    /// Request to send login verification code sms
    pub fn send_verification_code(&self) {}
}

/// Init App simulated client
pub fn init_app_sim_client(device_id: &str) -> Result<reqwest::blocking::Client, Error> {
    let builder: reqwest::blocking::ClientBuilder = reqwest::blocking::Client::builder();
    let mut headers = super::get_default_headers();
    headers.insert(
        "Domain-Name",
        reqwest::header::HeaderValue::from_static("campus"),
    );
    let result: reqwest::blocking::Client = builder
        .connect_timeout(std::time::Duration::new(5, 0))
        .user_agent(format!("{}{}", super::USER_AGENT, device_id))
        .default_headers(headers)
        .build()?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;
    #[test]
    fn device_id() {
        println!("{}", LoginHandler::gen_device_id())
    }

    #[test]
    fn login() -> Result<(), Error> {
        let handler = LoginHandler::new("1234567".into())?;
        println!("{:?}", handler.get_security_token()?);

        Ok(())
    }
}
