pub mod arg;
pub mod conf;
pub(crate) mod error;
pub mod req;
pub mod utils;

/// Entrance for bin application
pub fn run(conf: conf::Config, opts: arg::Options) -> Result<(), error::Error> {
    // Read the session cache
    let mut session = match &conf.cookie_file {
        None => None,
        Some(cookie_file) => {
            if opts.fresh == false {
                match std::fs::read_to_string(cookie_file) {
                    Ok(v) => {
                        println!("Using cached session id: {}", v);
                        Some(v)
                    }
                    Err(e) => {
                        eprintln!("Ignored session cache file reading error: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        }
    };

    let mut tried = false;
    loop {
        if session.is_none() {
            let (ses, _) = start_auth(&conf.info.id, &conf.cookie_file)?;
            session.replace(ses);
        }

        if let Err(e) = start_app(session.as_ref().unwrap()) {
            // Handle errors
            match e {
                error::Error::AuthExpired => {
                    if tried {
                        return Err(error::Error::Auth(
                            "Maximum auth retry number reached.".into(),
                        ));
                    }
                    session.take();
                    eprintln!("Auth may expired, trying to reauthorize.")
                }
                _ => return Err(e),
            }
        } else {
            break;
        }
        tried = true;
    }
    Ok(())
}

/// Authorization procedure
fn start_auth(
    id: &str,
    path: &Option<String>,
) -> Result<(String, req::auth::UserInfo), error::Error> {
    let client = req::init_default_client()?;
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
}

/// App procedure
fn start_app(session: &str) -> Result<(), error::Error> {
    // Init authorized handler
    let handler = req::Handler::new(session)?;

    // Query Bind Info
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
    Ok(())
}
