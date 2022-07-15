//! YXY Spider Library

pub mod error;
mod ffi;
pub mod req;
pub mod utils;

/// Authorization
/// ------------
/// just input uid to get authorization :) .
/// This could be a vulnerability
///
/// returns a tuple of (Session Token, User Info)
pub fn auth(uid: &str) -> Result<(String, req::auth::UserInfo), error::Error> {
    let client = req::init_default_client()?;

    let oauth_code = req::auth::get_oauth_code(&client, uid)?;

    let (ses, user) = req::auth::authorize(&client, &oauth_code)?;

    Ok((ses, user))
}

/// Query electricity
pub fn query_ele(session: &str) -> Result<req::app::ElectricityInfo, error::Error> {
    // Init authorized handler
    let handler = req::Handler::new(session)?;

    // Query Bind Info
    let bind_info = handler.query_bind()?;

    // Query Electricity Info
    let room_info = req::app::RoomInfo {
        area_id: &bind_info.area_id,
        building_code: &bind_info.building_code,
        floor_code: &bind_info.floor_code,
        room_code: &bind_info.room_code,
    };
    let electricity_info = handler.query_electricity(room_info)?;

    Ok(electricity_info)
}
