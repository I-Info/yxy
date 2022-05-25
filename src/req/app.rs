//! Application APIs
use serde::{Deserialize, Serialize};

use super::{check_response, url, Handler};
use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBindResponse {
    pub status_code: i32,
    pub success: bool,
    pub total: Option<u32>,
    pub message: Option<String>,
    pub rows: Option<Vec<BindInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BindInfo {
    pub id: String,
    pub school_code: String,
    pub school_name: String,
    pub job_no: String,
    pub user_name: String,
    pub bind_type_str: String,
    pub area_id: String,
    pub area_name: String,
    pub building_code: String,
    pub building_name: String,
    pub floor_code: String,
    pub floor_name: String,
    pub room_code: String,
    pub room_name: String,
    pub create_time: String,
    pub is_allow_change: u8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInfo<'a> {
    pub area_id: &'a str,
    pub building_code: &'a str,
    pub floor_code: &'a str,
    pub room_code: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryElResponse {
    pub status_code: i32,
    pub success: bool,
    pub message: String,
    pub data: Option<ElectricityInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElectricityInfo {
    pub school_code: String,
    pub area_id: String,
    pub building_code: String,
    pub floor_code: String,
    pub room_code: String,
    pub display_room_name: String,
    // pub remind: String, // unknown usage, removed
    pub soc: f32,
    pub total_soc_amount: f32,
    pub is_allow_change: u8,
    pub show_type: u8,
    pub record_show: u8,
    pub style: u8,
    pub surplus_list: Vec<ElSurplus>,
    pub top_up_type_list: Vec<ElTopUpType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElSurplus {
    pub surplus: f32,
    pub amount: f32,
    pub subsidy: f32,
    pub subsidy_amount: f32,
    pub total_surplus: f32,
    pub mdtype: String,
    pub mdname: String,
    pub room_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElTopUpType {
    pub mdname: String,
    pub cztype: String,
}

impl Handler {
    /// Query Bind infos
    ///
    /// Only return one bind info from list
    pub fn query_bind(&self) -> Result<BindInfo, Error> {
        let form = vec![("bindType", "3")];
        let resp = self
            .client
            .post(url::application::QUERY_BIND)
            .form(&form)
            .send()?;
        check_response(&resp)?;
        let resp_ser: QueryBindResponse = resp.json()?;
        if resp_ser.success == false {
            if resp_ser.status_code == 204 {
                return Err(Error::AuthExpired);
            }
            return Err(Error::Runtime(format!(
                "Fail to query bind: {}",
                resp_ser.message.unwrap()
            )));
        }

        // Take data
        if let Some(mut bind_info) = resp_ser.rows {
            Ok(match bind_info.pop() {
                Some(v) => v,
                None => return Err(Error::EmptyResp),
            })
        } else {
            Err(Error::EmptyResp)
        }
    }

    pub fn query_electricity(&self, info: RoomInfo) -> Result<ElectricityInfo, Error> {
        let resp = self
            .client
            .post(url::application::QUERY_ELECTRICITY)
            .json(&info)
            .send()?;
        check_response(&resp)?;
        let resp_ser: QueryElResponse = resp.json()?;

        if resp_ser.success == false {
            if resp_ser.status_code == 204 {
                return Err(Error::AuthExpired);
            }
            return Err(Error::Runtime(format!(
                "Fail to query electricity: {}",
                resp_ser.message
            )));
        }

        if let Some(v) = resp_ser.data {
            Ok(v)
        } else {
            Err(Error::EmptyResp)
        }
    }
}
