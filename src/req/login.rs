//! Simulate app login requests
use std::collections::HashMap;

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
pub struct SecurityToken {
    pub level: u8,
    pub security_token: String,
}

/// Login response data definition
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub id: String, // UID
    pub token: String,
    pub account: String,
    pub account_encrypt: String,
    pub mobile_phone: String,
    pub sex: u8, // 1 as male, 0 as female
    pub school_code: Option<String>,
    pub school_name: Option<String>,
    pub qrcode_pay_type: Option<u8>,
    pub user_name: Option<String>,
    pub user_type: Option<String>,
    pub job_no: Option<String>,
    pub user_idcard: Option<String>,
    pub identity_no: Option<String>,
    pub user_class: Option<String>,
    pub real_name_status: u8,
    pub regiser_time: String, // typo: register time
    pub bind_card_status: u8,
    pub last_login: String,
    pub head_img: String,
    pub device_id: String,
    pub test_account: u8,
    pub join_newactivity_status: u8,
    pub is_new: u8,
    pub create_status: u8,
    pub eacct_status: u8,
    pub school_classes: Option<i32>,
    pub school_nature: Option<i32>,
    pub platform: String,
    pub uu_token: String,
    pub qrcode_private_key: String, // Private key
    pub bind_card_rate: u8,
    pub points: i32,
    pub school_identity_type: Option<u8>,
    pub alumni_flag: Option<u8>,
    pub ext_json: Option<String>,
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
    pub fn get_basic_request_body(&self) -> HashMap<&str, serde_json::Value> {
        let mut result = HashMap::new();
        result.insert("appVersion", json!(super::APP_VER));
        result.insert("deviceId", json!(self.device_id));
        result.insert("platform", json!("YUNMA_APP"));
        result.insert("testAccount", json!(1u8));

        result
    }

    /// Return security token & level
    pub fn get_security_token(&self) -> Result<SecurityToken, Error> {
        let mut body = self.get_basic_request_body();
        body.insert("sceneCode", json!("app_user_login"));
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

        let resp = self
            .client
            .post(url::app::SEND_VERIFICATION_CODE)
            .json(&body)
            .send()?;
        check_response(&resp)?;

        /// Define data object
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data {
            user_exists: bool,
        }

        let resp_ser: BasicResponse<Data> = resp.json()?;
        if resp_ser.success == false {
            return Err(Error::Runtime(format!(
                "Send verification code error: {}",
                resp_ser.message
            )));
        }

        // User status
        let user_exists = resp_ser.data.unwrap().user_exists;

        Ok(user_exists)
    }

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

        let resp = self
            .client
            .post(url::app::DO_LOGIN_BY_CODE)
            .json(&body)
            .send()?;
        check_response(&resp)?;

        let resp_ser: BasicResponse<LoginResponse> = resp.json()?;
        if resp_ser.success == false {
            return Err(Error::Runtime(format!("Login error: {}", resp_ser.message)));
        }
        let result = resp_ser.data.unwrap();

        Ok(result)
    }
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
        println!("Ok final: {}", result);
        assert_eq!("RxTdUD90Eg91tGZHyhTKwjX9v3fH8WWGgQ3vQ5CuiC", &result[..42]);

        Ok(())
    }
}
