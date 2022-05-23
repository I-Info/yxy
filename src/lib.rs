use std::error::Error;

pub mod arg;
pub mod conf;
pub(crate) mod error;
pub mod req;
pub mod utils;

/// Entrance of the main application
pub fn run(conf: conf::Config, opts: arg::Options) -> Result<(), Box<dyn Error>> {
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

    if session.is_none() {
        let (ses, _) = start_auth(&conf.info.id, &conf.cookie_file)?;
        session.replace(ses);
    }

    // Query Bind Info
    println!("Querying bind info...");
    let handler = req::Handler::new(session.as_ref().unwrap())?;
    let bind_info = handler.query_bind()?;
    println!("Bind info: {:?}", bind_info);

    Ok(())
}

/// Authorization procedure
fn start_auth(id: &str, path: &Option<String>) -> Result<(String, req::UserInfo), Box<dyn Error>> {
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
