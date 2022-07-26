//! Simulate app login requests
use std::collections::HashMap;
use std::io::Read;

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

use super::{check_response, url, APP_VER_NAME};
use crate::error::Error;
use crate::utils::{md5, pkcs7_padding};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicResponse<T> {
    pub status_code: i32,
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityTokenResponse {
    pub level: u8,
    pub security_token: String,
}

/// Login response data definition
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub id: String,    // UID
    pub token: String, // App login token
    pub account: String,
    pub account_encrypt: String,
    pub mobile_phone: String,
    pub sex: Option<i8>, // 1 as male, 0 as female
    pub school_code: Option<String>,
    pub school_name: Option<String>,
    pub qrcode_pay_type: Option<u8>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub job_no: Option<String>,
    pub user_idcard: Option<String>,
    pub identity_no: Option<String>,
    pub user_class: Option<String>,
    pub real_name_status: i32,
    pub regiser_time: String, // typo: register time
    pub bind_card_status: i32,
    pub last_login: String,
    pub head_img: String,
    pub device_id: String,
    pub test_account: i32,
    pub join_newactivity_status: i32,
    pub is_new: i8,
    pub create_status: i32,
    pub eacct_status: i32,
    pub school_classes: Option<i32>,
    pub school_nature: Option<i32>,
    pub platform: String,
    pub uu_token: String,
    pub qrcode_private_key: String, // Private key
    pub bind_card_rate: Option<i32>,
    pub points: Option<i32>,
    pub school_identity_type: Option<i32>,
    pub alumni_flag: Option<i32>,
    pub ext_json: Option<String>,
}

pub struct LoginHandler {
    pub phone_num: String,
    pub device_id: String,
    client: Client,
}

mod error_messages {
    pub const WRONG_VERIFY_CODE: &'static str = "您已输错";
    pub const BAD_PHONE_NUM: &'static str = "请输入正确的手机号";
    pub const BAD_PHONE_NUM_FORMAT: &'static str = "手机号码格式错误";
    pub const TOO_FREQUENT: &'static str = "经过你的";
    pub const TOO_MANY_TRIES: &'static str = "发送超限，请明天再来";
    pub const FLOW_CONTROL: &'static str = "触发号码天级流控";
}

impl LoginHandler {
    pub fn new(phone_num: String) -> Result<Self, Error> {
        let device_id = gen_device_id();

        Ok(Self {
            phone_num,
            client: init_app_sim_client(&device_id)?,
            device_id,
        })
    }

    pub fn init(phone_num: &str, device_id: &str) -> Result<Self, Error> {
        Ok(Self {
            phone_num: phone_num.to_string(),
            device_id: device_id.to_string(),
            client: init_app_sim_client(&device_id)?,
        })
    }

    /// Init general request body
    pub fn get_basic_request_body(&self) -> HashMap<&str, serde_json::Value> {
        let mut result = HashMap::new();
        result.insert("appVersion", json!(super::APP_VER));
        result.insert("deviceId", json!(self.device_id));
        result.insert("platform", json!("YUNMA_APP"));
        result.insert("testAccount", json!(1u8));

        result
    }

    /// Return security token & level
    pub fn get_security_token(&self) -> Result<SecurityTokenResponse, Error> {
        let mut body = self.get_basic_request_body();
        body.insert("sceneCode", json!("app_user_login"));
        let mut resp = self
            .client
            .post(url::app::GET_SECURITY_TOKEN)
            .json(&body)
            .send()?;
        check_response(&mut resp)?;

        let resp_ser: BasicResponse<SecurityTokenResponse> = resp.json()?;
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

    /// Get image captcha
    /// ------------
    /// Return image captcha base64 string
    pub fn get_captcha_image(&self, security_token: &str) -> Result<String, Error> {
        let mut body = self.get_basic_request_body();
        body.insert("securityToken", json!(security_token));

        let mut resp = self
            .client
            .post(url::app::GET_IMAGE_CAPTCHA)
            .json(&body)
            .send()?;
        check_response(&mut resp)?;

        let resp_ser: BasicResponse<String> = resp.json()?;
        if resp_ser.success == false {
            Err(Error::Runtime(format!(
                "Get image captcha failed: {}",
                resp_ser.message
            )))
        } else {
            Ok(resp_ser.data.unwrap())
        }
    }

    /// Request to send login verification code sms
    pub fn send_verification_code(
        &self,
        security_token: &str,
        captcha: Option<&str>,
    ) -> Result<bool, Error> {
        let mut body = self.get_basic_request_body();
        let app_security_token = get_app_security_token(security_token, &self.device_id)?;
        body.insert("appSecurityToken", json!(app_security_token));
        body.insert("securityToken", json!(security_token));
        body.insert("sendCount", json!(1u8));
        body.insert("mobilePhone", json!(self.phone_num));

        // If image captcha required
        if let Some(v) = captcha {
            body.insert("imageCafptchaValue", json!(v));
        }

        let mut resp = self
            .client
            .post(url::app::SEND_VERIFICATION_CODE)
            .json(&body)
            .send()?;
        check_response(&mut resp)?;

        /// Define data object
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data {
            user_exists: bool,
        }

        let resp_ser: BasicResponse<Data> = resp.json()?;
        if resp_ser.success == false {
            if resp_ser.status_code == 203 {
                if resp_ser.message == error_messages::BAD_PHONE_NUM
                    || resp_ser.message == error_messages::BAD_PHONE_NUM_FORMAT
                {
                    return Err(Error::BadPhoneNumber);
                }
                if resp_ser.message.starts_with(error_messages::TOO_FREQUENT)
                    || resp_ser.message == error_messages::FLOW_CONTROL
                    || resp_ser.message == error_messages::TOO_MANY_TRIES
                {
                    return Err(Error::VerificationLimit);
                }
            }

            return Err(Error::Runtime(format!(
                "Send verification code error: {{code: {}, message: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }

        // User status
        let user_exists = resp_ser.data.unwrap().user_exists;

        Ok(user_exists)
    }

    /// Do login with verification code
    pub fn do_login(&self, code: &str) -> Result<LoginResponse, Error> {
        let mut body = self.get_basic_request_body();
        body.insert("appPlatform", json!("Android"));
        body.insert("clientId", json!("65l01gpo3p8v6rk"));
        body.insert("mobilePhone", json!(self.phone_num));
        body.insert("oaid", json!(""));
        body.insert("osType", json!("Android"));
        body.insert("osUuid", json!(self.device_id));
        body.insert("osVersion", json!(11u8));
        body.insert("verificationCode", json!(code));

        let mut resp = self
            .client
            .post(url::app::DO_LOGIN_BY_CODE)
            .json(&body)
            .send()?;
        check_response(&mut resp)?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;
        // println!("resp: {}", buf);

        let resp_ser: BasicResponse<LoginResponse> = match serde_json::from_str(&buf) {
            Ok(v) => v,
            Err(e) => {
                return Err(Error::Runtime(format!(
                    "Parsing login response failed: {}\nData: {}",
                    e, buf
                )))
            }
        };

        if resp_ser.success == false {
            if resp_ser
                .message
                .starts_with(error_messages::WRONG_VERIFY_CODE)
            {
                return Err(Error::BadVerificationCode);
            }

            return Err(Error::Runtime(format!(
                "Login error: {{code: {}, msg: {}}}",
                resp_ser.status_code, resp_ser.message
            )));
        }
        let result = resp_ser.data.unwrap();

        Ok(result)
    }
}

/// Random device id generator
pub fn gen_device_id() -> String {
    let mut uuid = uuid::Uuid::new_v4().to_string();
    uuid.retain(|c| c != '-');
    uuid.insert_str(0, "yunma");

    uuid
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

/// Encrypt device id
pub fn get_app_security_token(security_token: &str, device_id: &str) -> Result<String, Error> {
    let key = GenericArray::clone_from_slice(security_token[..16].as_bytes());
    let cipher = Aes128::new(&key);

    let text = base64::decode(security_token[32..].as_bytes())?;

    let mut blocks = Vec::new();
    (0..text.len()).step_by(16).for_each(|x| {
        blocks.push(GenericArray::clone_from_slice(text[x..x + 16].as_ref()));
    });

    cipher.decrypt_blocks(&mut blocks);

    let t: Vec<u8> = blocks.iter().flatten().map(|&x| x as u8).collect();

    let last = t.last().unwrap().clone();
    let index: usize = t.len() - usize::from(last);
    let t_final: String = t[..index].iter().map(|&x| x as char).collect();

    let time_stamp = chrono::prelude::Local::now().timestamp();

    let stage_1 = md5(format!(
        "{}|YUNMA_APP|{}|{}|{}",
        device_id, t_final, time_stamp, APP_VER_NAME
    ))
    .to_uppercase();

    let stage_2 = md5(stage_1).to_uppercase();

    let stage_3 = format!(
        "{}|YUNMA_APP|{}|{}|{}|{}",
        device_id, t_final, time_stamp, APP_VER_NAME, stage_2
    );

    let padded_text = pkcs7_padding(&stage_3, 16);

    let mut blocks_2 = Vec::new();
    (0..padded_text.len()).step_by(16).for_each(|x| {
        blocks_2.push(GenericArray::clone_from_slice(
            padded_text[x..x + 16].as_ref(),
        ));
    });

    cipher.encrypt_blocks(&mut blocks_2);

    let encrypted_text: Vec<u8> = blocks_2.iter().flatten().map(|&x| x as u8).collect();

    let stage_4 = base64::encode(encrypted_text);

    Ok(stage_4)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;

    #[test]
    fn app_security_token() -> Result<(), Error> {
        let result = get_app_security_token(
            "ce295733862b93cb376efef661c21b4dEW6CpH8wFHp/RvViKZiJ8A==",
            "12345678",
        )?;
        assert_eq!("RxTdUD90Eg91tGZHyhTKwjX9v3fH8WWGgQ3vQ5CuiC", &result[..42]);

        Ok(())
    }
}
