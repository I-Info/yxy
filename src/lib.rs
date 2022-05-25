pub mod arg;
pub mod conf;
pub mod error;
pub mod req;
pub mod utils;

/// Entrance for bin application
pub fn query_ele(
    uid: &str,
    cookie_file: &Option<String>,
    verbose: bool,
) -> Result<req::app::ElectricityInfo, error::Error> {
    // Read the session cache
    let mut session = match cookie_file {
        None => None,
        Some(cookie_file) => match std::fs::read_to_string(cookie_file) {
            Ok(v) => {
                if verbose {
                    println!("Using cached session id: {}", v);
                }
                Some(v)
            }
            Err(e) => {
                if verbose {
                    eprintln!("Session cache file reading error: {}", e);
                }
                return Err(error::Error::IO(e));
            }
        },
    };

    let mut tried = false;
    loop {
        if session.is_none() {
            let (ses, _) = start_auth(uid, &cookie_file, verbose)?;
            session.replace(ses);
        }
        match start_app(session.as_ref().unwrap(), verbose) {
            Err(e) => {
                // Handle errors
                match e {
                    error::Error::AuthExpired => {
                        if tried {
                            return Err(error::Error::Auth(
                                "Maximum auth retry number reached.".into(),
                            ));
                        }
                        session.take();
                        if verbose {
                            eprintln!("Auth may expired, trying to reauthorize.")
                        }
                    }
                    _ => return Err(e),
                }
                tried = true;
            }
            Ok(v) => {
                return Ok(v);
            }
        }
    }
}

/// Authorization procedure
fn start_auth(
    id: &str,
    path: &Option<String>,
    verbose: bool,
) -> Result<(String, req::auth::UserInfo), error::Error> {
    let client = req::init_default_client()?;

    if verbose {
        println!("Trying to get oauth code...");
        let oauth_code = req::auth::get_oauth_code(&client, id)?;
        println!("OAuth Code: {}", oauth_code);

        println!("Trying to login...");
        let (ses, user) = req::auth::authorize(&client, &oauth_code)?;
        println!("Authorized, the session id is: {}", ses);
        // Cache the session
        if let Some(cookie_file) = path {
            if let Err(e) = utils::file_write(cookie_file, &ses) {
                eprintln!("Fail to cache the session id: {}", e);
            } else {
                println!("Session cached.")
            }
        }

        Ok((ses, user))
    } else {
        let oauth_code = req::auth::get_oauth_code(&client, id)?;

        let (ses, user) = req::auth::authorize(&client, &oauth_code)?;
        // Cache the session
        if let Some(cookie_file) = path {
            utils::file_write(cookie_file, &ses)?;
        }

        Ok((ses, user))
    }
}

/// App procedure
fn start_app(session: &str, verbose: bool) -> Result<req::app::ElectricityInfo, error::Error> {
    // Init authorized handler
    let handler = req::Handler::new(session)?;

    // Query Bind Info
    if verbose {
        println!("Querying bind info...");
        let bind_info = handler.query_bind()?;
        println!("Bind info: {:?}", bind_info);

        // Query Electricity Info
        println!("Query electricity info...");
        let room_info = req::app::RoomInfo {
            area_id: &bind_info.area_id,
            building_code: &bind_info.building_code,
            floor_code: &bind_info.floor_code,
            room_code: &bind_info.room_code,
        };
        let electricity_info = handler.query_electricity(room_info)?;
        println!("Electricity info: {:?}", electricity_info);

        Ok(electricity_info)
    } else {
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
}
