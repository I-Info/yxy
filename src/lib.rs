use std::error::Error;

pub mod conf;
pub(crate) mod error;
pub mod flags;
pub mod req;
pub mod utils;

/// Entrance of the main application
pub fn run(conf: conf::Config) -> Result<(), Box<dyn Error>> {
    let mut handler = req::Handler::new()?; // Global handler

    println!("Trying to get oauth code...");
    let oauth_code = handler.get_oauth_code(&conf.info.id)?;
    println!("OAuth Code: {}", oauth_code);

    println!("Trying to login...");
    handler.authorize(&oauth_code)?;
    println!("Authorized: {}", handler.session.unwrap());

    Ok(())
}
