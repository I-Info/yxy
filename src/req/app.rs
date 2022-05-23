//! Application APIs

use std::error::Error;

use serde::{Deserialize, Serialize};

use super::{check_response, url, Handler};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryBindResponse {
    pub status_code: u32,
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

impl Handler {
    pub fn query_bind(&self) -> Result<Vec<BindInfo>, Box<dyn Error>> {
        let form = vec![("bindType", "3")];
        let resp = self
            .client
            .post(url::application::QUERY_BIND)
            .form(&form)
            .send()?;
        check_response(&resp)?;
        let resp_ser: QueryBindResponse = resp.json()?;
        if resp_ser.success == false {
            return Err(Box::new(crate::error::Error {
                code: 6,
                msg: format!("Fail to query bind: {}", resp_ser.message.unwrap()),
            }));
        }

        if let Some(bind_info) = resp_ser.rows {
            Ok(bind_info)
        } else {
            Err(Box::new(crate::error::Error {
                code: 2,
                msg: "Empty response".into(),
            }))
        }
    }

    pub fn query_electric(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
