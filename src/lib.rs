use std::error::Error;

pub mod arg;
pub mod conf;
pub(crate) mod error;
pub mod req;
pub mod utils;

/// Entrance of the main application
pub fn run(conf: conf::Config, opts: arg::Options) -> Result<(), Box<dyn Error>> {
    let session = match &conf.cookie_file {
        None => None,
        Some(cookie_file) => {
            if opts.fresh == false {
                match std::fs::read_to_string(cookie_file) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        eprintln!("Ignored cookie cache file reading error: {}", e);
                        None
                    }
                }
            } else {
                None
            }
        }
    };

    if let None = session {
        let mut handler = req::Handler::new()?; // Global handler

        println!("Trying to get oauth code...");
        let oauth_code = handler.get_oauth_code(&conf.info.id)?;
        println!("OAuth Code: {}", oauth_code);

        println!("Trying to login...");
        let (session, user) = handler.authorize(&oauth_code)?;
        println!("Authorized, the session id is: {}", session);
        if let Some(cookie_file) = &conf.cookie_file {
            if let Err(e) = utils::file_write(cookie_file, &session) {
                eprintln!("Fail to cache the session id: {}", e);
            }
        }
        println!("Logged in as: {:?}", user);
    } else if let Some(session) = &session {
        println!("Using cached session id: {}", session)
    }

    Ok(())
}
